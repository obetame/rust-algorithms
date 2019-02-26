use time::PreciseTime;
use array::swap;

fn adjust(val: &mut Vec<i32>, i: usize) {
    let mut index = i;
    let mut parent_index = (index - 1) / 2;
    println!("i: {}, parent_index: {}", i, parent_index);

    while parent_index >= 0 {
        let left_index = parent_index * 2 + 1;
        let right_index = parent_index * 2 + 2;
        println!("left: {}, right: {}, parent: {}", left_index, right_index, parent_index);

        if right_index >= val.len() {
            // only left child
            if val[left_index] > val[parent_index] {
                println!("left: {}, parent: {}", left_index, parent_index);
                swap(val, left_index, parent_index);
            }
        } else {
            let mut result_index = left_index;
            if val[left_index] < val[right_index] {
                result_index = right_index;
            }
            if val[result_index] > val[parent_index] {
                println!("result: {}, parent: {}", result_index, parent_index);
                swap(val, result_index, parent_index);
            }
        }

        if parent_index == 0 {
            break;
        }

        index = left_index - 2;
        parent_index = (index - 1) / 2;
    }
}

// 堆
fn head(val: &Vec<i32>) -> Vec<i32> {
    let mut value = vec![5, 6, 5, 9, 8, 9, 6, 5, 1, 7];
    let len = value.len();
    println!("{:?}", value);

    for i in (len / 2..len).rev() {
        adjust(&mut value, i);
    }
    println!("{:?}", value);
    value
}

pub fn test(array_value: &Vec<i32>) {
    println!("Start test head sort");

    let start_head = PreciseTime::now();
    head(&array_value);
    let end_head = PreciseTime::now();
    println!("head take time {}", start_head.to(end_head));
}