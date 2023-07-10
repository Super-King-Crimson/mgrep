use std::{fs, error::Error};

//Wait, all parse_config does is make a Config
//Let's just impl it!
#[allow(unused)]
pub struct Config {
    pub query: String,
    pub path_to_file: String,
}

//ok refactor time!
#[allow(unused)]
pub fn parse_config(args: &[String]) -> /* (&str, &str) */ Config {
    //wait a second...
    // let query = &args[1];
    // let path_to_file = &args[2];

    //We have to clone, otherwise we're taking ownership through a borrow
    //Cloning has a runtime cost, but it's very simple 
    let query = args[1].clone();
    let path_to_file = args[2].clone();

    Config { query, path_to_file }

    // (query, path_to_file)
    //Why are we returning a tuple, then immediately breaking the tuple up?
    //Also, the configs are a bunch of settings, so probably we should group them as one struct
}



impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //And let's add an error message!
        if args.len() < 3 {
            return Err("Not enough arguments: Did you forget to include the path to file?");
        }

        let query = args[1].clone();
        let path_to_file = args[2].clone();

        Ok(Config { query, path_to_file })
    }
}

//Okay now let's make a run function to handle all the stuff main shouldn't do
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.path_to_file)?;
    
    println!("Contents of file: \n------------------------\n{contents}\n------------------------");

    Ok(())
}

//Alright now that everything's in lib let's just use it for now