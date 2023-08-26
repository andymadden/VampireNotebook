use std::{fs::File, io::Read};


pub struct DataService {
    header: String
}

impl DataService {
    pub fn new() -> Self {
        let header_file_path = "./resources/header.txt";
        let mut header_file = match File::open(header_file_path) {
            Ok(file) => file,
            Err(_) => panic!("Could not open header file")
        };
        let mut header = String::new();
        match header_file.read_to_string(&mut header) {
            Ok(_) => (),
            Err(_) => panic!("Could not read header file")
        }

        DataService { header }
    }

    pub fn get_header(&self) -> &str {
        &self.header.as_str()
    }
}
