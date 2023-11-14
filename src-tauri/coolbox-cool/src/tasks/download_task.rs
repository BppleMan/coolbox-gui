use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use color_eyre::eyre::{eyre, Context};
use futures::stream::StreamExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::result::ExecutableResult;
use crate::tasks::{Executable, MessageSender};
use crate::IntoInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DownloadTask {
    pub url: String,
    #[serde(deserialize_with = "crate::template_string")]
    pub dest: String,
}

impl DownloadTask {
    pub fn new(url: String, dest: String) -> Self {
        Self { url, dest }
    }
}

impl Display for DownloadTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "curl -L {} -o {}", self.url, self.dest)
    }
}

impl<'a> Executable<'a> for DownloadTask {
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> ExecutableResult {
        let url = self.url.clone();
        let dest = PathBuf::from(&self.dest);
        let (tx, rx) = crossbeam::channel::bounded(1);
        let (msg_tx, msg_rx) = crossbeam::channel::unbounded();
        rayon::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let result = rt.block_on(async {
                let res = reqwest::get(&url)
                    .await
                    .map_err(|e| ExecutableError::ReqwestError(eyre!(e)))?;
                let mut written = 0u64;
                let total_size = res.content_length();
                let mut bytes_stream = res.bytes_stream();
                if let Some(parent) = dest.parent() {
                    fs_extra::dir::create_all(parent, true)
                        .with_context(|| format!("failed to create {}", parent.display()))
                        .map_err(ExecutableError::CreatePathError)?;
                }
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(&dest)
                    .with_context(|| format!("failed to open {}", dest.display()))
                    .map_err(ExecutableError::CreatePathError)?;
                while let Some(Ok(chunk)) = bytes_stream.next().await {
                    written += chunk.len() as u64;
                    file.write_all(&chunk).unwrap();
                    msg_tx
                        .send(
                            format!("downloaded {}/{}", written, total_size.unwrap_or(0))
                                .into_info(),
                        )
                        .unwrap();
                }
                ExecutableResult::Ok(())
            });
            tx.send(result).unwrap();
        });
        while let Ok(msg) = msg_rx.recv() {
            send(msg);
        }
        rx.recv().unwrap()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tempfile::{Builder, NamedTempFile};

    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::{spawn_task, DownloadTask};

    #[test]
    fn smoke() -> CoolResult<()> {
        init_backtrace();
        let mut server = mockito::Server::new();
        let url = server.url();

        // download server
        let mock = server
            .mock("GET", "/download")
            .with_status(200)
            .with_header("content-type", "binary/octet-stream")
            .with_body([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
            .create();

        let base_dir = Builder::new().prefix("cool").suffix("download").tempdir()?;
        let path = NamedTempFile::new_in(base_dir.path())?;
        let download = DownloadTask::new(
            format!("{}/download", url),
            path.path().display().to_string(),
        );
        spawn_task(download, |_| {})?;
        mock.assert();
        assert!(path.path().exists());
        Ok(())
    }
}
