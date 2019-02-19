extern crate time;
extern crate rand;

mod array;
mod bubble;

// 选择
//fn select(val: &Vec<i32>) -> Vec<i32> {
//
//}

fn main() {
    let array_value: Vec<i32> = array::init();

    bubble::test(&array_value);
}


