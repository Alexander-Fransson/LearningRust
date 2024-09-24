fn main() {
    
}

fn _trait_with_type() {
    // Associated type to trait
    trait MyTrait{
        type MyType;
        fn _do_something(&self) -> Self::MyType;
    }

    struct MyStruct {}

    impl MyTrait for MyStruct {
        type MyType = String;
        fn _do_something(&self) -> Self::MyType {
            String::from("hello")
        }
    }
}

fn _returns_impl() {
    trait Animal {
        fn name(&self) -> String {
            let name = String::from("Sandy");
            return name
        }
    }

    struct Dog {}
    struct Cat {}

    impl Animal for Dog {}
    impl Animal for Cat {}

    fn call_animal() -> impl Animal {
        Dog{}
    }

    fn call_a_animal(is_dog:bool) -> Box<dyn Animal> {
        match is_dog {
            true => Box::new(Dog{}),
            false => Box::new(Cat{}),
        }
    }

    fn call_animal_in_box(is_dog:bool) -> Box<dyn Animal> {
        if is_dog { 
            Box::new(Dog{})
        } else {
            Box::new(Cat{})
        }
    }

    let animal = call_a_animal(true).name();
    println!("{}",  animal);
}

fn _array_with_trait_object() {
    trait Bird {
        fn quack(&self);
    }

    struct Duck;
    impl Bird for Duck {
        fn quack(&self) {
            println!("duck quack");
        }
    }

    struct Swan;
    impl Bird for Swan {
        fn quack(&self) {
            println!("swan quack");
        }
    }

    let birds: [& dyn Bird; 2] = [&Duck{}, &Swan{}];

    for bird in birds {
        bird.quack();
    }
}

fn _modify_types_standard(){
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8{
        fn draw(&self) -> String {
            format!("u8: {}", self)
        }
    }
}

fn _static_and_dynamic_dispatch() {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Foo for String {
        fn method(&self) -> String {
            format!("String: {}", *self)
        }
    }

    fn static_dispatch<T:Foo>(a:T){
        a.method(); // compilor will create separate functions for each type that implements Foo
        // fast but type size must be known at compile time
        // would work for u8 but not for string
    }

    fn dynamic_dispatch(a:&dyn Foo){
        a.method();
        // slower but works for string also
    }
}

fn _vector_from_array() {
    let arr: [u8; 3] = [1,2,3];
    let _vec: Vec<u8> = Vec::from(arr);
    let vec: Vec<u8> = arr.into(); // alternative way 
    let _v: Vec<u8> = vec!(1,2,3);
    let _v: Vec<u8> = vec![1,2,3]; // also wworks
    println!("{:?}", vec);
}

fn _vec_in_looå() {
    let full_vec = vec![1,2,3];
    let mut empty_vec: Vec<i32> = Vec::new();

    for num in &full_vec {
        empty_vec.push(*num);
    }

    empty_vec.pop(); // remove last
    empty_vec.extend([4,5,6].iter());

    println!("{}", empty_vec[0]); // index like a list
    // reference out of bounds will cause panic jsyk
}

fn _vec_slice_shenanigans() {
    let mut _v: Vec<i32> = vec![1,2,3];
    let slice1 = &_v[..];
    let slice2 = &_v[0.._v.len()];

    assert_eq!(slice1, slice2);

    // vec slices are read only
    // slice and reference to &Vec are diferent
}

fn _vector_with_many_types_through_enum() {
    enum IPAddr{
        V4(String),
        V6(String),
    }

    let _v: Vec<IPAddr> = vec![
        IPAddr::V4(String::from("127.0.0.1")), 
        IPAddr::V6(String::from("::1"))
    ];
}

fn _hashmap_shenanigans() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Sunface", 95);
    scores.insert("Daniel", 95);
    scores.insert("Laylay", 67);
    scores.insert("Garry", 54);

    let score = scores.get("Laylay");
    assert_eq!(score, Some(&67));

    if scores.contains_key("Daniel") {
        let _socre = scores["Daniel"]; // get accounts for panic
        scores.remove("Daniel");
    }

    // loop through all key value pairs
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    let _hashmap_from_tuples = HashMap::from([
        ("Sunface", 95),
        ("Daniel", 95),
        ("Laylay", 67),
        ("Garry", 54),
    ]);
}

fn _hashmap_functions() {
    use std::collections::HashMap;
    
    let teams = [
        ("key", 0)
    ];

    let mut teams_map: HashMap<&str, i32> = teams.into_iter().collect();

    // insert key only if it does not exist
    let &mut entry = teams_map.entry("pancakes").or_insert(2);
    assert_eq!(entry, 2);

    teams_map.entry("pancakes").or_insert_with(get_pancakes);

    fn get_pancakes() -> i32 {
        return 900;
    }
}

fn _hashmap_from_custom_type() {
    use std::collections::HashMap;

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking{
        name: String,
        origin: String,
    }

    impl Viking {
        fn _new(name: &str, origin: &str) -> Viking {
            Viking {
                name: String::from(name),
                origin: String::from(origin),
            }
        }
    }

    let viking = Viking::_new("Laylay", "Westeros");

    let vikings = HashMap::from([
        (Viking::_new("Laylay", "Westeros"), 67),
        (Viking::_new("Garry", "Westeros"), 54),
        (Viking::_new("Daniel", "Westeros"), 95),
        (Viking::_new("Sunface", "Westeros"), 95),
        (viking, 42)
    ]);

    println!("{:?}", vikings);
}

fn _type_conversions_using_from(){
    struct CustomNum {
        _value: i32,
    }
    impl From<i32> for CustomNum {
        fn from(item: i32) -> Self {
            CustomNum { _value: item }
        }
    }

    let num = CustomNum::from(5);
    let num2: CustomNum = 5.into(); // if you have from you also get into
    assert_eq!(num._value, num2._value);
}

fn _make_anything_to_string() {
    use std::fmt;

    struct Point {
        x: i32,
        y: i32
    }

    // enambles to string
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "the point is ({}, {})", self.x, self.y)
        }
    }

    let p = Point { x: 3, y: 4 };
    println!("{}", p.to_string());

    // getting to a number is similar but with parse
}

fn _example_of_result() {
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Divide by zero error"))
        } else {
            Ok(a/b)
        }
    }

    let result = divide(10, 2);
    match result {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }
}

fn _question_mark_operator() {
    use std::num::ParseIntError;
    fn multiply_with_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
        let num1 = a.parse::<i32>()?;// will unwrap Ok(num) to num
        let num2 = b.parse::<i32>()?;
        Ok(num1 * num2) // Return of result type needs to return an OK
    }

    assert_eq!(multiply_with_strings("10", "2"), Ok(20));
}

fn _map_example() {
    use std::num::ParseIntError;

    fn add_two_to_string_number(num: &str) -> Result<i32, ParseIntError> {
        // map is similar to that in ts
        num.parse::<i32>().map(|n| n+2)
    }

    // and_then needs to return a result
    fn add_three_to_string_number(num: &str) -> Result<i32, ParseIntError> {
        // map is similar to that in ts
        num.parse::<i32>().and_then(|n| Ok(n+2))
    }
}

