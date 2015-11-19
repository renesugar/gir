extern crate case;
extern crate docopt;
extern crate env_logger;
extern crate git2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate xml;
extern crate toml;
extern crate regex;

use std::error::Error;
use std::process;

use env::Env;
use library::Library;

//generated by build.rs
mod gir_version;

mod analysis;
mod chunk;
mod codegen;
mod config;
mod env;
mod file_saver;
mod git;
mod gobjects;
mod library;
mod nameutil;
mod parser;
mod regexlist;
mod traits;
mod version;
mod writer;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    if let Err(err) = do_main() {
        println!("{}", err);
        process::exit(1);
    }
}

fn do_main() -> Result<(), Box<Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "gir=warn");
    }
    try!(env_logger::init());

    let cfg = match config::Config::new() {
        Ok(cfg) => cfg,
        Err(config::Error::CommandLine(ref err)) if !err.fatal() => {
            println!("{}", err);
            return Ok(());
        }
        Err(err) => return Err(Box::new(err)),
    };

    let mut library = Library::new(&cfg.library_name);
    library.read_file(&cfg.girs_dir, &cfg.library_full_name());
    library.fill_in();

    let namespaces = analysis::namespaces::run(&library);

    let env = Env{
        library: library,
        config: cfg,
        namespaces: namespaces,
    };
    codegen::generate(&env);

    Ok(())
}
