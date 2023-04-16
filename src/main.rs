use std::io;

fn main() {
    println!("What is your name?: ");

    match get_name() {
        Ok(name) => println!("Hello, {}!", name),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_name() -> Result<String, io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let name = input.trim().to_string();
            return Ok(name);
        }
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Error reading name"));
        }
    }
}
