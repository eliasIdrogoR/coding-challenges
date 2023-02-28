pub fn reversed_words_in_string(string: &str) -> String {
    let split_string = string.split(" ");
    let mut words: Vec<&str> = split_string.collect();
    words.reverse();
    words.join(" ")
}
