fn main() {
    let input = "ӵaúfirst";

    // 'deref coercing' kicks in in this method call, and when we call letter_is_vowel.
    // i.e. String is converted a &str behind the scenes. &input would also be a valid call
    let (first_letter, remainder) = get_first_letter_and_remainder(input);

    let output: String;

    if character_is_vowel(first_letter) {
        output = input.to_string() + "-hay";
    } else {
        let end_of_word = format!("-{}ay", first_letter);
        output = remainder.to_string() + &end_of_word;
    }
    println!("The pig-latin for: {} is {}", input, output);
}

// Splits the string by accessing the first char, checking it's length in bytes, then
// dividing the string slice into two, around index = length_of_char, parsing the first &str
// into a char type.
fn get_first_letter_and_remainder(input: &str) -> (char, &str) {
    match input.chars().next() {
        Some(c) => {
            let (first_letter, remainder) = input.split_at(c.len_utf8());
            (first_letter.chars().nth(0).unwrap(), remainder)
        }
        None => {
            let (first_letter, remainder) = input.split_at(0);
            (first_letter.chars().nth(0).unwrap(), remainder)
        }
    }
}

// Could/should access a common library of vowels across all languages.
fn character_is_vowel(character: char) -> bool {
    character == 'a' || character == 'e' || character == 'i' || character == 'o' || character == 'u'
}
