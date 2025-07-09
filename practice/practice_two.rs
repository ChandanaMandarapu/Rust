fn main() {
    // match on character grade
    let grade = 'A';
    match grade {
        'A' => println!("Excellent"),
        'B' => println!("Good"),
        'C' => println!("Can do better"),
        _   => println!("Invalid grade"),
    }

    // match on marks
    let marks = 92;
    match marks {
        90..=100 => println!("Grade: A"),
        75..=89  => println!("Grade: B"),
        60..=74  => println!("Grade: C"),
        40..=59  => println!("Grade: D"),
        0..=39   => println!("Grade: F"),
        _        => println!("Invalid marks"),
    }

    for i in 1..=10 {
        println!("{} * {} = {}", i, i, i * i);
    }

    // marks to grade using get_grade()
    let marks_list = [91, 68, 75, 39, 100];
    for val in marks_list {
        let grade = get_grade(val);
        println!("Student scored {} → Grade: {}", val, grade);
    }
}

// function that returns grade char based on marks
fn get_grade(mark: i32) -> char {
    match mark {
        90..=100 => 'A',
        75..=89  => 'B',
        60..=74  => 'C',
        40..=59  => 'D',
        0..=39   => 'F',
        _        => '?',
    }
}

