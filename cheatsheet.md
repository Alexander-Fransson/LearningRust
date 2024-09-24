# Rust Cheat Sheet

## Basic Commands

### Install rust
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compile a file
```sh
rustc file.rs
```

### Creat project with cargo
```sh
cargo new project_name
```

### Compile and run file when in a cargo project
navigate to project folder and then run
```sh
cargo build
```
```sh
cd target
cd debug
./project_name
```

alternatively you can automatically recompile and run the code directly by running the command bellow from inside the project folder
```sh
cargo run
```

### Command to check if there are bugs without beginning to compile
```sh
cargo check
```

### Does rust need ;
yes 

### Format your code in rust file
```sh
rustfmt file_name.rs
```

## Variables
- In rust they can be both explicitly and implicitly typed, default implicit
- ones they are typed they cannot change
- all variables are immutable by default
- you can still in a way change imutable variables but this will overwrite the variable in memory
- variables are isolated to their scope {} but you can still use the ones in the exterior from the interior
- consts are more than immutable they cannot be redeclared and are commonly written with capitalization and must have their type defined

## Data types
### Two categoris
- sclar: which is data types with a single value. ints, chars, etc
- compact: which is data containing multiple values. arrays, objects

## Primitive Scalar types

### ints
- i8-i128, intigers with specific memory allocation. i8 can hold a value if -128->127. Default is i32
- u8-u128, positive integers intigers that otherwise behave similarly to i

good to konw: you cannot add ints of diferent types

### floats
- f32 and f64

### booleans
- just true and false

### characters
- char: single quotation character 'a'

## Primitive Compound types

### Lists
- tuple: imutable and fixed length. let tup:(char,char,char) = ('a','b','c'). can be mutable by mut. To access values use tup.0 for first.
- array: fixed lenght and contains single type of value. acces values with arr[0]. define type by [type; numberOfValues]

## Import crates
```ru
use module::function
```
this is like import in js, ruby and python

## Tyoe conversion
you cannot do operations with parameters of diferent types if something expects something I expect you have to explicitly change that using the "as" keyword becouse rust is strongly typed and lacks inheritance.

## Expressions vs statements
Statements are instructions that does not return a value like an if or fn. Expressions result in a value. A function in rust may not return a statement.


## Ownership "a concept exclusive to rust"
- Each value in rust has an owner which is the variable or data structure thet holds it and is responsible for allocating and freeing up memoryused to store the data
- There can only be one owner at a time
- The value is dropped when the owner goes out of scope

## Stack vs heap
Stack is super fast and ooperates on first in first out principles like a stakck of pancakes you eat what whas made most recently, primitive data types like chars and ints as well as function declarations are stored in the stack. The heap is like an object as it holds values in memory accesible through an adress. Variables related to the heap are for example the string as a string variable does not realy hold the string values but rather its adress on the heap.

### Copy vs Move when a value is assigned
In scalar values such as ints and chars if you assign a variable the value of another the value is copied in the stack as it is comparatively cheap. Dynamicallys sized data on the other hand will get moved as copying is to expensive. This means that the pointer will be copied by a variable assigned to another so if you do something with the data it will affect them both. This is forbidden by the second ownership rule in rust "there can be only one owner of a value" so rust will drop the first assignee. You can explicitly copy dynamically sized variables with the clone variable.

## Borrowing
- Temporarily access the data without taking ownership of it
- take a reference "pointer" towards the data not the thing in itself
- you can ither have one mutable reference or many imutable references at a time, not both
- If you transferr ownership, aka dont use the reference again you may make another &mut

## Compound types

### String vs &str
- String is mutable, owns its contents and lives in the heap
- &str is immutable and does not own its underlying data
- use &str if you just want to view the string otherwise use String
- a reference to a String can be implicitly converted to &str by the compiler

### Arrays
- fixed size collections of the same data type stored as a contigous block in stack memory
- signofied by [T; Length] and can nither grow nor shrink
- array length cannot be set by code as it must be seen at compile time
- index like normal, out of bound index will cause compiler panic

