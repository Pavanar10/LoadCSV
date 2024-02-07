use std::error::Error;

use csv::ReaderBuilder;

use crate::models::ta_rapport::TaRapportConnection;

use super::connection::Connection;

#[derive(Clone,Debug,serde::Deserialize,serde::Serialize)]
pub struct TARecords {
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
    pub land: String, 
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
    //#[serde(rename = "Column1")]
    //pub column1: String, 
    //#[serde(rename = "_1")]
    //pub minusone: String, 
}


pub async fn read_ta_monitoring_csv(path: &str,conn:&Connection)->Result<Vec<csv::StringRecord>, Box<dyn Error>>{
    println!("called ta monitoring");
    let mut rdr = ReaderBuilder::new()
                .delimiter(b';').has_headers(false)
                .flexible(true)
                .from_path(path)?;

    let headers = rdr.headers()?.clone();
    println!("Headers{:#?}",headers);

    let data = rdr
    .records()
    .collect::<Result<Vec<csv::StringRecord>, csv::Error>>()?;

    let mut filtered_data:Vec<TARecords> = Vec::new();

    for result in data.iter(){
        if  &result[23] == "29" || &result[24] == "29" 
        || &result[23]=="140" || &result[24]=="140"
        ||&result[23]=="141" || &result[24]=="141"  
        ||&result[23]=="6" || &result[24]=="6" {
        if &result[1] == "TA" {

            let record: TARecords = result.deserialize(Some(&headers)).unwrap();
                filtered_data.push(record); 
            }
        }
    
    }
    println!("Length{}",filtered_data.len());

    let mut filtered_mrn:Vec<TARecords> = Vec::new(); 

        for items in filtered_data{
            let exists = filtered_mrn.iter().any(|s|s.mrn == items.mrn);
            if exists == false{
                filtered_mrn.push(items);
            }
            else if exists == true  {
                let new_customs_duty:f64 = items.customs_duty.replace(",", ".").parse::<f64>().unwrap();
                let new_vat_amy:f64 = items.vat_amt.replace(",", ".").parse::<f64>().unwrap();

                let existing_values:Vec<_> = filtered_mrn.iter().filter(|s|s.mrn == items.mrn).collect();

                if existing_values.len()>0 {
                    let existing_custom = existing_values[0].customs_duty.replace(",", ".").parse::<f64>().unwrap();
                    let existing_vat = existing_values[0].vat_amt.replace(",", ".").parse::<f64>().unwrap();

                    let total_customs = (existing_custom+ new_customs_duty).to_string();
                    let total_vat = (existing_vat + new_vat_amy).to_string();

                        for item in filtered_mrn.iter_mut(){
                            if item.mrn == items.mrn{
                                item.customs_duty = total_customs.to_string();
                                item.vat_amt = total_vat.to_string();
                            }
                        }
                }
            }
    }

   // TaRapportConnection::check_for_existing_mrns(&conn,filtered_mrn).await;
    if filtered_mrn.len() > 0 {
       TaRapportConnection::load_ta_monitoring(filtered_mrn,conn).await;
    }
    Ok(data)
}
