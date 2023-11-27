use crate::common::split::*;
use crate::hspice::{
    analysis::{Configuration, Lib},
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
    pub ckts: Circuit,
    pub cfg: Configuration,
    is_sub: bool,
    is_alter: bool,
    lib_size: (u32, u32),
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            ckts: Circuit::new(),
            cfg: Configuration::new(),
            is_sub: false,
            is_alter: false,
            lib_size: (0, 0),
        }
    }
    pub fn read(&mut self, filename: &Path) -> Lines<BufReader<File>> {
        let input = File::open(filename).unwrap();
        let buf = BufReader::new(input);
        let mut lines_iter = buf.lines();
        //println!("lines_iter: {:?}", lines_iter);

        let ckt_name = filename
            .file_stem()
            .expect("🛑 The file cannot be opened in this path!")
            .to_str()
            .expect("🛑 Illegal file naming in this path");
        //println!("ckt_name: {}", ckt_name);

        lines_iter.next();

        lines_iter
    }
    pub fn analysis_data_collation(
        &mut self,
        data_iter: Lines<BufReader<File>>,
    ) -> Vec<Vec<String>> {
        let mut new_data: Vec<Vec<String>> = Vec::new();
        let mut lib_size: (u32, u32) = (1, 0);
        for data_line in data_iter {
            // println!("{:#?}", data_line);

            let line = data_line.unwrap();
            let mut item: Vec<&str> = line.split_whitespace().collect();

            // 如果该行为空和或者是注释就跳过
            if item.is_empty() {
                continue;
            }
            if item[0] == "*" || item[0].starts_with('*') {
                continue;
            }
            if item[0] == ".alter" {
                lib_size.0 += 1;
            }
            item = if item[0].starts_with(".lib") {
                lib_size.1 += 1;
                let mut new_lib_bit: Vec<&str> = Vec::new();
                for i in item {
                    if i.contains("\'") {
                        let mut result: Vec<&str> = i.split("\'").collect();
                        for r in result {
                            if r.is_empty() {
                                continue;
                            }
                            new_lib_bit.push(r);
                        }
                    } else {
                        new_lib_bit.push(i);
                    }
                }
                new_lib_bit
            } else {
                item
            };

            item = clean_comments_and_end_identifiers(item);

            // 消除语句的注释以及结束标识符

            let bits: Vec<String> = item.iter().map(|&s| s.to_owned()).collect();
            // println!("{:#?}", bits);
            new_data.push(bits);
        }
        self.lib_size = lib_size;

        //println!("💌 : {:?}", new_data);
        new_data
    }
    pub fn analysis_iter(&mut self, data_iter: Lines<BufReader<File>>) {
        let mut alter_name = "dufault".to_string();

        let mut libs: Vec<Lib> = Vec::new();
        let mut lib_len = 0;

        let mut sub_circuit = Sub_circuit::new();

        // 处理读取到的每一行数据

        let mut bit_iter = self.analysis_data_collation(data_iter);
        for bits in bit_iter {
            //由于技术原因重构成本过高只能降低性能
            let mut bits: Vec<&str> = bits.iter().map(|s| s.as_str()).collect();
            // 对数据进行解析
            match bits[0] {
                ".end" => println!("🆗 <end> Analysis over !!"),
                ".option" => self.cfg.option_analysis(bits),
                ".lib" => {
                    let lib = self.cfg.lib_extract_path_and_name(bits);
                    libs.push(lib);
                    lib_len += 1;
                    if lib_len == self.lib_size.1 / self.lib_size.0 {
                        lib_len = 0;
                        self.cfg.lib_analysis(alter_name.clone(), libs.clone())
                    }
                }
                ".dc" => self.cfg.dc_analysis(bits),
                ".print" => self.cfg.print_analysis(bits),
                ".global" => self.cfg.global_analysis(bits),
                ".subckt" => {
                    //println!("sub_circuit: <start> ");
                    self.is_sub = true;
                    sub_circuit.add_name_And_Nodes(bits);
                }
                ".ends" => {
                    self.is_sub = false;
                    self.ckts.add_sub_circuits(sub_circuit);
                    sub_circuit = Sub_circuit::new();
                    //println!("sub_circuit: <end>");
                }
                ".tran" => self.cfg.tran_analysis(bits),
                ".ac" => self.cfg.ac_analysis(bits),
                ".probe" => self.cfg.probe_analysis(bits),
                ".param" => self.cfg.param_analysis(bits),
                ".meas" => println!("🗨 <meas>: To be implemented! "),
                ".alter" => {
                    self.is_alter = true;
                    alter_name = bits[1].to_string();
                    continue;
                }
                // 器件的解析
                _ => {
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
                            if self.is_sub {
                                sub_circuit.add_device(device);
                            } else {
                                self.ckts.add_device(device);
                            }
                        }
                    }
                }
            }
        }
        println!("🔵 ckts: \n{:?}", self.ckts);
        println!("🟣 cfg: \n{:?}", self.cfg);
    }
}
