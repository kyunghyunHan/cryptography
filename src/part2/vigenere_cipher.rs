use std::io;

pub fn main() {
    let mut str_input = String::new();
    let mut key_input = String::new();
    let mut select = String::new();


    println!("Input Cipher Text / Plain Text");
    io::stdin()
        .read_line(&mut str_input)
        .expect("Failed to read input");

    println!("Encrypt : 1, Decrypt : 2");
    io::stdin()
        .read_line(&mut select)
        .expect("Failed to read input");

    let select: u32 = select.trim().parse().expect("Invalid selection");

    println!("Input key (lower case)");
    io::stdin()
        .read_line(&mut key_input)
        .expect("Failed to read input");

    let str_input = str_input.trim();
    let key_input = key_input.trim();

    let str_size = str_input.len();
    let key_size = key_input.len();

    let str_bytes = str_input.as_bytes();
    let key_bytes = key_input.as_bytes();

    let mut result = String::new();

    for i in 0..str_size {
        let j = i % key_size; // Key stream index

        if select == 1 {
            // Encryption
            if str_bytes[i].is_ascii_lowercase() {
                let str_val = str_bytes[i] - b'a';
                let key_val = key_bytes[j] - b'a';

                let mut encrypted_val = str_val + key_val;

                if encrypted_val >= 26 {
                    encrypted_val %= 26;
                }

                result.push((encrypted_val + b'a') as char);
            } else if str_bytes[i].is_ascii_uppercase() {
                let str_val = str_bytes[i] - b'A';
                let key_val = key_bytes[j] - b'a';

                let mut encrypted_val = str_val + key_val;

                if encrypted_val >= 26 {
                    encrypted_val %= 26;
                }

                result.push((encrypted_val + b'A') as char);
            } else {
                result.push(str_input.chars().nth(i).unwrap());
            }
        } else if select == 2 {
            // Decryption
            if str_bytes[i].is_ascii_lowercase() {
                let str_val = str_bytes[i] - b'a';
                let key_val = key_bytes[j] - b'a';

                let mut decrypted_val = str_val - key_val;

                if decrypted_val < 0 {
                    decrypted_val += 26;
                }

                result.push((decrypted_val + b'a') as char);
            } else if str_bytes[i].is_ascii_uppercase() {
                let str_val = str_bytes[i] - b'A';
                let key_val = key_bytes[j] - b'a';

                let mut decrypted_val = str_val - key_val;

                if decrypted_val < 0 {
                    decrypted_val += 26;
                }

                result.push((decrypted_val + b'A') as char);
            } else {
                result.push(str_input.chars().nth(i).unwrap());
            }
        }
    }

    println!("Encrypt or Decrypt Data:");
    println!("{}", result);
}