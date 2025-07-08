fn check_pass(marks: i32) {
    if marks >= 40 {
        println!("You passed with {} marks.", marks);
    } else {
        println!("You failed with {} marks. Try again.", marks);
    }
}

fn get_grade(marks: i32) -> char {
    if marks >= 90{
        return 'A';
    } else if marks >= 75{
        return 'B';
    } else if marks >= 60 {
        return 'C';
    } else if marks > 40 {
        return 'D';
    } else{
        return 'F';
    }
}

fn check_result (marks : i32) {
   
        if marks >= 40 {
            println!(" pass with {} marks",marks);
        } else {
            println!("fail with {} marks",marks);
        }

}

fn is_passed (marks : i32) -> bool {
    if marks >= 40 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut age = 20;
    println!("I'm {} years old.", age);
    age = 25;
    println!("Now I'm {} years old.", age);

    let grade: char = 'A';
    let is_passed: bool = true;
    let percentage: f32 = 92.5;
    println!("Grade: {}, Passed: {}, Percentage: {}%", grade, is_passed, percentage);

    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 75 {
        println!("Grade: B");
    } else {
        println!("Grade: C");
    }

    check_pass(77);
    check_pass(32);
    
    let student_marks = [91, 76, 64, 39, 52];

    for mark in student_marks {
    let grade = get_grade(mark);
    println!("Student scored {} and received grade: {}", mark, grade);
}


    let class_marks = [75, 23, 48, 91, 35];
    // learn here
    for mark in class_marks {
        check_result(mark);
    }
    
    let marks_list = [75, 23, 48, 91, 35, 60];
    let mut count = 0;

    for mark in marks_list {
        if is_passed(mark) {
            println!("Passed : {}", mark);
            count += 1;
        }
    }

    println!("Total passed: {}",count);
} 

