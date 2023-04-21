#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct Command {
    #[serde(rename = "type")]
    func: String,
    item_type_id: Option<String>,
}
