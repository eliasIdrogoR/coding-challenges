pub fn two_number_sum(array: &mut [i32], target_sum: i32) -> Option<(i32, i32)> {

    array.sort();

    let mut left = 0;
    let mut right = array.len() - 1;

    while left < right {
        let sum = array[left] + array[right];
        if sum == target_sum {
            return Some((array[left], array[right]));
        } else if sum < target_sum {
            left += 1;
        } else {
            right -= 1;
        }
    }

    None
}