use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "mode")]
pub enum ModuleSpec {
    // users should see this as `component: null`
    #[serde(alias = "no-module")]
    NoModule,
    #[serde(alias = "filepath")]
    File {
        #[serde(alias = "value")]
        path: PathBuf,
    },
    #[serde(alias = "source")]
    Source {
        #[serde(alias = "value")]
        code: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetDataForSlug {
    /// /some/url or some/url
    pub slug: String,
    pub component: Option<ModuleSpec>,
    pub data: Option<serde_json::Value>,
    pub wrapper: Option<ModuleSpec>,
}

impl SetDataForSlug {
    pub fn normalize(&mut self) {
        // all paths are absolute paths
        if !self.slug.starts_with('/') {
            self.slug = "/".to_owned() + &self.slug;
        }
        match &self.data {
            // object with 0 keys is an empty object and shouldn't result
            // in the creation of a file on disk, and shouldn't blow away
            // any other data
            Some(Value::Object(v)) => {
                if v.len() > 0 {
                    // Some(Value::Object(v));
                } else {
                    self.data = None;
                }
            }
            _ => {}
        }
    }
    pub fn slug_as_relative_filepath(&self) -> PathBuf {
        let s = self.slug.trim_start_matches('/');
        let mut buf = PathBuf::from(s);

        if s.ends_with('/') {
            buf = buf.join("index");
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Result, Value};

    #[test]
    fn test_deserialize_all() -> Result<()> {
        let data = r#"
        {
            "slug": "/something",
            "component": {
                "mode": "source",
                "value": "import { h } from 'preact'; export default props => <div>hi</div>"
            },
            "data": {
                "some": "thing"
            },
            "wrapper": {
                "mode": "filepath",
                "value": "./some/where.js"
            }
        }"#;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data)?;

        // Access parts of the data by indexing with square brackets.
        let u: SetDataForSlug = serde_json::from_value(v).unwrap();
        assert_eq!(
            SetDataForSlug {
                slug: String::from("/something"),
                component: Some(ModuleSpec::Source {
                    code: String::from(
                        "import { h } from 'preact'; export default props => <div>hi</div>"
                    )
                }),
                data: Some(json!({
                    "some": "thing"
                })),
                wrapper: Some(ModuleSpec::File {
                    path: [".", "some", "where.js"].iter().collect::<PathBuf>()
                })
            },
            u
        );
        Ok(())
    }
    #[test]
    fn test_deserialize_without_data_and_wrapper() -> Result<()> {
        let data = r#"
        {
            "slug": "/something",
            "component": {
                "mode": "source",
                "value": "import { h } from 'preact'; export default props => <div>hi</div>"
            }
        }"#;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data)?;

        // Access parts of the data by indexing with square brackets.
        let u: SetDataForSlug = serde_json::from_value(v).unwrap();
        assert_eq!(
            SetDataForSlug {
                slug: String::from("/something"),
                component: Some(ModuleSpec::Source {
                    code: String::from(
                        "import { h } from 'preact'; export default props => <div>hi</div>"
                    )
                }),
                data: None,
                wrapper: None
            },
            u
        );
        Ok(())
    }

    #[test]
    fn test_file_paths_from_slugs() -> Result<()> {
        let set = SetDataForSlug {
            slug: String::from("/something/here"),
            component: None,
            data: None,
            wrapper: None,
        };

        assert_eq!(
            set.slug_as_relative_filepath(),
            PathBuf::from("something/here")
        );
        Ok(())
    }
    #[test]
    fn test_file_paths_from_slug_directories() -> Result<()> {
        let set = SetDataForSlug {
            slug: String::from("/something/here/"),
            component: None,
            data: None,
            wrapper: None,
        };

        assert_eq!(
            set.slug_as_relative_filepath(),
            PathBuf::from("something/here/index")
        );
        Ok(())
    }
}
