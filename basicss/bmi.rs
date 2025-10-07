// bmi index

fn main (){
    // calling bmi function

    let weight: f64 = 60.0;
    let height: f64 = 1.50;
    let bmi = caluclate_bmi(weight,height);
    println!("Your BMI is: {:?}",bmi)
}

fn caluclate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}


// ownership borrowing and references

// ownereship c and c++ -> memory management control issue 
// garbage collector solved this issue but created a new issue - slow performances:
// [stopping/resuming the program]

// what is ownership every value has a single owner [every variable has one value and it is its sole owner]

