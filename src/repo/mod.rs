use crate::model::GratitudeList;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

const GRATITUDE_KEY: &str = "grats";

pub struct GratitudeRepo {
    pub storage_service: StorageService,
}

impl GratitudeRepo {
    pub fn new() -> Self {
        let storage_service =
            StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self { storage_service }
    }

    pub fn read_entries(&self) -> GratitudeList {
        if let Json(Ok(restored_model)) = self.storage_service.restore(GRATITUDE_KEY) {
            restored_model
        } else {
            GratitudeList::empty()
        }
    }

    pub fn save_entries(&mut self, grat_list: &GratitudeList) {
        let value = Json(grat_list);
        self.storage_service.store(GRATITUDE_KEY, value)
    }
}
