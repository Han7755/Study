fn main() {
    let y = {
        let x = 3;
        x + 1 // rust에서는 ;이 없으면 return 한다는 의미로 사용됨.
    };
    println!("{}", y);
}
