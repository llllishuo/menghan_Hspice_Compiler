use crate::hspice::device::*;

#[derive(Debug)]
pub struct Circuit {
    devices: Vec<Device>, // 器件组
}
impl Circuit {
    pub fn new() -> Self {
        Circuit {
            devices: Vec::new(),
        }
    }
    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
        println!(
            "<update> Number of devices added to the current circuit: {}",
            self.devices.len()
        );
    }
    // 打印每个添加的器件
    pub fn trace(&mut self) {
        println!("{:?}", self.devices);
    }
}
