use std::collections::HashMap;

fn main(){
    let mut users: HashMap<String, u32> = HashMap::new();

    users.insert(String::from("Yoma"), 20);
    users.insert(String::from("DevYoma"), 19);

    let first_user_age = users.get("Yoma"); // returns Some(20)
    let no_valid_key_user = users.get("Yoma1"); // returns None

    println!("{:?}", users); // returns {"Yoma": 20, "DevYoma": 19}
    // println!("{:?}", first_user_age); // returns Some(20)

    // Now the best part of it => we can PATTERN MATCH.
    match first_user_age{
        Some(age) => println!("Yoma's age is {}", age),
        None => println!("User not found")
    }

    match no_valid_key_user{
        Some(age) => println!("Yoma's age is {}", age),
        None => println!("User not found")
    }
}