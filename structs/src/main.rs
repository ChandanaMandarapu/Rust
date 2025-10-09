fn main() {
    println!("Hello, world!");
    // struct is basically a custom blueprint that lets u group related data together in  asingle type. in one term it creates your own custom data type that fits ur prblm domain 

    // here i created a struct called character where Im defining my specification what all I need

    struct Character {
        name: String,
        health: i32,
        level: u32,
        is_alive:bool,
    }

    // creating instances
    // Once you assign hero, that instance owns all its data. When hero goes out of scope (the func ends or the block ends), Rust will automatically clean up that String, free the memory, everything. You don't write free() or delete. Rust does it automatically through its ownership system.

    let mut hero = Character {
        name: String::from("balaram"),
        health: 100,
        level: 10,
        is_alive: true,
    };

    // accesing fields

    println!("character: {}",hero.name);
    hero.health -= 20;

    // implementation block for roles type everything inside block is associated with role &mut self is first paramet i need to modify this instance self here is a keyword that is referencing to the instance of hero 

    impl Character{
        fn take_damage(&mut self, amount:i32){
            self.health -= amount;
            if self.health <= 0{
                self.is_alive = fals;
                println!("{} has died", self.name);
            }
        }
    }

    fn heal (&mut self, amount:i32){
        if self.is_alive{
            self.health += amount;
            println!("{} healed for {} HP",self.name,amount)
        }
    }

    fn display_stats(&self) {
        println!("Name: {}", self.name);
        println!("Health: {}", self.health);
        println!("Level: {}", self.level);
        println!("Status: {}", if self.is_alive { "Alive" } else { "Dead" });
    }

    // associated func 

    // The shorthand syntax: See how we wrote name, instead of name: name,? If the field name and the variable name are identical, Rust lets you use this shorthand.
    // Why is this pattern so common? - It's called a "constructor pattern." It provides a consistent way to create instances with sensible defaults. Maybe you always want characters to start at 100 health. Maybe you want to log character creation. All that logic goes in new().

    impl Character{
        fn new(name:String,level:u32) -> Character{
            Character{
                name,
                health: 100,
                level,
                is_alive: true,
            }
        }
    }

    let dhruva = Character::new(String::from("bhagvatam"),50);

    // tuple structs

    struct Color(u8, u8, u8);
    struct Point3D(f64, f64, f64);

    let red = Color(255, 0, 0);
    let origin = Point3D(0.0, 0.0, 0.0);

    println!("Red channel: {}", red.0);

    // unit structs with no fields 

    struct AlwaysSucceeds;

    
}
