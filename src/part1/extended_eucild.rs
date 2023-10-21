fn extended_eucild(a:i32,p:i32)->i32{
    let mut q:i32;
    let mut x2:i32;
    let mut x3:i32;
    let mut y2:i32;
    let mut y3:i32;
    let mut z2:i32;
    let mut z3:i32;

    //0일떄 처리
    if p==0{
        return 0;
    }
    //초기값 설정
    y3= p;
    x3= a;
    x2=0;
    y2= 1;
   //역수를 구하기 위해 y3가 1이 될떄까지 몫과 나머지를 반복적으로 구한다.
    while y3 >1 {
        q= x3/y3;


        z3= x3-(q*y3);
        x3= y3;
        y3= z3;

        z2= x2-(q*y2);
        x2= y2;
        y2= z2;

     
    }
    //역원이 존재하면 역원을 반환하는데 음수이면 양의 정수로 변환
    if y3==1 {
            
        //음수일 경우 모듈러 a를 더해준다.
        if y2 <0 {
            y2+=a;
            return y2;

            
        }
        return 0;
    }


    0
}
pub fn main() {

  let mut inverse= extended_eucild(100,23);
  if inverse !=0{
     println!("두수의 역원 {}",inverse)
  }else{
    println!("{}","역원이 존재하지 않습니다.")

  }
}