use num_traits::abs;

pub(crate) fn minimal_distance() {
    let arr = [4, 9, 15, 22, 30, 39, 49];
    let mut start_of_minimal_distance = 0;
    let mut end_of_minimal_distance = 0;
    let mut minimum_difference = i32::MAX;

    for i in 0..arr.len() {
        if(i > 0){
            let mut delta = arr[i] - arr[i - 1];
            delta = abs(delta);

            if(delta < minimum_difference){
                minimum_difference = delta;
                start_of_minimal_distance = i - 1;
                end_of_minimal_distance = i;
            }
        }
    }

    println!("Minimal Distance: {}\n[{}, {}] at Index {}", minimum_difference, arr[start_of_minimal_distance], arr[end_of_minimal_distance], start_of_minimal_distance)
}