use cool::COOL_LIST;
use std::ops::Deref;

use crate::cool_data::CoolData;

#[tauri::command]
pub fn serialize_cool_list() -> Vec<CoolData> {
    let mut cool_list = COOL_LIST
        .iter()
        .map(|v| v.value().read().unwrap().deref().into())
        .collect::<Vec<_>>();
    cool_list.sort();
    cool_list
}
