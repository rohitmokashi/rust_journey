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
