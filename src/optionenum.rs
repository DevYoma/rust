fn main(){
    let name: Option<String> = Some(String::from("Alice")); // There's a value
    let no_name: Option<String> = None; // No value

    greet(Some((String::from("Yoma")))); // Hello, Yoma
    greet(None); // Hello, stranger
}

fn greet(name: Option<String>){
    match(name){
        Some(name) => println!("Hello, {}", name),
        None => println!("Hello, stranger"),
    }
}   