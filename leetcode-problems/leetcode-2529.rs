//use std::cmp;
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
    /*    
        Attempt One: Just using the simplest Brute Force Approach (Uncomment use std::cmp if running this attempt)
        Take Aways: 
        - Didn't really learn too much except the syntax of max function 

        let mut neg = 0;
        let mut pos = 0;
        for n in nums{
            if n < 0{
                neg += 1;
            }
            else if n > 0{
                pos += 1;
            }
        }
        cmp::max(pos,neg)
    */

    /*
        Attmept Two: Tried to mix in more Rust specific syntax implementing a minor optimization (Uncomment use std::cmp if running this attempt)
        Take Aways: 
        - For loop counter is in usize (pointer size) something I was not too familiar with 
        - i..n is the formatting for range(i,n) in pyhton
        - if you add an equal to above like i..=n then the range is inclusive which is the same as range(i,n+1) in python
        - has the same incremental pattern as python so no option to do neg++ instead need to do neg += 1 or neg = neg + 1
        - to return early you have to make an explicit statement unlike at the end of the method where its more implicit
        - variables can be given type hints 
        - type casting is done with as keyword but is only allowed with proper casting

        let mut neg : i32 = 0;
        for i in (0..nums.len()){
            if nums[i] > 0{
                return cmp::max(neg, (nums.len() as i32) - (i as i32));
            }
            else if nums[i] < 0{
                neg += 1;
            }
        }
        (nums.len() as i32) - neg
    */

    /*
        Attempt three: Looking at how other do this type of problem, in this case binary search
        Take Aways:
        - there are some built in binary search functions but require a specified value to find which wasn't useful in this case but could be for others
        - partition_point is seen a lot in use for binary search 
        - partition_point goes from left to right stopping at the point where the expression rings out false and return the count of true values 1 indexed
        can do conditional returns with the use of if statements at the end of a function 
    */
        let neg = nums.partition_point(|x| *x < 0);
        let pos = nums.len() - nums.partition_point(|x| *x <= 0);
        if neg > pos { neg as i32 } else { pos as i32 }
    }
}
