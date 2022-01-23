fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let a = 5;
    let a = a + 1; // this is a shadow of orig a

    {
        let a = a * 2; // this is again a shadow of orig a in new scope
        println!("The value of x in the inner scope is: {}", a);
    }

    println!("The value of x is: {}", a);
}