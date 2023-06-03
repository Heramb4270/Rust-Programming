

fn main() {
    println!("Hello, world!");
   // another_function(20,'h');
    // let mut x = plus_one(10);
    // println!("Value of X is {x}");
    // let y = farenhiete_to_celcius(64);
    // println!("Celcius Value of 64 Degree Farenheite is {y}");
    println!("FIbonacci Series of 1st 10 numbers are ");
    let mut a = 0;
    let mut b= 1;
    let mut temp = 0;
    println!("{a}");
    println!("{b}");

    for i in (1..10) {
        temp = a+b;
        a = b;
        b = temp;
        println!("{temp}");
    }
    

}

fn another_function(a : i32 , c : char){
    println!("Paramters Passed are {a} and {c}");
}

fn plus_one(value : i32) -> i32{
    value + 1
}

fn farenhiete_to_celcius(faren : i32)->i32{
    let Celcius : i32 = ((faren - 32)*5)/9;
    Celcius 
}

fn find_fibonacci_series(num : i32){

 
}