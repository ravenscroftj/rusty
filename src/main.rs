/**
 * Simple, configurable, extensible rust IRC bot
 *
 *
 */
extern crate irsc;
extern crate toml;
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
extern crate serialize;

use std::path::Path;
use std::io::fs::PathExtensions;
use std::os::args;

use docopt::Docopt;
use serialize::Decodable;

mod rustybot;

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: rusty <configfile>
           ";

//arg parser struct
#[deriving(RustcDecodable, Show)]
struct Args {
    arg_configfile: String,
}
           
fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then( |a| a.decode() )
        .unwrap_or_else(|e| e.exit());

    //try and load config file
    let configpath = Path::new(args.arg_configfile);

    if(!configpath.is_file()){
        println!("Config path {} does not exist.", configpath.display());
        return ()
    }

    rustybot::config::load_config(configpath);

    let mut rbot = rustybot::RustyBot::new();
}
