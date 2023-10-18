pub struct Cli {
    pub file_name: String,
    pub output_src: String,
    pub output_method: String,
    pub only_sim: bool,
}

impl Cli {
    pub fn new(file: &str, method: &str, src: &str, only: bool) -> Cli {
        Cli {
            file_name: file.to_string(),
            output_method: method.to_string(),
            output_src: src.to_string(),
            only_sim: only,
        }
    }
}

pub fn help() {
    println!(
        " 
*********************
***---------------***
***--User Manual--***
***---------------***
*********************

Basic format: cargo run <file-name> [<-o> <file-format> <output-src>]
            
-o \t Output a file in the format <file-format> to the path <output-src>
            

            "
    );
}
