//First we need to make this accept CLI arguments
//We need std::env::args function, which returns an iterator of args passed to the function
use std::{env, process};
use mgrep::Config; //must preface with mgrep, lib exposes functionality

fn main() {
    //collect to turn it into a collection with all the args
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    //If your args don' Unicode values, you have to use std::env::args_os, which returns an OsString (check docs)
    
    //By the way you pass args like this:
    //--cargo run -- arg1 arg2... arg_n
    
    //Also the first arg is always the file path, this is useful apparently???

    // [[ PRE REFACTOR ]]
    // let query = &args[1]; //we have to borrow, Vecs and Strings are stored on the heap
    // let path_to_file = &args[2];

    // println!("Looking for {query} in {path_to_file}");

    // [[ REFACTOR 1 ]]
    // let configs = parse_config(&args);

    // [[ REFACTOR 2 ]]
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        //this exits program immediately and returns exit code (not 0, meaning not good)
        process::exit(1);
    });

    //[[ PRE REFACTOR ]]

    //fs stands for file system
    // let contents = fs::read_to_string(config.path_to_file).expect("File should exist and be readable");

    // println!("Contents of file: \n------------------------\n{}\n------------------------", contents);
    
    //[[ POST REFACTOR ]]
    //Use if let instead of unwrap because run isn't supposed to return anything important (just a unit)
    if let Err(e) = mgrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    
    //Right now main is responsible for too much and there's little error handling,
    //let's refactor now (refactoring is easier with less code after all)

    //Let's talk about the problems:
    /*
     
    1. main has more than one task: it accepts arguments and reads files. if main keeps having to do
    a bunch of separate tasks, it will be hard to change anything about main without breaking other tasks

    2. some variables are configuration variables (query and path_to_file, they influence what program does),
    and others are used to perform logic (contents). all config variables should be in a struct
    so their purposes are clear

    3. We use expect to read the file, but this could error in a lot of ways, we should make it more specific

    4. better error handling. if user doesn't enter enough args, they get index out of bounds, and we use expect
    we should put all error handling code in one place, it'll make it easier to handle and allow us to print 
    useful error messages to end user 
    
    To fix this, Rust has guidelines when main starts getting big

    Split program into lib.rs, move program logic to lib.rs
    if command line parsing starts getting complicated, move it to lib

    main should only:
    call command line parsing logic
    setting up configs
    calling a run function in lib.rs
    handling the error if run returns error
    
    */
}

//so a bunch of stuff got moved into lib.rs