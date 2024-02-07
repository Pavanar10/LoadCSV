#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Land {
    pub id:Option<u64>,
    pub land_code_twee: String,
}