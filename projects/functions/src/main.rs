fn five() -> i32 {
    5
}

fn main() {
    another_function(5, 6);

    let x = plus_one(five());

    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}