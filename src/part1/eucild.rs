fn gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let remainder = x % y;
        x = y;
        y = remainder;
    }
    x
}

pub fn  main(){
    
} 