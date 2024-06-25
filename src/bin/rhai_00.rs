use rhai::{Engine};

fn main() {
    let engine = Engine::new();
    let script = r#"
        let x = 5;
        let y = x * 2;
        print(y);
    "#;

    // Evaluate the script
    engine.eval::<()>(&script).unwrap();
}

