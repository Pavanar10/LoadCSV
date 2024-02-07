use std::error::Error;

use csv::ReaderBuilder;

use super::{connection::Connection, tg_rapport::TgRapportConnection};

#[derive(Clone,Debug,serde::Deserialize,serde::Serialize,PartialEq, Eq,PartialOrd,Ord)]
pub struct TGRecords {
    #[serde(rename = "BU / Operation")]
    pub klant: String,
    #[serde(rename = "Type of declaration")]
    pub type_of_declaration: String,
    #[serde(rename = "Declaration number")]
    pub declaration_number: String, 
    #[serde(rename = "MRN Number")]
    pub mrn: String, 
    #[serde(rename = "Declaration date")]
    pub declaratie_datum: String, 
    #[serde(rename = "HTS code")]
    pub hts_code: String, 
    #[serde(rename = "Quantity / Collia")]
    pub quantity: String, 
    #[serde(rename = "Gross weight")]
    pub gross_weight: String, 
    #[serde(rename = "Netweight")]
    pub net_weight: String, 
    #[serde(rename = "Unit value")]
    pub unit_value: String, 
    #[serde(rename = "Total line value")]
    pub total_line_value: String, 
    #[serde(rename = "Exchange rate")]
    pub exchange_rate: String, 
    #[serde(rename = "Customs Duty")]
    pub customs_duty: String, 
    #[serde(rename = "VAT amount")]
    pub vat_amt: String, 
    #[serde(rename = "Controlled / dual use goods")]
    pub controllled_dual_goods_value: String, 
    #[serde(rename = "ECCN classification")]
    pub eccn_classification: String, 
    #[serde(rename = "Country of destination")]
    pub country_dest: String, 
    #[serde(rename = "Ship to name, address details")]
    pub ship_address: String, 
    #[serde(rename = "Ship to country")]
    pub ship_to_country: String, 
    #[serde(rename = "PID number (FHC:FDA Number)")]
    pub pid_number: String,  
     #[serde(rename = "T1 cleared Y/N")]
    pub t1_cleared: String, 
    #[serde(rename = "T1 clearance date")]
    pub t1_clearance_date: String,  
     #[serde(rename = "T1 Office of Clearance")]
    pub t1_office_clearance: String, 
    #[serde(rename = "NCTS status")]
    pub ncts_status: String, 
}

pub async fn read_tg_monitoring_csv(path: &str,conn:&Connection) -> Result<Vec<csv::StringRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
    .delimiter(b';').has_headers(false)
     .flexible(true)
     .from_path(path)?;
 
     let headers = rdr.headers()?.clone();
     let data = rdr
             .records()
             .collect::<Result<Vec<csv::StringRecord>, csv::Error>>()?;
 
     let mut filtered_data:Vec<TGRecords> = Vec::new();
 
     for result in data.iter(){
 
             if &result[23] == "43" || &result[24] == "43" || &result[23]== "8" || &result[24]=="8" 
             || &result[23]=="58" || &result[24]=="58" ||&result[23]=="60" || &result[24]=="60"{
                 if &result[1] == "TG" {
                    
                     let exists = filtered_data.iter().any(|record|record.mrn.to_string() == &result[3]);
                     if !exists {
                          let record: TGRecords = result.deserialize(Some(&headers)).unwrap();
                         filtered_data.push(record); 
                         }
                     }
                     }else{
                         continue;
                     }
     }
     /*for data in &filtered_data.clone().iter(){
         println!("New Data{:#?}",data)
     }*/
     TgRapportConnection::load_tg_monitoring(&conn,&filtered_data).await;
     Ok(data)
 }