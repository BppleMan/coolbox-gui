use std::hash::Hash;

use crate::CoolFormula;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Cool2 {
    pub name: String,
    pub identifier: String,
    pub description: String,
    #[serde(default)]
    pub macos: Option<Platform>,
    #[serde(default)]
    pub windows: Option<Platform>,
    #[serde(default)]
    pub linux: Option<Platform>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Platform {
    #[serde(default)]
    pub need_restart: bool,
    pub formula: CoolFormula,
}

// impl From<Cool> for Cool2 {
//     fn from(value: Cool) -> Self {
//         Cool2 {
//             name: value.name.clone(),
//             identifier: value.name,
//             description: value.description,
//             macos: Some(Platform {
//                 need_restart: value.need_restart,
//                 use_package_manager: false,
//                 dependencies: value.dependencies.into_iter().collect::<HashSet<String>>(),
//                 install_tasks: value.install_tasks,
//                 uninstall_tasks: value.uninstall_tasks,
//                 check_tasks: value.check_tasks,
//             }),
//             windows: None,
//             linux: None,
//         }
//     }
// }

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        // let llvm_cool = Cool2 {
        //     name: "llvm".to_string(),
        //     identifier: "llvm".to_string(),
        //     description: "ABC".to_string(),
        //     macos: Some(Platform {
        //         need_restart: false,
        //         use_package_manager: false,
        //         dependencies: HashSet::from(["homebrew".to_string()]),
        //         install_tasks: Tasks(vec![Task::install(
        //             "llvm",
        //             None::<Vec<&str>>,
        //             None::<Vec<(&str, &str)>>,
        //             Installer::Brew(Brew),
        //         )]),
        //         uninstall_tasks: Tasks(vec![Task::uninstall(
        //             "llvm",
        //             None::<Vec<&str>>,
        //             Installer::Brew(Brew),
        //         )]),
        //         check_tasks: Tasks(vec![Task::check("llvm", Installer::Brew(Brew))]),
        //     }),
        //     windows: Some(Platform {
        //         need_restart: false,
        //         use_package_manager: false,
        //         dependencies: HashSet::new(),
        //         install_tasks: Tasks(vec![Task::install(
        //             "LLVM.LLVM",
        //             None::<Vec<&str>>,
        //             None::<Vec<(&str, &str)>>,
        //             Installer::Brew(Brew),
        //         )]),
        //         uninstall_tasks: Tasks(vec![Task::uninstall(
        //             "LLVM.LLVM",
        //             None::<Vec<&str>>,
        //             Installer::Brew(Brew),
        //         )]),
        //         check_tasks: Tasks(vec![Task::check("LLVM.LLVM", Installer::Brew(Brew))]),
        //     }),
        //     linux: Some(Platform {
        //         need_restart: false,
        //         use_package_manager: false,
        //         dependencies: HashSet::new(),
        //         install_tasks: Tasks(vec![Task::install(
        //             "clang-devel",
        //             None::<Vec<&str>>,
        //             None::<Vec<(&str, &str)>>,
        //             Installer::Apt(Apt),
        //         )]),
        //         uninstall_tasks: Tasks(vec![Task::uninstall(
        //             "clang-devel",
        //             None::<Vec<&str>>,
        //             Installer::Apt(Apt),
        //         )]),
        //         check_tasks: Tasks(vec![Task::check("clang-devel", Installer::Apt(Apt))]),
        //     }),
        // };
        // let string = serde_yaml::to_string(&llvm_cool).unwrap();
        // println!("{}", string);
    }

    #[test]
    fn migrate() {
        // let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        // let assets_dir = manifest_dir.join("assets");
        // let cools_dir = assets_dir.join("cools");
        // let cools2_dir = assets_dir.join("cools2");
        // if !cools2_dir.exists() {
        //     fs_extra::dir::create(&cools2_dir, true).unwrap();
        // }
        // let wd = walkdir::WalkDir::new(cools_dir);
        // wd.into_iter().flatten().for_each(|de| {
        //     if de.file_type().is_file() {
        //         let content = fs_extra::file::read_to_string(de.path()).unwrap();
        //         let cool: Cool = serde_yaml::from_str(&content).unwrap();
        //         let cool2: Cool2 = cool.into();
        //         let cool2_path = cools2_dir.join(de.file_name());
        //         let cool2_content = serde_yaml::to_string(&cool2).unwrap();
        //         fs_extra::file::write_all(cool2_path, &cool2_content).unwrap();
        //     }
        // });
    }
}
