fn main() {
    let words = ["first", "apple", "hello", "world", "Egg"];
    for word in &words {
        println!("{word} → {}", to_pig_latin(word));
    }
}

fn is_vowel(c: char) -> bool {
    let lower = c.to_ascii_lowercase();
    lower == 'a' || lower == 'e' || lower == 'i' || lower == 'o' || lower == 'u'
}

fn to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {
        None => String::new(),
        Some(first) if is_vowel(first) => format!("{word}-hay"),
        Some(first) => {
            let rest = chars.as_str();
            format!("{rest}-{first}ay")
        }
    }
}
