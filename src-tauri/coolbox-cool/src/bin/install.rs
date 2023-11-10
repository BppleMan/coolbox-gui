use cool::result::CoolResult;
use cool::{init_backtrace, COOL_LIST};

fn main() -> CoolResult<()> {
    init_backtrace();
    let curl = COOL_LIST.get("curl").unwrap();
    curl.lock().unwrap().install()?;
    Ok(())
}
