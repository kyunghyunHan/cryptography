use std::io;
/*고전암호 */
pub fn main() {
    let mut input = String::new();
    let mut key_str = String::new();

    println!("Input Plain Text / Cipher Text");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    println!("Input key");
    io::stdin().read_line(&mut key_str).expect("Failed to read key");

    let key: i32 = key_str.trim().parse().expect("Invalid key");

    let mut result = String::new();

    for c in input.chars() {
        if c.is_ascii_lowercase() {
            let mut val = c as i32 - 'a' as i32;
            if val + key < 0 {
                val += 26;
            }
            val = (val + key) % 26;
            val += 'a' as i32;
            result.push(val as u8 as char);
        } else if c.is_ascii_uppercase() {
            let mut val = c as i32 - 'A' as i32;
            if val + key < 0 {
                val += 26;
            }
            val = (val + key) % 26;
            val += 'A' as i32;
            result.push(val as u8 as char);
        } else {
            result.push(c);
        }
    }

    println!("암호화 또는 복호화된 결과 출력");
    println!("{}", result);

}
