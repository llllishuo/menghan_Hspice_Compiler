use clap::Parser;
use jni::objects::*;
use jni::JNIEnv;
use std::path::Path;
use HspiceCompiler::common::offic::excel;
use HspiceCompiler::hspice::spice;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
    #[clap(help = "Hspice file name")]
    pub file_name: String,
    #[clap(help = "output path")]
    pub output_path: String,
}

fn main() {
    let args = Args::parse();

    let spice_file = Path::new(&args.file_name);
    let spice_file_name = {
        let path_split: Vec<&str> = args.file_name.rsplit("/").collect();
        let mut file = path_split[0];
        let spice_name: Vec<&str> = file.split(".").collect();
        spice_name[0]
    };

    let output_path = Path::new(&args.output_path);

    loading(args.clone());

    spice_file.try_exists().expect("Can't access hspice file");
    let mut reader = spice::Reader::new();
    let data_iter = reader.read(spice_file);
    reader.analysis_iter(data_iter);

    excel::write_to_excel(reader, output_path, spice_file_name);
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
