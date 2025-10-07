
struct Point<T> {
    x: T,
    y: T,
}


struct Name<A> {
    first_name: A,
    last_name: A,
}

fn main() {
    let int_point = Point { x: 9, y: 10 };
    let float_point = Point { x: 9.8, y: 8.9 };
    println!("Integer point: {:?}", int_point);
    println!("Float point: {:?}", float_point);

    let fname = Name {
        first_name: String::from("chandana"),
        last_name: String::from("mandarapu"),
    };

    println!("Full name: {:?}", fname);
}
