// Using Scope to transfer the data between RHAI Script and Rust main application.

use rhai::{Engine, Scope};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::new();

    let script1 = r#"
        let z = x + y;
        print(z);
    "#;

    let script2 = r#"
        let result = z * 2;
        print(result);
    "#;

    let mut scope = Scope::new();
    scope.push("x", 10_i64); // Ensure x is i64
    scope.push("y", 20_i64); // Ensure y is i64

    // Evaluate script1 and capture the value of 'z'
    engine.eval_with_scope::<()>(&mut scope, script1)?;

    // Get the value of 'z' from the scope
    if let Some(z) = scope.get_value::<i64>("z") {
        println!("Value of z in Rust Main App: {}", z);
    } else {
        println!("Variable 'z' not found.");
    }

    // Evaluate script2
    engine.eval_with_scope::<()>(&mut scope, script2)?;

    Ok(())
}
