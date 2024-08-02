fn main() {
    // 1个英文字母在UTF-8编码下占1个字节
    let s = String::from("hello");
    let len_s = s.len();
    println!("The length of '{}' is {}.", s, len_s);

    // 1个汉字在UTF-8编码下占3个字节
    let s2 = String::from("你好");
    let len_s2 = s2.len();
    println!("The length of '{}' is {}.", s2, len_s2);

    // 在UTF-8编码中,我们将 "你" 这种字符称为Unicode标量值
    // 一个Unicode标量值可能由多个字节组成
    // 本例中的 "你" 字 由3个字节组成:
    // E4: 1110 0100 -> 0xE4 -> 228
    // BD: 1011 1101 -> 0xBD -> 189
    // A0: 1010 0000 -> 0xA0 -> 160
    // 字符串是字节的集合(Vec<u8>),如果允许索引访问,那么在本例中,访问
    // s2[0] 时,我们得到的是第一个字节的值,即 228
    // 这是一个很大的问题,因为我们期望得到的是字符 "你" 的Unicode标量值,而非某个字节的值
    // 因此,Rust不允许我们使用索引访问字符串
}
