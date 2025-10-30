fn main() {
    let s: String = String::from("hello");
    take_ownership(s);
}fn take_ownership(name: String) {
    println!("{}", name);
}