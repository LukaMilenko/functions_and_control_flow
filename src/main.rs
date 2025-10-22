fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    for i in 1..=10 {
        println!("{} is even: {}", i, is_even(i));
    }
}