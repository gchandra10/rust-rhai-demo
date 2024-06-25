fn check_guess(target, input) {
    if input < target {
        "Too low!"
    } else if input > target {
        "Too high!"
    } else {
        "You got it!"
    }
}
