use crate::hspice::circuit;
use crate::hspice::circuit::*;
use crate::hspice::device::*;
use crate::hspice::spice::*;
use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

pub fn write_to_excel(data: Reader, file_name: String) -> std::io::Result<()> {
    let device_path: String = String::from(file_name.clone() + "/device.csv");
    let circuit_path: String = String::from(file_name.clone() + "/circuit.csv");
    let cfg_path: String = String::from(file_name.clone() + "/cfg.csv");

    write_device_csv(data.ckts, device_path)?;

    Ok(())
}

fn write_device_csv(circuit: Circuit, device_path: String) -> std::io::Result<()> {
    let mut dervice_output = File::create(device_path).unwrap();
    dervice_output.write_all(b"some bytes").unwrap();
    Ok(())
}
