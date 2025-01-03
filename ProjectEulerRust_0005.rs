fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn main() {
    println!("Hello, world!");

    let range = 1..=20;
    let smallest_multiple = range.fold(1, |acc, x| lcm(acc, x));

    println!("The smallest positive number enenly divisible by all numbers from 1 to 20 is: {}", smallest_multiple);
}
