use crate::hspice::device::*;

#[derive(Debug, Clone)]
pub struct Circuit {
    pub devices: Vec<Device>, // 器件组
    pub sub_circuits: Vec<Sub_circuit>,
}
impl Circuit {
    pub fn new() -> Self {
        Circuit {
            devices: Vec::new(),
            sub_circuits: Vec::new(),
        }
    }
    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
        //println!(
        //    "<update> Number of devices added to the current circuit: {}",
        //    self.devices.len()
        //);
    }
    // 打印
    pub fn trace(&mut self) {
        println!("devices: {:?}", self.devices);
        println!("sub_circuits: {:?}", self.sub_circuits);
    }

    pub fn add_sub_circuits(&mut self, sub: Sub_circuit) {
        self.sub_circuits.push(sub);
    }
    pub fn retrieve_sub_circuits(&mut self, name: String) -> Vec<Device> {
        let sub_name = String::new();
        let sub_nodes: Vec<String> = Vec::new();

        let sub_devices: Vec<Device> = Vec::new();

        let mut new_sub = Sub_circuit::new();
        let mut sub_circuits = self.sub_circuits.iter_mut();
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
}
#[derive(Debug, Clone)]
pub struct Sub_circuit {
    pub name: String,
    pub nodes: Vec<String>,
    pub devices: Vec<Device>,
}

impl Sub_circuit {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            nodes: Vec::new(),
            devices: Vec::new(),
        }
    }
    pub fn add_name_And_Nodes(&mut self, bits: Vec<&str>) {
        let mut name = String::new();
        let mut nodes: Vec<String> = Vec::new();
        name = bits[1].to_string();
        for i in 2..bits.len() {
            nodes.push(bits[i].to_string());
        }
        self.name = name;
        self.nodes = nodes;
    }
    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
        /*println!(
            "{}: <update> Number of devices added to the current circuit: {}",
            self.name,
            self.devices.len()
        );*/
    }
}
