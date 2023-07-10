use std::{fs, error::Error, env};

//Wait, all parse_config does is make a Config
//Let's just impl it!
#[allow(unused)]
pub struct Config {
    pub query: String,
    pub path_to_file: String,
    pub ignore_case: bool,
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

    
    //Alrighy t 


    Config { query, path_to_file, ignore_case: false }

    // (query, path_to_file)
    //Why are we returning a tuple, then immediately breaking the tuple up?
    //Also, the configs are a bunch of settings, so probably we should group them as one struct
}



impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //And let's add an error message!
        if args.len() < 3 {
            return Err("Not enough arguments: Did you forget to include the path to file and/or string to look for?");
        }

        let query = args[1].clone();
        let path_to_file = args[2].clone();

        //We don't care abt the value of IGNORE_CASE, just that it got a value (not unset/false)
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, path_to_file, ignore_case })
    }
}

//Okay now let's make a run function to handle all the stuff main shouldn't do
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.path_to_file)?;
    
    // println!("Contents of file: \n------------------------\n{contents}\n------------------------");

    let matches: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in matches {
        println!("{line}");
    }

    Ok(())
}

//By the way, this is good, but not the best: it doesn't take full advantage of iterators
//Come back after Chapter 13 to try and refactor this!
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            matching_lines.push(line);
        }
    }

    matching_lines
}   

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //to_lowercase returns a new String
    let query = query.to_lowercase();
    
    let mut matching_lines = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matching_lines.push(line);
        }
    }

    matching_lines
}   

//Alright now that everything's in lib let's just use it from now on
//Ok tests time!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let query = "s the";

        //we put a backslash to make rust ignore this newline
        let contents = "\
High was the sky,
deep was the forest,
unmoving were the people.";
        
        //let's make a barebones search function so this test will compile (it will fail)
        //ok now let's actually implement search so this runs correctly
        assert_eq!(
            vec!["High was the sky,", "deep was the forest,"],  
            search(query, contents)
        );
    }

    
//Ok we did it! CLI tool done!
//Let's make some environment variables, like case insensitive search

//we can set our env variable (in this case IGNORE_CASE) in Powershell by doing
// $Env:IGNORE_CASE=[anything]; [whatever command, like cargo run]

    #[test]
    fn search_case_insensitive_test() {
        let contents = "\
God I love RusT
RUST is so great
I hate TypeScript
It's not as cool as Bust";

        let query = "RUST";
        
        assert_eq!(vec!["God I love RusT", "RUST is so great"], search_case_insensitive(query, contents));
    }
}

//Finally, let's write to stderr (standard error) instead of using println, 
//for when we want successful program runs to write to a file but errors to display in the console
//We'll put our errors in output.txt

//We can redirect stdout (standard output) to a file with >
//cargo run > output.txt (note we don't have arguments, so this will error)

//Hey, remember how main handles all of our errors? That makes this really easy