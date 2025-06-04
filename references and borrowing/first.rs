// references and borrowing

// safety and performance

// borowing and references are powerful concepts 

// understanding references
// references enable u to borrow values without taking ownership.
// immutable reference
// mutable reference
// creating a reference by addingg "&"
// using mut keyword to actuallly use the references
// u can have only one mutable refereence and many immutable references

// fn main () {
//     let mut _x : i32 = 5;
//     // creating immutable reference of varaible x
//     let _r : &mut i32 = &x;
//     *_r += 1;
//     println!("value of _x : {}",_x);

// }

fn main() {
    let mut account: BankAccount = BankAccount {
        owner: "andrew".to_string(),
        balance: 150.55,
    };

    // immutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withdraw money
    account.withdraw(50.89);
}

// struct
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "account owned by {} has a balance of {}",
            self.owner, self.balance
        );
    }
}
