fn main() {
    println!("Project Euler No.3.");

    let mut number: u64 = 600851475143;
    let mut largest_prime = 0;

    // start checking from the smallest prime number
    let mut factor = 2;

    while factor * factor <= number {
        while number % factor == 0 {
            number /= factor;
            largest_prime = factor;
        }
        factor += 1;
    }

    // if there's any prime factor larger than  the square root
    if number > 1 {
        largest_prime = number;
    }

    println!("The largest prime factor is: {}", largest_prime);
}
