fn main() {
    let x: i32 = 32; // x 는 Int32형식의 변수 (수정 불가이지만 Constant는 아님)
    println!("{}", x); // print value of x
                       // 일반적으로 변수는 변경할 수 없음
    let x: i32 = 5; // let 으로 변수를 새로 생성하여 같은 이름이지만 다른 값의 변수를 생성할 수 있음
    println!("{}", x);

    println!();
    let mut y: i32 = 2998; // let mut 으로 수정할 수 있는 변수를 선언할 수 있음
    println!("{}", y);

    y = 3200; // 수정 가능
    println!("{}", y);

    let _spaces: &str = "   "; // 문자열 변수 생성
    let _spaces: usize = _spaces.len(); // let으로 다시 변수를 생성 할 때에는 값과 타입 모두 바꿀 수 있음
}
