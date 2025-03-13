impl Solution {
    pub fn is_palindrome(s: String) -> bool {
    /*
        Version 1: Lots of looked up functions
        Take Aways:
        - learned a decent bit of fuctions in rust but need to get further practice on

        let st : String = s.chars() // creates an iterator of all chars in the slice
            .filter(|c| c.is_alphanumeric()) // filters out all now alphanumeric values
            .map(|c| c.to_ascii_lowercase()) // maps upper case to lower case and lowercase to itself
            .collect(); // takes the array and build out one var in this case a String
       st == st.chars().rev().collect::<String>() // rev is the reverse function but can only be done on iterators
    */

    /*
        Version 2: using more of my standard approach with ptrs
        Take Aways:
        - saturating is what you use when afraid of overflow on an operation
            * Examples saturating_sub (-) and saturating_add (+)
        - Strings are not indexable in rust unlike in python

    */
        let chars: Vec<char> = s.chars().collect();
        
        if chars.is_empty() {
            return true;
        }
        
        let mut l = 0;                  
        let mut r = chars.len() - 1;    
        
        while l < r {
            if !chars[l].is_alphanumeric() {
                l += 1;
                continue;
            }
            else if !chars[r].is_alphanumeric() {
                r = r.saturating_sub(1);
                continue
            }
            
            if chars[l].to_ascii_lowercase() != chars[r].to_ascii_lowercase() {
                return false;
            }

            l += 1;
            r = r.saturating_sub(1);
        }

        true
    }
}