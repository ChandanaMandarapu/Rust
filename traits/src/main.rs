// defining traits
// derive macros automatic trait
// here #derive() this tells rust please automatically implement these traits for me 

#[derive(Debug, Clone, PartialEq)]
struct Dummy; // derive can only be used on structs/enums, not traits

trait Swim {
    fn swim(&self) {
        println!("swimming in a generic way...");
    }

    // default behaviour
    fn swim_distance(&self, distance: u32) {
        for _ in 0..distance {
            self.swim();
        }
    }
}

trait MakeNoise {
    fn make_noise(&self);
}

struct Dog {
    name: String,
    weight: u32,
}

struct Fish {
    species: String,
    depth_pref: u32,
}

// using default feature
struct Duck {
    name: String,
}

// implementing for dog 
impl Swim for Dog {
    fn swim(&self) {
        println!("{} is doing the doggy", self.name);
        if self.weight > 50 {
            println!("its a bit slower because {} is a big dong", self.name);
        }
    }
}

// implementing for a fish
impl Swim for Fish {
    fn swim(&self) {
        println!(
            "the {} is gliding through water at {} meters depth",
            self.species, self.depth_pref
        );
    }
}

impl Swim for Duck {
    fn swim(&self) {
        println!("{} is paddling with feet", self.name);
    }
}

impl MakeNoise for Dog {
    fn make_noise(&self) {
        println!("woooo");
    }
}

// using polymorphism this make_it_swim function can work with many other diff things its just a refeernce away behind the scenees rust actually creates two versions of make_it_swim dog and one for fish this is called monomorphization rust generates specialised code for each type at compile time 
fn make_it_swim(creature: &impl Swim) {
    creature.swim();
}

// here  + symbol combines trait requirements. The impl Swim + MakeNoise means "I need something that can do BOTH swimming AND making noise
fn swim_and_bark(animal: &(impl Swim + MakeNoise)) {
    animal.swim();
    animal.make_noise();
}

// in real case scenarios
// The <T: Swim + MakeNoise> is called a "generic with trait bounds
// fn swim_and_bark<T: Swim + MakeNoise>(animal: &T){
//     animal.swim();
//     animal.make_noise();
// }

fn main() {
    let buddy = Dog {
        name: String::from("Buddy"),
        weight: 70,
    };
    let nemo = Fish {
        species: String::from("goldfish"),
        depth_pref: 10,
    };
    let dina = Duck {
        name: String::from("dina"),
    };

    // just testing the debug clone and partialeq traits
    // this lets you print the struct for debugging purposes using {:?}:
    // let buddy = Dog{name:String::from("BUddy"),age:5};
    // println!("{:?}",buddy);

    // clone trait
    // let buddy = Dog{name:String::from("BUddy"),age:5};
    // let buddy_clone = buddy.clone();

    // PartialEq trait - lets us compare like comparing 2 dogs

//     let buddy1 = Dog{name:String::from("BUddy"),age:5};
//     let buddy2 = Dog{name:String::from("BUddy"),age:5};

//     if buddy1 == buddy2 {
//     println!("These are the same dog!");
// }

    make_it_swim(&buddy);
    make_it_swim(&nemo);
    dina.swim_distance(3);
}
