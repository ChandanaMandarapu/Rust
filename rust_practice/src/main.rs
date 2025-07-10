use rand::Rng;

fn play_around() {
    let mut rng = rand::thread_rng();

    let player_roll = rng.gen_range(1..=6);
    let bot_roll = rng.gen_range(1..=6);

    println!("You rolled : {}", player_roll);
    println!("Bot rolled : {}", bot_roll);

    if player_roll > bot_roll {
        println!("you win this round");
    } else if bot_roll > player_roll {
        println!("bot wins the round");
    } else {
        println!("its a draw");
    }

    let lucky_number = 4;
    let mut won = false;

    for _ in 1..=9 {
        let roll = rng.gen_range(1..=6);
        println!("bot rolled : {}", roll);

        if roll == lucky_number {
            println!("lucky roll you win");
            won = true;
            break;
        }
    }

    if !won {
        println!("Better luck next time!");
    }
}


fn transform(n:i32) -> i32 {
    let result = (n*2 + 3 ).pow(2);
    result
}

fn main() {
    play_around();
    let output = transform(5);
    println!("final result {}",output);
}
