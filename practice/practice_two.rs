// match practice

let grade = 'A';
match grade {
    'A' => println!("excellent"),
    'B' => println!("good"),
    'C' => println!("can do better"),
    => println!("invalide"),
}


    let marks = 92;

    match marks {
        90..=100 => println!("Grade: A"),
        75..=89 => println!("Grade: B"),
        60..=74 => println!("Grade: C"),
        40..=59 => println!("Grade: D"),
        0..=39  => println!("Grade: F"),
        _ => println!("Invalid marks"),
    }

for i in 1..=10 {
    println!(" {} * {}= {}",i,i,i*i);
}

