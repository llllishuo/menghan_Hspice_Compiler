use crate::common::split::split_equal_sign;
use crate::hspice::source::*;
#[derive(Debug, Clone)]
pub enum DeviceType {
    Source(Source),
    R(R),
    C(C),
    L(L),
    K(K),
    D(D),
    Q(Q),
    MOS(MOS),
    Sub(Sub),
}

#[derive(Debug, Clone)]
pub struct Device {
    // 类型
    pub device_type: DeviceType,
    // 节点组
    pub node: Vec<String>,
}
impl Device {
    // 获取器件
    pub fn get(bits: Vec<&str>) -> Device {
        // 将每行第一项进行拆分如： m0 拆为 m，0
        // 根据第一个字母判断添加什么器件
        match bits[0].chars().next() {
            Some('m') | Some('M') => MOS::from(bits),
            // 添加电源
            Some('v') | Some('V') => Source::from(bits),
            Some('i') | Some('I') => Source::from(bits),
            Some('c') | Some('C') => C::from(bits),
            Some('x') | Some('X') => Sub::from(bits),
            Some('r') | Some('R') => R::from(bits),
            Some('l') | Some('L') => L::from(bits),
            Some('q') | Some('Q') => Q::from(bits),
            _ => {
                panic!("📛 <WARN>: This is an illegal device! -> {:#?}", bits);
            }
        }
    }
}

// 电阻
#[derive(Debug, Clone)]
pub struct R {
    pub name: String,
    pub value: String,
    // 温度系数
    pub TC: Vec<f32>,
    // 交流分析阻值
    pub AC: f64,
}
impl R {
    pub fn new() -> Self {
        R {
            name: String::new(),
            value: String::new(),
            TC: Vec::new(),
            AC: 0.0,
        }
    }
    pub fn from(bits: Vec<&str>) -> Device {
        let mut nodes: Vec<String> = Vec::new();
        nodes.push(bits[1].to_string());
        nodes.push(bits[2].to_string());
        let value: String = bits[3].to_string();
        Device {
            device_type: DeviceType::R(R {
                name: bits[0].to_string(),
                value,
                TC: vec![],
                AC: 0.0,
            }),
            node: nodes,
        }
    }
}

// 电容
#[derive(Debug, Clone)]
pub struct C {
    pub name: String,
    pub value: String,
    // 数量
    pub M: u32,
    // 电容两端节点的函数
    pub CTYPE: String,
    // 电容两端初始电压
    pub IC: String,
    // 多项式函数
    pub func: String,
}
impl C {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
            M: 0,
            CTYPE: String::new(),
            IC: String::new(),
            func: String::new(),
        }
    }
    pub fn from(bits: Vec<&str>) -> Device {
        let mut nodes: Vec<String> = Vec::new();
        let mut value: String = String::new();
        nodes = vec![bits[1].to_string(), bits[2].to_string()];
        value = split_equal_sign(bits[3]);
        Device {
            device_type: DeviceType::C(C {
                name: bits[0].to_string(),
                value: value,
                M: 0,
                CTYPE: String::new(),
                IC: String::new(),
                func: String::new(),
            }),
            node: nodes,
        }
    }
}
// 电感
#[derive(Debug, Clone)]
pub struct L {
    pub name: String,
    pub value: String,
    // 温度系数
    pub TC: Vec<f64>,
    // 多项式函数
    pub func: String,
}
impl L {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
            TC: Vec::new(),
            func: String::new(),
        }
    }
    pub fn from(bits: Vec<&str>) -> Device {
        let mut nodes = Vec::new();
        let name = bits[0].to_string();
        let mut value = String::new();
        let mut TC: Vec<f64> = Vec::new();
        let mut func = String::new();

        nodes.push(bits[1].to_string());
        nodes.push(bits[2].to_string());
        value = bits[3].to_string();
        Device {
            device_type: DeviceType::L(L {
                name: name,
                value: value,
                TC: TC,
                func: func,
            }),
            node: nodes,
        }
    }
}

// 互感
#[derive(Debug, Clone)]
pub struct K {
    pub name: String,
    // 耦合系数
    pub value: String,
}
// 二极管
#[derive(Debug, Clone)]
pub struct D {
    pub name: String,
    // 模型
    pub model: String,
}
// 三极管
#[derive(Debug, Clone)]
pub struct Q {
    pub name: String,
    // 模型
    pub model: String,
    pub value: String,
}
impl Q {
    pub fn from(bits: Vec<&str>) -> Device {
        let name = bits[0].to_string();
        let mut nodes: Vec<String> = Vec::new();
        nodes.push(bits[1].to_string());
        nodes.push(bits[2].to_string());
        nodes.push(bits[3].to_string());

        let model = bits[4].to_string();
        let value = split_equal_sign(bits[5]);

        Device {
            device_type: DeviceType::Q(Q {
                name: name,
                model,
                value,
            }),
            node: nodes,
        }
    }
}
// MOS管
#[derive(Debug, Clone)]
pub struct MOS {
    pub name: String,
    // 模型
    pub model: String,

    pub long: String,
    pub wide: String,
}
impl MOS {
    pub fn new(name: String, model: String, long: String, wide: String) -> Self {
        Self {
            name: String::new(),
            model: String::new(),
            long: String::new(),
            wide: String::new(),
        }
    }
    pub fn from(bits: Vec<&str>) -> Device {
        let name = bits[0].to_string();
        let node = vec![
            bits[1].to_string(),
            bits[2].to_string(),
            bits[3].to_string(),
            bits[4].to_string(),
        ];

        let model = bits[5].to_string();
        let wide = split_equal_sign(bits[6]);
        let long = split_equal_sign(bits[7]);

        Device {
            device_type: DeviceType::MOS(MOS {
                name,
                model,
                long,
                wide,
            }),
            node,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sub {
    pub name: String,
}
impl Sub {
    pub fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
    pub fn from(bits: Vec<&str>) -> Device {
        let mut nodes: Vec<String> = Vec::new();
        let mut name: String = String::new();
        let end = bits.len() - 1 as usize;
        name = bits[end].to_string();
        for i in 1..end {
            nodes.push(bits[i].to_string());
        }
        Device {
            device_type: DeviceType::Sub(Sub { name: name }),
            node: nodes,
        }
    }
}
