const CONS_SLICE: &[char; 42] = &[
    'q','w','r','t','y','p','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m',
    'Q','W','R','T','Y','P','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M'
];

pub fn convert_to_piglatin(text: &str) -> String {
    let mut piglatin_words = Vec::new();
    // Also need to make it handle punctuation
    let words: Vec<&str> = text.split(' ').collect();
    for word in words {
        let mut piglatin_word = String::from(word);
        if piglatin_word.starts_with(CONS_SLICE) {
            let first_char = piglatin_word.remove(0);
            piglatin_word = piglatin_word + &format!("{first_char}ay");
        } else {
            if word.len() > 0 {
                piglatin_word.push_str("hay");
            }
        }
        piglatin_words.push(piglatin_word);
    }
    piglatin_words.join(" ")
}