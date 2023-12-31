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



/*유한체
group,ring,field
집합 G내의 임의의 두개의 원소  a ,b ∈ G사이에 연산자 ＊룰 a＊b∈G와 같이 정의하고 다음의 성질을 만족하면 이잡합을 군(Group)이라고 한다.
1.닫힘:집합 G 상에서 이항 연산이 정의 되고 그 결과도 G의 원소가 된다.
a,b∈G,a＊b∈G
2.결합법칙:집합 G상에서 결합법칙이 성립한다.
a,b,c ∈ G  (a＊b)＊c = a＊(b＊c)
3.항등원 : 모든 원소 a ∈ G 에 대하여 닫은 연산이 성립하는 원소 e ∈ G가 존재한다.
a＊e= e＊a= a
4.역원:각 원소 a ∈ G 에 대하여 다음 연산이 성립하는 a` ∈ G 가 존재한다
a＊(a`)= (a`)＊a= e
만약 군이 유한개의 원소를 가지면 이를 유한이라 하고 군의 원소 개수를 위수라고 부른다.
군의 연산자 ＊가 덧셈 +일떄 덧셈군이라 하고 곱셈 x일떄는 곱셈군이라고 한다.
덧셈군일떄는 항등원은 0으로 표시하고 승산군일떄는 항등원은 1로 표시한다.
a+0=0+a= a∈ G
ax1= 1xa= a∈ G

또한 하나의 군에서 다음과 같이 교환법칙이 성립하는 경우를 가환군 혹은 아벨리안군 이라한다.

 a＊b=b＊a

 집합 R아 존재할떄 여기에 두가지 연산자 +와 x이 정의되고 다음의 성질을 만족하는 경우를 환이라고한다.
 그러나 경우에 따라서 두연산자가 덧셈과 곱셈으로 가정할수 있지만 다른 연산자를 정의할수 있다.
 1.(가환군)집합R은 덧셈에 대하여 가환군이다.
 2.(곱셈에 대한 닫힘)두개의 원소 a,b ∈ R 에 대하여 a x b ∈ R 이 정의된다.
 3.(곱셈에 대한 결하법칙)세개의 원소 a,b,c,∈ R에 대하여 결합법칙이 성립한다.
 ax(bxc)= (axb)xc
 4.(분배법칙)세개의 원소 a,b,c ∈ R 에 대하여 분배법칙이 성립한다.
 ax(b+c)= a x b + a x c
 (b+c)xa= b x a + c x a

 특히 두개의 원소 a,b ∈ R에 대하여 곱셈에 대한 교환법칙이 성립 , 즉  ax b= b xa 가 성립하면 이를 가환환 이라한다.

 특히 곱셈에 대한 단위원이 존재하고 영이 아닌 원소가 승산에 대하여 역원을 갖는 가환을 체F라고 정의한다.그리고 그 원소의 수가 유한한 체를 유한체 라고하며 GF()로 표시한다.
 1.(곱셈의역원)체 F에서 0을 제외한 각 a에대해 a x a⁻¹= a⁻¹ x a= 1을 만족하는 원소 a⁻¹가 존재한다.
 유한체를 구성하는 한 예로서 소수 p를 법으로 하는 두연산 덧셈과 곱셈을 정의하자
 집합 Z𝚙= {0,1,2,...,p-1}가 유한체이며 GF(p)로 표시된다.GF(p)는 크기가 P인 갈로아체라고도 한다. 예를들어 p= 7인경우 GF(7)위의 원소인 0,1,2,..,6은 위의 체를 이루는 성질을 만족하므로 유한체를 이룰수 있다.
 유한체의 GF(7)위의 덧셈표와 곱셈표는 다음과 같다.
 GF(7)위의 덧셈 :(a+b)mod p
 GF(7)위의 곱셉 :(a*b)mod p
 
 Fermat의 정리를 살펴보기 위해 각 원소에 대한 멱승연산을 수행해보면 다음과 다음과 같다.유한체 GF(7)상의 0을 제외한 모든 원소에 대해 6승 즉 p-1은 모두 1승이다.
 즉 유한체의 GF(p)에서 0을 제외한 모든 원소의 (p-1)승은 1이된다.
 이것이 Fermat의 정리와 연결된다.
 또한 정수 a∈Z와 m이 서로소이면 다음을 만족시키는 양의 정수 b가 존재한다.가장 작은 b를 법 m에 대한 a의 위수라고한다.
 aᴮ≡ mod m
 그리고 법 m에 관한 정수a의 위수가ϕ(m)일떄 a를 법 m에 대한 원시근또는 원시 원소라고 정의한다.
 GF(7)상에서의 멱승표를 살펴보면 a= 3일경우와 a= 5일경우 멱승을 하면 유한체의 모든 원소가 나타나게 되고 a의 위수가 ϕ(7)= 6이 되므로 3과 5는 원소원소임을 알수 있다.
 a= 4인 경우에는 그위수ϕ(m)가 4가 되지만 aᴮ≠1 mod m 이므로 a는 원시 원소가 아니다.
 암호학에서 널리 사용되는 체중에는 GF(2)가 있다.이체는 다음과 같이 정의된다.
 GF(2)에서는 원소는 {0,1}밖에 없으며 덧셈과뺼셈은 실제로 두수에 대한 XOR연산이다.그리고 곱셈연산과 나눗셈은 두수에 대한 AND연산이다
 
 p가 소수이고 m이 양의 정수일떄 GF(pᵐ)역시 유한체를 이룬다.
 GF(pᵐ)를 기초체 GF(p)의 확장체라 하며 p를 표수라고한다.즉 기초체룰 m차로 확대시킴으로 원소수를 pᵐ로 구성된 유한체를 구성할수 있다.
 P(x)를 GF(p)상의 기약 다항식이라 할떄 GF(pᵐ)상의 원소는 p(x)= 0인 x로 표시할수 있다.
  GF(p)상의 m차 기약다항식의 근인 x의 멱승 즉 x⁰,x¹,x²,...,xᵐ⁻¹ 은 GF(p)상에서 GF(pᵐ)상의 벡터공간의 기저가 된다.

 GF(pᵐ)상에서의 원소는 모두 m비트로 표현되는데 이러한 m비트의 워드는 m-1차의 다항식 형태로 생각하는 것이 훨씬 간단하다.
 즉 m-1차의 다항식은 다음과 같이 표현된다.
 p(x)= a𝚖₋₁xᵐ⁻¹+a𝚖₋₂xᵐ⁻²+...+a₁x¹+a₀x⁰

 여기서 a𝗂= i번쨰 항의 계수를 나타내며 GF(p)상의 원소이다. 
 예로 8비트 워드인 1010110을 다항식으로 이용해서 표현하면 다음과 같다.

 1x⁷+0x⁶+1x⁵+0x⁴+1x³+1x²+0x¹+1x⁰= x⁷+x⁵+x³+x²+1

 간단한 확장체로 예로 p= 2,m= 3인 GF(2³)인 경우 GF(2)위의 기약 다항식은 m= 3차인 p(x)= x³+x+1이라고 했을떄 P(x)근을 x라 하면 p(x)= 0이므로 x³+x+1= 0,x³=x+1이 성립한다.


 다항식의 연산에서 모듈러 덧셈과 곱셉은
 다항식의 덧셈 연산에서 두 다항식이 같은 항을 가지고 있으면 상쇄가 된다.

 f(x)= x²+1
 g(x)= x²+x+1
 f(x)+ g(x)= x

 다항식의 곱셉연선에서는 다음과 같이 두 다항식을 서로 곱하게 된다.
 f(x) * g(x)= (x²+1)(x²+x+1)
            = x⁴+x³+x²+x²+x+1
            = x⁴+x³+x+1

기약 다항식에 의한 모듈러 곱셉은 다음과 같다.
 f(x) * g(x)/p(x)= (x⁴+x³+x+1)/(x³+x+1)
 = x+1
 따라서 f(x) * g(x) mod p(x)= (x⁴+x³+x+1)mod (x³+x+1)
 =x²+x
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


확장 유클리드 알고리즘이 계산동안에는 다음 관계들이 지속적으로 성립한다.
만약 gcd(d,f)=1이라면 일정 단계에서의 y3= 1이다.따라서 y3=1 이라면 다음원리에 의해 y2는 모듈러f에 의해 d의 역원이 된다.
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