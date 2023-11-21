use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Mutex;

use color_eyre::eyre::Context;
use crossbeam::channel::internal::SelectHandle;
use lazy_static::lazy_static;
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::error;

use crate::env_manager::{EnvManagerBackend, EnvVar};
use crate::error::EnvError;
use crate::local_storage::LocalStorage;
use crate::login_shell::LoginShell;
use crate::result::CoolResult;
use crate::shell::ShellExecutor;
use crate::StringExt;

static PATH_ENV_TEMPLATE: &str = r#"
# ===start===
# @path={{value}}
case ":${PATH}:" in
    *:"{{value}}":*)
        ;;
    *)
        # Prepending path in case a system-installed rustc needs to be overridden
        export PATH="{{value}}:$PATH"
        ;;
esac
# ===end===
"#;

static EXPORT_ENV_TEMPLATE: &str = r#"
# ===start===
# {{key}}={{value}}
if [ -z "${% raw %}{{% endraw %}{{key}}}" ]; then
    export {{key}}="{{value}}"
fi
# ===end===
"#;

static SOURCE_PROFILE_TEMPLATE: &str = r#"
# ===start===
# @source={{value}}
source {{value}}
# ===end===
"#;

static FILTER_ENV: Lazy<HashSet<&'static str>> =
    Lazy::new(|| HashSet::from(["HOME", "LOGNAME", "_", "SHLVL", "PWD", "OLDPWD"]));

pub static COOL_PROFILE: Lazy<Mutex<ShellProfile>> =
    Lazy::new(|| Mutex::new(LocalStorage.cool_profile()));

