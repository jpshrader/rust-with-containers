use std::io;

pub fn get_name() -> Result<String, io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n == 0 {
                return Err(io::Error::new(io::ErrorKind::Other, "No input"));
            }
            let name = input.trim().to_string();
            return Ok(name);
        }
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Error reading name"));
        }
    }
}