pub fn is_palindrome(string: &str) -> bool {

    let string = string.to_lowercase();

    let mut chars: Vec<char> = string.chars().collect();

    chars.reverse();

    let reversed_string: String = chars.into_iter().collect();

    string == reversed_string
}