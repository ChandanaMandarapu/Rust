// collections rust standard library includes a number of very useful data structures called collections. Most other data types reperesnt one specific value but collections can contain multiple values the data these collections point to is stored on the heap

// vectors allow u to store more than one value in a single datastructure that puts all the values next to each other in memory 

fn main () {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}",vec);
}

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
        new_vec.push(val);
        }
    }

    return new_vec;
}