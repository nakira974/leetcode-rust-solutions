pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
