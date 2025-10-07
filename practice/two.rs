// structs

struct User {
    first_name : String,
    last_name : String,
    age : i32,
}
// structs implementing

struct Triangle {
    base : u32,
    height : u32,
}

impl Triangle{
    fn area(&self) -> u32 {
        1/2 * self.base * self.height
    }

    fn debug() -> i32 {
        return 1;
    }

}

fn main () {
    let user = User{
    first_name : String::from ( "chandu"),
    last_name : String::from ("mndrpu"),
    age : 21,
    };

    println!("{}",user.first_name);
    println!("{}",user.last_name);
    println!("{}",user.age);

    let triangle = Triangle {
        base: 30,
        height: 40,
    };

    print!("area of triangle is ",triangle.area());
    print!("debug is {}",Triangle::debug());
}
