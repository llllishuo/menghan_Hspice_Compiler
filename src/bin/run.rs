use crate::common::offic::excel;
use clap::Parser;
use jni::objects::*;
use jni::JNIEnv;
use std::path::Path;
use HspiceCompiler::hspice::spice;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[clap(help = "Hspice file name")]
    pub file_name: String,
    #[clap(short, long)]
    pub output_path: String,
}

fn main() {
    let args = Args::parse();

    let spice_file = Path::new(&args.file_name);

    let output_path = Path::new(&args.output_path);

    loading(args);

    spice_file.try_exists().expect("Can't access hspice file");
    let mut reader = spice::Reader::new();
    let data_iter = reader.read(spice_file);
    reader.analysis_iter(data_iter);

    excel::write_to_excel(data_iter, output_path);
}
fn loading(args: Args) {
    println!(
        "
🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈
🎈                          🎈
🎈                          🎈
🎈      HspiceCompiler      🎈
🎈                          🎈
🎈                          🎈
🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈🎈

🚩
    🚗: File Path 💨 {:?}

    💌: Output Path 🔦 {:?}

🚩

⛅ The compiler is ready ⛅

🐔尼钛镁: 💬


        ",
        args.file_name, args.output_path,
    );
}
