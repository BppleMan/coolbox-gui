use thiserror::Error;

use crate::error::{InnerError, ShellError};
use crate::tasks::{
    CheckTask, CommandTask, CompressTask, CopyTask, DecompressTask, DeleteTask, DownloadTask,
    EnvTask, GitTask, InstallTask, MoveTask, UninstallTask, WhichTask,
};

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Check task error: [{task}] with [{source}]")]
    CheckTaskError {
        task: CheckTask,
        #[source]
        source: CheckTaskError,
    },

    #[error("Command task error: [{task}] with [{source}]")]
    CommandTaskError {
        task: CommandTask,
        #[source]
        source: CommandTaskError,
    },

    #[error("Compress task error: [{task}] with [{source}]")]
    CompressTaskError {
        task: CompressTask,
        #[source]
        source: CompressTaskError,
    },

    #[error("Copy task error: [{task}] with [{source}]")]
    CopyTaskError {
        task: CopyTask,
        source: CopyTaskError,
    },

    #[error("Decompress task error: [{task}] with [{source}]")]
    DecompressTaskError {
        task: DecompressTask,
        #[source]
        source: CompressTaskError,
    },

    #[error("Delete task error: [{task}] with [{source}]")]
    DeleteTaskError {
        task: DeleteTask,
        source: DeleteTaskError,
    },

    #[error("Download task error: [{task}] with [{source}]")]
    DownloadTaskError {
        task: DownloadTask,
        source: DownloadTaskError,
    },

    #[error("Env task error: [{task}] with [{source}]")]
    EnvTaskError {
        task: EnvTask,
        #[source]
        source: EnvTaskError,
    },

    #[error("Exists task error: [{0}] not exists")]
    ExistsTaskError(String),

    #[error("Git task error: [{task}] with [{source}]")]
    GitTaskError {
        task: GitTask,
        #[source]
        source: GitTaskError,
    },

    #[error("Install task error: [{task}] with [{source}]")]
    InstallTaskError {
        task: InstallTask,
        #[source]
        source: InstallTaskError,
    },

    #[error("Move task error: [{task}] with [{source}]")]
    MoveTaskError {
        task: MoveTask,
        source: MoveTaskError,
    },

    #[error("Uninstall task error: [{task}] with [{source}]")]
    UninstallTaskError {
        task: UninstallTask,
        #[source]
        source: InstallTaskError,
    },

    #[error("Which task error: [{task}] with [{source}]")]
    WhichTaskError {
        task: WhichTask,
        #[source]
        source: which::Error,
    },
}

#[derive(Debug, Error)]
pub enum CheckTaskError {
    #[error("Check [{name}] with [{installer}] is not available")]
    NotAvailable { name: String, installer: String },

    #[error(transparent)]
    ShellError(#[from] ShellError),
}

#[derive(Debug, Error)]
pub enum CommandTaskError {
    #[error(transparent)]
    ShellError(#[from] ShellError),
}

#[derive(Debug, Error)]
pub enum CompressTaskError {
    #[error("Source: [{0}] not found parent")]
    SourceNoParent(String),

    #[error("Dest: [{0}] not found parent")]
    DestNoParent(String),

    #[error("Dest: [{0}] is file")]
    DestIsFile(String),

    #[error("Unsupported compress type: [{0}]")]
    UnsupportedCompressType(String),

    #[error(transparent)]
    InnerError(#[from] InnerError),
}

#[derive(Debug, Error)]
pub enum CopyTaskError {
    #[error(transparent)]
    InnerError(#[from] InnerError),
}

#[derive(Debug, Error)]
pub enum DeleteTaskError {
    #[error(transparent)]
    InnerError(#[from] InnerError),
}

#[derive(Debug, Error)]
pub enum DownloadTaskError {
    #[error(transparent)]
    InnerError(#[from] InnerError),
}

#[derive(Debug, Error)]
pub enum EnvTaskError {
    #[error(transparent)]
    ShellError(#[from] ShellError),

    #[error(transparent)]
    InnerError(#[from] InnerError),
}

#[derive(Debug, Error)]
pub enum GitTaskError {
    #[error(transparent)]
    GitError(#[from] git2::Error),

    #[error("Remote not found")]
    NotFoundRemote,

    #[error("Rebase {head_branch_name:?}:{head_commit_id} onto {remote_branch_name:?}:{remote_commit_id} cannot fast-forward")]
    CannotFastForward {
        head_branch_name: Option<String>,
        head_commit_id: String,
        remote_branch_name: Option<String>,
        remote_commit_id: String,
    },

    #[error(transparent)]
    OtherError(InnerError),
}

#[derive(Debug, Error)]
pub enum InstallTaskError {
    #[error(transparent)]
    ShellError(#[from] ShellError),
}

#[derive(Debug, Error)]
pub enum MoveTaskError {
    #[error(transparent)]
    InnerError(#[from] InnerError),
}
