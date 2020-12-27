use std::path::PathBuf;

#[derive(Debug)]
pub struct Source {
    pub source: String,
    pub kind: SourceKind,
}
#[derive(Debug)]
pub enum SourceKind {
    File { relative_path: PathBuf },
    Raw,
}
