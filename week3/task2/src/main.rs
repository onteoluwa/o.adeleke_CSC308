fn main() {
    let mut name = String::from("Ada");
    
    print_name(&name);
    append_title(&mut name);
    
    println!("Final name: {}", name);
}

fn print_name(n: &String) {
    println!("Name: {}", n);
}

fn append_title(n: &mut String) {
    n.push_str(" Lovelace");
}

//Both print_name() and append_title() can be called safely.
//print_name() borrows immutably.
//append_title() borrows mutably.
//The program compiles and prints the final full name.
