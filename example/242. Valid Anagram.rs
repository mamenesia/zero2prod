fn main() {
    is_anagram("anagram".to_string(), "nagaram".to_string());
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut char_count = [0i32; 26];

    // Count characters from both strings
    for (s_char, t_char) in s.bytes().zip(t.bytes()) {
        char_count[(s_char - b'a') as usize] += 1;
        char_count[(t_char - b'a') as usize] -= 1;
    }

    // Check if all counts are zero
    char_count.iter().all(|&count| count == 0)
}
