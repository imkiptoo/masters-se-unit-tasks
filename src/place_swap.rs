pub(crate) fn reverse_array<T>(arr: &mut [T])
where
    T: Copy,
{
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        // Verbose swap
        let temp = arr[left];
        arr[left] = arr[right];
        arr[right] = temp;

        // Move towards the center
        left += 1;
        right -= 1;
    }
}