pub trait Summary{
    fn summarize(&self) -> String{
        return String::from("Default Implementation. This only works when there is no implementation of Summary on the user Struct");
    }
}

struct User{
    name: String, 
    age: u32
}

impl Summary for User{
    fn summarize(&self) -> String {
        return format!("Name: {}, Age: {}", self.name, self.age)
    }
}

// impl Summary for User{}
impl Summary for String{} 
// this line of code allows us to pass in a string to the notify func below
// notify(String::from("asdfbla bla bla")) will work

fn main(){
    let user = User{
        name: String::from("John"),
        age: 25
    };

    println!("{}", user.summarize());

    notify(user);
}

fn notify(u: impl Summary){
    // anything that implements the Summary trait will have u.summarize() method

    println!("Summary: {}", u.summarize());
}