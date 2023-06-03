use std::io::{self, stdin};
fn main() {
    let a = [1,2,3,4,5];
    println!("Please Enter the Array Index");

    let mut index = String::new();

    io::stdin()
            .read_line(&mut index)
            .expect("Failure");

    let index : usize = index.trim().parse().expect("Index Entered Was Not Number");

    let element = a[index];

    println!("Value of Entered Index is {element}");

    // Data Types

    let arr = [5;3];

    let mut i = 0;

    loop{
        println!("Element at Index {i} is {}",arr[i]);
        i = i +1;

        if i == 2 {
            break;
        }
    }

}
