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
    pub output_src: Option<String>,
    #[clap(long)]
    pub output_method: Option<String>,
    #[clap(long, default_value = "false")]
    pub only_sim: bool,
}

fn main() {
    let args = Args::parse();

    let spice_file = Path::new(&args.file_name);

    loading(spice_file);

    spice_file.try_exists().expect("Can't access hspice file");
    let mut reader = spice::Reader::new();
    let data_iter = reader.read(spice_file);
    reader.analysis_iter(data_iter);
}
fn loading(spice_file: &Path) {
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

🚩

⛅ The compiler is ready ⛅

🐔尼钛镁: 💬


        ",
        spice_file
    );
}
