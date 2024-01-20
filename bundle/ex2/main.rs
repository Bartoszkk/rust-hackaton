fn main() {
    let name = "Thomas";
    let age = 19;
    const MAX_AGE = 100;
  
    age = age + 1;
    println!("Hello, {}!", name);
    println!(
        "You are {} years old and you can still live {} years.",
        age,
        MAX_AGE - age
    );
  }