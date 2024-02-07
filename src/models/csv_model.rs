use csv::{ StringRecord};

#[derive(Clone,Debug)]
pub struct ReadCSV {
    pub id:u64,
    pub filename: String,
    pub data:Vec<StringRecord>,
}





