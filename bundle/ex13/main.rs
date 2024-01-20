
fn main() {
    let closure = |list:&[i32]| -> (i32,i32)  { return (list[0], list[list.len() - 1])};
  
    println!("{:?}", closure(&[1, 2, 3, 4, 5, 6]));
  }
  