use std::sync::Arc;

mod salsa_db;

use salsa_db::{Files, SalsaToastDatabaseStruct};

pub struct Cache {
    db: SalsaToastDatabaseStruct,
    npm_bin_dir: String,
}

impl Cache {
    pub fn set_source(&mut self, key: &str, source: Vec<u8>) {
        let db: &mut dyn Files = &mut self.db;
        db.set_source(key.to_string(), Arc::new(source));
    }
    pub fn get_js_for_browser(&mut self, key: &str) -> String {
        let db: &mut dyn Files = &mut self.db;
        let js = db.js_for_browser(key.to_string(), self.npm_bin_dir.clone());
        println!("js4browser: {}", js);
        js
    }
    pub fn get_js_for_server(&mut self, key: &str) -> String {
        let db: &mut dyn Files = &mut self.db;
        let js = db.js_for_server(key.to_string());
        println!("js4server: {}", js);
        js
    }
}

pub fn init(npm_bin_dir: String) -> Cache {
    let mut db = SalsaToastDatabaseStruct::default();

    Cache { db, npm_bin_dir }
}
