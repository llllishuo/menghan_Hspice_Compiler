// 按等于号拆分
pub fn split_equal_sign(bit: &str) -> String {
    if bit.to_string().contains("=") {
        let mut b = bit.split("=");
        b.next();
        let Some(iter) = b.next() else { todo!() };
        iter.to_string()
    } else {
        bit.to_string()
    }
}
