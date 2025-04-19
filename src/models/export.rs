use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExportInfo {
    pub name: String,
    pub kind: String,
}