fn main() {
    print_some_shit();
    let thing = six();
    println!("thing={}", thing);
}

fn six() -> i32 {
    6
    // println!("macro after expression");
}

fn print_some_shit() {
    println!("Some shit")
}

