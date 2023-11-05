use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use coolbox_macros::State;

use crate::result::CoolResult;
use crate::tasks::{Executable, ExecutableState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct DownloadTask {
    pub url: String,
    pub dest: String,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl DownloadTask {
    pub fn new(url: String, dest: String) -> Self {
        Self {
            url,
            dest,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }
}

impl Display for DownloadTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "curl -L {} -o {}", self.url, self.dest)
    }
}

impl Executable for DownloadTask {
    fn _run(&mut self) -> CoolResult<()> {
        let mut bytes = reqwest::blocking::get(&self.url)?.bytes()?;
        std::fs::write(&self.dest, &mut bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tempfile::{Builder, NamedTempFile};

    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::{DownloadTask, Executable, ExecutableState};

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
        let mut download = DownloadTask::new(
            format!("{}/download", url),
            path.path().display().to_string(),
        );
        download.execute()?;
        mock.assert();
        pretty_assertions::assert_eq!(ExecutableState::Finished, download.state);
        assert!(path.path().exists());
        Ok(())
    }
}
