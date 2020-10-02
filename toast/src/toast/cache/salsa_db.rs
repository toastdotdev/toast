use crate::toast::{
    breadbox::ImportMap,
    sources::Source,
    swc_ops::{compile_js_for_browser, compile_js_for_server},
};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::instrument;

#[salsa::query_group(FilesStorage)]
pub trait Files: salsa::Database {
    #[salsa::input]
    fn source(&self, key: String) -> Arc<Source>;

    // compile js for targets
    fn js_for_browser(&self, key: String, npm_bin_dir: PathBuf, import_map: ImportMap) -> String;
    fn js_for_server(&self, key: String, npm_bin_dir: PathBuf) -> String;

    // not meant to be used by users
    fn read(&self, path: PathBuf) -> String;
    fn read_and_watch(&self, path: PathBuf) -> String;
}

#[instrument(skip(db))]
fn read(db: &dyn Files, path: PathBuf) -> String {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    // db.watch(&path);
    std::fs::read_to_string(&path).unwrap_or_default()
}

#[instrument(skip(db))]
fn read_and_watch(db: &dyn Files, path: PathBuf) -> String {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    // db.watch(&path);
    std::fs::read_to_string(&path).unwrap_or_default()
}

#[instrument(skip(db))]
fn js_for_browser(
    db: &dyn Files,
    key: String,
    npm_bin_dir: PathBuf,
    import_map: ImportMap,
) -> String {
    let source_file = db.source(key.to_string());
    compile_js_for_browser(source_file.source.clone(), key, npm_bin_dir, import_map)
}

#[instrument(skip(db))]
fn js_for_server(db: &dyn Files, key: String, npm_bin_dir: PathBuf) -> String {
    let source_file = db.source(key.to_string());
    compile_js_for_server(source_file.source.clone(), key, npm_bin_dir)
}

#[salsa::database(FilesStorage)]
#[derive(Default)]
pub struct SalsaToastDatabaseStruct {
    pub storage: salsa::Storage<Self>,
}

impl salsa::Database for SalsaToastDatabaseStruct {}
