
class BST

    attr_accessor :value, :left, :right

    def initialize(value)

        @value = value

        @left = nil

        @right = nil

    end

end

def branch_sums(root)

    sums = []
    calculate_branch_sums(root,0,sums)
    return sums
end

def calculate_branch_sums(node,running_sum,sums)

    if node.nil?
        return
    end

    new_running_sum = running_sum + node.value

    if node.left.nil? && node.right.nil?
        sums.append(new_running_sum)
        return
    end

    calculate_branch_sums(node.left,new_running_sum,sums)
    calculate_branch_sums(node.right,new_running_sum,sums)

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

puts branch_sums(tree)