fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x, y and z are: {}, {} and {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four =tup.1;
    let one = tup.2;

    println!("The value of x, y and z are: {}, {} and {}", five_hundred, six_point_four, one);
}