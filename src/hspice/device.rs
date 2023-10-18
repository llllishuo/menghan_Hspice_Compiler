use std::boxed::Box;

pub struct Device<T: ?Sized> {
    // 类型
    dev_type: Box<T>,
    // 节点组
    node: Vec<String>,
}

pub enum Dev_type {
    Source(Source),
    R(R),
    C(C),
    L(L),
    K(K),
    D(D),
    Q(Q),
    MOS(MOS),
}

// 源 既 电压源 与 电流源
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

impl Device<Source> {
    pub fn new() -> Self {
        Self {
            dev_type: Box::new(Source::new()),
            node: Vec::new(),
        }
    }
}
// 电阻
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
impl Device<R> {
    fn new(&mut self) -> Self {
        Device {
            dev_type: Box::new(R::new()),
            node: Vec::new(),
        }
    }
    fn add(&mut self) {}
}

// 电容
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
pub struct L {
    name: String,
    value: String,
    // 温度系数
    TC: Vec<u32>,
    // 多项式函数
    func: String,
}

// 互感
pub struct K {
    name: String,
    // 耦合系数
    value: String,
}
// 二极管
pub struct D {
    name: String,
    // 模型
    model: String,
}
// 三极管
pub struct Q {
    name: String,
    // 模型
    model: String,
}
// MOS管
pub struct MOS {
    name: String,
    // 模型
    model: String,

    long: String,
    wide: String,
}
impl MOS {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            model: String::new(),
            long: String::new(),
            wide: String::new(),
        }
    }
    pub fn from(name: String, model: String, long: String, wide: String) -> Self {
        Self {
            name: name,
            model: model,
            long: long,
            wide: wide,
        }
    }
}
impl Device<MOS> {
    pub fn new() -> Self {
        Device {
            dev_type: Box::new(MOS::new()),
            node: Vec::new(),
        }
    }
    pub fn add(bits: Vec<&str>) -> Device<MOS> {
        if bits.len() < 8 {
            panic!("MOS statement syntax error, please modify!!!!");
        }
        let mut name = bits[0].to_string();
        let mut node = Vec::new();
        node.push(bits[1].to_string());
        node.push(bits[2].to_string());
        node.push(bits[3].to_string());
        node.push(bits[4].to_string());
        let model = bits[5].to_string();
        let long = bits[6].to_string();
        let wide = bits[7].to_string();
        println!(
            "<MOS>: {{name: {}, node: {:?}, model: {}, long: {}, wide: {}}}",
            name, node, model, long, wide
        );
        let m = MOS::from(name, model, long, wide);
        Device {
            dev_type: Box::new(m),
            node,
        }
    }
}
