use std::io;

fn q3(k: u32) -> u128 {
    let mut sum: u128 = 0;
    for i in 1..=k {
        sum += (i as u128).pow(2)
    }
    sum
}

fn main() {
    println!("Please input your k.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let number: u32 = input.parse().expect("Not a good number!");
    println!("The result is {}", q3(number));
}
