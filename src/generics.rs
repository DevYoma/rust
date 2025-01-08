fn main(){
    let bigger_number = largest(2, 4);
    let bigger_char = largest('a', 'b');

    println!("The bigger number is {}", bigger_number);
    println!("The bigger character is {}", bigger_char);
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T{
    if a > b{
        a
    }else{
        b
    }
}