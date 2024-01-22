mod helpers;

fn main() {
    println!("what is your name?: ");

    match helpers::get_name() {
        Ok(name) => println!("hello, {}!", name),
        Err(e) => println!("error: {}", e),
    }

    let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let length = numbers.len();
    println!("numbers: {:?}", numbers);
    println!("length: {}", length);

    let even_numbers: Vec<i32> = numbers.iter().filter(|n| *n % 2 == 0).cloned().collect();
    let odd_numbers: Vec<i32> = numbers.iter().filter(|n| *n % 2 != 0).cloned().collect();
    println!("iter even numbers: {:?}", even_numbers);
    println!("iter odd numbers: {:?}", odd_numbers);

    let even_nums = get_numbers(&numbers, |n| n % 2 == 0);
    let odd_nums = get_numbers(&numbers, |n| n % 2 != 0);
    println!("even numbers: {:?}", even_nums);
    println!("odd numbers: {:?}", odd_nums);
}

fn get_numbers(input_numbers: &Vec<i32>, predicate: fn(i32) -> bool) -> Vec<i32> {
    let mut result = Vec::new();
    for n in input_numbers {
        if predicate(*n) {
            result.push(*n);
        }
    }
    result
}