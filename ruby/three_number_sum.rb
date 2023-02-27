def three_number_sum(arr,target)

    j = 0
    k = arr.length - 1
    triple_pair = []

    for i in 0..arr.length - 2

        while j < k

            sum = arr[i] + arr[j] + arr[k]

            if sum == target
                triple_pair.push(arr[i],arr[j],arr[k])
                j+=1
                k-=1
                
            elsif sum > target
                k -= 1
            else
                j += 1
            end

        end

    end

    return triple_pair
end

# Driver code
arr = [1,2,3,4,5,6,7,8,9,10]
target = 10
p three_number_sum(arr,target)