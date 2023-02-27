mod sorted_squared_array;

fn main() {
    let mut arr = vec![-10, -5, 0, 5, 10];
    let result = sorted_squared_array::sorted_squared_array(&mut arr);
    println!("{:?}", result);
}
