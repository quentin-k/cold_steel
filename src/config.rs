// Set the directory for storing screenshots
pub static OUTPUT_DIR: &str = "output"; 

pub fn e_header(error: std::io::Error) {
    println!("\n\n!!!ERROR BEGIN!!!\n{}\n!!!ERROR END!!!\n\n", error);
}

pub fn e_shorthand(e_type: &str, e_for: &str) {
    println!("Unable to {} {}!", e_type, e_for);
}

pub fn e_full(error: std::io::Error, e_type: &str, e_for: &str) {
    e_header(error);
    e_shorthand(e_type, e_for);
}