pub fn smallest_difference(arr1: &mut [i32],arr2: &mut [i32]) -> Vec<i32>{

    arr1.sort();
    arr2.sort();

    let (mut idx_one, mut idx_two) = (0, 0);

    let (mut smallest, mut current) = (i32::MAX, i32::MAX);

    let mut smallest_pair = vec![];

    while idx_one < arr1.len() && idx_two < arr2.len() {
        let first_num = arr1[idx_one];
        let second_num = arr2[idx_two];

        if first_num < second_num {
            current = second_num - first_num;
            idx_one += 1;
        } else if second_num < first_num {
            current = first_num - second_num;
            idx_two += 1;
        } else {
            return vec![first_num, second_num];
        }

        if smallest > current {
            smallest = current;
            smallest_pair = [first_num, second_num].to_vec();
        }
    }

    return smallest_pair;
}