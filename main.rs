use std::io;

fn main(){
    println!("Area Calculator");
    println!("Choose a shape:");
    println!("1. Rectangle");
    println!("2. Square");
    println!("3. Circle");
    println!("4. Triangle");
    println!("5. Trapezium");

    let choice = read_number("enter your choice 1-5") as u32;
     
    let area = match choice{
        1 =>{
        let length:f64 = read_number("enter the length of rectangle");
        let width:f64 = read_number("enter the width of the rectangle");
        rectangle_area(length,width)
        }
        2 =>{
        let side:f64 = read_number("enter the side of the square");    
        square_area(side)     
        }
        3 =>{
        let rad:f64 = read_number("enter the radius of the circle:");
        circle_area(rad)
        }
        4 =>{
        let base:f64 = read_number("enter the base of the triangle");
        let height:f64 = read_number("enter the height of the triangle");
        triangle_area(base, height)
        }
        5 =>{
        let a = read_number("Enter first parallel side: ");
        let b = read_number("Enter second parallel side: ");
        let height = read_number("Enter height: ");
        trapezium_area(a, b, height)
        }
        _ =>{
            println!("enter valid input");
            return;
         }
    };
 println!("the are is {}",area);
}

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn rectangle_area(length:f64,width:f64) ->f64{
    lenght * width
}
fn square_area(side:f64) -> f64{
    side * side
}
fn circle_area(rad:f64) -> f64{
    std::consts::PI*rad*rad
}
fn triangle_area(base:f64, height:f64) -> f64{
    0.5*base*height
}
fn trapezium_area(a: f64, b: f64, height: f64) -> f64 {
    0.5 * (a + b) * height
}
