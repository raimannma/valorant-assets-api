use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct APIResult<T> {
    pub status: u16,
    pub data: T,
}
