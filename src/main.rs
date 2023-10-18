use std::env;
use std::path::Path;
use HspiceCompiler::cli::{self, help, Cli};
use HspiceCompiler::hspice::spice::*;
use HspiceCompiler::hspice::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);
    if args.len() < 2 {
        panic!(
            " 
-------------------------------------------------------------------
---! Pleace add hspice filename and output method               ---
---OR                                                           ---
---You can enter \" cargo run -- help \" to view the user manual---
-------------------------------------------------------------------
            "
        );
    }

    if args.len() == 2 && (args[1] == "help" || args[1] == "h") {
        help();
        return;
    }

    let cli = {
        if args.len() == 5 {
            Cli::new(&args[1], &args[3], &args[4], false)
        } else {
            Cli::new(&args[1], "", "", true)
        }
    };

    match cli.only_sim {
        true => {
            println!("只执行仿真");
        }
        false => {
            println!("执行仿真及其其它");
        }
    }

    let spice_file = Path::new(&args[1]);
    spice_file.try_exists().expect("Can't access hspice file");
    let mut reader = spice::Reader::new();
    let data_iter = reader.read(spice_file);
    reader.Analysis_iter(data_iter);
}
