struct Person {
    name: String;
    age: u8;
}

fn main () {
// arrays
// arrays with same datype and a fixed length


let arr = [1,23,4,5];

let firstel = arr[0];
println!("first element {}",firstel);

for element in arr.iter(){
    println!("element:{}",element);
}
// custom dataypes

// structs

let person = Person {
    name: String::from("alex"),
    age: 92,
};

println!("person name {} and age is {}",person);

let color = Colors::blue;

match color {
    
}

}



// enums 

enum Colors{
    Blue,
    Green,
    White
}