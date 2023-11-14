use std::collections::HashMap;
use std::iter::Map;
use std::ops::Deref;
use std::str::FromStr;

use rayon::prelude::*;

use cool::error::CoolError;
use cool::result::CoolResult;
use cool::state::CoolState;
use cool::{SafeCool, COOL_LIST};

use crate::cool_data::CoolData;
use crate::event::EventLoop;
use crate::server::ASK_PASS_TRIGGER_CHANNEL;

#[tauri::command(async)]
pub fn serialize_cool_list() -> Vec<CoolData> {
    let mut cools = COOL_LIST
        .par_iter()
        .map(|v| v.lock().unwrap().deref().into())
        .collect::<Vec<CoolData>>();
    cools.sort_by(|a, b| a.name.cmp(&b.name));
    cools
}

#[tauri::command(async)]
pub fn install_cools(cools: Vec<String>) -> CoolResult<(), CoolError> {
    cools.par_iter().try_for_each(|c| {
        let cool = SafeCool::from_str(c)?;
        let (tx, rx) = cool::channel::unbounded();

        rayon::spawn(move || {
            while let Ok(event) = rx.recv() {
                println!("{}", event);
                EventLoop::task_event(event);
            }
        });
        cool.lock().unwrap().install(&Some(tx))?;
        Ok(())
    })
}

#[tauri::command(async)]
pub fn uninstall_cools(cools: Vec<String>) -> CoolResult<(), CoolError> {
    cools.par_iter().try_for_each(|c| {
        let cool = SafeCool::from_str(c)?;
        let (tx, rx) = cool::channel::unbounded();

        rayon::spawn(move || {
            while let Ok(event) = rx.recv() {
                EventLoop::task_event(event);
            }
        });
        cool.lock().unwrap().uninstall(&Some(tx))?;
        Ok(())
    })
}

#[tauri::command(async)]
pub fn check_cools(cools: Vec<String>) -> Vec<CoolState> {
    cools
        .par_iter()
        .map(|c| {
            let cool = SafeCool::from_str(c).unwrap();
            let state = cool.lock().unwrap().check();
            state
        })
        .collect::<Vec<_>>()
}

#[tauri::command]
pub fn callback_ask_pass(password: String) {
    ASK_PASS_TRIGGER_CHANNEL
        .0
        .lock()
        .unwrap()
        .send(password)
        .unwrap();
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
