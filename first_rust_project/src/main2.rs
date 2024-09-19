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

    fn call_a_animal(is_dog:bool) -> &dyn Animal {
        match is_dog {
            true => &Dog{},
            false => &Cat{},
            _ => &Dog{}
        }
    }

    fn call_animal_in_box(is_dog:bool) -> Box<dyn Animal> {
        if is_dog { 
            Box::new(Dog{})
        } else {
            Box::new(Cat{})
        }
    }

    Println!("{}",  call_animal(true).name());
}