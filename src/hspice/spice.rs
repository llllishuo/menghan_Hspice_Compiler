use crate::common::split::*;
use crate::hspice::{
    analysis::Configuration,
    circuit::{Circuit, Sub_circuit},
    device::*,
    source::*,
};

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

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
    pub fn analysis_iter(&mut self, data_iter: Lines<BufReader<File>>) {
        let mut is_sub = false;
        let mut sub_circuit = Sub_circuit::new();
        // 处理读取到的每一行数据
        for data_line in data_iter {
            // println!("{:#?}", data_line);

            let line = data_line.unwrap();
            let item: Vec<&str> = line.split_whitespace().collect();

            // 如果该行为空和或者是注释就跳过
            if item.is_empty() {
                continue;
            }
            if item[0] == "*" || item[0].starts_with('*') {
                continue;
            }

            let mut bits = clean_comments_and_end_identifiers(item);

            // 消除语句的注释以及结束标识符

            // println!("{:#?}", bits);

            // 对数据进行解析
            match bits[0] {
                ".end" => {
                    println!("<end> Analysis over !!");
                }
                ".option" => {
                    self.cfg.option_analysis(bits);
                }
                ".lib" => {
                    self.cfg.lib_analysis(bits);
                }
                ".dc" => {
                    self.cfg.dc_analysis(bits);
                }
                ".print" => {
                    self.cfg.print_analysis(bits);
                }
                ".global" => {
                    self.cfg.global_analysis(bits);
                }
                ".subckt" => {
                    println!("sub_circuit: <start> ");
                    is_sub = true;
                    sub_circuit.add_name_And_Nodes(bits);
                }
                ".ends" => {
                    is_sub = false;
                    self.ckts.add_sub_circuits(sub_circuit);
                    sub_circuit = Sub_circuit::new();
                    println!("sub_circuit: <end>");
                }
                ".tran" => {
                    self.cfg.tran_analysis(bits);
                }
                ".ac" => {
                    self.cfg.ac_analysis(bits);
                }
                ".probe" => {
                    self.cfg.probe_analysis(bits);
                }
                // 器件的解析
                _ => {
                    if bits[0].starts_with(".lib") {
                        self.cfg.lib_analysis(bits);
                        continue;
                    }
                    let device = Device::get(bits);
                    match device.device_type {
                        DeviceType::Sub(i) => {
                            let name = i.name;

                            let mut devices = self.ckts.retrieve_sub_circuits(name);

                            for iter in devices {
                                self.ckts.add_device(iter);
                            }
                        }
                        _ => {
                            if is_sub {
                                sub_circuit.add_device(device);
                            } else {
                                self.ckts.add_device(device);
                            }
                        }
                    }
                }
            }
        }
        println!("ckts: \n{:?}", self.ckts);
        println!("cfg: \n{:?}", self.cfg);
    }
}
