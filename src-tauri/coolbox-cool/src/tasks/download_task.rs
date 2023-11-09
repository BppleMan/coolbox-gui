use color_eyre::eyre::eyre;
use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::Write;

use crate::error::ExecutableError;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::result::ExecutableResult;
use crate::tasks::{Executable, ExecutableSender};
use crate::IntoInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DownloadTask {
    pub url: String,
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

impl Executable for DownloadTask {
    fn _run(&self, sender: &ExecutableSender) -> ExecutableResult {
        let url = self.url.clone();
        let dest = self.dest.clone();
        let sender = sender.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let res = reqwest::get(&url)
                    .await
                    .map_err(|e| ExecutableError::ReqwestError(eyre!(e)))?;
                let mut written = 0u64;
                let total_size = res.content_length();
                let mut bytes_stream = res.bytes_stream();
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&dest)
                    .unwrap();
                while let Some(Ok(chunk)) = bytes_stream.next().await {
                    written += chunk.len() as u64;
                    file.write_all(&chunk).unwrap();
                    sender
                        .send(
                            format!("downloaded {}/{}", written, total_size.unwrap_or(0))
                                .into_info(),
                        )
                        .unwrap();
                }
                ExecutableResult::Ok(())
            })
        })
        .join()
        .unwrap()?;
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
