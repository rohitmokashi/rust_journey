# Rust expedition

## Installing Rust
I am starting with Rust in WSL (Windows Subsystem for Linux).  
Command to install Rust in Debian WSL  
`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

To check if Rust is installed properly, run:
`rustup --version`  
The output would contain `rustup` and `rustc` version installed.

## Writing your first Rust program
Entry point of every Rust program is the main function.  

Make a file main.rs.  
```rust
fn main() {
    println!("Hello World!");
}
```
To run the code:  
`rustc main.rs && ./main`  

Output:  
`Hello World!`

#### Let's understand the above code
`fn` : keyword used to define a function followed by  
`main` : the name of the function followed by
`()` : paranthesis to pass arguments to a function (in this case we have not passed any) followed by  
`{ }` : inside curly brackets we define the function  
`println!` : it is a macro used to print anyting  
*macros are a bit different than functions which we will see further in the journey.*  
`"Hello World!"` : the string we want to print

**You have successfully executed your first Rust program.**

There is one more way of running and managing your rust project other than first compiling it using `rustc` and then running the executable created.
### Cargo

Cargo is Rust’s build system and package manager.  
Cargo handles a lot of tasks for you, such as building your code, downloading the dependencies of your code, and building those dependencies.

Cargo comes preinstalled with Rust.  
To check the version of cargo installed on your system, run:  
`cargo --version`  
Output:  
`cargo 1.77.2 (e52e36006 2024-03-26)`  

#### Creating a new cargo project
`cargo new hello_cargo`

This command creates a project named *hello_cargo*.  

Inside the project there is a **Cargo.toml** file containing the project information and all the dependencies we want to use will be added here only.  
*TOML stands for Tom’s Obvious, Minimal Language*

Apart from **Cargo.toml** there is also a **src** folder where rust expects us to keep all our source code, *which we will do*.  
Initially there is only a `main.rs` file in **src**.

To run your cargo project navigate to the folder where your **Cargo.toml** is present.  
Running a cargo project includes two steps:  
1. ***Building the project***  
run the following command to build  
`cargo build`  
this command compiles the code and creates a **target** folder, containing the executable of our code at *target/debug/hello_cargo* (in place of guessing game it would be the name you gave to your project).

2. ***Running the target***  
now we can run the executable file created after building the code.  
enter this command to run the executable:  
`./target/debug/hello_cargo`  

We can avoid the hassle of first building the code and then running if by using  
`cargo run`  
this command automatically builds the code and runs the executable created all in one command.

Apart from **cargo run** and **cargo build** there is one more command  
`cargo check`  
It only checks if the project is compling successfully without any errors but does not create any executable file.  
*It is faster than cargo build.*

---

## Basic Programming Concepts

### Variables

We use the `let` keyword to declare a new variable.  
- By default a variable is of immutable type.

```rust
let x = 5;
```
the above statement creates a immutable var `x` holding the integer value `5`.  

If you want to declare a **mutable** variable use the `mut` keyword  
```rust
let mut x = 5;
x = 6;
```
- The above statement creates a mutable variable `x` holding value `5`
- Then in the next line changing the value `x` to `6`

If you try to reassign a value to an immutable variable, it will throw an **error**  
```rust
let x = 1;
x = 2;
```
Compiler will throw the following error  
```
error[E0384]: cannot assign twice to immutable variable `x`
```

### Constants

- Like **immutable** variables, **constants** are values that are bound to a 
name and are not allowed to change.
- **Constants** may be set only to a constant expression, not the result of a value that could only be computed at runtime

For example:
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Scalar Data Types

Scalar data type is a type that holds a single value

#### Integer Types

- Integer is mainly of 6 sizes based on bit length [8, 16, 32, 64, 128] bits
- and **signed** and **unsigned**

| Length | Signed |          Range        | Unsigned|    Unsigned    |
|:------:|:------:|:---------------------:|:-------:|:--------------:|
| 8-bit  |   i8   |        -127 to 128    |    u8   |  0 to 255      |
| 16-bit |   i16  |    -2^15 -1 to 2^15   |    u16  | 0 to 2^16 -1   |
| 32-bit |   i32  |    -2^31 -1 to 2^31   |    u16  | 0 to 2^32 -1   |
| 64-bit |   i64  |    -2^63 -1 to 2^63   |    u16  | 0 to 2^64 -1   |
| 128-bit|   i128 |   -2^128 -1 to 2^128  |    u128 | 0 to 2^128 -1  |
|  arch  |   isize|2^isize-1 -1 to 2^isize|    usize| 0 to 2^usize -1|

***NOTE:*** *arch* means the size depends on the CPU architecture i.e., for a 32-bit system size is 32 bits and for a 64-bit system size is 64 bits.

Syntax to define a u32 integer:
```rust
let x: u32 = 3424;
```
The above code creates a **unsigned 32-bit integer** variable holding value **3424**.

#### Floating-Point Types

- Floats are of two sizes in Rust **32-bit** and **64-bit**
- By default a floating point is of 64-bit

```rust
let x = 2.0;
let y: f32 = 3.0;
```
In above code `x` is a **64-bit** float and `y` is a **32-bit** floating point number.


#### Numeric Operations

Rust supports many arithmetic operations for integer and floats.
1. *Addition*
2. *Subtraction*
3. *Multiplication*
4. *Division*
5. *Modulus*

```rust
// addition
let sum = 5 + 10;

