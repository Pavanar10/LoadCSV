use std::error::Error;

use chrono::NaiveDate;
use csv::ReaderBuilder;
use rbatis::crud;

use crate::models::{ncts_status::NctsStatus, dossier::Dossier, land::Land};

use super::{connection::Connection, ta_monitoring::TARecords, klant::Klant};
crud!(TaRapportView{});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaRapport{
    pub id:Option<i64>,
    pub fk_dossier_id:i64,
    pub opmerking:Option<String>,
    pub fk_ncts_status_id:i64,
    pub aanvullend:Option<String>,
    pub risico:Option<String>,
    pub opt_in_out:Option<String>,
    pub vervoerder:Option<String>,
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaRapportView {
    pub id:i64,
    pub declaratie_datum:Option<String>,
    pub mrn: String,
    pub referentie:Option<String>,
    pub land_code_twee:String,
    pub risico:Option<String>,
    pub vervoerder:Option<String>,
    pub opt_in_out:Option<String>,
    pub status:String,
    pub documents: i16,
    pub opmerking: Option<String>,
    pub aanvullend:Option<String>,
    pub klant_id: i16,
    pub dossier_id: i64,
    pub fk_ncts_status_id:i64,
}

pub struct TaRapportConnection{}
impl TaRapportConnection{
    pub async fn insert_new_klant(conn:&Connection,filtered_data:&Vec<TARecords>)->String{
        for item in filtered_data{
    
            let Klant = match Klant::select_all(&mut &conn.db).await{
                Ok(res)=>res,
                Err(err)=>panic!("Klant not found{:#?}",err)
            } ;
           match  Klant.iter().find(|s|s.klantnaam == item.klant) {
               Some(_s)=>{
                println!("Klant found")
               }
            None=>{
                let new_klant =Klant {
                        id:None,
                        klantnaam:item.klant.clone(),
                        referentie:None
                };
        
                match Klant::insert(&mut &conn.db, &new_klant).await{
                    Ok(r)=>r,
                    Err(e)=>panic!("Record inserted successully{:#?}",e)
                };
            }
           }
        }
            "Ok".to_string()
    }
    
    
    pub async fn insert_new_ncts(conn:&Connection,filtered_data:&Vec<TARecords>) -> String {
        println!("Calling Ncts Status");
        for item in filtered_data{
            let status:Vec<NctsStatus> = match NctsStatus::select_all(&mut &conn.db).await{
                Ok(res)=>res,
                Err(err)=>panic!("Klant not found{:#?}",err)
            };
            match status.iter().find(|s|s.status == item.ncts_status){
               Some(_s)=>{ println!("Land found");
            },
             None=>{
                    let new_status =NctsStatus {
                        id:None,
                        status:item.ncts_status.clone(),
                    };
            
                    match NctsStatus::insert(&mut &conn.db, &new_status).await{
                        Ok(r)=>r,
                        Err(e)=>panic!("Record inserted successully{:#?}",e)
                    };
                
                }
            }
        }
        "Ok".to_string()
    }


    pub async fn check_for_existing_mrns(path: &str,conn:&Connection)-> Result<String, Box<dyn Error>>{
        println!("Inside check mrns");
                        let mut rdr = ReaderBuilder::new()
                        .delimiter(b';').has_headers(false)
                        .flexible(true)
                        .from_path(path)?;
        
                       let headers = rdr.headers()?.clone();
                        let data = rdr
                                .records()
                                .collect::<Result<Vec<csv::StringRecord>, csv::Error>>()?;
        
                        let ta_rapport_records = match TaRapportView::select_all(&mut &conn.db).await{
                            Ok(r)=>r,
                            Err(e)=>panic!("Could not get recorsds{:#?}",e),
                        } ;
        
                        let ncts = match NctsStatus::select_all(&mut &conn.db).await{
                            Ok(r)=>r,
                            Err(e)=>panic!("Could not get recorsds{:#?}",e),
                        };
                    let mut delete_data:Vec<TARecords> = Vec::new();
                    let mut update_data:Vec<TARecords> = Vec::new();
                    let mut filtered_data:Vec<TARecords> = Vec::new();

                    for result in data.iter(){
                        if &result[1] == "TA" {
                            let record: TARecords = result.deserialize(Some(&headers)).unwrap();
                                filtered_data.push(record); 
                            }
                    }
                    for items in filtered_data.iter(){
                    let mrn_exists = ta_rapport_records.iter().any(|record|record.mrn.to_string() == items.mrn);
                    let ncts_exists = ncts.iter().any(|record|record.status.to_string() ==items.ncts_status);
        
                    if mrn_exists && !ncts_exists {
                        println!("Reached here");
                            let exists = delete_data.iter().any(|record|record.mrn.to_string() == items.mrn);
                            if !exists{
                           // let record: TARecords = items.deserialize(Some(&headers)).unwrap();
                                delete_data.push(items.clone()); 
                        }
                    }           
                    else if mrn_exists && ncts_exists{
                        let exists = update_data.iter().any(|record|record.mrn.to_string() == items.mrn);
                        if !exists {
                            //let record: TARecords = items.deserialize(Some(&headers)).unwrap();
                            update_data.push(items.clone()); 
                            }
                        }
                }
        
            if delete_data.len()>0{
            for item in delete_data {
                println!("Delete {:#?} {:#?}",item.mrn,item.ncts_status);
                    let ta_rapport_view:Vec<TaRapportView> = TaRapportView::select_by_column(&mut &conn.db, "mrn", item.mrn).await.unwrap();
                    let d_id= ta_rapport_view[0].dossier_id;
        
                    println!("Dossier id{:?}",d_id);
                    let tarapport:Vec<TaRapport> = TaRapport::select_by_column(&mut &conn.db, "fk_dossier_id", d_id).await.unwrap();
                     
                     if tarapport.len()>0{
                     let _res2 = match TaRapport::delete_by_column(&mut &conn.db, "fk_dossier_id",d_id ).await {
                            Ok(r)=>r,
                            Err(e)=>panic!("Could not get recorsds{:#?}",e),
                           };
                          
                    let _res =match Dossier::delete_by_column(&mut &conn.db, "id", d_id).await {
                            Ok(r)=>r,
                            Err(e)=>panic!("Could not get recorsds{:#?}",e),
                           };
                        }
                     }}
            if update_data.len()>0 {
                        println!("Length{:#?}",update_data.len());
                        for item in update_data{

                        println!("Update{:#?} {:#?}",item.mrn,item.ncts_status);
                        let tarapport:Vec<TaRapportView> = TaRapportView::select_by_column(&mut &conn.db, "mrn", item.mrn).await.unwrap();
                       

                        let ncts = NctsStatus::select_by_column(&mut &conn.db, "status", item.ncts_status).await.unwrap();
                        //   println!("Ncts{:#?}",ncts);
                           let ncts_id = ncts[0].id;
                          if tarapport.len()>1 {
                            let d_id= tarapport[1].dossier_id;
                            println!("{:#?} {:#?}",tarapport[0].dossier_id,tarapport[1].dossier_id);
                            //delete last record from ta rapport and keep the first with other details
                                let _res = match TaRapport::delete_by_column(&mut &conn.db, "fk_dossier_id", d_id).await {
                                    Ok(r)=>r,
                                    Err(e)=>panic!("Could not get recorsds{:#?}",e),
                                };
                                let _res1 = match Dossier::delete_by_column(&mut &conn.db, "id", d_id).await {
                                    Ok(r)=>r,
                                    Err(e)=>panic!("Could not get recorsds{:#?}",e),
                                };
                        }// else update existing ncts and other details
                        else if tarapport.len()>0{
                         if  tarapport[0].fk_ncts_status_id != ncts_id.unwrap(){
                            let d_id= tarapport[0].dossier_id;
                            let mut tarapport1:Vec<TaRapport> = TaRapport::select_by_column(&mut &conn.db, "fk_dossier_id", d_id).await.unwrap();
                            //update ta rapport
                            tarapport1[0].fk_ncts_status_id=ncts_id.unwrap();
                            println!("tgrapport{:#?}",tarapport1);

                             let _res = match TaRapport::update_by_column_value(&mut &conn.db, &tarapport1[0], "id",&rbs::to_value!(tarapport1[0].id)).await {
                                        Ok(r)=>r,
                                        Err(e)=>panic!("Could not get recorsds{:#?}",e),
                                };
                            } 
                        }
                        }

                    } 

Ok("ok".to_string())
}

pub async fn  load_ta_monitoring(filtered_data:Vec<TARecords>,conn:&Connection){
    //insert all ncts
    let _res= TaRapportConnection::insert_new_ncts(&conn,&filtered_data).await;
    let  _res2 = TaRapportConnection::insert_new_klant(&conn,&filtered_data).await;
    println!("Calling TA upload");
    let klants= match Klant::select_all(&mut &conn.db).await{
        Ok(res)=>res,
        Err(err)=>panic!("Klant not found{:#?}",err)
    };

    let land:Vec<Land> = match Land::select_all(&mut &conn.db).await{
        Ok(res)=>res,
        Err(err)=>panic!("Klant not found{:#?}",err)
    };

    let status = match  NctsStatus::select_all(&mut &conn.db).await{
        Ok(res)=>res,
        Err(err)=>panic!("Klant not found{:#?}",err),
    };

    for items in &filtered_data{
        //get klant id 
        let klants= klants.iter().find(|s|s.klantnaam == items.klant).unwrap();
    
        let land= land.iter().find(|s|s.land_code_twee == items.land).unwrap();

        let ncts_status= status.iter().find(|s|s.status == items.ncts_status).unwrap();

       let date_only = NaiveDate::parse_from_str(&items.declaratie_datum, "%d/%m/%Y").unwrap();
        //insert dossier data
        let  dossier = Dossier { 
            id: None, 
            mrn:items.mrn.clone(),
             fk_klant_id: klants.id.unwrap(),
              fk_landscode_id: land.id,
               declaratie_datum: date_only.to_string(),
                 manually_created: 0,
                 };
                 println!("Dossier{:#?}",dossier);

                match  Dossier::insert(&mut &conn.db, &dossier).await{
                        Ok(res)=>{  println!("Record Successfully inserted");
                                    res
                                },
                        Err(_)=>panic!("Could not insert dossier"),
                    };
    
        //get latest dossier id
       let dossierid: i64 = conn.db
        .query_decode("select top 1 id from dossier order by id desc", vec![])
        .await
        .unwrap();

        //insert ta records
        let customs_duty:f64;
        let customs_duty_replace = items.customs_duty.replace(",", ".");
        customs_duty = customs_duty_replace.to_string().parse::<f64>().unwrap();

        let  vat_amt:f64;
        let vat_amt_replace= items.vat_amt.replace(",", ".");
        vat_amt = vat_amt_replace.to_string().parse::<f64>().unwrap();
        let risico = customs_duty + vat_amt;
        let risoco_str = risico.to_string();
     
       
        let truncated_risico = if let Some(index)=risoco_str.find('.'){
                let end_index = index+2;
                if &end_index < &risoco_str.len(){
                    &risoco_str[..=end_index]
                }
                else{
                    &risoco_str
                }
        }else{
            &risoco_str
        }; 
                    
        //insert ta_rapport
        let tarapport = TaRapport{
        id:None,
        fk_dossier_id:dossierid,
        fk_ncts_status_id:ncts_status.id.unwrap(),
        opmerking: None,
        opt_in_out:None,
        aanvullend:None,
        risico:Some(truncated_risico.to_string()),
        vervoerder:None,
        };
        match TaRapport::insert(&mut &conn.db, &tarapport).await{
                Ok(res)=>{
                    println!("Record successfully inserted");
                    res
                },
                Err(err)=>{
                    println!("Record could not be inserted{}",err);
                    std::process::exit(0); }
            };
        
    }
}



}