fn main() {
    // 아래의 변수 타입에서 i를 u로 바꾸면 Unsigned가 된다.
    let _int8_var: i8 = 0; // signed 8-bit integer
    let _int16_var: i16 = 0; // signed 16-bit integer
    let _int32_var: i32 = 0; // signed 32-bit integer
    let _int64_var: i64 = 0; // signed 64-bit integer
    let _int128_var: i128 = 0; // signed 128-bit integer
    let _arch_var: isize = 0; // architecture가 32비트이면 32비트 정수, 64비트이면 64비트 정수이다.
}
