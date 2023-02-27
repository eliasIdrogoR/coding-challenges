def find_closest_value_in_bst(tree,target)

    return tree.value if tree.left.nil? && tree.right.nil?
    
    if target < tree.value
        return tree.value if tree.left.nil?
        left = find_closest_value_in_bst(tree.left,target)
    else
        return tree.value if tree.right.nil?
        right = find_closest_value_in_bst(tree.right,target)
    end

    return right if left.nil? 
    return left if right.nil?
    return left if (target - left).abs < (target - right).abs
    return right

end

# This is the class of the input tree. Do not edit.

class BST

    attr_accessor :value, :left, :right

    def initialize(value)

        @value = value

        @left = nil

        @right = nil

    end

end

#testing the code

tree = BST.new(10)

tree.left = BST.new(5)

tree.left.left = BST.new(2)

tree.left.left.left = BST.new(1)

tree.left.right = BST.new(5)

tree.right = BST.new(15)

tree.right.left = BST.new(13)

tree.right.left.right = BST.new(14)

tree.right.right = BST.new(22)

puts find_closest_value_in_bst(tree,12)

