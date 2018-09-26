use std::io;

fn main() {
    println!("I'll tell you the nth Fibonacci number");

    loop {
        println!("Give me an n:");
        let mut n = String::new();

        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let nth_fib = find_nth_fib(&n);

        println!("{}", &nth_fib);
        break;
    }
}

fn find_nth_fib(n: & u32) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 1;
    let mut count: u32 = 2;

    while count < *n {
        let temp = a + b;
        a = b;
        b = temp;
        count = count + 1;
    }

    b
}

