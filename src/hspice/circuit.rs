use crate::hspice::device::*;
use std::any::Any;
macro_rules! trace {
    ($dev:expr) => {};
}
pub struct Circuit {
    // 器件组
    // 使用Box<>的形式存放智能指针
    devices: Vec<Box<dyn Any + 'static>>,
}
impl Circuit {
    // 初始化
    pub fn new() -> Self {
        Circuit {
            devices: Vec::new(),
        }
    }
    // 添加器件
    // device: 为泛型器件
    pub fn set_device<T>(&mut self, device: Device<T>)
    where
        T: 'static + Any,
    {
        self.devices.push(Box::new(device));
        println!(
            "<update>Number of devices added to the current circuit: {}",
            self.devices.len()
        );
    }
    // 打印每个添加的器件
    pub fn trace_device(&mut self) {
        for item in &mut self.devices {
            if let Some(i) = item.downcast_mut::<Device<MOS>>() {
                i.trace();
            }
        }
    }
}
