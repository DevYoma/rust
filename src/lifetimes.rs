fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main(){
    let longest_str;
    let str1 = String::from("small");

    {
        // scoping str2 and the longest_str variable that has the fn longest()
        let str2 = String::from("biggest"); // str2 is removed from the heap when it goes out of scope
        longest_str = longest(&str1, &str2);
    }
    
    println!("{}", longest_str);
}

// assume the first parameter in the logest function is 8 lines, and the second is 4 lines, How long would the return type be valid(the shorter of the lines, i.e 4lines)

// So basically, rust wants us to tell it how long the lifetimes of the variables are, so that it can determine how long the return type would be valid for.    

// So, ITS JUST TELLING US TO SPECIFY A RELATIONSHIP BETWEEN THE LIFESPAN OF ans, str1 and str2

//GENERIC LIFETIME ANNOTATION 

// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {} ========== GENERIC
 