use std::env;
use std::process;
use std::fs::OpenOptions;
use std::io::{Read, Write, Seek, SeekFrom};

pub(crate) struct RemainderFileHandler {
    file: std::fs::File
}

impl RemainderFileHandler {
    fn load_output_txt_path() -> String {
        dotenvy::dotenv().ok();

        match env::var("lastRemainderTxtPath") {
	    Ok(val) => {
	        val
	    }
	    Err(e) => {
	        eprintln!("Error while reading `lastRemainderTxtPath` from .env: {}", e);   
	        process::exit(1);
	    }
        }
    }

    pub(crate) fn new() -> Self {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(RemainderFileHandler::load_output_txt_path())
            .expect("Failed to open remainder file path provided in .env");
        RemainderFileHandler {file}
    }

    pub(crate) fn write(&mut self, text: &str) {
        self.file.seek(SeekFrom::Start(0)).expect("Failed to Seek to file start");
        self.file.set_len(0).expect("Failed to empty file");
        writeln!(self.file, "{}", text).expect("Failed to write to file.");
        self.file.flush().expect("Failed to flush file");
    }

    pub(crate) fn read(&mut self) -> String {
        self.file.seek(SeekFrom::Start(0)).expect("Failed to Seek to file start");
        
        let mut content = String::new();
        let _ = self.file.read_to_string(&mut content); // ignore if fails, we will assume the file
                                                        // is empty in that case
        content
    }
}


