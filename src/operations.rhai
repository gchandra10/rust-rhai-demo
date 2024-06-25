fn add(a, b) {
    a + b
}

fn subtract(a, b) {
    a - b
}

fn multiply(a, b) {
    a * b
}

fn divide(a, b) {
    if b == 0 {
        print("Denominator cannot be zero");
        throw "Division by zero!";
    }
    a / b
}
