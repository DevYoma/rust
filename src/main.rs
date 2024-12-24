// fn main() {
//     // println!("Hello, world!");
//     let x = 5;
//     let y = -15;
//     let _z: u32 = 1000; 

//     let _float_number = 1000.004;

//     print!("x: {}, y: {}", x, y);
// }

fn main(){
    // let is_male = false;
    // let is_above_18 = true;

    // if is_male {
    //     println!("You are a guy");
    // }else{
    //     println!("You are not a guy");
    // }

    // if is_male && is_above_18 {
    //     println!("You are a legal male")
    // }

    // let greeting = String::from("hello world");
    // println!("{}", greeting);

    // let char1 = greeting.chars().nth(0);
    // print!("char1 {}", char1.unwrap());

    // Loops
    // for i in 0..10{
    //     print!("{} ", i);
    // };

    let a = 2;
    let b = 10;
    let sum = do_something(a, b);
    println!("Sum of {} and {} is {}", a, b, sum);

}

fn do_something(a: i32, b: i32) -> i32 {
    return a + b;
}