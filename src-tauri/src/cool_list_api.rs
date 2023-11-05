use cool::{Cool, COOL_LIST};
use std::collections::HashMap;

#[tauri::command]
pub fn serialize_cool_list() -> Vec<Cool> {
    let mut cool_list = COOL_LIST
        .read()
        .unwrap()
        .iter()
        .map(|(_, v)| v.read().unwrap().clone())
        .collect::<Vec<_>>();

    cool_list.sort_by(|a, b| a.name.cmp(&b.name));
    cool_list
}
