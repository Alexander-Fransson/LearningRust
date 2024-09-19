use std::io;
use std::mem::size_of_val;


fn main() {
}



fn _implementations_requirement_in_function_parameters() {

    fn multiply_any_type<T: std::ops::Mul<Output = T>>(a:T,b:T) -> T {
        a * b
    }

    trait Summerize{
        fn make_summary(&self) -> String {
            let summary: &str = "this is a summary";
            summary.to_string()
        }
    }

    struct Essay {}

    impl Summerize for Essay {}

    fn needs_summary(summary: &impl Summerize) {
        println!("{}", summary.make_summary());
    }

    let summary = Essay {};

    needs_summary(&summary);
}

fn _derive_stuff(){
    #[derive(Debug)] // quick implements the debug trait    
    struct Point {
        x: i32,
        y: i32,
    }

    println!("{:?}", Point{x: 1, y: 2});
}

fn _trait_shenanigans(){
    trait Greeting {
        fn _say_hi(&self) -> String { // a default implementation that does not have to be implemented each time
            let x = String::from("hello");
            return x
        }

        fn _say_something(&self) -> String; // no default implementation so it has to be manually implemented
    }

    struct Sempai{}
    struct Sensei{}

    impl Greeting for Sempai {

        fn _say_hi(&self) -> String {
            return String::from("hiiiiiiiiiiiiiiiiii")
        }

        fn _say_something(&self) -> String {
            return String::from("salam")
        }
    }

    impl Greeting for Sensei {
        fn _say_something(&self) -> String {
            return String::from("salam")
        }
    }

    println!("{} {}", Sempai{}._say_hi(), Sensei{}._say_something());
}

fn _const_generics() {
    struct ConstArray<T, const N: usize> {
        data: [T; N],
    }

    let _a = ConstArray::<i32, 10> { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] };
}

fn _gnerics_in_methods() {
    struct Point<T> {
        x: T,
    }
    impl<T> Point<T> {
        fn _get_x(&self) -> &T { 
            &self.x
        }
    }
}

fn _multiple_generics_in_structs() {
    struct Point<T,U> {
        x: T,
        y: U,
    }

    let _p: Point<i32,String> = Point{x: 1, y: String::from("hello")};
}

fn _generics_shenanigans() {
    struct A;       // concrete type A
    struct S(A);    // concrete type S, just takes an argument of type A
    struct SGen<T>(T); // generic type SGen

    fn regular_fn(_s:S) {}
    fn generic_spec_i32_fn(_s: SGen<i32>) {}
    fn gneeric_fn<T>(_s:SGen<T>) {}

    fn use_functions() {
        regular_fn(S(A));
        gneeric_fn(SGen(A));
        generic_spec_i32_fn(SGen(88));
    }

    // explicitly specify char to generic()
    gneeric_fn::<char>(SGen('a')); // called turbo fish syntax

    // implicitly specify i32 to generic()
    gneeric_fn(SGen(32));

    fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
        x + y 
    } // makes sure that T implements Add

    sum(1,2);
    sum(1.2,3.3);
    sum(2_i8, 3_i8);

}

fn _methods_in_enums() {
    enum Foo {
        Bar,
        Poo(String)
    }
    impl Foo {
        fn _new() -> Self {
            Self::Bar
        }

        fn _set_poo(&mut self, value: String) {
            if let Self::Bar = self {
                *self = Self::Poo(value);
            }
        }
    }
}

fn _method_shenanigans() {
    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        pub fn _new(width: u32, height: u32) -> Self {
            Self {width, height} // Self could be Rectangle
        }

        pub fn _area(&self) -> u32 {
            self.width * self.height
        }

        pub fn _widen(&mut self, amount: u32) {
            self.width += amount // can be used to change the value of the struct
        }
    }

    let mut rect = Rectangle::_new(100, 200); // called on the type not on an instance
    rect._widen(100);
    assert_eq!(rect._area(), 300);
    println!("Success");
}

fn _match_first_and_last_values_of_tuple() {
    let tuple = (1, 2, 3);
    match tuple {
        (first, .., last) => println!("first = {}, last = {}", first, last),
    }
}

fn _match_guards() {
    let x = Some(5);
    let aditional_requirement_to_match = 10;
    // put in extra requirement in match
    match x {
        Some(50) => println!("Got 50"),
        Some(y) if aditional_requirement_to_match == y => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
}

fn _at_match_explained() {
    struct Point {
        x: i32,
        y: i32
    }

    let p: Point = Point {x: 0, y: 0};

    match p {
        Point {x, y:1} => println!("{} we get x through a variable byt not y", x),
        Point {x, y:y @ 0} => println!("Now we can both get and mesure y {} {}", x, y),
        Point {x:x @ 0..=10, y:y @ (21|22)} => println!("{} is between 0 and 10 and {} is 21 or 22", x, y),
        _ => ()
    }
}

fn _matches_macro() {
    let alphabet = ['a', 'b', 'c'];
    for letter in alphabet {
        assert!(matches!(letter, 'a'..='z' | 'A'..='Z'));
    }
}

fn _if_let_explained() {
    let x = Some(5);

    match &x {
        Some(val) => println!("{}", val),
        None => ()
    }
    // is the same as

    if let Some(val) = &x {
        println!("{}", val);
    }

}

fn _nested_loop_loops_needs_to_be_labled() {
    let mut i = 0;
    'outer: loop {
        let mut j = 0;
        'inner1:loop { 
            println!("i is {}, j is {}", i, j);
            j += 1;
            if j == 5 {
                break 'inner1;
            }
        }

        'inner2: loop {
            println!("i is {}, j is {}", i, j);
            break 'inner2;
        }

        i += 1;
        if i == 5 {
            break 'outer;
        }
    }
}

