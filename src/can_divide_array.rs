pub(crate) fn can_divide_array() {
    let mut divisible;

    let array: Vec<i32> = vec![1, 2, 5, 7, 5, 3, 1, 8];

    let array_length = array.len();

    // Ensure array has an even number of elements
    if array_length % 2 != 0 {
        divisible = false
    } else {
        // Split the array into left and right sides
        let (left_side, right_side) = array.split_at(array_length / 2);

        // Sum both sides and compare
        divisible = left_side.iter().sum::<i32>() == right_side.iter().sum::<i32>()
    }

    if divisible {
        println!("array is equal on both sides");
    } else {
        println!("array is not equal on both sides");
    }
}
