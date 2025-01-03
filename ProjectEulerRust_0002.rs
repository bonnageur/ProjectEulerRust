fn main() {
    println!("Project Euler No.2.");
    
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;

    while b <= 4_000_000 {
        if b % 2 == 0 {
            sum += b;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    println!("The sum of even-valued terms in the Fibonacci sequence below 4,000,000 is: {}", sum);
}