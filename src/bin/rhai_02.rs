use rand::Rng;
use rhai::{Engine, Scope};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::new();

    let script = fs::read_to_string("src/game_script.rhai")?; // Load the Rhai script from a file
    let ast = engine.compile(&script)?; // Compile the script into an AST

    let target: i64 = 18; // Define the target number in Rust

    let mut rng = rand::thread_rng(); // Create a random number generator

    loop {
        let input: i64 = rng.gen_range(1..=20); // Generate a random input from 1 to 50
        let mut scope = Scope::new();
        scope.push("target", target);
        scope.push("input", input);

        // Call the `check_guess` function from the AST
        let result: String = engine.call_fn(&mut scope, &ast, "check_guess", (target, input))?;
        println!("Guess: {}, Result: {}", input, result);

        if result == "You got it!" {
            break;
        }
    }

    Ok(())
}
