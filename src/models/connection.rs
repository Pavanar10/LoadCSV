use rbatis::Rbatis;
use rbdc_mssql::driver::MssqlDriver;
use std::fs;
use toml::Value;

#[derive(Debug)]
pub struct Connection {
    pub db: Rbatis,
}

impl Connection {
    fn new() -> Connection {
        Self {
            db: Rbatis::new(),
        }
    }
    fn init(&self) {
        let conf: ConnectionConfig = ConnectionConfig::new();
        self.db.init(
            MssqlDriver {},
            format!(
                "jdbc:sqlserver://{}:{};User={};Password={};Database={}",
                conf.server, conf.port, conf.user, conf.password, conf.database
            )
                .as_str()
        )
            .unwrap();
    }

    pub fn create_and_init() -> Connection {

        let conn: Connection = Connection::new();

        conn.init();
        conn
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct ConnectionConfig {
    server: String,
    port: String,
    user: String,
    password: String,
    database: String,
}

impl ConnectionConfig {
    pub fn new() -> ConnectionConfig {
        let config_content = fs::read_to_string("config.toml").expect("Failed to read config.toml");
        let config = config_content.parse::<Value>().expect("Failed to parse config file");

    let server = config["serverDetails"]["server"].as_str().expect("Missing path in config.toml");
    let port = config["serverDetails"]["port"].as_str().expect("Missing path in config.toml");
    let user = config["serverDetails"]["user"].as_str().expect("Missing path in config.toml");
    let password = config["serverDetails"]["password"].as_str().expect("Missing path in config.toml");
    let database = config["serverDetails"]["database"].as_str().expect("Missing path in config.toml");

        Self {
            server: String::from(server),
            port: String::from(port),
            user: String::from(user),
            password: String::from(password),
            database: String::from(database),
        }
    }
}
