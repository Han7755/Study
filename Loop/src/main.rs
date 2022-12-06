fn main() {
    let mut count = 0;
    // loop는 무한루프 수행
    // counting_up은 해당 loop의 라벨
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // 외부루프 탈출
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}
