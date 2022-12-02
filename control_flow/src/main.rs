fn main() {
    // if 문 <- C++과 비슷하나, 숫자형을 조건에 사용할 수 없음
    let number = 3;
    if number < 5 {
        println!("number is smaller than 5");
    } else if number == 5 {
        println!("number is 5");
    } else {
        println!("number is greater than 5");
    }
}
