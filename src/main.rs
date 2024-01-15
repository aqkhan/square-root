use rand::Rng;

fn generate_random_number(min: i32, max: i32, old_value: Option<i32>) -> i32 {
    let old_value = old_value.unwrap_or(max - 1);
    let mut new_random_number = rand::thread_rng().gen_range(min..max);
    while old_value == new_random_number {
        new_random_number = rand::thread_rng().gen_range(min..max);
    }
    return new_random_number;
}

fn square_root(input: i32) -> i32 {
    let mut random_number = generate_random_number(1, input, None);
    let mut iteration = 0;
    while (random_number * random_number) != input {
        iteration = iteration + 1;
        println!("Iteration number: {} | Random number: {}", iteration, random_number);
        random_number = generate_random_number(1, input, Some(random_number));
    }
    return random_number
}

fn main() {
    let input = 81;
    println!("The square root of {} is {}", input, square_root(input));
}
