use std::fs;

pub struct Config {
    pub address: String,
    pub is_unix_address: bool,
    pub pg_config: String,
}

impl Config {
    fn make_from_json(fs_read_result: std::io::Result<String>) -> Self {
        match fs_read_result {
            Ok(cfg_json) => {
                let config: serde_json::Value = serde_json::from_str(&cfg_json).unwrap();
    
                let unix_specified: bool;
                Self {
                is_unix_address:
                    match config["unix"].as_bool() {
                        Some(b) => {
                            unix_specified = true;
                            b
                        },
                        None => {
                            unix_specified = false;
                            false
                        },
                    },
                
                address:
                    match config["address"].as_str() {
                        Some(s) => s.to_string(),
                        None => {
                            if unix_specified {
                                panic!("If \"unix\" is specified then you have to specify.")
                            }
                            else {
                                "127.0.0.1:8088".to_string()
                            }
                        },
                    },
    
                pg_config:
                    match config["pgConfig"].as_str() {
                        Some(s) => s.to_string(),
                        None => panic!("No \"pgConfig\" field found in the file \"Config.json\"."),
                    },
                }
            },
            Err(_) => panic!("Failed to read a file."),
        }
    }

    pub fn read(path: &str) -> Self {
        Self::make_from_json(fs::read_to_string(&path))
    }
}
