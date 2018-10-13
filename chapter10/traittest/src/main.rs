//use std::num::trait::Signed;

pub trait GoodBadUgly{
    fn good(&self) -> String;
    fn bad(&self) -> String;
    fn ugly(&self) -> String {
        String::from("Nothing is ugly, everything is beautiful!")
    }
    fn all_things(&self) -> String {
        format!("The good: {}\nThe bad: {}\nThe ugly: {}", self.good(), self.bad(), self.ugly())
    }
}

impl GoodBadUgly for char {
    fn good(&self) -> String {
        String::from("It's nice and little")
    }
    fn bad(&self) -> String {
        String::from("It's too little")
    }
}

//impl<T: Signed> GoodBadUgly for T {
//    fn good(&self) -> String {
//        String::from("Numbers can go backwards")
//    }
//    fn bad(&self) -> String {
//        String::from("Nobody likes a negative number, it's all about being positive")
//    }
//}

impl GoodBadUgly for i128 {
    fn good(&self) -> String {
        String::from("This can be big!")
    }
    fn bad(&self) -> String {
        String::from("Nobody likes a negative number, but it's so big we'll let it off")
    }
}

fn get_gbu<T> (thing: T) -> String
    where T: GoodBadUgly {
    thing.all_things()
}

fn main() {
    println!("\nThe good, bad and ugly for i128");
    let a: i128 = -123456789;
    println!("{}", get_gbu(a));

    println!("\nThe good, bad and ugly for char");
    let b: char = 'h';
    println!("{}", get_gbu(b));
}
