fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn max_of_three(a: i32, b: i32, c: i32) -> i32
{
    let mut max = a;

    if b > max {
        max = b;
    }
    if c > max {
        max = c;
    }

    max
}

fn sum_of_even(n: u32) -> u32
{
    let mut sum: u32 = 0;
    let mut i = 0;

    while i <= n{
        if (i % 2) == 0{
            sum = sum + i;
        }
        i += 1;
    }
    sum
}

fn main() {
    let a = 5;
    let b = 15;
    let c = 55;

    println!("\nExercise 1:");
    for i in 1..=10 {
        println!("{} is even: {}", i, is_even(i));
    }
    println!("\nExercise 2:");
    println!("Max of three: {}", max_of_three(a, b, c));
    println!("\nExercise 3:");
    for i in 1..=10 {
        for j in 1..=10 {
            println!("{} x {} = {}", i, j, i*j)
        }
    }
    println!("\nExercise 4:");
    println!("Suma parnih brojeva do 20 je: {}", sum_of_even(20));
    println!("\nExercise 5:");
    for i in 1..=100{
        if (i % 3) == 0 && (i % 5) == 0
        {
            println!("FizzBuzz");
        }else if (i % 3) == 0
        {
            println!("Fizz");
        }else if (i % 5) == 0
        {
            println!("Buzz");
        }else {
            println!("{}", i);
        }
    }
}