use std::env;

extern crate time;
extern crate rand;

mod array;
mod bubble;
mod select;

fn main() {
    if let Some(name) = env::args().nth(1) {
//        let array_value: Vec<i32> = array::init();
        let array_value = vec![0, 5, 4, 1, 8, 0, 7, 0, 2, 3];

        match name.as_ref() {
            "bubble" => bubble::test(&array_value),
            "select" => select::test(&array_value),
            _ => println!("please input sort name")
        }
    }
}


