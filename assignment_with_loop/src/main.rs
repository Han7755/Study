fn main() {
    let mut counter = 0; // 수정가능한 변수

    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2; // 
        }
    }
}
