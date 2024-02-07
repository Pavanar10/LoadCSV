#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Klant {
    pub id:Option<u64>,
    pub klantnaam: String,
    pub referentie:Option<String>,
}