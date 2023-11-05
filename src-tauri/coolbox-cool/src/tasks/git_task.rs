use std::fmt::{Display, Formatter};
use std::path::Path;

use color_eyre::eyre::eyre;
use git2::build::RepoBuilder;
use git2::{BranchType, Direction, FetchOptions, ProxyOptions, Repository};
use proxyconfig::{ProxyConfig, ProxyConfigProvider};
use serde::{Deserialize, Serialize};
use tracing::info;

use coolbox_macros::State;

use crate::result::CoolResult;
use crate::tasks::{Executable, ExecutableState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct GitTask {
    pub command: GitCommand,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl GitTask {
    pub fn new(command: GitCommand) -> Self {
        Self {
            command,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }

    pub fn clone(&mut self, url: &str, dest: &str) -> CoolResult<()> {
        let mut repo_builder = RepoBuilder::new();
        repo_builder.fetch_options(default_fetch_options());
        repo_builder.clone(url, Path::new(dest))?;
        Ok(())
    }

    pub fn pull(&mut self, src: &str) -> CoolResult<()> {
        let repo = Repository::open(src)?;
        let remotes = repo.remotes()?;
        let remote = match remotes.iter().find(|r| r == &Some("origin")) {
            Some(o) => o.unwrap(),
            None => match remotes.iter().find(|r| r.is_some()) {
                None => {
                    return Err(eyre!("no remote found"));
                }
                Some(o) => o.unwrap(),
            },
        };
        let mut remote = repo.find_remote(remote)?;
        remote.fetch::<&str>(&[], Some(&mut default_fetch_options()), None)?;
        let head = match repo.head() {
            Ok(head) => head,
            Err(_) => {
                self.checkout_empty(&repo, remote.name().unwrap())?;
                repo.head()?
            }
        };
        let head_branch = repo.find_branch(head.shorthand().unwrap(), BranchType::Local)?;
        let head_commit = head_branch.get().peel_to_commit()?;
        let remote_branch = head_branch.upstream()?;
        let remote_commit = remote_branch.get().peel_to_commit()?;
        if repo.graph_descendant_of(remote_commit.id(), head_commit.id())? {
            repo.set_head_detached(remote_commit.id())?;
            repo.checkout_head(None)?;
        } else if remote_commit.id() != head_commit.id() {
            let err = format!(
                "rebase {:?}[{}] onto {:?}[{}] cannot fast-forward",
                head_branch.name()?,
                head_commit.id(),
                remote_branch.name()?,
                remote_commit.id(),
            );
            self.errors.push(err.clone());
        } else {
            let msg = "already up to date".to_string();
            self.outputs.push(msg.clone());
        }
        Ok(())
    }

    pub fn checkout(&mut self, src: &str, branch: &str, create: bool) -> CoolResult<()> {
        let repo = Repository::open(src)?;
        let branch = match repo.find_branch(branch, BranchType::Local) {
            Ok(branch) => Ok(branch),
            Err(e) => {
                if create {
                    let head = repo.head()?;
                    let head_commit = head.peel_to_commit()?;
                    Ok(repo.branch(branch, &head_commit, true)?)
                } else {
                    Err(eyre!(e))
                }
            }
        }?;
        let commit = branch.get().peel_to_commit()?;
        repo.set_head_detached(commit.id())?;
        repo.set_head(branch.get().name().unwrap())?;
        repo.checkout_head(None)?;
        Ok(())
    }

    fn checkout_empty(&self, repo: &Repository, remote: &str) -> CoolResult<()> {
        let mut remote = repo.find_remote(remote)?;
        remote.connect(Direction::Fetch)?;
        let default_branch = String::from_utf8(remote.default_branch()?.to_vec())?;
        let short_name = Path::new(&default_branch)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        if let Some(reference) = repo.references()?.flatten().find(|r| {
            r.name()
                .map(|name| r.is_remote() && name.contains(&short_name))
                .unwrap_or(false)
        }) {
            let commit = reference.peel_to_commit()?;
            let mut main = repo.branch(&short_name, &commit, false)?;
            main.set_upstream(Some(reference.shorthand().unwrap()))?;
            repo.checkout_head(None)?;
        }
        Ok(())
    }
}

impl Display for GitTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.command {
            GitCommand::Clone { url, dest } => {
                write!(f, "git clone {} {}", url, dest)
            }
            GitCommand::Pull { src } => {
                write!(f, "git -C {} pull", src)
            }
            GitCommand::Checkout {
                src,
                branch,
                create,
            } => {
                if *create {
                    write!(f, "git -C {} checkout -b {}", src, branch)
                } else {
                    write!(f, "git -C {} checkout {}", src, branch)
                }
            }
        }
    }
}

impl Executable for GitTask {
    fn _run(&mut self) -> CoolResult<()> {
        match self.command.clone() {
            GitCommand::Clone { .. } => {}
            GitCommand::Pull { src } => self.pull(&src)?,
            GitCommand::Checkout {
                src,
                branch,
                create,
            } => self.checkout(&src, &branch, create)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GitCommand {
    Clone {
        url: String,
        dest: String,
    },
    Pull {
        src: String,
    },
    Checkout {
        src: String,
        branch: String,
        create: bool,
    },
}

fn default_fetch_options<'fo>() -> FetchOptions<'fo> {
    let mut fetch_options = FetchOptions::new();
    fetch_options.proxy_options(default_proxy_options());
    fetch_options
}

fn default_proxy_options<'po>() -> ProxyOptions<'po> {
    let mut proxy_options = ProxyOptions::new();
    if let Ok(proxy_config) = ProxyConfig::try_get() {
        let mut proxy = proxy_config.proxies.socks_proxy.as_ref();
        if proxy_config.proxies.http_proxy.is_some() {
            proxy = proxy_config.proxies.http_proxy.as_ref();
        } else if proxy_config.proxies.https_proxy.is_some() {
            proxy = proxy_config.proxies.https_proxy.as_ref();
        }
        if let Some(proxy) = proxy {
            info!("proxy: {}", proxy.host);
            proxy_options.url(format!("http://{}:{}", proxy.host, proxy.port).as_str());
        } else {
            info!("no proxy");
            proxy_options.auto();
        }
    } else {
        info!("no proxy_config");
        proxy_options.auto();
    }
    proxy_options
}

#[cfg(test)]
mod test {
    use std::process::{Command, Stdio};

    use git2::Repository;

    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::{Executable, GitCommand, GitTask};

    #[test]
    fn test_pull() -> CoolResult<()> {
        init_backtrace();
        let base_dir = tempfile::Builder::new()
            .prefix("cool")
            .suffix("git")
            .tempdir()?;
        println!("{:?}", base_dir);
        let test_repo = base_dir.path().join("test_repo");
        fs_extra::dir::create(&test_repo, true)?;
        let output = Command::new("bash")
            .arg("-c")
            .arg(
                "git init -b main
git remote add origin https://bppleman:4875c8b28457f4b7c2535eb12bbc66d6@gitee.com/bppleman/proxy_config.git
git fetch
git remote -v
            ",
            )
            .current_dir(&test_repo)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()?;
        println!("{}", String::from_utf8(output.stdout)?);

        GitTask::new(GitCommand::Pull {
            src: test_repo.to_string_lossy().to_string(),
        })
        .execute()?;
        println!("12");
        Ok(())
    }

    #[test]
    fn test_checkout() -> CoolResult<()> {
        init_backtrace();
        let base_dir = tempfile::Builder::new()
            .prefix("cool")
            .suffix("git")
            .tempdir()?;
        let checkout_dir = base_dir.path().join("checkout");
        fs_extra::dir::create(&checkout_dir, true)?;

        let output = Command::new("bash")
            .arg("-c")
            .arg(
                "\
git init -b main
touch README.md
git add README.md
git commit -m 'init'
            ",
            )
            .current_dir(&checkout_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()?;
        println!("{}", String::from_utf8(output.stdout)?);

        GitTask::new(GitCommand::Checkout {
            src: checkout_dir.to_string_lossy().to_string(),
            branch: "dev".to_string(),
            create: true,
        })
        .execute()?;

        let repo = Repository::open(&checkout_dir)?;
        assert!(repo.find_branch("dev", git2::BranchType::Local).is_ok());
        pretty_assertions::assert_eq!(repo.head()?.shorthand(), Some("dev"));

        GitTask::new(GitCommand::Checkout {
            src: checkout_dir.to_string_lossy().to_string(),
            branch: "main".to_string(),
            create: false,
        })
        .execute()?;
        pretty_assertions::assert_eq!(repo.head()?.shorthand(), Some("main"));
        Ok(())
    }
}
