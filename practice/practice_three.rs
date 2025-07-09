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

fn is_passed(mark: i32) -> bool {
    mark >= 40
}

fn main() {
    let marks = [91, 76, 64, 39, 52];

    let mut total_students = marks.len();
    let mut passed = 0;
    let mut failed = 0;
    let mut sum = 0;
    let mut highest = 0;

    println!("--- Report Card ---");

    for mark in marks {
        let grade = get_grade(mark);
        let result = if is_passed(mark) { "Passed" } else { "Failed" };

        println!("Marks: {} → Grade: {} → Result: {}", mark, grade, result);

        sum += mark;

        if is_passed(mark) {
            passed += 1;
        } else {
            failed += 1;
        }

        if mark > highest {
            highest = mark;
        }
    }

    let average = sum as f32 / total_students as f32;

    println!("\n--- Summary ---");
    println!("Total Students: {}", total_students);
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Average Marks: {:.1}", average);
    println!("Topper Score: {}", highest);
}
