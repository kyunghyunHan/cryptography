fn gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {//y가 0이 될떄  x가 Greastest Common Divisor GCD이다.
        let remainder = x % y;
        x = y;
        y = remainder;
    }
    x  //최대 공약수를 리턴한다.
}
//최대공배수
fn lcm(a:i32,b:i32)->i32 {
     a*b /gcd(a, b)
}
pub fn  main(){
    println!("{}",lcm(5 ,6));

    println!("{}",gcd(12 ,18))
} 