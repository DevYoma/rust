pub trait Summary{
    fn summarize(&self) -> String{
        return String::from("Default Implementation. This only works when there is no implementation of Summary on the user Struct")
    }
}

struct User{
    name: String, 
    age: u32
}

// impl Summary for User{
//     fn summarize(&self) -> String {
//         return format!("Name: {}, Age: {}", self.name, self.age)
//     }
// }

impl Summary for User{}

fn main(){
    let user = User{
        name: String::from("John"),
        age: 25
    };

    println!("{}", user.summarize());
}