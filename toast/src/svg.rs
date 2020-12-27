use std::path::Path;
use swc_ecma_ast::ImportDecl;
use swc_ecma_visit::{noop_visit_mut_type, VisitMut};

pub struct SVGImportToComponent<'a> {
    pub filepath: &'a Path,
    pub npm_bin_dir: String,
}

impl VisitMut for SVGImportToComponent<'_> {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, decl: &mut ImportDecl) {
        if !decl.src.value.ends_with(".svg") {
            return;
        }
        println!(
            "SVGImportToComponent processing: {:?}",
            self.filepath.parent()
        );
        let decl_string = format!("{}", decl.src.value);
        let svg_path_import = Path::new(&decl_string);
        if let Some(filedir) = self.filepath.parent() {
            let svg_filepath = filedir.join(svg_path_import).canonicalize();
            if let Ok(svg_path) = svg_filepath {
                println!("svg_path: {:?}", svg_path);
                let svg_final_docx_pdf = svg_path.to_str().unwrap();
                println!("{}", svg_final_docx_pdf);
            }
        }
    }
}

// TODO: Raw Rust Implementation
// this is an experimental version of where we want to go in the future
//
// impl VisitMut for SVGImportToComponent<'_> {
//     noop_visit_mut_type!();

//     fn visit_mut_import_decl(&mut self, decl: &mut ImportDecl) {
//         if decl.src.value.ends_with(".svg") {
//             println!("visit_mut: {:?}", self.filepath.parent());
//             let decl_string = format!("{}", decl.src.value);
//             let svg_path_import = Path::new(&decl_string);
//             if let Some(filedir) = self.filepath.parent() {
//                 let svg_filepath = filedir.join(svg_path_import).canonicalize();
//                 if let Ok(svg_path) = svg_filepath {
//                     println!("svg_path: {:?}", svg_path);
//                     let svg_final_docx_pdf = svg_path.to_str().ok_or(std::io::Error::new(
//                         std::io::ErrorKind::Other,
//                         "Couldn't convert SVG PathBuf to str",
//                     ));

//                     let svg_contents = svg_final_docx_pdf.and_then(|svg_path| load_file(svg_path));
//                     let svg_document = match svg_contents {
//                         Ok(file_contents) => parse_data(
//                             file_contents.as_str(),
//                             &svgcleaner::ParseOptions {
//                                 ..Default::default()
//                             },
//                         ),
//                         // TODO: no panic for you
//                         Err(e) => panic!("askfljaf"),
//                     };

//                     svg_document
//                         .and_then(|mut doc| {
//                             clean_doc(
//                                 &mut doc,
//                                 &svgcleaner::CleaningOptions {
//                                     remove_title: true,
//                                     ..Default::default()
//                                 },
//                                 &svgcleaner::WriteOptions {
//                                     ..Default::default()
//                                 },
//                             );
//                             Ok(doc)
//                         })
//                         .and_then(|document| {
//                             let mut buf: Vec<u8> = vec![];
//                             write_buffer(
//                                 &document,
//                                 &svgcleaner::WriteOptions {
//                                     ..Default::default()
//                                 },
//                                 &mut buf,
//                             );
//                             write_stdout(&buf);
//                             Ok(document)
//                         });
//                 }
//             }
//         }
//     }
// }
