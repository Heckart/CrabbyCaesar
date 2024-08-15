pub fn caesar_translate(input_text: String, mode: String, key: isize) -> String {
    // Purpose:    Does the Caesar cipher logic.
    // Parameters: input_text to translate,
    //             mode (encrypt or decrypt), and
    //             key (in size).
    // User Input: None.
    // Prints:     None.
    // Returns:    Translated text as a std::String.
    // Modifies:   Nothing.
    // Calls:      Pure rust, no imports. (rem_euclid)
    // Tests:      ./unit_tests/*
    // Status:     Done
    // asserteq!(caesar_translate("abc".to_string(), "encrypt".to_string(), 1), "bcd".to_string())
    // asserteq!(caesar_translate("bcd".to_string(), "decrypt".to_string(), 1), "abc".to_string())

    let symbols: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

    let mut translated: String = String::new();

    let symbols_len: isize = symbols.len() as isize;

    for symbol in input_text.chars() {
        if let Some(symbol_index) = symbols.find(symbol) {
            // https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/option/index.html
            let translated_index = match mode.as_str() {
                // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
                "encrypt" => (symbol_index as isize + key).rem_euclid(symbols_len),
                "decrypt" => (symbol_index as isize - key).rem_euclid(symbols_len),
                _ => symbol_index as isize,
            };

            translated.push(symbols.chars().nth(translated_index as usize).unwrap());
        } else {
            translated.push(symbol);
        }
    }
    return translated;
}
