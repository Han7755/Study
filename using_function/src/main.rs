fn main() {
    let y = {
        let x = 3;
        x + 1 // rust에서는 ;이 없으면 return 한다는 의미로 사용됨.
    };
    println!("{}", y);

    let five = five();
    println!("using five() function : {}", five);
}

fn five() -> i32 {
    // 명시적으로 리턴타입을 지정할 수 있다.
    5 // 세미콜론이 없으므로 5를 리턴한다는 의미이다.
}
