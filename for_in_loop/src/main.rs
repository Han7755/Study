fn main() {
    let a = [10, 20, 30, 40, 50];

    // for in 문은 파이썬의 for문과 비슷하게 작동
    for elements in a {
        println!("{}", elements);
    }
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
