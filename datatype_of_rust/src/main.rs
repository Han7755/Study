fn main() {
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
}
