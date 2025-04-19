use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImportInfo {
    pub name: String,
    pub source: String,
    pub kind: String,
}