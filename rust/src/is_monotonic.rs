pub fn is_monotonic(array: &mut [i32]) -> bool {

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..array.len(){

        if array[i] < array[i - 1]{
            is_increasing = false;
        }

        if array[i] > array[i - 1]{
            is_decreasing = false;
        }
    }

    return is_increasing || is_decreasing
}