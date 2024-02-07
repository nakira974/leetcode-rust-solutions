///
/// # See
/// https://leetcode.com/problems/remove-element
///
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // Start with a mutable reference to the start of the vector
    let mut i = 0;
    while i < nums.len() {
        // If the current value matches the value to be removed
        if nums[i] == val {
            // Remove the element at the current index
            // This will shift all elements to the left
            nums.remove(i);
        } else {
            // If the current value doesn't match the value to be removed
            // Move on to the next element
            i += 1;
        }
    }
    // Return the new length of the vector
    return nums.len() as i32;
}

///
/// # See
/// https://leetcode.com/problems/merge-sorted-array
///
pub fn merge(nums1: &mut [i32], m: i32, nums2: &[i32], n: i32) {
    let mut res = Vec::with_capacity((m + n) as usize);
    let mSize: usize = m as usize;
    let nSize: usize = n as usize;
    let mut i : usize = 0;
    let mut j : usize = 0;

    while i < mSize && j < nSize {
        if nums1[i] <= nums2[j] {
            res.push(nums1[i]);
            i += 1;
        } else {
            res.push(nums2[j]);
            j += 1;
        }
    }

    while i < mSize {
        res.push(nums1[i]);
        i += 1;
    }

    while j < nSize {
        res.push(nums2[j]);
        j += 1;
    }

    nums1[..mSize + nSize].copy_from_slice(&res[..mSize + nSize]);
    res.clear();
}

///
/// # See
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
///
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut count: usize = 0; // Variable to keep track of the count of duplicate elements
    let mut current_element = nums[0]; // Variable to store the current element being compared

    for i in 1..nums.len() { // Loop through the vector starting from the second element
        if current_element == nums[i] { // If the current element is equal to the previous element
            count += 1; // Increment the count of duplicates
        } else {
            nums[i - count] = nums[i]; // Move the non-duplicate element to its correct position
            current_element = nums[i]; // Update the current element being compared
        }
    }

    return (nums.len() - count) as i32; // Return the length of the vector after removing duplicates
}

///
/// # See
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
///
pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
    let mut current_counter: i32 = 1;
    let mut current_index : usize = 0;
    for i in 1..nums.len() {
        if nums[current_index] == nums[i] && current_counter <2{
            current_index+=1;
            current_counter +=1;
            nums[current_index] = nums[i];
        }else if(nums[current_index] !=   nums[i]){
            current_counter = 1;
            current_index+=1;
            nums[current_index] = nums[i];
        }
    }
    return (current_index+1) as i32;
}