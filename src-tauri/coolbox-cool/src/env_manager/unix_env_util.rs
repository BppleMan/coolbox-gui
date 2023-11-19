use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::env_manager::{EnvLevel, EnvVariable};
use color_eyre::eyre::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::error::{EnvError, StorageError};
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
    Lazy::new(|| HashSet::from(["LOGNAME", "_", "SHLVL", "PWD", "OLDPWD"]));

pub struct EnvUtil {
    pub cool_profile: CoolProfile,
}

impl EnvUtil {
    pub fn new(cool_profile: CoolProfile) -> Self {
        Self { cool_profile }
    }

    pub fn login_shell_envs(
        &self,
        login_shell: &LoginShell,
        inherit_env: bool,
    ) -> CoolResult<Vec<EnvVariable>, EnvError> {
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
    ) -> CoolResult<Vec<EnvVariable>, EnvError> {
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
            .flat_map(|m| EnvVariable::try_from(m.message.as_str()))
            .filter(|e| !FILTER_ENV.contains(e.key.as_str()))
            .collect())
    }

    pub fn source<'a>(
        &self,
        login_shell: &LoginShell,
        profile: impl Into<&'a str>,
    ) -> CoolResult<(), EnvError> {
        todo!();
        // let envs = self.profile_envs(login_shell, profile, false)?;
        // envs.iter()
        //     .for_each(|EnvItem { key, value }| std::env::set_var(key, value));
        // Ok(())
    }
}

impl super::EnvManagerBackend for EnvUtil {
    fn export(
        &mut self,
        env_var: impl Into<EnvVariable>,
        level: EnvLevel,
    ) -> CoolResult<(), EnvError> {
        todo!()
    }

    fn unset(&mut self, key: impl AsRef<str>, level: EnvLevel) -> CoolResult<(), EnvError> {
        todo!()
    }

    fn append_path(&mut self, value: impl AsRef<str>, level: EnvLevel) -> CoolResult<(), EnvError> {
        todo!()
    }

    fn remove_path(&mut self, value: impl AsRef<str>, level: EnvLevel) -> CoolResult<(), EnvError> {
        todo!()
    }

    fn add_source(&mut self, value: impl AsRef<str>, level: EnvLevel) -> CoolResult<(), EnvError> {
        todo!()
    }

    fn remove_source(
        &mut self,
        value: impl AsRef<str>,
        level: EnvLevel,
    ) -> CoolResult<(), EnvError> {
        todo!()
    }
}

pub struct CoolProfile {
    profile_path: PathBuf,
    path: Vec<String>,
    envs: HashMap<String, EnvVariable>,
    source_profiles: HashSet<String>,
}

impl CoolProfile {
    pub fn new(path: PathBuf) -> CoolResult<Self, StorageError> {
        let content = fs_extra::file::read_to_string(&path)
            .with_context(|| format!("path: {}", path.display()))
            .map_err(StorageError::FsExtraError)?;
        let envs = Self::parse(&content);
        Ok(Self {
            profile_path: path,
            envs,
        })
    }

    fn parse(content: &str) -> HashMap<String, EnvVariable> {
        todo!();
        // let export = r#"#\s[^=\s]+?=[^=\s]+"#;
        // let export_block = format!(r#"# ===start===${}$((.|\s)*?)# ===end===$"#, export);
        // let export_block_regex = regex::RegexBuilder::new(&export_block)
        //     .crlf(true)
        //     .multi_line(true)
        //     .build()
        //     .unwrap();
        // let export_regex = regex::RegexBuilder::new(r#"#\s[^=\s]+?=[^=\s]+"#)
        //     .multi_line(true)
        //     .build()
        //     .unwrap();
        // let source_regex = regex::RegexBuilder::new(r#"#\s[^=\s]+$"#)
        //     .multi_line(true)
        //     .build()
        //     .unwrap();
        // export_block_regex
        //     .find_iter(content)
        //     .filter_map(|m| {
        //         let item = m.as_str();
        //         if export_regex.is_match(item) {
        //             let value = export_regex
        //                 .find(item)
        //                 .unwrap()
        //                 .as_str()
        //                 .trim()
        //                 .trim_start_matches("# ");
        //             Some(EnvItem::try_from(value).map(|e| (e.key.clone(), e)))
        //         } else {
        //             None
        //         }
        //     })
        //     .flatten()
        //     .collect::<HashMap<_, _>>()
    }

    pub fn append(&mut self, env_var: impl Into<EnvVariable>) -> CoolResult<()> {
        todo!();
        // let env_var = env_var.into();
        // self.envs.insert(env_var.key.clone(), env_var);
        // self.write()?;
        // Ok(())
    }

    pub fn remove(&mut self, env_key: impl AsRef<str>) -> CoolResult<()> {
        self.envs.remove(env_key.as_ref());
        self.write()?;
        Ok(())
    }

    pub fn append_path(&mut self, path: impl AsRef<str>) -> CoolResult<()> {
        todo!();
        // let path = path.as_ref();
        // let key = "PATH";
        // let value = self.envs.get(key).map(|e| e.value.as_str()).unwrap_or("");
        // let value = format!("{}:{}", path, value);
        // self.append((key, value))
    }

    pub fn write(&self) -> CoolResult<()> {
        fs_extra::file::write_all(
            &self.profile_path,
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

impl Display for EnvVariable {
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
    use crate::env_manager::unix_env_util::{
        EXPORT_ENV_TEMPLATE, PATH_ENV_TEMPLATE, SOURCE_PROFILE_TEMPLATE,
    };
    use std::process::Command;

    use crate::init_backtrace;
    use crate::local_storage::LocalStorage;
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
