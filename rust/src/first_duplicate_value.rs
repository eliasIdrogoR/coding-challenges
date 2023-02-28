use std::collections::HashSet;


pub fn first_duplicate_value(array: &mut [i32]) -> i32 {
    let mut seen = HashSet::new();

    for num in array {
        if seen.contains(num) {
            return *num;
        } else {
            seen.insert(num);
        }
    }

    return -1;
}