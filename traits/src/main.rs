// traits are like interfaces or abstract classes in cpp in simple words rust actually needs a way to share behaviour without sharing data structure so traits help u align with that method without focing them to be the same type of thing. 

    // &self - this meethod needs to look ar data inside the thing thats their but it wont changee it 

    // implementing a trait for a Type 

    trait Swim {
    fn swim(&self);
    } 
    trait MakeNoise {
        fn make_noise(&self);
    }
    struct Dog{
        name:String,
        weight: u32,
    }

    struct Fish{
        species:String,
        depth_preference: u32,
    }
    impl Swim for Dog{
        fn swim(&self){
            println!("{} is doing the paddle",self.name);
            if self.weight > 50 {
                println!("its a bit slower",self.name);
            }
        }
    }

    impl Swim for Fish{
        fn swim(&self){
            println!("the {} is swimming through water at {} meters depth",
            self.species, self.depth_preference);
        }
    }

    impl MakeNoise for Dog{
        fn make_noise(&self) {
            println!("bow bow"); //wwwaaat the ff  haha
        }
    }

    fn make_it_swim(creature:&impl Swim){
        creature.swim();
    }

    fn swim_and_bark(animal : &(impl Swim + MakeNoise)){
        animal.swim();
        animal.make_noise();
    }
fn main() {
   let buddy = Dog{
    name: String::from("Buddy"),
    weight:65,
   };

   let gold = Fish{
    species:String::from("goldfish");
    depth_preference: 10,
   };

   make_it_swim(&buddy);
   make_it_swim(&nemo);
}
