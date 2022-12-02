fn main() {
    // Scalar Type 변수들
    // 아래의 변수 타입에서 i를 u로 바꾸면 Unsigned가 된다.
    let _int8_var: i8 = 0; // signed 8-bit integer
    let _int16_var: i16 = 0; // signed 16-bit integer
    let _int32_var: i32 = 0; // signed 32-bit integer
    let _int64_var: i64 = 0; // signed 64-bit integer
    let _int128_var: i128 = 0; // signed 128-bit integer
    let _arch_var: isize = 0; // architecture가 32비트이면 32비트 정수, 64비트이면 64비트 정수이다.

    // 가독성을 좋게 하기위한 Number literals
    let _decimal_value = 98_222; // 98222와 같지만, 긴 숫자를 _로 구분하여 가독성을 향상시킬 수 있음
    let _hex_value = 0xff; // 16진수에는 0x를 접두어로 붙인다.
    let _octal_value = 0o77; // 8진수에는 0o를 접두어로 붙인다.
    let _binary_value = 0b1111_0000; // 2진수에는 0b를 접두어로 붙인다.
    let _byte_value = b'A'; // Byte는 Unsigned 8-bit 전용이다.

    // Char 타입은 1 byte가 아닌 4 byte 여서 한국어, 중국어, 일본어, 이모티콘 등 아스키가 아닌 문자도 저장할 수 있다.
    let lower_case = 'a';
    let upper_case = 'A';
    let korean_character = '가';
    let chinese_character = '天';
    let emoji = '😊';
    println!("{}", lower_case);
    println!("{}", upper_case);
    println!("{}", korean_character);
    println!("{}", chinese_character);
    println!("{}", emoji);

    // Compound Type 변수들
    let tup: (i64, f64, u8) = (500, 6.4, 1); // tuple 타입은 괄호를 사용하여 명시적으로 타입 설정 가능(길이 변경 불가)
    let (x, y, z) = tup; // tuple 타입 변수의 값을 쪼개서 저장하는 방법
    println!(
        "value of x : {}\nvalue of y : {}\nvalue of z : {}\n",
        x, y, z
    );

    // tuple 을 인덱스로 접근하기
    let var1 = tup.0;
    let var2 = tup.1;
    let var3 = tup.2;
    println!(
        "value of var1 : {}\nvalue of var2 : {}\nvalue of var3 : {}\n",
        var1, var2, var3
    );

    // Array Type
    let arr = [1, 2, 3, 4, 5]; // Array의 길이는 고정이고, 같은 타입의 값만 들어갈 수 있다.
}
