use color_eyre::Report;

pub type CoolResult<T, E = Report> = color_eyre::Result<T, E>;
