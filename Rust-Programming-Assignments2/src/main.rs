fn is_even(n: i32) -> bool {
    n % 2 == 0
}
fn main() {
    let x = [1, 2, 3, 4, 5, 15, 7, 8, 9, 10];
    for &num in x.iter() {
        if num %3==0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    }
    
    let mut total = 0;
    let mut count = 0;
    while count < x.len() {
        total += x[count];
        count += 1;
    }
    println!("Sum: {}", total);

    let mut largest = x[0];
    for &num in x.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest: {}", largest);
}