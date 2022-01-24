// You made it! That was a sizable chapter: you learned about variables, 
// scalar and compound data types, functions, comments, if expressions, 
// and loops! To practice with the concepts discussed in this chapter, 
// try building programs to do the following:

// Convert temperatures between Fahrenheit and Celsius.
fn fahrenheit_celsius(fahren:f32)->f32{
    (fahren -32.0)/ 1.8

}

// generate nth fib number
fn get_n_fib(mut n:u8)->u32{
    let mut fib: u32 = 1;
    let mut prev_fib: u32 = 0;
    while n > 0{
        let new_fib = fib + prev_fib;
        prev_fib = fib; 
        n = n - 1;
        fib = new_fib;
    }
    return fib
}

fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let celcius = 12.0;
    let fahren = fahrenheit_celsius(celcius);
    println!("{} C is {} F", celcius, fahren);

    let x = 6;
    println!("{} fib is {}", x, get_n_fib(x));
}