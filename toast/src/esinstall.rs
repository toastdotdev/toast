use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::BTreeMap;
use string_cache::Atom;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Hash, Clone)]
pub struct ImportMap {
    pub imports: BTreeMap<Atom<swc_atoms::JsWordStaticSet>, Atom<swc_atoms::JsWordStaticSet>>,
}

pub fn parse_import_map(data: &str) -> Result<ImportMap> {
    let mut map: ImportMap = serde_json::from_str(data)?;
    for (_key, value) in map.imports.iter_mut() {
        // relative paths in the import-map.json values
        // mean relative to the web_modules directory
        if value.starts_with("./") {
            let v = value.trim_start_matches("./");
            *value = Atom::from(format!("/web_modules/{}", v));
        }
    }
    Ok(map)
}

// pub let empty =
