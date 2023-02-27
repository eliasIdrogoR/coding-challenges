pub fn three_number_sum(arr: &mut [i32], target_sum: i32) -> Vec<[i32; 3]> {
    arr.sort();
    
    let mut triple_sum_arr = Vec::new();

    for i in 0..arr.len() - 2 {

        let mut left = i + 1;
        let mut right = arr.len() - 1;

        while left < right {
            let sum = arr[i] + arr[left] + arr[right];
            if sum == target_sum {
                triple_sum_arr.push([arr[i], arr[left], arr[right]]);
                left += 1;
                right -= 1;
            } else if sum < target_sum {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    return triple_sum_arr
}