fn main() {
    let i = 6;
    
    call_int(i);
    println!("after calling the func value od i : {}",i);

    let s = String::from("hello");
    call_string(s);
    println!("after calling fun the value of s :{}",s);
}

fn call_int(i:32){
    println!("call_int i: {}",i);
}

fn call_string(s:String) {
    println!("call_string s: {}",s);
}