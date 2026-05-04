use std::env;
use std::process;
use std::fs::{File, OpenOptions};
use std::io::Write;

struct FileHandler {
    file: std::fs::File
}

impl FileHandler {
    fn load_output_txt_path() -> String {
        dotenvy::dotenv().ok();

        match env::var("outputTxtPath") {
	    Ok(val) => {
	        val
	    }
	    Err(e) => {
	        eprintln!("Error while reading `outputTxtPath` from .env: {}", e);   
	        process::exit(1);
	    }
        }
    }
}


