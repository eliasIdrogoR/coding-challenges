mod smallest_difference;

fn main() {
    // // create a binary search tree
    // let mut root = BST::new(10);
    // root.left = Some(Box::new(BST::new(5)));
    // root.right = Some(Box::new(BST::new(15)));
    // root.left.as_mut().unwrap().left = Some(Box::new(BST::new(2)));
    // root.left.as_mut().unwrap().right = Some(Box::new(BST::new(5)));
    // root.right.as_mut().unwrap().left = Some(Box::new(BST::new(13)));
    // root.right.as_mut().unwrap().right = Some(Box::new(BST::new(22)));

    // let target = 12;
    // let closest_value = find_closest_value_in_bst(&root, target);
    // println!("The closest value to {} is {}", target, closest_value);

    let mut arr1 = [1, 3, 15, 11, 2];
    let mut arr2 = [23, 127, 235, 19, 8];

    let result = smallest_difference::smallest_difference(&mut arr1, &mut arr2);

    println!("Smallest difference is: {:?}", result);
}
