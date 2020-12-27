use std::path::PathBuf;
use std::sync::Arc;
use tracing::instrument;

mod salsa_db;

use crate::{esinstall::ImportMap, sources::Source};
use salsa_db::{Files, SalsaToastDatabaseStruct};

pub struct Cache {
    db: SalsaToastDatabaseStruct,
    npm_bin_dir: PathBuf,
}

impl Cache {
    pub fn set_source(&mut self, key: &str, source: Source) {
        let db: &mut dyn Files = &mut self.db;
        db.set_source(key.to_string(), Arc::new(source));
    }
    pub fn read(&mut self, key: PathBuf) -> String {
        let db: &mut dyn Files = &mut self.db;
        db.read(key)
    }
    pub fn get_js_for_browser(&mut self, key: &str, import_map: ImportMap) -> String {
        let db: &mut dyn Files = &mut self.db;
        db.js_for_browser(key.to_string(), self.npm_bin_dir.clone(), import_map)
    }
    pub fn get_js_for_server(&mut self, key: &str) -> String {
        let db: &mut dyn Files = &mut self.db;
        db.js_for_server(key.to_string(), self.npm_bin_dir.clone())
    }
}

#[instrument]
pub fn init(npm_bin_dir: PathBuf) -> Cache {
    let db = SalsaToastDatabaseStruct::default();

    Cache { db, npm_bin_dir }
}
