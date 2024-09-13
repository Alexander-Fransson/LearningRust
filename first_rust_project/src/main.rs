use std::io;

fn main() {
    
}

fn _individual_bit_operations_that_move_bit_values_in_byte_words() {
    assert_eq!(1_i32 << 5, 32);
    assert_eq!((0x80u32 >> 2) as u8, 0x20);
}

fn _biteise_or_and_xor() {
    assert_eq!(true & false, false);
    assert_eq!(true || false, true);
    assert_eq!(true ^ false, true);
    assert_eq!(true ^ true, false);
    println!("Success");
}

fn _can_also_loop_through_chars() {
    for letter in 'a'..='z'{
        // the = meains that z will be included
        println!("{}",letter)
    }
}

fn _for_loop_that_iterates_through_a_range() {
    let mut x: i32 = 0;
    for y in -3..2 {
        // iterates through the range -3, -2, -1, 0, 1
        // 2 is exclueded
        x += y;
    }
    assert_eq!(x, -5);
    println!("Success");
}

fn _floats_can_be_written_in_many_ways() {
    assert_eq!(0.1_f32+0.2 as f32, 0.3_f32);
    println!("Success");
}

fn _floats_can_be_extant() {
    let x: f64 = 1_000.000_1;
    println!("{}", x);
}

fn _max_of_types() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success");
}

fn _type_offer() {
    fn type_of_displayed_as_string<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

    let x:u32 = 5;

    assert_eq!(type_of_displayed_as_string(&x), "u32".to_string());
    println!("Success");
}

fn _another_way_to_assign_type() {
    let v: u16 = 38_u8 as u16;
    println!("the value is {}", v);
}

fn _mutation_must_be_done_to_the_same_type() {
    // variable assignment must be of same type
    let x:i32 = 10;
    let mut y: u32 = 5;
    y += x as u32;

    assert_eq!(y, 15);
    print!("Success");
}

fn _assert_with_tuple_and_array_and_ignor_with_dots(){

    let (x,y); // declare variables

    (x,..) = (3,4);
    [.., y] = [1,2];// works on both tuples and arrays

    assert_eq!([x,y], [3,2]);
    println!("I dont care I like it")
}

fn _tuple_shenanigans() {
    let mut x = (1,2);
    let (mut q, _p) = (1,2); // declare many variables at once
    x.0 += 2;
    q += 1;

    assert_eq!(x.0,q);
    println!("tuple shenanigans")
}

fn _functions_dont_return_statements_in_rust() {
    fn define_x() -> String {
        String::from("hellow")
        // let &str = "hello";
    }

    let x = define_x();
    println!("{} world", x);
}

fn _scope_is_an_thing_in_rust() {
    // scope

    let x: i32 = 10;
    {
        //let y: i32 = 20;
        println!("x is {} and can still be here though", x);
    }
    let y: i32 = 20;

    println!("thy need to be on the same scope to be used together {} {}", x, y);
}

fn _mutability_makes_variables_changable() {
    // mutability

    let mut x: i32 = 1;
    x = x + 2;
    assert_eq!(x, 3);
    println!("Sekai des");
}

fn _some_if_statement () -> String {
    if true {
        return String::from("you could not just return after if");
    } else {String::from("not this string")} // this is another way to return
    // code here would run though
}

fn _add_numbers(x: i32, y: i32) -> i32 {
    return x + y;
}

fn _get_string() -> String {
    return String::from("this was harder than I thought");    
}

fn _how_to_use_terminal_input() {
    println!("hi there");

    let mut input: String = String::new();

    // &mut variable must be used so that a function can modify the actual variable and not just use a copy of it
    io::stdin().read_line(&mut input).expect("faliled to read line");
    println!("you typed: {}", input);
}

// fn _xmain() {
//     let _firstvariable: i32 = 5; // implicitly typed
//     let firstvariable = "redeclated";
//     println!("the variable has a value of {}", firstvariable);

//     let mut firstmutablevariable: i32 = 5;
//     firstmutablevariable = 6;
//     // might still lead to warining as it is unnececary
//     println!("the mutable variable has a value of {}", firstmutablevariable);

//     const FIRSTCONSTANT: i32 = 5;
//     println!("the constant has a value of {}", FIRSTCONSTANT);
// }
