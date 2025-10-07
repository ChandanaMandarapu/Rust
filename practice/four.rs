// option and result enums in rust nullpointers and error handling in rust
// the option enum lets u return either some value or none value

// without usin null we can use option which has some and none variants

// pub enum Option <T> {
//     None, 
//     Some(T),
// }

// here we are returning a optopn which has 2 things some and none and option is same like enum only seee noww

// enum CustomOption {
//     Some(i32),
//     None,
// }

// fn main () {
//     let index = find_first_a(String::from("chandu"));

//     match index {
//         CustomOption::Some(value) => println!("index is {},"value),
//         CustomOption::None => println!("a not found"),
//     }
// }

// fn find_first_a(s:String) -> CustomOption<i32> {
//     for(index,char) in s.chars().enumerate(){
//         if char == 'a' {
//             return CustomOption::Some(index as i32);
//         }
//     }
//     return CustomOption::None;
// }

fn main () {
    let index = find_first_a(String::from("chandu"));

    match index {
        Some(value) => println!("index is {},"value),
        None => println!("a not found"),
    }
}

fn find_first_a(s:String) -> Option<i32> {
    for(index,char) in s.chars().enumerate(){
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

// Result enum lets u return either ok value or err value the rust enum is how u can do error handling in rust

