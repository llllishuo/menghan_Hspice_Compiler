use crate::hspice::analysis::Configuration;
use crate::hspice::circuit::Circuit;
use crate::hspice::device::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

macro_rules! next_to_string {
    ($start:expr) => {
        $start.next().expect("value not exist!").to_string()
    };
}
pub struct Reader {
    // Circuit information
    /// `ckts[0]` is the toplevel
    ckts: Circuit,
    /// Options and analysis commands
    cfg: Configuration,
    /// Track which circuit in the `ckts` list that we're adding things too
    c: usize,
    /// Flag if problems were encountered during parsing
    there_are_errors: bool,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            ckts: Circuit::new(),
            cfg: Configuration::new(),
            c: 1,
            there_are_errors: false,
        }
    }
    pub fn read(&mut self, filename: &Path) -> Lines<BufReader<File>> {
        let input = File::open(filename).unwrap();
        let buf = BufReader::new(input);
        let mut lines_iter = buf.lines();
        println!("lines_iter: {:?}", lines_iter);

        let ckt_name = filename
            .file_stem()
            .expect("The file cannot be opened in this path!")
            .to_str()
            .expect("Illegal file naming in this path");
        println!("ckt_name: {}", ckt_name);

        lines_iter.next();

        lines_iter
    }
    pub fn Analysis_iter(&mut self, data_iter: Lines<BufReader<File>>) {
        // 处理读取到的每一行数据
        for data_line in data_iter {
            //println!("{:#?}", data_line);

            let line = data_line.unwrap();
            let item: Vec<&str> = line.split_whitespace().collect();

            /// 如果该行为空和或者是注释就跳过
            if item.is_empty() {
                continue;
            }
            if item[0] == "*" || item[0].starts_with("*") {
                continue;
            }

            /// 消除语句的注释以及结束标识符
            let mut bits: Vec<&str> = vec![];
            for bit in item {
                if bit == "$" || bit.starts_with("$") {
                    break;
                }
                if bit == ";" {
                    break;
                }

                bits.push(bit);
            }
            //println!("{:#?}", bits);

            // 对数据进行解析
            match bits[0] {
                ".option" => {
                    self.cfg.option_analysis(bits);
                }
                ".lib" => {
                    println!("This is a library file path: {}", bits[1]);
                }
                // 器件的解析
                _ => {
                    // 将每行第一项进行拆分如： m0 拆为 m，0
                    let mut start = bits[0].chars();
                    //println!("char: {:#?}", start);
                    /// 根据第一个字母判断添加什么器件
                    match start.next() {
                        // 添加MOS
                        Some('m') | Some('M') => {
                            let device = Device::<MOS>::add(bits);
                            self.ckts.set_device(device);
                            self.ckts.trace_device();
                        }
                        // 添加电源
                        Some('v') | Some('V') => {}
                        _ => {
                            panic!("This is an illegal device! -> {:?}", start);
                        }
                    }
                }
            }
        }
    }
}
