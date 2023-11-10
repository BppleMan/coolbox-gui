use std::ops::Deref;
use std::str::FromStr;

use rayon::prelude::*;

use cool::error::CoolError;
use cool::result::CoolResult;
use cool::state::CoolState;
use cool::{SafeCool, COOL_LIST};

use crate::cool_data::CoolData;

#[tauri::command]
pub fn serialize_cool_list() -> Vec<CoolData> {
    let mut cool_list = COOL_LIST
        .par_iter()
        .map(|v| v.lock().unwrap().deref().into())
        .collect::<Vec<_>>();
    cool_list.sort();
    cool_list
}

#[tauri::command]
pub fn install_cools(cools: Vec<String>) -> CoolResult<(), CoolError> {
    cools.par_iter().try_for_each(|c| {
        let cool = SafeCool::from_str(c)?;
        cool.lock().unwrap().install(&None)?;
        Ok(())
    })
}

#[tauri::command]
pub fn uninstall_cools(cools: Vec<String>) -> CoolResult<(), CoolError> {
    cools.par_iter().try_for_each(|c| {
        let cool = SafeCool::from_str(c)?;
        cool.lock().unwrap().uninstall(&None)?;
        Ok(())
    })
}

#[tauri::command]
pub fn check_cools(cools: Vec<String>) -> Vec<CoolResult<CoolState, CoolError>> {
    cools
        .par_iter()
        .map(|c| {
            let cool = SafeCool::from_str(c)?;
            let state = cool.lock().unwrap().check();
            Ok(state)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use cool::init_backtrace;
    use cool::result::CoolResult;

    use crate::api::serialize_cool_list;

    #[test]
    fn smoke() -> CoolResult<()> {
        init_backtrace();
        let cools = serialize_cool_list();
        println!("{:#?}", cools);
        Ok(())
    }
}
