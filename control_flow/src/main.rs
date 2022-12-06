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

    let condition = true;
    let number = if condition { 5 } else { 6 }; // if 문, else문 안의 코드에 세미콜론이 없으므로 if문의 결과에 따라 5 또는 6을 변수에 할당하는 배정문이다.
    println!("Value of number = {}", number);

    // let number = if condition { 5 } else { "six" }; <- 컴파일 타임에 타입이 명확하지 않으므로 오류 발생
}
