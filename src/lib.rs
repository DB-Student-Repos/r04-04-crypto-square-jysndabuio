pub fn encrypt(input: &str) -> String {
    // Check for empty string
    if input.is_empty() {
        return String::new()
    }

    // Normalized string input
    let norm_text: String = input.chars()
        .filter(|&x| x.is_ascii() && x.is_alphanumeric())
        .map(|x| x.to_lowercase().to_string())
        .collect();
                        
    // Calculate column
    let norm_text_len = norm_text.len();
    let col: usize = (norm_text_len as f64).sqrt().ceil() as usize;
    //let row: usize = norm_text_len / col;
    //let row: usize = (row as f64).ceil() as usize;

    //Slice the string to size of the column.
    let encrypt_text: Vec<String> = norm_text.chars()
        .collect::<Vec<char>>()
        .chunks(col)
        .map(|x| x.iter().collect::<String>())
        .collect();

    //For the encription, combining all character of ecery index of each slice.
    let mut encrypted_text = String::new();

    // Iterate through each index up to the maximum length
    for i in 0..col {
        // Collect characters at index i from each string in encrypt_text and join them into a new string
        let grouped_chars: String = encrypt_text.iter().filter_map(|string| string.chars().nth(i)).collect();
        
        // Append the grouped characters to the encrypted text
        encrypted_text.push_str(&grouped_chars);
        // Append a space after each group of characters from each string
        if i != col - 1 {
            encrypted_text.push(' ');
        }
    }
    encrypted_text
}
