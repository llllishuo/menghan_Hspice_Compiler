// 源 既 电压源 与 电流源
#[derive(Debug)]
pub struct Source {
    name: String,
    // 正极
    pe: String,
    // 负极
    ne: String,
    // 直流电压值
    DC: String,
    // 瞬态电压源
    tranfun: String,
}
impl Source {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            pe: String::new(),
            ne: String::new(),
            DC: String::new(),
            tranfun: String::new(),
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
}

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
}

#[derive(Debug)]
pub struct Device {
    // 类型
    device_type: DeviceType,
    // 节点组
    node: Vec<String>,
}

pub fn from_mos(bits: Vec<&str>) -> Device {
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
    let long = bits[6].to_string();
    let wide = bits[7].to_string();

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
