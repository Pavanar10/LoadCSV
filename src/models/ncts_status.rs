#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NctsStatus{
    pub id:Option<i64>,
    pub status:String,
}