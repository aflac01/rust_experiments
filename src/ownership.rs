pub fn ownership() {
    variable_scope();
}

fn variable_scope(){
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}