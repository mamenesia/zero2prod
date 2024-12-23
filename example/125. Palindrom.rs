fn main() {
    is_palindrome(String::from("A man, a plan, a canal: Panama"));
    is_palindrome(String::from("race a car"));
    is_palindrome(String::from("0P"));
}

fn is_palindrome(s: String) -> bool {
     let mut s = s.to_lowercase();
        let chars: Vec<char> = s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    
    if chars.is_empty() {
        return true;
    }
        let (mut left, mut right) = (0, chars.len() - 1);
        
        while left < right {
             if chars[left] != chars[right] {
            return false;
            }
        left += 1;
        right -= 1;
        }

        true
}
