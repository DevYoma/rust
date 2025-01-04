fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", even_filter(&vec)); // returns [2]
    println!("{:?}", vec); // returns [1, 2]
    // println!("{:?}", vec); // this wont work (ownership)
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32>{
    // iterating over a vec
    let mut new_vec = Vec::new();
    for val in vec{
        if val % 2 == 0{
            new_vec.push(*val); // derefereced borrow(*)
        }
    }

    return new_vec;
}