mod two_number_sum;

fn main() {
    let mut array = [3, 5, -4, 8, 11, 1, -1, 6];
    let target_sum = 10;
    let result = two_number_sum::two_number_sum(&mut array, target_sum);
    println!("{:?}", result);
}
