use crate::model::*;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

const GRATITUDE_LIST_KEY: &str = "gratitude_list";
const ENTRY_BUTTONS_KEY: &str = "entry_buttons";

pub struct GratitudeListRepo {
    pub storage_service: StorageService,
}

impl GratitudeListRepo {
    pub fn new() -> Self {
        let storage_service =
            StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self { storage_service }
    }

    pub fn read(&self) -> GratitudeList {
        if let Json(Ok(restored_model)) = self.storage_service.restore(GRATITUDE_LIST_KEY) {
            restored_model
        } else {
            GratitudeList::empty()
        }
    }

    pub fn save(&mut self, grat_list: &GratitudeList) {
        let value = Json(grat_list);
        self.storage_service.store(GRATITUDE_LIST_KEY, value)
    }
}

pub struct EntryButtonsRepo {
    pub storage_service: StorageService,
}

impl EntryButtonsRepo {
    pub fn new() -> Self {
        let storage_service =
            StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self { storage_service }
    }

    pub fn read(&self) -> EntryButtonCollection {
        if let Json(Ok(restored_model)) = self.storage_service.restore(ENTRY_BUTTONS_KEY) {
            restored_model
        } else {
            EntryButtonCollection::empty()
        }
    }

    pub fn save(&mut self, buttons: &EntryButtonCollection) {
        let value = Json(buttons);
        self.storage_service.store(ENTRY_BUTTONS_KEY, value)
    }
}
