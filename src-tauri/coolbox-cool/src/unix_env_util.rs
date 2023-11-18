use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::env_util::EnvVar;
use color_eyre::eyre::Context;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::error::StorageError;
use crate::login_shell::LoginShell;
use crate::result::CoolResult;
use crate::shell::ShellExecutor;
use crate::StringExt;

static PATH_ENV_TEMPLATE: &str = r#"
# ===start===
# {{key}}={{value}}
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

static FILTER_ENV: Lazy<HashSet<&'static str>> =
    Lazy::new(|| HashSet::from(["LOGNAME", "_", "SHLVL", "PWD", "OLDPWD"]));

pub struct EnvUtil {
    pub cool_profile: CoolProfile,
}

impl EnvUtil {
    pub fn new(cool_profile: CoolProfile) -> Self {
        Self { cool_profile }
    }

    pub fn envs(&self) -> Vec<EnvVar> {
        std::env::vars()
            .map(|(k, v)| EnvVar { key: k, value: v })
            .collect()
    }

    pub fn login_shell_envs(
        &self,
        login_shell: &LoginShell,
        inherit_env: bool,
    ) -> CoolResult<Vec<EnvVar>> {
        self.profile_envs(
            login_shell,
            login_shell.user_profile.display().to_string().as_str(),
            inherit_env,
        )
    }

    pub fn profile_envs<'a>(
        &self,
        login_shell: &LoginShell,
        profile: impl Into<&'a str>,
        inherit_env: bool,
    ) -> CoolResult<Vec<EnvVar>> {
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

    pub fn source<'a>(
        &self,
        login_shell: &LoginShell,
        profile: impl Into<&'a str>,
    ) -> CoolResult<()> {
        let envs = self.profile_envs(login_shell, profile, false)?;
        envs.iter()
            .for_each(|EnvVar { key, value }| std::env::set_var(key, value));
        Ok(())
    }

    pub fn export(&mut self, env_var: impl Into<EnvVar>) {
        let env_var = env_var.into();
        self.cool_profile.append(env_var.clone()).unwrap();
        std::env::set_var(env_var.key, env_var.value);
    }

    pub fn unset(&mut self, key: impl AsRef<str>) -> CoolResult<()> {
        self.cool_profile.remove(key.as_ref()).unwrap();
        std::env::remove_var(key.as_ref());
        Ok(())
    }
}

pub struct CoolProfile {
    path: PathBuf,
    envs: HashMap<String, EnvVar>,
}

impl CoolProfile {
    pub fn new(path: PathBuf) -> CoolResult<Self, StorageError> {
        let content = fs_extra::file::read_to_string(&path)
            .with_context(|| format!("path: {}", path.display()))
            .map_err(StorageError::FsExtraError)?;
        let envs = Self::parse(&content);
        Ok(Self { path, envs })
    }

    fn parse(content: &str) -> HashMap<String, EnvVar> {
        let regex = Regex::new(r#"# ===start===((.|\s)*?)# ===end==="#).unwrap();
        let regex2 = Regex::new(r#"#\s[^=^\s]+?=[^=^\s]+"#).unwrap();
        regex
            .find_iter(content)
            .flat_map(|m| {
                let item = m.as_str();
                let value = regex2
                    .find(item)
                    .unwrap()
                    .as_str()
                    .trim()
                    .trim_start_matches("# ");
                EnvVar::try_from(value).map(|e| (e.key.clone(), e))
            })
            .collect::<HashMap<_, _>>()
    }

    pub fn append(&mut self, env_var: impl Into<EnvVar>) -> CoolResult<()> {
        let env_var = env_var.into();
        self.envs.insert(env_var.key.clone(), env_var);
        self.write()?;
        Ok(())
    }

    pub fn remove(&mut self, env_key: impl AsRef<str>) -> CoolResult<()> {
        self.envs.remove(env_key.as_ref());
        self.write()?;
        Ok(())
    }

    pub fn append_path(&mut self, path: impl AsRef<str>) -> CoolResult<()> {
        let path = path.as_ref();
        let key = "PATH";
        let value = self.envs.get(key).map(|e| e.value.as_str()).unwrap_or("");
        let value = format!("{}:{}", path, value);
        self.append((key, value))
    }

    pub fn write(&self) -> CoolResult<()> {
        fs_extra::file::write_all(
            &self.path,
            self.envs
                .values()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n")
                .as_str(),
        )?;
        Ok(())
    }
}

impl Display for EnvVar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content = EXPORT_ENV_TEMPLATE
            .render(
                &tera::Context::from_value(serde_json::to_value(self).unwrap()).unwrap(),
                false,
            )
            .unwrap();
        write!(f, "{}", content.trim_start())
    }
}

#[cfg(test)]
mod test {
    use std::process::Command;

    use crate::init_backtrace;
    use crate::local_storage::LocalStorage;
    use crate::login_shell::LoginShell;
    use crate::result::CoolResult;
    use crate::unix_env_util::EnvUtil;

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
        let mut cool_profile = LocalStorage.cool_profile()?;
        cool_profile.append(("COOL_HOME", "/Users/bppleman/coolbox"))?;
        cool_profile.append(("JAVA_HOME", "This is JAVA Home"))?;
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
