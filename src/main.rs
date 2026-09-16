fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
fn main() {
    let number = 4;
    println!("Is {} even? {}", number, is_even(number));
}
