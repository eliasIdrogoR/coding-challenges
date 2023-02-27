struct BST {
    value: i32,
    left: Option<Box<BST>>,
    right: Option<Box<BST>>,
}

impl BST{
    fn new(value: i32) -> Self {
        BST {
            value,
            left: None,
            right: None,
        }
    }
}

fn find_closest_value_in_bst(tree: &BST, target: i32) -> i32 {
    find_closest_value_in_bst_helper(tree, target, tree.value)
}

fn find_closest_value_in_bst_helper(tree: &BST, target: i32, mut closest: i32) -> i32 {
    let mut current_node = Some(tree);

    while let Some(node) = current_node {
        if (target - closest).abs() > (target - node.value).abs() {
            closest = node.value;
        }

        if target < node.value {
            current_node = node.left.as_ref().map(|n| &**n);
        } else if target > node.value {
            current_node = node.right.as_ref().map(|n| &**n);
        } else {
            break;
        }
    }

    closest
}