fn _loops_can_also_be_used_to_assign_variables() {
    let mut i = 0;
    let _j = loop { // will loop forever untill manually break
        println!("i is {}", i);
        i += 1;

        if i == 5 {
            break i;
        }
    };
}

fn _for_with_index() {
    for (i, j) in (5..10).enumerate() {
        println!("{} {}", i, j);
    }
}

fn _if_else_can_be_used_when_assigning_variables() {
    let n = 32;
    let depends_on_n = if n < 2 {5} else {7};
    println!("{}", depends_on_n);
}

fn _options_enum() {
    // a type that can be either Some of anything or None

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let _none: Option<i32> = plus_one(None);

    if let Some(n) = six {
        println!("six is {}", n);
    }
    
}

fn _enum_in_array() {
    #[derive(Debug)]
    enum Message {
        Title(String),
        Length(i32),
        BackgroundColor(u8, u8, u8),
    }

    let messages: [Message; 3] = [
        Message::Title(String::from("hello")),
        Message::Length(5),
        Message::BackgroundColor(255, 255, 255),
    ];

    for message in messages {
        println!("{:?}", message);
    }
}

fn _instantiate_enum() {

    #[derive(Debug)]
    enum Hero {
        Name(String),
        Move {x: i32,y: i32,},
        ChangeColor(u8, u8, u8),
        GiveUp,
    }

    let action1 = Hero::Name("Sandy".to_string());
    let action2: Hero = Hero::GiveUp;
    println!("{:?}", action1);
    println!("{:?}", action2);

    let moovement = Hero::Move {x: 1, y: 1};
    if let Hero::Move { x:a, y:b } = moovement {
        // a way to get the values but it seams overly complex
        assert_eq!(a, b)
    }
}

fn _enum_shenanigans() {
    enum ShoeSize {
        Small, // implicitly this has a value of 0
        Middle, // 1
        Large, // 2
    }

    enum Number {
        First = 8, // cannot be 8.0 folat
        Second, // 9 implicitly follows the pattern
    }

    assert_eq!(8, Number::First as i32);
}

fn _printable_struct() {
    #[derive(Debug)]
    struct Printable {
        print: String,
        me: String,
    }

    let i = Printable {
        print: String::from("hello"),
        me: String::from("world"),
    };
    dbg!(&i); // prints to error
    println!("{:?}", i) // to print out things with the debug trait use {:?}
}

fn _tuple_struct_shenanigans() {
    struct Rgb(u8,u8,u8);
    let rgb = Rgb(255,255,255);

    fn show_color(color: &Rgb) {
        println!("{} {} {}", color.0, color.1, color.2);
    }

    show_color(&rgb);

    println!("{} {} {}", rgb.0, rgb.1, rgb.2);
}

fn _struct_shenanigans() {
    struct Humunculus {
        status: String,
        number: i32,
    }

    let mut i:Humunculus = Humunculus {
        status: String::from("alive"),
        number: 5,
    };

    i.number += 1;

    fn build_humunculus(status: String, number: i32) -> Humunculus {
        Humunculus {
            status,
            number,
        }
    }

    let mut dave = build_humunculus(String::from("dead"), 2);
    dave.status = "alive".to_string();

    

    println!("{} {} {}", i.status, i.number, dave.status);
}

fn _tuple_shenanigans2_electric_boogaloo() {
    let t:(i32, f32, char) = (5, 3.14, 'c');
    let (x , y, z) = t;
    println!("{} {} {}", x, y, z);

    // another way to declare them
    let (a, b, c);
    (a,b,c) = (1,2,3);
    println!("{} {} {}", a, b, c);
}

fn _slice_shenanigans() {
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..3];
    let _first_two = &arr[..2];
    assert!(slice.len() == 2);
}

fn _if_you_want_an_array_of_a_single_number() {
    let _a = [1; 100];
}

fn _array_shenanigans() {
    let a:[i32;5] = [1, 2, 3, 4, 5];
    let _inferred = ['a','b'];
    let less_inferred: [_; 2] = ['a','b'];
    assert!(a.len() == 5);

    fn calculate_memsize_of_char_array(array:&[char;2]) -> usize {
        array.len() * size_of_val(&array[0])
    }

    assert!(size_of_val(&less_inferred) == calculate_memsize_of_char_array(&less_inferred));
    println!("{}", a[0]);
}

fn _iterates_through_string() {
    let s = String::from("hello");
    for c in s.chars() {
        println!("{}", c);
    }
}

fn _access_pars_of_string() {
    let s = String::from("hello");
    let hello = &s[0..5];
    let world = &s[6..11];
    // index does not work in rust, use slices instead
    let _just_h: &str = &s[0..1]; // in js it would be s[0]
    println!("{} {}", hello, world);    }

fn _raw_strings_dont_care_about_backslashes() {
    let raw_str = r"Escapes don't work here! \n";
    println!("{}", raw_str);
}

fn _two_ways_to_make_a_string() {
    let sliced: &str = "hello";
    let string = &sliced.to_string();
    let other_way_to_make_a_string = String::from(sliced);

    println!("{} {} {} two ways to make a string", sliced, string, other_way_to_make_a_string);
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
