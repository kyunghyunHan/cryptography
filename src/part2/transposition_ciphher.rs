use std::io;

const BLOCK_SIZE: usize = 6;

fn main() {
    let mut p_text = String::new();
    let mut c_text = ['x'; 64]; // Initialize with 'x' characters
    let mut d_text = ['x'; 64]; // Initialize with 'x' characters
    let key = [3, 5, 1, 6, 4, 2];

    println!("Input Plain text");
    io::stdin().read_line(&mut p_text).expect("Failed to read input");

    let size = p_text.trim().len();

    let block_num: usize;
    if size % BLOCK_SIZE > 0 {
        block_num = size / BLOCK_SIZE + 1;

        for i in size..block_num * BLOCK_SIZE {
            p_text.push('x');
        }
    } else {
        block_num = size / BLOCK_SIZE;
    }

    for i in 0..block_num {
        for j in 0..BLOCK_SIZE {
            c_text[i * BLOCK_SIZE + j] = p_text.chars().nth(key[j] - 1 + i * BLOCK_SIZE).unwrap();
        }
    }

    println!("Encrypt Data:");
    for i in 0..block_num * BLOCK_SIZE {
        print!("{}", c_text[i]);
    }
    println!();

    for i in 0..block_num {
        for j in 0..BLOCK_SIZE {
            d_text[key[j] - 1 + i * BLOCK_SIZE] = c_text[i * BLOCK_SIZE + j];
        }
    }

    println!("Decrypt Data:");
    for i in 0..size {
        print!("{}", d_text[i]);
    }
    println!();
}
