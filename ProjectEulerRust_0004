fn is_palindrome(num: u32) -> bool {
    let s = num.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    println!("Project Euler No4.");

    let mut largest_palindrome = 0;

    // iterate over all 3-digit numbers in reverse order (optimization)
    for i in (100..=999).rev() {
        for j in (i..=999).rev() {     // start from 'i' to avoid duplicate calculations
            let product = i * j;
            if product <= largest_palindrome {
                break;                 // no need to check further if the product is smaller than the largest found
            }
            if is_palindrome(product) {
                largest_palindrome = product;
            }
        }
    }

    println!("The largest palindrome made from the product of two 3-digit numbers is: {}", largest_palindrome);
}
