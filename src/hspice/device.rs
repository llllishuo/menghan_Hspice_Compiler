use crate::common::split::split_equal_sign;
use crate::hspice::source::*;
#[derive(Debug)]
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

#[derive(Debug)]
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
            Some('c') | Some('C') => C::from(bits),
            _ => {
                panic!("This is an illegal device! -> {:?}", bits[0].chars());
            }
            x => {
                panic!("This is an illegal device! -> {:?} {:?}", x, bits[0]);
            }
        }
    }
}

// 电阻
#[derive(Debug)]
pub struct R {
    name: String,
    value: u32,
    // 温度系数
    TC: Vec<u32>,
    // 交流分析阻值
    AC: f64,
}
impl R {
    pub fn new() -> Self {
        R {
            name: String::new(),
            value: 32,
            TC: vec![1, 2],
            AC: 1e10,
        }
    }
}

// 电容
#[derive(Debug)]
pub struct C {
    name: String,
    value: String,
    // 数量
    M: u32,
    // 电容两端节点的函数
    CTYPE: String,
    // 电容两端初始电压
    IC: String,
    // 多项式函数
    func: String,
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
                name: String::new(),
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
#[derive(Debug)]
pub struct L {
    name: String,
    value: String,
    // 温度系数
    TC: Vec<u32>,
    // 多项式函数
    func: String,
}

// 互感
#[derive(Debug)]
pub struct K {
    name: String,
    // 耦合系数
    value: String,
}
// 二极管
#[derive(Debug)]
pub struct D {
    name: String,
    // 模型
    model: String,
}
// 三极管
#[derive(Debug)]
pub struct Q {
    name: String,
    // 模型
    model: String,
}
// MOS管
#[derive(Debug)]
pub struct MOS {
    name: String,
    // 模型
    model: String,

    long: String,
    wide: String,
}
impl MOS {
    pub fn new(name: String, model: String, long: String, wide: String) -> Self {
        Self {
            name,
            model,
            long,
            wide,
        }
    }
    pub fn from(bits: Vec<&str>) -> Device {
        assert!(
            bits.len() >= 8,
            "MOS statement syntax error, please modify!!!!"
        );

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

#[derive(Debug)]
pub struct Sub {
    name: String,
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
        Device {
            device_type: DeviceType::Sub(Sub { name: name }),
            node: nodes,
        }
    }
}
