// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // "blue" 是一个字面量字符串，它是一个 &str 类型
    string_slice("blue");

    // "red".to_string() 将字面量字符串转化为 String 类型
    string("red".to_string());

    // String::from("hi") 创建一个新的 String 类型
    string(String::from("hi"));

    // "rust is fun!".to_owned() 将 &str 转化为 String
    string("rust is fun!".to_owned());

    // "nice weather".into() 将字面量字符串转换成 String
    string("nice weather".into());

    // format! 创建一个 String 类型
    string(format!("Interpolation {}", "Station"));

    // &String::from("abc")[0..1] 是一个切片，从 "abc" 中截取字符 "a"
    string_slice(&String::from("abc")[0..1]);

    // "  hello there ".trim() 移除字符串前后的空白，返回一个 &str
    string_slice("  hello there ".trim());

    // "Happy Monday!".to_string().replace("Mon", "Tues") 创建一个新的 String
    string("Happy Monday!".to_string().replace("Mon", "Tues"));

    // "mY sHiFt KeY iS sTiCkY".to_lowercase() 将字符串转为小写，返回一个 String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