### Slices
- reference to contigous sequence of elements in a collection
- provides a way to borrow a part of the collection without taking ownership
- can be created from arrays, vectors and Strings and other collections implementing a Dref trait
- becouse slices size is not known at compile time we have to use a & pointer when defining
- as a slice contains a pointers that is a two word characters it is 8 bytes larger than an array of the same size

### Tuples
- a way to strore related values of diferent types in a single variable
- stored on the stack a fixed size contigous value
- tuples can be nested
- members can be extracted through indexong tup.0
- tuples can be used and returned by functions

### Structs
- custom types
- compound data types allowing you to group data together into named structures
- similar to tuples but each value is accessed through a name instead of the index
- have to be instantiated with data like classes that lack inheritance
- instances fields can be accessed and mutated provided they are mut
- you can not make individual fields mutable in a struct just all or nothing
- functions can instantiate and return structs
- you can copy the data of a struct instance by Struct {..instance} like ... in ts
- if a variable gains ownership of a struct parameter the struct as a whole cannot be accessed just like its parameter that lost custedy of its value

### Enums
- a custom type that can have only one of a possible number of values
- we can only access one variant of an enum at a time,
- they can contain extra information using tuples
- enums cannot have float values
- can be converted to ints type by as
- an option in an enum is called a descriminator

### The Option Enum
- An enum that represents a value that may or may not be present
- The rust alternative to null

## Flow Control
- normal flow of a program is line by line top to bottom
- concepts include conditionals like if/else and match in adition to loops like for and while
- continues and breaks are also included under this concept
- the only new concept is continue which hops to the next looping in a loop
- loop is just an infinite while loop that has to be manually broken out of.

## Pattern matching
- compare a value to diferent patterns and then execute code based on diferent patterns
- all possible cases must be handled
- if let is a match that handles a non value
- if conditionals dont allow for type based pattern matching so you have to use matches!
- you can also use else if let but that is even more varbose than match
- in an if let/ match statements variables are destructured so Some(30) -> 30, shadowing
- | can be used to match multiple patterns and ..= can be used to match a range
- the @ operator can be used to extract a variable and test it at the same time in a match
- you can access and modify a mutable reference in a match clause

## Methods
- a function that is associated with a particular type of struct
- you put it into the struct using an impl block
- you can access the values of the struct by an &self keyword
- a method that does not take self as its first parameter is called an associated function
- associated functions are often used as constructors for structs
- one struct can have many impl blocks

## Generics
- placeholders for concrete types
- enables wrighting more reusable and flexible code
- avoid duplicating code for diferent types
- zero cost abstraction, rust compiler will fill it out to concrete types at compile time
- a const generic is the ability to provide a constant value to a type f.ex if you were to make an array with a certain length a struct then you could provide a const generic to set the length.

## Traits
- set of methods that can be implemented for multiple types in order to provide common functionality between them
- traits consist of method signatures that then have to be implemented by the target type
- similar to classes in other languages
- can be parameters in functions
- can also be requirements for input types in function f.ex when a function that adds needs inputs that implement the Add trait
- where clauses makes it easyer to implement many trait constraints on functions
- you may also specify a trait implementation for the return type of a generic function

### Associated types
- Allows types to be associated with traits
- concrete type needs to be specified
- Is basically a placeholder type that the trait method can use in the signature
- basically a more flexible generic type 
- allows traits to have diferent associated types for diferent implementing types

### Derivable traits
- fast traits like debug, clone and copy that can be automatically implemented by the compiler

### Traiut Objects
- using impl trait does not work when returning multiple types
- this is becouse diferent types use diferent amounts of memory
- a trait object is a pointer that implements the trait when the type can only be known on run
- this works becouse a pointer always has the same size even at compile time

#### Static dispatch
- resolves method call at compile time, when types are known and clear
- compiler generates function code for each concrete type that implements trait
- calls each function based on concrete type
- this way is faster and more efficient that dynamic dispatch but is less flexible

