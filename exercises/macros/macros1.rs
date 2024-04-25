// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

//

macro_rules! my_macro {
    // The '()' indicates that this macro doesn't take any arguments.
    () => {
        // This is the code that will be inserted whenever the macro is called.
        println!("Check out my macro!");
    };
}

// This is your main function, where the program starts executing.
fn main() {
    // Here you're calling the macro you defined earlier.
    my_macro!(); // Note the '!' after the macro name.
}