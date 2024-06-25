use rhai::{Engine, Scope};
use std::error::Error;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a Rhai engine
    let engine = Engine::new();

    // Load the Rhai script from file
    let script = fs::read_to_string("src/operations.rhai")?;
    let ast = engine.compile(&script)?;

    // Take two numbers as input
    let mut input = String::new();
    println!("Enter the first number: ");
    io::stdin().read_line(&mut input)?;
    let a: i64 = input.trim().parse()?;

    input.clear();
    println!("Enter the second number: ");
    io::stdin().read_line(&mut input)?;
    let b: i64 = input.trim().parse()?;

    // Ask for the operation
    input.clear();
    println!("Enter the operation (add, subtract, multiply, divide): ");
    io::stdin().read_line(&mut input)?;
    let operation = input.trim();

    // Create a scope and add the numbers
    let mut scope = Scope::new();
    scope.push("a", a);
    scope.push("b", b);

    // Call the appropriate function in the Rhai script
    let result: i64 = match operation {
        "add" => engine.call_fn(&mut scope, &ast, "add", (a, b))?,
        "subtract" => engine.call_fn(&mut scope, &ast, "subtract", (a, b))?,
        "multiply" => engine.call_fn(&mut scope, &ast, "multiply", (a, b))?,
        "divide" => engine.call_fn(&mut scope, &ast, "divide", (a, b))?,
        _ => {
            println!("Invalid operation");
            return Ok(());
        }
    };

    // Print the result
    println!("The result is: {}", result);

    Ok(())
}
