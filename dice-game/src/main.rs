use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut lives = 3;
    let mut coins = 0;

    for level in 1..=10 {
        println!("-- Level {} --", level);

        let event = rng.gen_range(1..=3);
        match event {
            1 => battle(&mut rng, &mut lives, &mut coins),
            2 => treasure(&mut rng, &mut coins),
            3 => trap(&mut rng, &mut lives),
            _ => (),
        }

        if lives <= 0 {
        println!("you're out of lives game over early");
        break;
        }
    }

    println!("\n Game over");
    println!("you finished with {} coins and {} lives",coins,lives);

}


fn battle (rng: &mut rand::prelude::ThreadRng, lives: &mut i32, coins: &mut i32) {

    let player = rng.gen_range(1..=6);
    let enemy = rng.gen_range(1..=6);

    println!("You rolled : {}",player);
    println!("Enemy rolled : {}",enemy);

    if player > enemy {
        println!("You defeated the enemy");
        *coins += 1;
    } else if enemy > player{
        println!("you were hit , lost life");
        *lives -= 1;
    } else {
        println!("its a draw you both step back");
    }
}

fn treasure (rng: &mut rand::prelude::ThreadRng, coins: &mut i32) {
    let found = rng.gen_range(1..=6);
    *coins += found;
    println!("you found {} coins",found);
}

fn trap(rng : &mut rand::prelude::ThreadRng,lives: &mut i32){
    let flip = rng.gen_range(1..=2);
    if flip == 1 {
        println!("you dodged the trap close call");
    } else {
        println!("OUCH! The trap caught you. -1 life.");
        *lives -= 1;
    }
}