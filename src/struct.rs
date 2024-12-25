// STRUCTS => TUPLE && UNIT
// UNIT STRUCTS: Structs with no attributes. (more like classes without any atttributes)

struct User{
    name: String,
    age: u32,
    active: bool
}

fn main(){
    let name = String::from("Yoma");

    let user = User{
        name, 
        age: 30, 
        active: true
    };

    println!("{} is {} years old.", user.name, user.age);
}