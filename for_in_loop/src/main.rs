fn main() {
    let a = [10, 20, 30, 40, 50];

    // for in 문은 파이썬의 for문과 비슷하게 작동
    for elements in a {
        println!("{}", elements);
    }
    for number in (1..4).rev() {
        // (1..4)는 파이썬에서 range(1,4)와 같다고 볼 수 있고, .rev()는 이들의 순서를 뒤집어준다.
        println!("{}", number);
    }
}
