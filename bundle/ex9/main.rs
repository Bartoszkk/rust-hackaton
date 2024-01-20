fn print_counter(number: &u32) {
    println!("{}", number);
}

fn increment_counter(number: &mut u32) {
    *number += 1;
}


fn main() {
    let mut counter = 0u32;
  
    while counter < 5 {
      print_counter(&counter);
      increment_counter(&mut counter);
    }
  }