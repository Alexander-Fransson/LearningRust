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
Statements cannot be returned by functions in rust so no variable definitions or 




