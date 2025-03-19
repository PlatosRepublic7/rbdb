use std::env;
use std::process;
use std::collections::HashMap;

use rbdb::rbdb_run;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args {
        println!("{arg}");
    }

    // Here we create the main storage for the application
    // NOTE: Later, this will be selectable from data stored on-disk. So for now it is best placed here
    let mut store: HashMap<String, String> = HashMap::new();

    println!("Database has started...");
    if let Err(e) = rbdb_run(&mut store) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}


