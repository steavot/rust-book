use std::io;

fn main() {
    println!("I'll tell you the nth Fibonacci number");

    loop {
        println!("Give me an n:");
        let mut n = String::new();

        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        let n: u128 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n > 186 {
            println!("Too big, give me a smaller n!");
            continue;
        }

        let nth_fib = find_nth_fib(n);

        println!("{}", &nth_fib);
        break;
    }
}

fn find_nth_fib(n: u128) -> u128 {
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    let mut count: u128 = 2;

    while count < n {
        let temp = a + b;
        a = b;
        b = temp;
        count = count + 1;
    }

    b
}

