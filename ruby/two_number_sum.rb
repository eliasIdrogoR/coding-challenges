def two_number_sum(arr,target)

    i = 0
    j = arr.length - 1
    double_pair = []

    while i < j
            
            sum = arr[i] + arr[j]
    
            if sum == target
                double_pair.push(arr[i],arr[j])
                i+=1
                j-=1
            elsif sum > target
                j -= 1
            else
                i += 1
            end
    
        end

    return double_pair
end

# Driver code
arr = [1,2,3,4,5,6,7,8,9,10]
target = 10
p two_number_sum(arr,target)