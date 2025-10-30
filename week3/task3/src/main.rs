fn main() {
    let mut name = String::from("Firstname ");
    add_surname_to_firstname(& mut name);
    println!("{name}");
}

fn add_surname_to_firstname(name: &mut String) {
    name.push_str("Lastname");
}
//The function add_surname_to_firstname() does not take ownership of the name variable.
//Instead, it should borrow the string, modify it, and return nothing.
//The final print statement in main() should still print Firstname Lastname.
