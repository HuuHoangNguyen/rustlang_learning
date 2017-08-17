fn main() 
{
    let mut a = 5;
    let (x, y) = (10, 20);
    
    println!("Hello RustLang! a = {}\n", a);
    println!("Hello RustLang! x & y = {0} & {1}\n", x, y);
    a = 10;
    println!("Hello RustLang! a = {}\n", a);
 
    let b : i32 = 8;
    println!("Hello RustLang! b = {}\n", b);


    let m : i32 = 17;
    {
        let n : i32 = 10;
         println!("The value of m & n is {} & {}", m, n);
    }
    
    //intln!("The value of m & n is {} & {}", m, n);


    let c : i32 = 8;
    {
        println!("{}", c); // Prints "8"
        
        let c = 12;          
        println!("{}", c); // Prints "12"
    }
	
    println!("{}", c); // Prints "8"
    let c = 42;


    println!("{}", c); // Prints "42"

}
