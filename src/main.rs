use rand::Rng;

fn generate_random_number(min: i32, max: i32, old_values: &Vec<i32>) -> i32 {
    let mut new_random_number = rand::thread_rng().gen_range(min..max);
    while old_values.contains(&new_random_number) {
        new_random_number = rand::thread_rng().gen_range(min..max);
    }
    return new_random_number;
}

fn square_root(input: i32) -> i32 {
    let mut old_values = vec![generate_random_number(1, input, &Vec::new())];
    let mut iteration = 0;
    while (old_values.last().unwrap() * old_values.last().unwrap()) != input {
        iteration = iteration + 1;
        println!("Iteration number: {} | Random number: {}", iteration, old_values.last().unwrap());
        old_values.push(generate_random_number(1, input, &old_values));
    }
    return *old_values.last().unwrap()
}

fn main() {
    let input = 81;
    println!("The square root of {} is {}", input, square_root(input));
}