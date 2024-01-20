

fn remove_first_and_last(list: &Vec<i32>) -> Vec<i32>{
    let mut list2 = list.clone();
    if !list2.is_empty() {
        list2.pop();
        list2.remove(0);
    }
    list2
}

fn concat_vec(list: &Vec<i32>, list2: &Vec<i32>) -> Vec<i32> {
    let mut list1 = list.clone();
    list1.extend(list2.iter().cloned());
    list1_iter = list1.iter();
    for element in list1_iter :
        element *= 2;
    list1
}

fn main() {
    println!("{:?}", remove_first_and_last(&vec![1, 2, 3, 4, 5]));
    println!("{:?}", concat_vec(&vec![1, 2, 3, 4, 5], &vec![6, 7, 8, 9, 10]));
  }
  