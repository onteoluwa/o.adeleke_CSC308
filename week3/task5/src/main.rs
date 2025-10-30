fn main() {
    let s1 = String::from("Hi");
    let s2 = String::from("amazing!");
    
    let result = longest(&s1, &s2);
    println!("The longer string is: {}", result);
}


fn longest<'a >(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

//longest() borrows both strings and returns a reference to the longer one.
//The function should not take ownership of any string.
//The program prints:
//The longer string is: amazing!

