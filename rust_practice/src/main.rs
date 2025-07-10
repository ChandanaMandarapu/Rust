use rand::Rng;

fn play_around() {

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..=6); //generates number from 1 to 6

    let player_roll = rng.gen_range(1..=6);
    let bot_roll = rng.gen_range(1..=6);

    println!("You rolled : {}",player_roll);
    println!("Bot rolled : {}",bot_roll);

    if player_roll > bot_roll {
        println!("you win this round");
    } else if bot_roll > player_roll {
        println!("bot wins the round");
    } else {
        println!("its a draw");
    }

}
fn main () {
    play_around();
}

