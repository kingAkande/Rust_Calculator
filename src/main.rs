fn main() {
    
    println!("Find the result below");

    let q = calc(3, 8, "sum"); // Call calc with the "sum" operator
    let r = calc(9, 3, "sub"); // Call calc with the "sub" operator
    let s = calc(10,2,"div"); //Call calc with the "div" operator
    let t = calc(2,4, "mul"); //Call calc with the "mull" operator

    println!("summation is {}", q);
    println!("subtraction is {}", r);
    println!("division is {}", s);
    println!("multiplication is {}", t)
}

fn sum(x: i8, y: i8) -> i8 {
    x + y
}

fn sub(x:i8, y:i8) -> i8{

     x-y
     }

fn mul(x: i8, y: i8)->i8{

    x * y
}    

fn div(x:i8 , y:i8) -> i8{

        x/y

}     

fn calc(w: i8, m: i8, operator: &str) -> i8 {
    match operator {
        "sum" => sum(w, m),

        "sub" => sub(w,m),
        
        "mul" => mul(w,m),

        "div" => div(w, m),

        _ => {
            println!("Unknown operator: {}", operator);
            0
        }
    }
}
