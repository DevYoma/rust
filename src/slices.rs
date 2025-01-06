// fn main(){
//     let name = String::from("hello world");
//     let ans = first_word(name);
//     println!("{}", ans);
// }

// fn first_word(str: String) -> String{
//     let mut ans = String::from("");
//     for i in str.chars(){
//         if i == ' '{
//             break;
//         }
//         // ans.push(i);
//         ans.push_str(&i.to_string());
//     }

//     return ans;
// }

fn main(){
    let word = String::from("Hello world");
    let word2 = &word[0..];

    // you can do this for arrays too
    let arr = [1,2,3];
    let arr_slice = &arr[0..2];

    println!("{}", word2);
    println!("{:?}", arr_slice);
}