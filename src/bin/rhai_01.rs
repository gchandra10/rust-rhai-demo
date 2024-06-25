// import the scripting engine
use rhai::Engine;

fn main() {
    // Create the scripting engine
    let engine = Engine::new();

    // Run the script at "src/my_script_01.rhai"
    let result = engine
        .eval_file::<String>("src/my_script_01.rhai".into())
        .unwrap();

    // print the results of the script
    println!("{}", result);
}
