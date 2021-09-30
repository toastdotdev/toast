use crate::esinstall::ImportMap;
// use string_cache::Atom;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ImportDecl, Str};
use swc_ecma_visit::{noop_fold_type, Fold};
pub struct SWCImportMapRewrite<'a> {
    pub import_map: &'a ImportMap,
}

// fn is_source_import(val: Atom<swc_atoms::JsWordStaticSet>) -> bool {
//     val.starts_with("/") || val.starts_with(".") || val.starts_with("\\")
// }

impl Fold for SWCImportMapRewrite<'_> {
    noop_fold_type!();

    fn fold_import_decl(&mut self, decl: ImportDecl) -> ImportDecl {
        if self.import_map.imports.contains_key(&decl.src.value) {
            ImportDecl {
                src: Str {
                    value: self
                        .import_map
                        .imports
                        .get(&decl.src.value)
                        .unwrap()
                        .clone(),
                    span: DUMMY_SP,
                    ..decl.src
                },
                ..decl
            }
        } else {
            decl
        }
    }
}
