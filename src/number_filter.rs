pub fn filter(input_numbers: &Vec<i32>, predicate: fn(i32) -> bool) -> Vec<i32> {
    let mut result = Vec::new();
    for n in input_numbers {
        if predicate(*n) {
            result.push(*n);
        }
    }
    result
}