mod helpers;

fn main() {
    println!("What is your name?: ");

    match helpers::get_name() {
        Ok(name) => println!("Hello, {}!", name),
        Err(e) => println!("Error: {}", e),
    }
}
