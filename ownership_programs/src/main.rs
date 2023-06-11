fn main() {
    // println!("Hello, world!");
    // let x : i32 = 5;
    // let y :i32 = x;

    // let s1 = String::from("Hellow");
    // let s2 = s1;

    // println!("{} , World",s2);

    let mut s1 = String::from("Hello");
    let s2 = &s1;
    println!("Value of s1 is {}",s2);
    let mut s3 = &mut s1;


    println!("Value of s1 is {}",s3);

    let (s2,len) = calculate_length(s1);

    println!("The Length of '{}' is {}",s2,len);


    let mut a1 = String::from("Hello World");
    let a2 = first_word(&a1);
    println!("First Word is {}",a2);
}

fn calculate_length(s1 : String) -> (String,usize){
    let length = s1.len();
    (s1,length)
}

fn first_word(s1:&str) -> &str{
    let bytes = s1.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s1[0..i];
        }
    }
    return &s1[..];
}