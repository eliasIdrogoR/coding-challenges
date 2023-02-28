pub fn move_element_to_end(array: &mut [i32], to_move: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = array.len() - 1;

    while left < right {

        if array[right] == to_move {
            right -= 1;
            continue;
        }

        if array[left] == to_move {
            array.swap(left, right);
        }
        left += 1;
    }

    array.to_vec()
}