#### Dynamic dispatch &dyn <- probably
- Specific methods to be called are determined at runtime
- uses &dyn or Box<dyn> to create pointers to trait object
- compiler creates vtable in heap memory to store the pointers

### Object safe traits
- you can only make object safe traits into trait objects
- traits are object safe when there are no generic type parameters
- object safe traits also cannot have return type Self

## & vs Box
- box transferrs data to heap and owns it but & just points to a value that is already there
- box can be passed across scopes while references has a limited lifetime
- box can be pattern matched and cloned but references cannot

## Capacity
- strings stored on the heap has a capacity of 8 with the power of 2*n 
- if the string is mutated so that it needs more memory rust will double what is reserved for it
- therefor it is good if you know how it behaves to manually assign the memory allocated to it
- this can be done with the .with_capacity(num) function

## Vectors
- Like arrays but dynamically sized
- allocated on the heap as contigous block of memory, like string of any type
- all elements must be of same type
- special macro vec![]
- String = vec!<u8>

## Hashmap
- data structure to store key value pairs
- Allocated on the heap, and dynamically sized
- efficient lokup, insertion and deletion of data
- each key is hashed to a unique index in underlying array
- capacities allocated to hashmaps is not exact, you can manually allocate and shringk but it wont be exact
- for values implementing the copy trait like i32 the value will be copied into the hashmap
- for owned values like String ownership will be transferred to the hashmap

### Requirements of hashmap key
- any type that implements the Eq and hash traits can be keys in a hashmap
- You can include bool, ints and strings by default but not floats
- all collection classes will implement hash if their contained T does it
- so Vec!<i32> would but Vec!<f32> would not.

## Type coercion "as" keyword
- If a value out of range is typecast to something that cannot handle it the rust compiler will subtract the largest value allowed +1 *n ntill it fits the type.
- so if 1000 were to be typecast "as" u8 256 would be subtracted untill it fits.
- if negative is cast to something that cannot be negative it will turn around and be the largest possible. -1 as u8 will be 255.
- floats typecast to ints will have their fractions removed.

## From/Into conversions
- from and into functions are a part of the standard library and can be used to simply convert types
- if you implement from you automatically also get into
- by defaut Strings can be converted to &str, bools can be ints, chars can be ints
- from can be implemented without import
- from can even be used to change the types of errors
- try_from and try_into can be used when type conversion is not gauranteed

## Error handling

### Panic!
- prints out an error message, unwinds the stack and exits the programm
- in multithreded programms it will only exit its own thread
- simplest way of error handling

#### Common panic cases
* non equal assertion in assert_eq!
* trying to access an array out of bounds 
* unwrap a non value
* trying to make an intiger to large
* dividing by zero

### Result
- an enum that represents the outcome of an operation that could potentially fail
- two possible variants, OK and Err
- the unwrap() function takes a result and panicks in the case of error or returning whats in ok
- ? operator is basically the same as unwrap but returns an error message instad of panicing

## type alias
- if you write type on global scope you can rename default types
- dont confuse with associated types in traits
- are useful when your types are repeatedly nested and complex

## More about Cargo
- official package manager and building tool
- a crate is a compilation unit of rust source code, cargo manages crates
- cates.io is the repository for rust packages
- binary crates are basically programms that you can run
- library crates are what they sound like
- the crate root is the source file that is run, src/main.rs, src/lib.rs

## Modules
- a way of organizing code by grouping related items
- can be imported using namespaces
- also controles privacy of functions structs, enums and so on
- create a lib by 'cargo new package_name_you_find_in_Cargo.toml_file --lib' from src directory
- a package can contain as many binary crates as you want but only one library crate
- usually you put package binary crates in a bin folder in src
- you create binary crates by just adding mor main files
- to separate a module to another file just declare the module name in the lib or main and then make a separate rs file with the name of the module declared and the contennts of the module, you dont have to write mod {} again as it is already declared
- alternatively you can create a folder with the name of the module containing a mod.rs file that the becomes the source of the sub crate.
- sub crates can contain further sub crates
