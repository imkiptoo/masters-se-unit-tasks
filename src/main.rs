fn main() {
    println!("Hello, world!");

    let array: Vec<i32> = vec![1, 1];

    let equal_on_both_sides = can_divide_array(array);

    if equal_on_both_sides {
        println!("array is equal on both sides");
    } else {
        println!("array is not equal on both sides");
    }
}

fn can_divide_array(array: Vec<i32>) -> bool {
    if(array.len() % 2 != 0) {
        return false;
    }

    // split the array into left and right side
    let array_length = array.len();
    let left_side = &array[array_length / 2..];
    let right_side = &array[..array_length / 2];

    let mut left_side_sum = 0;
    let mut right_side_sum = 0;

    // sum left and right side
    for num in left_side.iter() {
        left_side_sum = left_side_sum + num;
    }

    for num in right_side.iter() {
        right_side_sum = right_side_sum + num;
    }

    // check that left and right side are equal
    if left_side_sum == right_side_sum {
        return true;
    }


    return false;
}
