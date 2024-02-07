use std::error::Error;

use chrono::NaiveDate;
use csv::ReaderBuilder;


use crate::models::{ncts_status::NctsStatus, tg_monitoring::TGRecords, dossier::Dossier};
use super::{connection::Connection, klant::Klant};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize,PartialEq,Eq)]
pub struct TgRapport{
    pub id:Option<u64>,
    pub fk_dossier_id:u64,
    pub fk_ncts_status_id:Option<i64>,
    pub notitie:Option<String>,
    pub referentie:Option<String>
}


#[derive(Clone,Debug,serde::Deserialize,serde::Serialize)]
pub struct TgRapportView {
    pub id:i64,
    pub declaratie_datum:Option<String>,
    pub mrn: String,
    pub referentie:Option<String>,
    pub status:String,
    pub notitie: Option<String>,
    pub klant_id: i16,
    pub dossier_id: i64,
    pub fk_ncts_status_id:i64
}

pub struct TgRapportConnection();

impl TgRapportConnection {
        

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
        
                        let tg_rapport_records = match TgRapportView::select_all(&mut &conn.db).await{
                            Ok(r)=>r,
                            Err(e)=>panic!("Could not get recorsds{:#?}",e),
                        } ;
        
                        let ncts = match NctsStatus::select_all(&mut &conn.db).await{
                            Ok(r)=>r,
                            Err(e)=>panic!("Could not get recorsds{:#?}",e),
                        };
                    let mut delete_data:Vec<TGRecords> = Vec::new();
                    let mut update_data:Vec<TGRecords> = Vec::new();
        
                    for result in data.iter(){
        
                    let mrn_exists = tg_rapport_records.iter().any(|record|record.mrn.to_string() == &result[3] && &result[1] == "TG");
                    let ncts_exists = ncts.iter().any(|record|record.status.to_string() == &result[23] && &result[1]=="TG");
        
                    if mrn_exists && !ncts_exists {
                    println!("MRN exists{:#?} {:#?}",&result[3],&result[23]);
        
                    let exists = delete_data.iter().any(|record|record.mrn.to_string() == &result[3] && &result[1] == "TG");
                    if !exists {
                         let record: TGRecords = result.deserialize(Some(&headers)).unwrap();
                            delete_data.push(record); 
                        }
                    }           
                    else if mrn_exists && ncts_exists{

                        if &result[23] == "43" || &result[24] == "43" || &result[23]== "8" || &result[24]=="8" 
                        || &result[23]=="58" || &result[24]=="58" ||&result[23]=="60" || &result[24]=="60" {
                        let exists = update_data.iter().any(|record|record.mrn.to_string() == &result[3] && &result[1]=="TG");
                        if !exists {
                             let record: TGRecords = result.deserialize(Some(&headers)).unwrap();
                                update_data.push(record); 
                            }
                        }
                    }
            }
        
