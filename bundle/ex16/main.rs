

enum Result<T, E> {
    Ok(T),
    Err(E),
 }

fn get_element_at(array: &[String], numb: i32) -> Result<String, String> {
    if numb < 0 {
        return Result::Err("IndexError: Index is negative".to_string());
    }
    if numb as usize > array.len() {
        return Result::Err("IndexError: Index is too big".to_string());
    }
    return Result::Ok(array[numb as usize].clone());
}


fn main() {
    let elemnt = get_element_at(&["Hello".to_string(), "World".to_string()], 3);
    let elemnt1 = get_element_at(&["Hello".to_string(), "World".to_string()], 1);
    match elemnt {
        Result::Ok(value) => println!("{}", value),
        Result::Err(error) => println!("{}", error),
    }
    match elemnt1 {
        Result::Ok(value) => println!("{}", value),
        Result::Err(error) => println!("{}", error),
    }
}
