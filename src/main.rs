use std::io;

fn main() {
    println!("What is your name?: ");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("Hello {}!", input.trim());
        }
        Err(error) => println!("error: {error}"),
    }
}
