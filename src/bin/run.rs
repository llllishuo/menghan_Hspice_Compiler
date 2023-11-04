use clap::Parser;
use std::path::Path;

use HspiceCompiler::hspice::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[clap(help = "Hspice file name")]
    pub file_name: String,
    #[clap(short, long)]
    pub output_src: Option<String>,
    #[clap(long)]
    pub output_method: Option<String>,
    #[clap(long, default_value = "false")]
    pub only_sim: bool,
}

fn main() {
    let args = Args::parse();

    match args.only_sim {
        true => {
            println!("只执行仿真");
        }
        false => {
            println!("执行仿真及其其它");
        }
    }

    let spice_file = Path::new(&args.file_name);
    spice_file.try_exists().expect("Can't access hspice file");
    let mut reader = spice::Reader::new();
    let data_iter = reader.read(spice_file);
    reader.analysis_iter(data_iter);
}
