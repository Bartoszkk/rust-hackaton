
fn is_none(option: Option<i32>) -> bool {
    match option {
        Some(_) => return false,
        None => return true
    }
}

fn get_str(array: &[String], word: &str) -> Option<String> {
    for element in array {
        if element == word
            {return Some(element.clone())}
    }
    return None
}

fn main() {
    println!("{}", is_none(Some(1)));
    println!("{}", is_none(None));

    println!("{:?}", get_str(&["Hello".to_string(), "World".to_string()], "World"));
}