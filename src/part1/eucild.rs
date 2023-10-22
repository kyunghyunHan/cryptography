/*
암호학에서는 정수론과 유한체 이론이 많이 사용되고 있다.
그중에 정수론을 살짝 알아보겟습니다.
 Cryptography

정수론


- 두정수 a와 b가 있고 a!=0이면 다음과 같은 등식을 만족하는 q와 r이 존재하며 이 두정수를 각각 몫과 나머지 라고한다.
b=a * q+r, 0<= r <|a|

- 또한 두 정수 a와 b가 있을 떄 b=a*C안 정수 c가 존재하면 a는 b의 약수라하고 b를 a의배수라고 한다.
- b는 a로 나누어 떨어진다고 말하는 관계를 다음과 같이 표시합니다.
a|b

- 임의의 세정수 a,b,c가 있을 떄 d가 a와 b의 약수라면 정수 d를 a와 b의 공약수 라고한다.
d|a,d|b


공약수 중에 a와 b의 최대공약수라 하고 최대 공약수를 gcd(a,b)라고 표시한다.
어떤 양의 정수 p가1과 자기 자신 이외에는 약수가 존재하지 않을 떄 이러한 p를 소수 라고한다.
반대로 소수가 아닌 a를 합성수 라고 한다.
ex)소수 2,3,5,7  합성수 4 6 8 9
정수 집합Z상에는 무한히 많은 소수가 분포되어 잇으나 어떠한 규칙을 가지고 분포하는지 밝혀진바는 없다.
임의의 정수a가 있을때 이 수는 다음과 같이 소수의 곱으로 유일하게 표시할수 잇다.
밑에서 p1,p2,....,pr은 각각 서로 다른 소수이며 위와 같이 a를 나타낸것을 표준적인 분해라고 한다.
a= p1^e1 * p2^e2...pr^er(ei>0,i= 1,2,3,...,r)
숫자 36의 소인수분해:

36 = 2^2 * 3^2

여기서 2와 3은 소수이며, 2의 지수는 2, 3의 지수는 2입니다.

숫자 90의 소인수분해:

90 = 2 * 3^2 * 5

이 경우, 2, 3, 그리고 5는 소수이며, 각각의 소수의 지수는 1, 2, 1입니다.

숫자 48의 소인수분해:

48 = 2^4 * 3

2와 3은 소수이며, 2의 지수는 4, 3의 지수는 1입니다.

숫자 210의 소인수분해:

210 = 2 * 3 * 5 * 7

여기서 2, 3, 5, 7은 모두 소수입니다. 지수가 1인 이 소수들이 바로 210의 소인수분해입니다.

숫자 126의 소인수분해:

126 = 2 * 3^2 * 7

2, 3, 그리고 7은 소수이며, 각각의 소수의 지수는 1, 2, 1입니다.


- 정수 a와 b가 공통적으로 약수를 가지지 못한다면 a와 b는 서로소 라 하며 gcd(a,b)= 1이라고 표시한다.
- 즉 두 정수의 공통 소인수는 1밖에 없다는 뜻이된다.

- 합동식과 잉여계
- 두정수 a와 b에 대하여 a-b가 정수 m의 배수일때 a와 b는 법 혹인 모듈러스에관하여 합동이라하고 
- a -=b mod m  -> a-b =km이라는 뜻이다.
- -=는 합동식을 나타내는 합동기호이다.
- 반사적성질 a-= a mod m
- 대칭적 성질 a-= b mod m 이면 b -=a mod m 
- 추이적 성질 a-= b mod m ,b -= c mod m 이면 a -= c mod m 
- a-=b mod m 에 대한 정수 집합 Z상의 동치류를 표현할떄 법 m에 대한 잉여류 라한다. 


a ≡ {x ≡ Z |x ≡ a mod m}
즉 임의의 정수 중에서 m에 대하여 합동을 이루는 모든 정수를 의미한다.이에 a ∈ z에 의해 만들어지는 잉여류를 a̅ 로 표현하는데 정수 집합 Z는 m개의 잉여류들로 나누어 지게된다

집합 Z𝚖 ={0,1,2,...,m-1}의 원소 중에서 m과 서로소인 원소 전체의 집합을 Z𝚖＊로 표시하고 Z𝚖＊의 원소의 개수는 ϕ(m) 으로 나타낸다

Z𝚖＊={a ∈Z𝚖 | gcd(a,m)=1}
ϕ(m)= | Z𝚖＊ |

이때 원소의 개수 ϕ(m)을 Euler 의 ϕ함수 혹은 Euler 의 토션함수 라고한다 특히 m이 소수 p가 소수일떄는 다음과 같은 성질을 만족한다.
Zp＊= {1,2,...,p-1},ϕ(p)= p-1


Euler정리와 Fermat의 정리
위에서 Z𝚖={0,1,2,…,m-1}상의 원소 가운데 m과 서로소인 원소의개수를  ϕ(m) 으로 표시하고 이를 Euler의 함수라고 하였다.이를 응용한 Euler 의 정리와 Fermat의 정리는 공개키 암호 시스템에 많이 사용되는 중요한 정리이다

Euler정리
양의 정수 m에 대하여 gcd(a,m)≡1이면 다음 식이 성립한다.

aᵠ⁽ᵐ⁾ ≡ 1 mod m

Fermat정리
Euler정리에서 Z𝚖의 m이 특별하게 소수 p인경우 gcd(a,p)=1 에 대하여  ϕ(p)= p-1가 되므로 다음식이 성립한다.
aᴾ⁻¹ ≡ 1 mod p

소수 p가 잇고 임의의 정수 a,b가 있을떄 다음식이 성립한다.
aᴾ ≡ a mod p
(a+b)ᴾ ≡ aᴾ + bᴾ mod p
(ab)ᴾ ≡ aᴾbᴾ mod p

Fermat의 정리를 응용하면 법이 소수인 경우 곱에대한 역원을 빨리 구할수 있다.만약 소수 p가 있고 정수 a가 P로 나누어 떨어지지 않는 수라면 다음과 같은 방법으로 a의 역수를 구할수 있다.

a⁻¹ mod p = a ⁻² mod p
*/

