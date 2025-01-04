// // GENERICS REFRESHER TO UNDERSTAND RESULT ENUM 👇

// struct Point<A, B>{
//     x: A, 
//     y: B, 
//     z: B
// }

// fn main(){
//     let integer_point = Point {x: 5, y: "Yoma", z: "codes"};
// }

use std::fs;

// RESULT ENUM 👇
enum Result<A, B> {
    Ok(A), 
    Err(B) // B is mosttime a struct that follows a certain format  
}

fn main(){
    // there is a fn that can error out/stop the thread

    let res = fs::read_to_string("example.txt"); // this file might or can crash 

    match res{
        Ok(content) => {
            println!("The content of the file is {}", content);
        }, 
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    println!("Hello there")
}

// check using res.unwrap()