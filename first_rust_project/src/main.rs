fn main() {
    let firstvariable: i32 = 5; // implicitly typed
    let firstvariable = "redeclated";
    println!("the variable has a value of {}", firstvariable);

    let mut firstmutablevariable = 5;
    firstmutablevariable = 6;
    // might still lead to warining as it is unnececary
    println!("the mutable variable has a value of {}", firstmutablevariable);

    const FIRSTCONSTANT: i32 = 5;
    println!("the constant has a value of {}", FIRSTCONSTANT);
}
