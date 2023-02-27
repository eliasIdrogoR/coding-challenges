pub fn sorted_squared_array(arr: &mut [i32]) -> Vec<i32> {

    let mut result = vec![];

    for i in 0..arr.len(){

        result.push(arr[i] * arr[i]);
    }

    result.sort();

    return result
}