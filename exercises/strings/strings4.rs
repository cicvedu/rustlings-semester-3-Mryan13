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
    println!("{}", String::from("hi"));
    println!("{}", String::from("rust is fun!"));
    println!("{}", <String as Into<String>>::into(String::from("nice weather")));
    println!("{}", format!("Interpolation {}", "Station"));
    println!("{}", &String::from("abc")[0..1]);
    println!("{}", "  hello there ".trim());
    println!("{}", "Happy Monday!".replace("Mon", "Tues"));
    println!("{}", "mY sHiFt KeY iS sTiCkY".to_lowercase());
}
