use super::Template;
use crate::template::PlatformType;
use crate::{Task, Tasks};

impl Template {
    pub(crate) fn name() -> &'static str {
        "template"
    }

    pub(crate) fn source() -> &'static str {
        "template_source"
    }

    pub(crate) fn dest() -> &'static str {
        "template_dest"
    }

    pub(crate) fn url() -> &'static str {
        "http://localhost/template"
    }

    pub(crate) fn git() -> &'static str {
        "http://localhost/template.git"
    }

    pub(crate) fn args() -> Vec<&'static str> {
        vec!["--template"]
    }

    pub(crate) fn envs() -> Vec<(&'static str, &'static str)> {
        vec![("COOL_TEMPLATE", "COOL_TEMPLATE_VALUE")]
    }

    pub(crate) fn check(plat: &PlatformType) -> Task {
        Task::check(Template::name(), Template::installer(plat))
    }

    pub(crate) fn command(plat: &PlatformType) -> Task {
        Task::command(
            Template::name(),
            Some(Template::envs()),
            Template::shell(plat),
        )
    }

    pub(crate) fn compress() -> Task {
        Task::compress(Template::source(), Template::dest())
    }

    pub(crate) fn copy_task() -> Task {
        Task::copy_task(Template::source(), Template::dest())
    }

    pub(crate) fn decompress() -> Task {
        Task::decompress(Template::source(), Template::dest())
    }

    pub(crate) fn delete() -> Task {
        Task::delete(Template::source())
    }

    pub(crate) fn download() -> Task {
        Task::download(Template::url(), Template::dest())
    }

    pub(crate) fn exists() -> Task {
        Task::exists(Template::name())
    }

    pub(crate) fn git_clone() -> Task {
        Task::git_clone(Template::git(), Template::dest())
    }

    pub(crate) fn git_pull() -> Task {
        Task::git_pull(Template::source())
    }

    pub(crate) fn git_checkout() -> Task {
        Task::git_checkout(
            Template::source(),
            format!("{}_branch", Template::name()),
            false,
        )
    }

    pub(crate) fn install(plat: &PlatformType) -> Task {
        Task::install(
            Template::name(),
            Some(Template::args()),
            Some(Template::envs()),
            Template::installer(plat),
        )
    }

    pub(crate) fn move_task() -> Task {
        Task::move_task(Template::source(), Template::dest())
    }

    pub(crate) fn uninstall(plat: &PlatformType) -> Task {
        Task::uninstall(
            Template::name(),
            Some(Template::args()),
            Template::installer(plat),
        )
    }

    pub(crate) fn which() -> Task {
        Task::which(Template::name())
    }

    pub(crate) fn tasks(plat: &PlatformType) -> Tasks {
        Tasks(vec![
            Template::check(plat),
            Template::command(plat),
            Template::compress(),
            Template::copy_task(),
            Template::decompress(),
            Template::delete(),
            Template::download(),
            Template::exists(),
            Template::git_clone(),
            Template::git_pull(),
            Template::git_checkout(),
            Template::install(plat),
            Template::move_task(),
            Template::uninstall(plat),
            Template::which(),
        ])
    }
}