// subtraction
let difference = 95.5 - 4.3;

// multiplication
let product = 4 * 30;

// division
let quotient = 56.7 / 32.2;
let truncated = -5 / 3; // Results in -1

// remainder
let remainder = 43 % 5;
```

#### The Boolean Type

A Boolean type in Rust has two possible values
- `true`
- `false`

```rust
let b: bool = true;
```

#### The Character Type

- char type is of 4-bytes in Rust
- Therefore, it can store more than just ASCII characters

```rust
let c: char = 'Z';
```
The above code create a immutable `char` variable `c` holding the value `'Z'`

---

### Compound Types

*Compound Types* can group multiple types into one data type.  
Rust has mainly two **Compound Data Types**
- Tuples
- Arrays

#### Tuples
- A Tuple is used to group two or more **values** and/or **data types** together
- Tuples have fixed lengh i.e., they cannot grow or shrink in size
- Tuples are indexed

How to declare a tuple
```rust
let mut t: (u32, f64, i8) = (432, 24.32, -4);
```
The above code binds the *mutable* variable `t` to a tuple holding a `u32`, `f64` and a `i8` **data types** together.

Elements of a **tuple** could be accessed using perios(`.`) operator
```rust
println!("{}", t.0);
```
The above code prints the **element** at **index** `0`  
The output is: `432`

#### Arrays
- Array is a collection of similar **data types**
- Arrays have a fixed length
- Arrays are indexed

How to declare an array
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```
The above code binds the **variable** to a *mutable* **array** holding `i32` data types and of fixed length `5`.

Elements of an **array** could be accessed using square brackets `[ ]`
```rust
println!("{}", a[1]);
```
The above code prints the **element** at **index** `1`  
The output is: `2`

---

### Functions

- Functions are pretty straight forward in Rust.
- Functions may take some input and may return some output as well.

For example let's make a function for adding two integers.
```rust
fn add(x: i32, y: i32) {
    let result: 32 = x + y;
    return result;
}
```
Let's break down the above code
- `fn` : **keyword** used to declare a *function* followed by
- ***add*** : name of the function
- `( )` : contain the parameters of the function, in this case two `i32` types namely `x` and `y`
- `{ }` : curly brackets contain the function body

In the `add` function we add `x` & `y` and store it in `result`  
Next we can return the `result` using the `return` statement.

***NOTE :***
- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resultant value.

---

### Comments

We use two forward slashes `//` to comment a line.  
There is no special concept of multi line comment in Rust, if you want to comment more than one line add `//` before every line.  

*Example*
```rust
// This is a comment
// This is another comment
```

---

## Controling the Flow of code

### Conditional Statements

- Conditional statements are used if you want to execute some piece of code only if a condition is *true*
-  We use `if` statement for this

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else {
        println!("number is not divisible by 4 or 3");
    }
}
```

- We first write `if` statement followed by condition we want to check in this case `number % 4`
- If this condition is `false` the code jumps to `else if` conditions
- and if all the conditions are `false` the `else` statement is **executed**

### Loops

There are three types of *loops* in Rust
1. infinite loop
2. while loop
3. for loop

#### Infinte Loop

We create a *infinite loop* by using the `loop` keyword followed by the loop body, the statements to be executed repeatedly.
```rust
fn main() {
    let mut i = 10;
    loop {
        if i==0 {
            break;
        }
        i -= 1;
    }
}
```
***NOTE :*** Make sure to add a `break` statement inside `loop` or the code will go into **infinte loop**

#### While Loop

`while` loop is used when we want to run a set of statements until a condition holds **true**

```rust
fn main() {
    let mut i = 0;

    while i < 5 {
        println!("{}", i);
        i += 1;
    }
}
```
The above code prints numbers from 0 to 4.  

First we write `while` statement followed by the *condition* `i<5` and loop body in *curly braces* `{ }`

#### For loop

For loop is used when you know exactly how many times you want to execute the loop body.

```rust
fn main() {
    for num in 1..=7 {
        println!("Number: {}", num);
    }
}
```
The above code prints numbers from **1** to **6** as *output*.  
Let's break down the code  
1. we write `for` statement
2. next the *variable* we want to bind the loop values to, in this case `num`
3. `in` statement and 
4. **values range** *starting value* followed by two period operators (`..`) and *exclusive endling value*

## Structs

Structs are simply collection of different *data types* and *named variables* and using it as one **data type**

**Structs** are very similar to tuples
```rust
let a: (bool, String, u8);
```
But the only difference is elements inside a **Struct** are named
```rust
struct User {
    active: bool,
    name: String,
    age: u8
}
```
elements of a `struct` can be accessed using structure name followed by variable
```rust
let u1 = User {
    active: true,
    name: String::from("Rohit Mokashi"),
    age: 20
}

println!("{}", u1.name);
```
The above code will print the name from **struct** `u1`, Hence **Rohit Mokashi**