/*유클리드 알고리즘 
- 정수론의 기본적인 기술중 하나
- 원래 두양의 정수들에 대한 최대 공약수를 찾아내기 위한 알고리즘
- 확장 유클리드 알고리즘은 두개의 정수가 서로소인 경우에 한수에 대한 다른수의 곱셈에 대한 역원을 계산하는 데 사용한다.
- 기본적으로 유클리드 알고리즘을 이용한 최대공약수를 구하는 방법은 어떠한 음이 아닌 정수 a와 그보다 작은 작은 정수 b가 있을 떄 다음등식에 기반
- gcd(a,b)= gcd(b,a mod b)
- 24와 14의 최대 공약수를 구하는 절치
- gcd(24,14)= gcd(14,10)
            = gcd(10,4)
            = gcd(4,2)
            = gcd(2,0)
            = 2

- 유클리드 알고리즘은 두 정수가 서로 소인 경우라면 최대 공약수 만을 찾을 뿐 만 아니라 d에 대한 곱셈의 역원을 구하는 것으로 확장할수 있다.
*/



//최대 공약수를 구하는 함수//
fn gcd(mut x: i32, mut y: i32) -> i32 {
    //두 수를 나눈 후 나눈 나머지를 구하여 0 이 될 떄까지 반복한다.
    while y != 0 {//y가 0이 될떄  x가 Greastest Common Divisor GCD이다.
        let remainder = x % y;//x와 y를 나눈 나머지
        x = y;//다음 연산에서 r= y % r을 수행해야 하므로
        y = remainder;//x에 y를 대입하고 y에 r을 대입한다.
    }
    x  //최대 공약수를 리턴한다.
}
//최대공배수
fn lcm(a:i32,b:i32)->i32 {
     a*b /gcd(a, b)
}
pub fn  main(){
    println!("{}",lcm(5 ,6));

    println!("{}",gcd(100 ,48))
} 