use std::env;
use std::process;
use std::fs::OpenOptions;
use std::io::Write;

pub(crate) struct OutputFileHandler {
    file: std::fs::File
}

impl OutputFileHandler {
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

    pub(crate) fn new() -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(OutputFileHandler::load_output_txt_path())
            .expect("Failed to open output file path provided in .env");
        OutputFileHandler {file}
    }

    pub(crate) fn append(&mut self, text: &str) {
        writeln!(self.file, "{}",  text).expect("Failed to append to file.");
        self.file.flush().expect("Failed to flush file");
    }
}