lazy_static! {
    static ref BLOCK_REGEX: Regex =
        regex::RegexBuilder::new(r#"#\s===start===\s*\n((.|\s)*?)#\s===end==="#)
            .multi_line(true)
            .build()
            .unwrap();
    static ref AT_PATH_REGEX: Regex = regex::RegexBuilder::new(r#"#\s@path=[^=\s]+$"#)
        .multi_line(true)
        .build()
        .unwrap();
    static ref AT_SOURCE_REGEX: Regex = regex::RegexBuilder::new(r#"#\s@source=[^=\s]+$"#)
        .multi_line(true)
        .build()
        .unwrap();
    static ref EXPORT_REGEX: Regex = regex::RegexBuilder::new(r#"#\s[^=\s]+?=[^=\s]+$"#)
        .multi_line(true)
        .build()
        .unwrap();
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ShellProfile {
    profile_path: PathBuf,
    paths: BTreeSet<String>,
    env_vars: BTreeMap<String, EnvVar>,
    sources: BTreeSet<String>,
}

impl ShellProfile {
    pub fn new(path: PathBuf) -> Self {
        let content = fs_extra::file::read_to_string(&path).unwrap_or_else(|_| {
            panic!(
                "Failed to read file: {}",
                path.display().to_string().as_str()
            )
        });
        let (env_vars, paths, sources) = Self::parse(&content);
        Self {
            profile_path: path,
            paths,
            env_vars,
            sources,
        }
    }

    pub fn write(&self) {
        let env_var_content = self
            .env_vars
            .values()
            .map(render_env_var)
            .collect::<Vec<_>>();
        let source_content = self
            .sources
            .iter()
            .map(|s| render_source(s))
            .collect::<Vec<_>>();
        let path_content = self
            .paths
            .iter()
            .map(|p| render_path(p))
            .collect::<Vec<_>>();
        let content = [
            env_var_content.join("\n"),
            source_content.join("\n"),
            path_content.join("\n"),
        ]
        .join("\n");
        if let Some(parent) = self.profile_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).unwrap();
            }
        }
        if !self.profile_path.exists() {
            std::fs::File::create(&self.profile_path).unwrap();
        }
        fs_extra::file::write_all(&self.profile_path, &content).unwrap();
    }

    pub fn watch(
        this: &'static Mutex<Self>,
    ) -> CoolResult<crossbeam::channel::Receiver<DebounceEventResult>> {
        let (tx, rx) = crossbeam::channel::unbounded();
        let mut debouncer = new_debouncer(std::time::Duration::from_secs(2), None, tx)?;
        debouncer.watcher().watch(
            &this.lock().unwrap().profile_path,
            RecursiveMode::NonRecursive,
        )?;
        let cloned_rx = rx.clone();
        rayon::spawn(move || {
            while let Ok(events) = cloned_rx.recv() {
                match events {
                    Ok(events) => events.into_iter().for_each(|_event| {
                        let mut this = this.lock().unwrap();
                        if this.profile_path.exists() {
                            this.update();
                        } else {
                            this.write();
                        }
                    }),
                    Err(errors) => errors.iter().for_each(|err| error!("{}", err)),
                }
            }
        });
        Ok(rx)
    }

    pub fn update(&mut self) {
        let content = fs_extra::file::read_to_string(&self.profile_path).unwrap();
        let (env_vars, paths, sources) = Self::parse(&content);
        self.env_vars = env_vars;
        self.paths = paths;
        self.sources = sources;
    }
}

impl EnvManagerBackend for ShellProfile {
    fn export(&mut self, env_var: impl Into<EnvVar>) -> CoolResult<(), EnvError> {
        let env_var = env_var.into();
        self.env_vars.insert(env_var.key.clone(), env_var);
        self.write();
        Ok(())
    }

    fn unset(&mut self, key: impl AsRef<str>) -> CoolResult<(), EnvError> {
        self.env_vars.remove(key.as_ref());
        self.write();
        Ok(())
    }

    fn append_path(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError> {
        self.paths.insert(value.as_ref().to_string());
        self.write();
        Ok(())
    }

    fn remove_path(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError> {
        self.paths.remove(value.as_ref());
        self.write();
        Ok(())
    }

    fn add_source(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError> {
        self.sources.insert(value.as_ref().to_string());
        self.write();
        Ok(())
    }

    fn remove_source(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError> {
        self.sources.remove(value.as_ref());
        self.write();
        Ok(())
    }
}

impl ShellProfile {
    pub fn login_shell_envs(
        login_shell: &LoginShell,
        inherit_env: bool,
    ) -> CoolResult<Vec<EnvVar>, EnvError> {
        Self::profile_envs(
            login_shell,
            login_shell.user_profile.display().to_string().as_str(),
            inherit_env,
        )
    }

    pub fn profile_envs<'a>(
        login_shell: &LoginShell,
        profile: impl Into<&'a str>,
        inherit_env: bool,
    ) -> CoolResult<Vec<EnvVar>, EnvError> {
        let (tx, rx) = crossbeam::channel::unbounded();
        login_shell.shell.run(
            format!("source {} && env", profile.into()).as_str(),
            if inherit_env { Some(&[]) } else { None },
            Some(tx),
        )?;
        let mut outputs = vec![];
        while let Ok(message) = rx.recv() {
            outputs.push(message);
        }
        Ok(outputs
            .into_iter()
            .flat_map(|m| EnvVar::try_from(m.message.as_str()))
            .filter(|e| !FILTER_ENV.contains(e.key.as_str()))
            .collect())
    }

    fn parse(content: &str) -> (BTreeMap<String, EnvVar>, BTreeSet<String>, BTreeSet<String>) {
        BLOCK_REGEX.find_iter(content).fold(
            (
                BTreeMap::<String, EnvVar>::new(),
                BTreeSet::<String>::new(),
                BTreeSet::<String>::new(),
            ),
            |(mut vars, mut paths, mut sources), item| {
                let item = item.as_str();
                if AT_PATH_REGEX.is_match(item) {
                    let path = AT_PATH_REGEX
                        .find(item)
                        .unwrap()
                        .as_str()
                        .trim_start_matches("# @path=")
                        .to_string();
                    paths.insert(path);
                } else if AT_SOURCE_REGEX.is_match(item) {
                    let source = AT_SOURCE_REGEX
                        .find(item)
                        .unwrap()
                        .as_str()
                        .trim_start_matches("# @source=");
                    sources.insert(source.to_string());
                } else {
                    let export = EXPORT_REGEX
                        .find(item)
                        .unwrap()
                        .as_str()
                        .trim_start_matches("# ");
                    let env = EnvVar::try_from(export).unwrap();
                    vars.insert(env.key.clone(), env);
                }
                (vars, paths, sources)
            },
        )
    }
}

pub fn render_env_var(env_var: &EnvVar) -> String {
    let content = EXPORT_ENV_TEMPLATE
        .render(
            &tera::Context::from_value(serde_json::to_value(env_var).unwrap()).unwrap(),
            false,
        )
        .unwrap();
    content.trim_start().to_string()
}

pub fn render_path(path: &str) -> String {
    let content = PATH_ENV_TEMPLATE
        .render(
            &tera::Context::from_value(serde_json::json!({ "value": path })).unwrap(),
            false,
        )
        .unwrap();
    content.trim_start().to_string()
}

pub fn render_source(profile: &str) -> String {
    let content = SOURCE_PROFILE_TEMPLATE
        .render(
            &tera::Context::from_value(serde_json::json!({ "value": profile })).unwrap(),
            false,
        )
        .unwrap();
    content.trim_start().to_string()
}

#[cfg(test)]
mod test {
    use std::process::Command;

    use crate::env_manager::shell_profile::{
        EXPORT_ENV_TEMPLATE, PATH_ENV_TEMPLATE, SOURCE_PROFILE_TEMPLATE,
    };
    use crate::init_backtrace;
    use crate::login_shell::LoginShell;
    use crate::result::CoolResult;

    #[test]
    fn smoke() -> CoolResult<()> {
        init_backtrace();
        let login_shell = LoginShell::detect()?;
        // println!(
        // "{:#?}",
        // EnvUtil.profile_envs(&login_shell, "../coolrc", false)?
        // );
        Ok(())
    }

    #[test]
    fn test_regex() -> CoolResult<()> {
        init_backtrace();
        let content = vec![
            PATH_ENV_TEMPLATE,
            EXPORT_ENV_TEMPLATE,
            SOURCE_PROFILE_TEMPLATE,
        ]
        .join("\r");
        // println!("{}", content);
        let export = r#"#\s[^=\s]+?=[^=\s]+"#;
        let export_block = format!(r#"# ===start===\n{}\n((.|\s)*?)# ===end==="#, export);
        let export_block_regex = regex::RegexBuilder::new(&export_block)
            // .crlf(true)
            .multi_line(true)
            .build()
            .unwrap();
        export_block_regex.find_iter(&content).for_each(|m| {
            let item = m.as_str();
            println!("{}", item);
        });
        Ok(())
    }

    #[test]
    fn test_env() -> CoolResult<()> {
        std::thread::spawn(move || {
            std::env::set_var("COOL_HOME", "123456");
        })
        .join()
        .unwrap();
        let output = Command::new("zsh")
            .args(["-c", "echo $COOL_HOME"])
            .spawn()?
            .wait()?;
        Ok(())
    }
}
