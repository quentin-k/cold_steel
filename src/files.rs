pub mod files {
    use std::fs::create_dir_all;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{self, BufRead};
    use std::path::Path;

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    pub fn get_lines_from<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    // Write data to a file
    pub fn write_data(data: &[u8], dest: String) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&dest);
        let mut file = match File::create(&path) {
            Err(e) => panic!(crate::config::e_full(e, "create file", &dest)),
            Ok(file) => file,
        };
        match file.write_all(data) {
            Err(e) => panic!(crate::config::e_full(e, "write to", &dest)),
            Ok(_) => (),
        }
        Ok(())
    }

    pub fn create_dir(dest: String) {
        match create_dir_all(dest.clone()) {
            Err(e) => crate::config::e_full(e, "create directory", &dest),
            _ => (),
        };
    }
}
