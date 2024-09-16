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