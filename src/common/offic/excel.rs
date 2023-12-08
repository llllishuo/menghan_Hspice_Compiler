use crate::hspice::circuit::Sub_circuit;
use crate::hspice::device::DeviceType;
use crate::hspice::{circuit, device, source, spice};
use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

pub fn write_to_excel(
    data: spice::Reader,
    file_name: &Path,
    spice_name: &str,
) -> std::io::Result<()> {
    let Some(mut output_path_str) = file_name.to_str() else {
        todo!()
    };
    let mut output_path = String::from(output_path_str);
    let device_path: String = String::from(output_path.clone() + "/" + spice_name + "_device.csv");
    let circuit_path: String =
        String::from(output_path.clone() + "/" + spice_name + "_circuit.csv");
    let cfg_path: String = String::from(output_path.clone() + "/" + spice_name + "_cfg.csv");

    write_device_csv(data.ckts, device_path.clone())?;

    println!(
        "\n\n🔹🔹🔹🔹🔹🔹🔹🔹🔹\n\n
        (～￣▽￣)～\n\nwrite to :
        \t{}\t⭕
        \n\n🔹🔹🔹🔹🔹🔹🔹🔹🔹\n\n",
        device_path.clone()
    );

    Ok(())
}

fn write_device_csv(circuit: circuit::Circuit, device_path: String) -> std::io::Result<()> {
    let mut dervice_output = File::create(device_path).unwrap();
    dervice_output
        .write_all(b"name,type,nodes,value_type,value,subckt \n")
        .unwrap();
    let devices = circuit.devices;
    let mut sub_circuit = circuit.sub_circuits;
    for device in devices {
        match device.device_type {
            device::DeviceType::Sub(sub) => {
                let lines: Vec<String> = lines_sub_by_name(sub.name, sub_circuit.clone());
                for line in lines {
                    dervice_output.write_all(line.as_bytes());
                }
            }
            _ => {
                let line = get_line(device.clone(), String::new());
                dervice_output.write_all(line.as_bytes());
            }
        }
    }
    Ok(())
}
fn get_line(device: device::Device, sub_name: String) -> String {
    match device.device_type {
        device::DeviceType::MOS(i) => line_MOS(i, device.node, sub_name),
        device::DeviceType::R(i) => line_R(i, device.node, sub_name),
        device::DeviceType::Source(i) => line_Source(i, device.node, sub_name),

        _ => {
            panic!("Content not written: {:?}", device);
        }
    }
}
fn line_R(r: device::R, node: Vec<String>, sub_name: String) -> String {
    let mut nodes = String::new();
    for node in node {
        let node_str = format!("{}/", node);
        nodes.push_str(&node_str);
    }
    format!(
        "{},{},{},{}/{},{}/{},{}\n",
        r.name,
        "R".to_string(),
        nodes,
        "R",
        "AC",
        r.value,
        r.AC,
        sub_name
    )
}
fn line_Source(source: source::Source, node: Vec<String>, sub_name: String) -> String {
    let mut nodes = String::new();
    for node in node {
        let node_str = format!("{}/", node);
        nodes.push_str(&node_str);
    }
    let Some(source_type) = source.name.as_str().get(0..1).or(Some("")) else {
        todo!()
    };
    let (value_type, value) = match source.source_type {
        source::Source_type::DC(i) => ("DC", i.value),
        _ => {
            panic!("unknown source: {:?}", source);
        }
    };

    format!(
        "{},{},{},{},{},{}\n",
        source.name,
        source_type.to_string(),
        nodes,
        value_type,
        value,
        sub_name
    )
}
fn line_MOS(mos: device::MOS, node: Vec<String>, sub_name: String) -> String {
    let mut nodes = String::new();
    for node in node {
        let node_str = format!("{}/", node);
        nodes.push_str(&node_str);
    }
    format!(
        "{},{},{},{}/{},{}/{},{}\n",
        mos.name,
        "MOS".to_string(),
        nodes,
        "wide",
        "long",
        mos.wide,
        mos.long,
        sub_name
    )
}
fn lines_sub_by_name(name: String, mut sub: Vec<Sub_circuit>) -> Vec<String> {
    let sub_devices = retrieve_sub_circuits(name.clone(), sub);
    let mut vec: Vec<String> = Vec::new();
    for sub in sub_devices {
        let line = get_line(sub, name.clone());
        vec.push(line);
    }
    vec
}
fn retrieve_sub_circuits(name: String, mut sub: Vec<Sub_circuit>) -> Vec<device::Device> {
    let sub_name = String::new();
    let sub_nodes: Vec<String> = Vec::new();

    let sub_devices: Vec<device::Device> = Vec::new();

    let mut new_sub = Sub_circuit::new();
    let mut sub_circuits = sub.iter_mut();
    while let Some(sub_iter) = sub_circuits.next() {
        if sub_iter.name == name {
            new_sub = sub_iter.clone();
            break;
        }
        /*println!(
            "name: {}, nodes: {:?}, devices: {:?}",
            sub_iter.name, sub_iter.nodes, sub_iter.devices,
        );*/
        continue;
    }
    new_sub.devices
}
