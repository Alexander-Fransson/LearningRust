use std::io;
use std::mem::size_of_val;

fn main() {

}

fn _diferent_ways_to_modify_a_string() {
    let mut string = String::from("hello");
    string.push(',');
    string.push_str("world");
    string += "!";

    string = string.replace("world", "wardo");
    
    let not_a_string_but_a_string_slice = "!!!";
    let not_str = String::from("!!!");

    string = string + not_a_string_but_a_string_slice + not_str.as_str() + &not_str; // only &str can be concatenated to String

    println!("{} {}", string, string.len());
}

fn _you_may_make_multiple_references_to_the_same_variable_if_ownership_is_transferred() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(" world");
    let r2 = &mut s; // takes over the ownership 
    r2.push_str("!");

    //r1.push_str("!"); you shall not use the abandoned variable
}

fn _can_borrow_mut_as_let_but_not_vice_versa(){
    let x: String = String::from("new");
    let mut _y: String = String::from("other");
    let z = &_y;
    //let mut w = &mut y;    

    println!("{} {}", *z, x);
}

fn _print_the_pointer() {
    let x: i32 = 5;
    let p = &x;
    println!("them memory adress is{:p}", p);
}

fn _if_one_node_in_a_tuple_looses_the_custedy_it_doesnt_invalidate_the_other_members_of_the_tyuple() {
    let x: (String, String) = (String::from("hello"), String::from("world"));
    let _s = x.0;

    println!("{:?}", x.1);
}

fn _structs_are_like_custom_types() {

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<i32>,
    }

    let person_instance: Person = Person {
        name: String::from("Megatron"),
        age: Box::new(21),
    };

    // get the values of the struct as normal variables
    let Person { name, ref age } = person_instance; // this does so we cannot use the person_instance

    println!("{} is {}, these are atributes of a person", name, age);
}

fn _using_heaped_variables_that_are_normally_on_the_stack() {
    let x: Box<i32> = Box::new(5); // assigns a value to heap and makes the variable a pointer
    let mut y: Box<i32> = Box::new(4);

    *y = 6; // access the value not the pinter using *

    assert_eq!(*x, 5);
}

fn _string_that_can_be_copied() {
    let _x: (i32, i32, (), String) = (1, 2, (), String::from("hello")); // cannot be copied as String is not copyable
    let y: (i32, i32, (), &str) = (1, 2, (), "hello"); // &str can be copied as it is imutable
    let _z = y;
    println!("Success");
}

fn _use_methods_taht_dont_consume_the_variables() {
    fn give_ownership() -> String {
        let s: String = String::from("hello");
        // s.into_bytes(); this would consume the string
        let _s = s.as_bytes(); // this just consumes a reference
        s
    }

    let s = give_ownership();
    println!("s is {}", s);
}

fn _assing_variables_value_to_another_variable() {
    let x:String = String::from("hello");
    let y:String = x.clone();

    // if we want to use x after assigning its value to another variable we have to clone it
    println!("{} {}", x,y);
}

fn _ownership_demonstrated() {
    let x: String = String::from("hello");

    fn takes_ownership(some_string: String) {
        println!("some_string is {}", some_string);
    }

    takes_ownership(x);
    // x is not valid here anymore unless it is returned and reassigned
}

fn _never_returns() -> ! {
    panic!("This function never returns");
    //unimplemented!("This function never returns");
    //todo!("This function never returns");
}

fn _declarations_are_expressions() {
    let v:i32 = {
        let mut x = 1;
        x += 1; // a variable assignment is an expression just like a declaration
        x
    };

    print!("{}", v);
}

fn _a_variable_may_be_calculated_as_long_as_the_scope_returns_an_expression() {
    let x:u32 = 5_u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        
        // y will be assigned to this
        x_cube + x_squared + x //; would make this an expression which cannot be returned by a function
    };
    
    println!("y is: {}", y);
}

fn _default_return_type() -> () {
    println!("unit type has size zero bytes")
}

fn _all_chars_have_a_size_of_4_bytes() {
    let c1:char = 'a';
    assert_eq!(size_of_val(&c1), 4);
    println!("Success");
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
