use std::io;


static USER: &'static str = "siddhanth";
static PASS: &'static str = "rustacean123";

fn main(){

    let mut username = String::new();
    let mut password = String::new();
    
    
    println!("enter username: ");
    io::stdin()
        .read_line(&mut username)
        .expect("failed to read username");
    username.pop();
    
    
  
    println!("enter password: ");
    io::stdin()
        .read_line(&mut password)
        .expect("failed to read password");
    password.pop();


    let access = login(username, password);

    println!("{}", access);
}    

fn login(username: String, password: String) -> String {
    if username == USER && password == PASS {
        return String::from("successfully logged in!");
    } else if username == USER && password != PASS {
        return String::from("access denied, check password");
    } else if PASS == password && username != USER {
        return String::from("access denied, check username");
    } else {
        return String::from("access denied, bad credentials");
    }
}