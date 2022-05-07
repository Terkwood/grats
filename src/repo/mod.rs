use crate::model::*;
use gloo_storage::{LocalStorage,Storage};


const GRATITUDE_LIST_KEY: &str = "gratitude_list";
const ENTRY_BUTTONS_KEY: &str = "entry_buttons";

pub struct GratitudeListRepo;

impl GratitudeListRepo {
    pub fn new() -> Self {
         
        Self  
    }

    pub fn read(&self) -> GratitudeList {
        if let Ok(restored_model) = LocalStorage::get(GRATITUDE_LIST_KEY) {
            restored_model
        } else {
            GratitudeList::empty()
        }
    }

    pub fn save(&mut self, grat_list: &GratitudeList) {
        let value = grat_list;
        LocalStorage::set(GRATITUDE_LIST_KEY, value).expect("storage fail")
    }
}

pub struct EntryButtonsRepo;

impl EntryButtonsRepo {
    pub fn new() -> Self {
         
        Self  
    }

    pub fn read(&self) -> EntryButtonCollection {
        if let Ok(restored_model) = LocalStorage::get(ENTRY_BUTTONS_KEY) {
            restored_model
        } else {
            EntryButtonCollection::new()
        }
    }

    pub fn save(&mut self, buttons: &EntryButtonCollection) {
        LocalStorage::set(ENTRY_BUTTONS_KEY, buttons).expect("buttons save")
    }
}
