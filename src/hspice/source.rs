use crate::common::split::*;
use crate::hspice::device::*;
#[derive(Debug, Clone)]
pub enum Source_type {
    DC(DC),
    AC(AC),
    TRANSIENT(Transient_tyoe),
}
// 交流源
#[derive(Debug, Clone)]
pub struct AC {}
// 直流源
#[derive(Debug, Clone)]
pub struct DC {
    value: String,
}
impl DC {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }
    pub fn from(name: String, pe: String, ne: String, bits: Vec<&str>) -> Device {
        let DC = Self::set(split_equal_sign(bits[3]));
        Device {
            device_type: DeviceType::Source(Source {
                name: name,
                pe: pe,
                ne: ne,
                source_type: DC,
            }),
            node: vec![bits[1].to_string(), bits[2].to_string()],
        }
    }
    pub fn set(value: String) -> Source_type {
        Source_type::DC(DC { value: value })
    }
}
// 瞬态源
#[derive(Debug, Clone)]
pub enum Transient_tyoe {
    PU(PU),
    SIN(SIN),
    EXP(EXP),
    PWL(PWL),
    SFFM(SFFM),
    AM(AM),
}
// Sinusoidal (SIN function) 正弦源
#[derive(Debug, Clone)]
pub struct SIN {}
// Exponential (EXP function) 指数源
#[derive(Debug, Clone)]
pub struct EXP {}

// Piecewise linear (PWL function) 分段线性源
#[derive(Debug, Clone)]
pub struct PWL {}
// Single-frequency FM (SFFM function) 调频源
#[derive(Debug, Clone)]
pub struct SFFM {}
// Single-frequency AM (AM function) 调幅源
#[derive(Debug, Clone)]
pub struct AM {}
// Pulse (PULSE function) 脉冲源
#[derive(Debug, Clone)]
pub struct PU {
    Low_voltage: String,
    High_voltage: String,
    Delay: String,
    Rise_time: String,
    Descending_time: String,
    Pulse_width: String,
    cycle: String,
}
impl PU {
    pub fn new() -> Self {
        Self {
            Low_voltage: String::new(),
            High_voltage: String::new(),
            Delay: String::new(),
            Rise_time: String::new(),
            Descending_time: String::new(),
            Pulse_width: String::new(),
            cycle: String::new(),
        }
    }
    pub fn from(name: String, pe: String, ne: String, bits: Vec<&str>) -> Device {
        let mut Low_voltage = bits[4].to_string();
        let mut High_voltage = bits[5].to_string();
        let mut Delay = bits[6].to_string();
        let mut Rise_time = bits[7].to_string();
        let mut Descending_time = bits[8].to_string();
        let mut Pulse_width = bits[9].to_string();
        let mut cycle = bits[10].to_string();
        let PU = Self::set(
            Low_voltage,
            High_voltage,
            Delay,
            Rise_time,
            Descending_time,
            Pulse_width,
            cycle,
        );
        Device {
            device_type: DeviceType::Source(Source {
                name: name,
                pe: pe,
                ne: ne,
                source_type: PU,
            }),
            node: vec![bits[1].to_string(), bits[2].to_string()],
        }
    }
    pub fn set(
        Low_voltage: String,
        High_voltage: String,
        Delay: String,
        Rise_time: String,
        Descending_time: String,
        Pulse_width: String,
        cycle: String,
    ) -> Source_type {
        Source_type::TRANSIENT(Transient_tyoe::PU(PU {
            Low_voltage,
            High_voltage,
            Delay,
            Rise_time,
            Descending_time,
            Pulse_width,
            cycle,
        }))
    }
}
// 源 既 电压源 与 电流源
#[derive(Debug, Clone)]
pub struct Source {
    name: String,
    // 正极
    pe: String,
    // 负极
    ne: String,
    // 电压类型
    source_type: Source_type,
}
impl Source {
    pub fn new(name: String, pe: String, ne: String, DC: String, tranfun: String) -> Self {
        Self {
            name: name,
            pe,
            ne,
            source_type: Source_type::DC(DC::new()),
        }
    }
    pub fn from(bits: Vec<&str>) -> Device {
        let mut name = bits[0].to_string();
        let mut pe = bits[1].to_string();
        let mut ne = bits[2].to_string();
        if bits[3].len() < 2 {
            return DC::from(name, pe, ne, bits);
        }
        // 拆分以判断前两个字符
        let mut first_two = bits[3].to_string();
        match &first_two[0..2] {
            "DC" | "dc" => DC::from(name, pe, ne, bits),
            "PU" | "pu" => PU::from(name, pe, ne, bits),
            _ => {
                panic!("Unknown power type： {}", bits[3]);
            }
        }
    }
}
