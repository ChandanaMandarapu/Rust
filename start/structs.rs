struct User {
    active: bool,
    username: String,
    email: String,
    sign_in: u64,
}

// implementation of structs 

struct Square {
    width: u32,
    height: u32,
}

struct Cube {
    l: u32,
    h: u32,
    b: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

impl Cube {
    fn area(&self) -> u32 {
        self.l * self.h * self.b
    }
}

fn main() {
    let name = String::from("chandu");

    let user1 = User {
        active: true,
        username: name,
        email: String::from("chandana@gmail.com"),
        sign_in: 12,
    };

    println!("user is username: {}", user1.username);

    let square = Square {
        width: 40,
        height: 40,
    };

    println!("area of square is {}", square.area());

    let cube = Cube {
        l: 9,
        h: 8,
        b: 7,
    };

    println!("area of cube is {}", cube.area());
}
