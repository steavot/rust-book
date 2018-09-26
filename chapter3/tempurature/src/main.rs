use std::io;

const ABSOLUTE_ZERO_F: f64 = -465.67;

fn main() {
    loop {
        println!("Input tempurature in Farenheit.");

        let mut temp_f = String::new();

        io::stdin().read_line(&mut temp_f)
            .expect("Failed to read line");

        let temp_f: f64 = match temp_f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if temp_f < ABSOLUTE_ZERO_F {
            println!("Tempurature {} degrees Farenheit is below absolute zero", temp_f);
            continue;
        } else {
            let temp_c: f64 = farenheit_to_celcius(&temp_f);
            println!("That tempurature in Celcius is {}", temp_c);
            break;
        };
    };
}

fn farenheit_to_celcius(temp_f: & f64) -> f64 {
    (*temp_f - 32.0) / 1.8
}

