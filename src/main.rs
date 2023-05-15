fn main() {
    println!("Hello, world!");
    let mut x = 5;
    const CONSTANT: usize = 100;
    println!("The value of CONSTANT is: {}", CONSTANT);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y : i32 = 5;

    let y :i32 = y + 1; // 6

    // シャドーイング
    {
        let z: i32 = 5;
        let y: i32 = y * 2; // 12
        println!("The value of y in the inner scope is : {}", y);
        println!("The value of z is : {}", z);
    }
    // println!("The value of z is : {}", z);
    println!("The Value of y is : {}", y);

    let some_strings: &str = "aaa";
    println!("The value of spaces is : {}", some_strings);

    let some_strings: usize = some_strings.len();
    println!("The value of spaces is : {}", some_strings);

    // Rustのデータ型
    // addition
    let sum = 5 + 10;
    println!("The Value of sum is : {}", sum);

    // subtration
    let diffrence = 99.5 - 4.3;
    println!("The Value of diffrence is : {}", diffrence);

    // multiplication
    let product = 4 * 30;
    println!("The Value of product is : {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The Value of quotient is : {}", quotient);

    let floored = 2 / 3;
    println!("The Value of floored is : {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The Value of remainder is : {}", remainder);

    {
        let x: usize = 6;
        let y: f64 = 1.5;
        // let z = x / y;
        let z = (x as f64) / y;
        println!("The Value of z is : {}", z);
        
        // 論理値型
        let t = true;
        println!("The Value of t is : {}", t);

        let f: bool = false; // with explicit type annotation
        println!("The Value of t is : {}", f);
    }

}
