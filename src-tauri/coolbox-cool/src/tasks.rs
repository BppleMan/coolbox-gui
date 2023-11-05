use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub use check_task::*;
pub use command_task::*;
pub use compress_task::*;
use coolbox_macros::TaskRef;
pub use copy_task::*;
pub use decompress_task::*;
pub use delete_task::*;
pub use download_task::*;
pub use exists_task::*;
pub use git_task::*;
pub use install_task::*;
pub use move_task::*;
pub use uninstall_task::*;
pub use which_task::*;

use crate::installer::Installer;
use crate::result::CoolResult;
use crate::shell::Shell;
use crate::state::StateAble;

mod check_task;
mod command_task;
mod compress_task;
mod copy_task;
mod decompress_task;
mod delete_task;
mod download_task;
mod exists_task;
mod git_task;
mod install_task;
mod move_task;
mod uninstall_task;
mod which_task;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ExecutableState {
    #[default]
    NotStarted,
    Running,
    Finished,
    Error,
}

pub trait Executable: StateAble + Display {
    fn execute(&mut self) -> CoolResult<()> {
        *self.current_state() = ExecutableState::Running;
        match self._run() {
            Ok(_) => {
                *self.current_state() = ExecutableState::Finished;
                Ok(())
            }
            Err(e) => {
                *self.current_state() = ExecutableState::Error;
                Err(e)
            }
        }
    }

    fn _run(&mut self) -> CoolResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TaskRef)]
pub enum Task {
    CheckTask(CheckTask),
    CommandTask(CommandTask),
    CompressTask(CompressTask),
    CopyTaskTask(CopyTask),
    DecompressTask(DecompressTask),
    DeleteTask(DeleteTask),
    DownloadTask(DownloadTask),
    ExistsTask(ExistsTask),
    GitTask(GitTask),
    InstallTask(InstallTask),
    MoveTask(MoveTask),
    UninstallTask(UninstallTask),
    WhichTask(WhichTask),
}

impl Task {
    pub fn command(
        script: impl Into<String>,
        args: Option<Vec<impl Into<String>>>,
        envs: Option<Vec<(impl Into<String>, impl Into<String>)>>,
        shell: Shell,
    ) -> Self {
        Self::CommandTask(CommandTask::new(
            script.into(),
            args.map(|args| args.into_iter().map(|arg| arg.into()).collect::<Vec<_>>()),
            envs.map(|envs| {
                envs.into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect::<Vec<_>>()
            }),
            shell,
        ))
    }

    pub fn compress(source: impl Into<String>, destination: impl Into<String>) -> Self {
        Self::CompressTask(CompressTask::new(source.into(), destination.into()))
    }

    pub fn copy_task(source: impl Into<String>, destination: impl Into<String>) -> Self {
        Self::CopyTaskTask(CopyTask::new(source.into(), destination.into()))
    }

    pub fn decompress(source: impl Into<String>, destination: impl Into<String>) -> Self {
        Self::DecompressTask(DecompressTask::new(source.into(), destination.into()))
    }

    pub fn delete(path: impl Into<String>) -> Self {
        Self::DeleteTask(DeleteTask::new(path.into()))
    }

    pub fn download(url: impl Into<String>, destination: impl Into<String>) -> Self {
        Self::DownloadTask(DownloadTask::new(url.into(), destination.into()))
    }

    pub fn exists(path: impl Into<String>) -> Self {
        Self::ExistsTask(ExistsTask::new(path.into()))
    }

    pub fn git_clone(url: impl Into<String>, dest: impl Into<String>) -> Self {
        Self::GitTask(GitTask::new(GitCommand::Clone {
            url: url.into(),
            dest: dest.into(),
        }))
    }

    pub fn git_pull(src: impl Into<String>) -> Self {
        Self::GitTask(GitTask::new(GitCommand::Pull { src: src.into() }))
    }

    pub fn git_checkout(src: impl Into<String>, branch: impl Into<String>, create: bool) -> Self {
        Self::GitTask(GitTask::new(GitCommand::Checkout {
            src: src.into(),
            branch: branch.into(),
            create,
        }))
    }

    pub fn install(
        name: impl Into<String>,
        args: Option<Vec<impl Into<String>>>,
        installer: Installer,
    ) -> Self {
        Self::InstallTask(InstallTask::new(
            name.into(),
            args.map(|args| args.into_iter().map(|arg| arg.into()).collect::<Vec<_>>()),
            installer,
        ))
    }

    pub fn move_task(source: impl Into<String>, destination: impl Into<String>) -> Self {
        Self::MoveTask(MoveTask::new(source.into(), destination.into()))
    }

    pub fn uninstall_task(
        name: impl Into<String>,
        args: Option<Vec<impl Into<String>>>,
        installer: Installer,
    ) -> Self {
        Self::UninstallTask(UninstallTask::new(
            name.into(),
            args.map(|args| args.into_iter().map(|arg| arg.into()).collect::<Vec<_>>()),
            installer,
        ))
    }

    pub fn which(name: impl Into<String>) -> Self {
        Self::WhichTask(WhichTask::new(name.into()))
    }
}

impl StateAble for Task {
    fn current_state(&mut self) -> &mut ExecutableState {
        self.as_mut().current_state()
    }

    fn outputs(&mut self) -> &mut Vec<String> {
        self.as_mut().outputs()
    }

    fn errors(&mut self) -> &mut Vec<String> {
        self.as_mut().errors()
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl Executable for Task {
    fn _run(&mut self) -> CoolResult<()> {
        self.as_mut()._run()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tasks(pub Vec<Task>);

impl Tasks {
    pub fn execute(&mut self) -> CoolResult<Vec<Vec<String>>> {
        self.0.iter_mut().try_fold(Vec::new(), |mut results, task| {
            task.as_mut().execute()?;
            results.push(task.outputs().clone());
            Ok(results)
        })
    }
}
