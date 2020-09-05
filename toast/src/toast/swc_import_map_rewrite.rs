use crate::toast::breadbox::ImportMap;
use string_cache::{Atom, EmptyStaticAtomSet};
use swc_ecma_ast::{ImportDecl, Str};
use swc_ecma_visit::{noop_fold_type, Fold};

pub struct SWCImportMapRewrite<'a> {
    pub import_map: &'a ImportMap,
}

fn is_source_import(val: Atom<swc_atoms::JsWordStaticSet>) -> bool {
    val.starts_with("/") || val.starts_with(".") || val.starts_with("\\")
}

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
                    ..decl.src
                },
                ..decl
            }
        } else if is_source_import(decl.src.value.clone()) {
            ImportDecl {
                src: Str {
                    value: Atom::from(format!("{}{}", decl.src.value, ".js")),
                    ..decl.src
                },
                ..decl
            }
        } else {
            decl
        }
    }
}
