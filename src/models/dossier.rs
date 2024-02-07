#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Dossier {
    pub id:Option<u64>,
    pub mrn: String,
    pub fk_klant_id:u64,
    pub fk_landscode_id:Option<u64>,
    pub declaratie_datum:String,
    pub manually_created:u64,
}