            if delete_data.len()>0 {
            for item in delete_data{
                println!("Delete {:#?} {:#?}",item.mrn,item.ncts_status);
                    let tg_rapport_view = TgRapportView::select_by_column(&mut &conn.db, "mrn", item.mrn).await.unwrap();
                    let d_id = tg_rapport_view[0].dossier_id; 
                    //let dossier:Vec<Dossier> = Dossier::select_by_column(&mut &conn.db, "mrn", item.mrn).await.unwrap();
                    //let d_id= dossier[0].id;
        
                    println!("Dossier id{:?}",d_id);
                    let tgrapport:Vec<TgRapport> = TgRapport::select_by_column(&mut &conn.db, "fk_dossier_id", d_id).await.unwrap();
                     
                     if tgrapport.len()>0 {
                     let _res2 = match TgRapport::delete_by_column(&mut &conn.db, "fk_dossier_id",d_id ).await {
                            Ok(r)=>r,
                            Err(e)=>panic!("Could not get recorsds{:#?}",e),
                           };
                          
                    let _res =match Dossier::delete_by_column(&mut &conn.db, "id", d_id).await {
                            Ok(r)=>r,
                            Err(e)=>panic!("Could not get recorsds{:#?}",e),
                           };
                        }
                     }
                    }
                     if update_data.len()>0 {
                        println!("Update Length{}",update_data.len());
                     for item in update_data{
                      println!("Update{:#?} {:#?}",item.mrn,item.ncts_status);
                        let tgrapport:Vec<TgRapportView> = TgRapportView::select_by_column(&mut &conn.db, "mrn", item.mrn).await.unwrap();
                        let d_id= tgrapport[0].dossier_id;
                         // let ncts_d=item.ncts_status.parse::<i64>().unwrap();
                       
                        let ncts = NctsStatus::select_by_column(&mut &conn.db, "status", item.ncts_status).await.unwrap();
                     //   println!("Ncts{:#?}",ncts);
                        let ncts_id = ncts[0].id;
                        
                        
                       // let mut tgrapport:Vec<TgRapport> = TgRapport::select_by_column(&mut &conn.db, "mrn", d_id).await.unwrap();
                        if tgrapport.len()>1 {
                            let d_id= tgrapport[1].dossier_id;
                            //delete last record from ta rapport and keep the first with other details
                                let _res = match TgRapport::delete_by_column(&mut &conn.db, "fk_dossier_id", d_id).await {
                                    Ok(r)=>r,
                                    Err(e)=>panic!("Could not get recorsds{:#?}",e),
                                };
                                let _res1 = match Dossier::delete_by_column(&mut &conn.db, "id", d_id).await {
                                    Ok(r)=>r,
                                    Err(e)=>panic!("Could not get recorsds{:#?}",e),
                                };
                        }

                      else if tgrapport.len() > 0{
        
                        if tgrapport[0].fk_ncts_status_id != ncts_id.unwrap()  {
                          //  tgrapport[0].fk_ncts_status_id=ncts_id.unwrap();
                            println!("tgrapport{:#?}",tgrapport);
                            let mut tgrapport1:Vec<TgRapport> = TgRapport::select_by_column(&mut &conn.db, "fk_dossier_id", d_id).await.unwrap();
                            tgrapport1[0].fk_ncts_status_id=ncts_id;
                             let _res = match TgRapport::update_by_column_value(&mut &conn.db, &tgrapport1[0], "id",&rbs::to_value!(tgrapport1[0].id)).await {
                                Ok(r)=>r,
                                Err(e)=>panic!("Could not get recorsds{:#?}",e),
                            };
                        } }
                    }
                }
                Ok("ok".to_string())
            
        }

    pub async fn insert_new_klant(conn:&Connection,filtered_data:&Vec<TGRecords>)->String{
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

    pub async fn insert_new_ncts(conn:&Connection,filtered_data:&Vec<TGRecords>) -> String {
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



    pub async fn  load_tg_monitoring(conn:&Connection,filtered_data:&Vec<TGRecords>){
   
        let _res= TgRapportConnection::insert_new_klant(&conn, &filtered_data).await;
      let _res1= TgRapportConnection::insert_new_ncts(&conn, &filtered_data).await;
    
    
        let klants= match Klant::select_all(&mut &conn.db).await{
            Ok(res)=>res,
            Err(err)=>panic!("Klant not found{:#?}",err)
        };
      
        let status = match  NctsStatus::select_all(&mut &conn.db).await{
            Ok(res)=>res,
            Err(err)=>panic!("Klant not found{:#?}",err),
        };
    
        for items in filtered_data{
    
                //get klant id 
                let klants = klants.iter().find(|s|s.klantnaam == items.klant).unwrap().clone();
                let klantid = klants.id.unwrap();
                let date_only = NaiveDate::parse_from_str(&items.declaratie_datum, "%d/%m/%Y").unwrap();
      
                let dossier = Dossier{
                    id:None,
                    mrn:items.mrn.clone(),
                    declaratie_datum:date_only.to_string(),
                    fk_landscode_id:None,
                    fk_klant_id:klantid,
                    manually_created:0,
                };
                match  Dossier::insert(&mut &conn.db, &dossier).await{
                    Ok(res)=>{  println!("Record Successfully inserted");
                                res
                            },
                    Err(_)=>panic!("Could not insert dossier"),
                };
    
                //get latest dossier id
                let dossierid: u64 = conn.db
                .query_decode("select top 1 id from dossier order by id desc", vec![])
                .await
                .unwrap();
    
                //get ncts Status id
                let ncts_status= status.iter().find(|s|s.status == items.ncts_status).unwrap().clone();
                
                        //insert tg_rapport
                        let tgrapport = TgRapport{
                        id:None,
                        fk_dossier_id:dossierid,
                        fk_ncts_status_id:ncts_status.id,
                        notitie: None,
                        referentie:None
                    };
    
                match TgRapport::insert(&mut &conn.db, &tgrapport).await{
                    Ok(res)=>{
                        println!("Record successfully inserted");
                        res
                    },
                    Err(err)=>{
                        println!("Record could not be inserted{}",err);
                        std::process::exit(0);
                    }
                };
        }
      
     }
}