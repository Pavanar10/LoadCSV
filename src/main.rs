use std::env;

pub mod models;
use crate::models::land::Land;
use crate::models::ncts_status::NctsStatus;
use crate::models::ta_monitoring::read_ta_monitoring_csv;
use crate::models::ta_rapport::TaRapportConnection;
use crate::models::tg_monitoring::read_tg_monitoring_csv;
use crate::models::tg_rapport::TgRapportConnection;
use crate::models::{ta_rapport::TaRapport, tg_rapport::TgRapport};
use csv::StringRecord;
use models::connection::Connection;
use models::csv_model::ReadCSV;
use models::dossier::Dossier;
use models::klant::Klant;
use models::tg_rapport::TgRapportView;
use rbatis::crud;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str;
use std::time::Instant;
use toml::Value;

crud!(Dossier {});
crud!(Klant {});
crud!(TgRapport {});
crud!(NctsStatus {});
crud!(Land {});
crud!(TaRapport {});
crud!(TgRapportView {});

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /*let args:Vec<String> = env::args().collect();
    println!("File :{}",args[0]);

    if args.len() < 2 {
        println!("Usage {} <file_path>",args[0]);
    }

    let filepath = &args[1];
    println!("Filename{}",filepath);*/

    let config_content = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config = config_content
        .parse::<Value>()
        .expect("Failed to parse config file");

    let filepath = config["file"]["path"]
        .as_str()
        .expect("Missing path in config.toml");

    impl ReadCSV {
        pub fn new(id: u64, filepath: &str, data: Vec<StringRecord>) -> ReadCSV {
            Self {
                id: id,
                filename: filepath.to_string(),
                data: data,
            }
        }
    }
    let mut ncts_file = ReadCSV::new(1, &filepath, Vec::new());
    println!("{:#?}", ncts_file);
    if Path::new(&ncts_file.filename).exists() {
        let conn: Connection = Connection::create_and_init();
        println!("Connection{:#?}", conn);

        //measure time spent reading csv file
        let start = Instant::now();

        //load ta
        match read_ta_monitoring_csv(&ncts_file.filename, &conn).await {
            Ok(csv_data) => {
                ncts_file.data = csv_data.clone();
            }
            Err(err) => {
                println!("{}", err);
            }
        }

        match TaRapportConnection::check_for_existing_mrns(&ncts_file.filename, &conn).await {
            Ok(r) => r,
            Err(err) => {
                panic!("{}", err);
            }
        };

        //load tg

        match read_tg_monitoring_csv(&ncts_file.filename, &conn).await {
            Ok(csv_data) => {
                ncts_file.data = csv_data.clone();
            }
            Err(err) => {
                println!("{}", err);
            }
        }
        match TgRapportConnection::check_for_existing_mrns(&ncts_file.filename, &conn).await {
            Ok(r) => {
                let res = fs::rename(
                    filepath,
                    "C:/customs/backend/tatg_parser/ta_tg_old/NCTSREPORT OLD.csv",
                );
                println!("Moved File{:#?}", res);
                r
            }
            Err(err) => {
                panic!("{}", err);
            }
        };
        println!("CSV Row count: {}", ncts_file.data.len());
        let duration = start.elapsed();
        println!("Time elapsed in read_csv_file() is: {:?}", duration);
    } else {
        println!("File {:#?} does not exist", filepath);
    }

    Ok(())
}
