fn main() {
    println!("Hello, world!");
    let x = check_greatest_number_among_three(10,20,30);
    println!("Greatest Number is {x}");
}
fn check_greatest_number_among_three(a:i32,b:i32,c:i32) -> i32{

    if a>b && a>c{
        a
    }else if b>c{
        b
    }else{
        c
    }
}