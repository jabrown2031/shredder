use std::env;
use std::process;
use shredder::Parse;

/* writes 0's and random byes to a specified file
 * shredder <file> <rounds>
 */

fn main() {
    let parse = Parse::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = shredder::shred(parse) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
