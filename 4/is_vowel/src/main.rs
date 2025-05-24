fn is_vowel(c: char) {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' |
        'A' | 'E' | 'I' | 'O' | 'U' => println!("Vowel"),
        _ => println!("Consonant or symbol"),
    }
}

fn main() {
    is_vowel('a'); // Vowel
    is_vowel('Z'); // Consonant or symbol
    is_vowel('u'); // Vowel
    is_vowel('1'); // Consonant or symbol
}
