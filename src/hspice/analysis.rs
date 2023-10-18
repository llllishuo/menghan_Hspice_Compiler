/// Program execution trace macro - prefix `<spice>`
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        println!(concat!("<spice> ", $fmt), $($($arg)*)?);
    };
}

// 基本控制选项
pub struct Configuration {
    // 0 取消报告 1 允许报告 2 允许矩阵统计报告
    ACCT: NUM,
    // 简化仿真报告
    brief: bool,
    // 设置输出的列数
    CO: u32,
    //
    /* 设定输出数据格式，默认 ingold=0, 设置为 2 可以与 SPICE 工具兼
        容, ingold 的选项有：
        ingold=0 工程格式，指数被表示成单个字母：
        1G=1e9 1X=1e6 1K=1e3 1M=1e-3
        1U=1e-61N=1e-9 1P=1e-12 1F=1e-15
        ingold=1 固定与指数共用格式，数值为 0.1 到 999 之间时，直接
        表示。小于 0.1 或大于 999 表示为指数形式
        E RFIC 30 / 63
        ingold=2 纯指数格式，可与数据后处理工具兼容。
        注意，将.options measdgt 与 ingold 共同使用来控制.measure
        的输出数据格式
    */
    //
    ingold: NUM,
    // 产生器件数目及关键参数值的摘要
    list: String,
    // 列出跟每一个节点相连的所有器件
    node: String,
    // 不输出模型参数
    nomod: bool,
    // 设置库和包含文件的搜索路径
    search: String,
    // 允许保存图形界面的数据。post=1,保存为二进制格式。post=2,保存为 ASCII 格式。post=3,保存为新波形二进制格式。默认为 1
    post: NUM,
    // 限制输出数据为.print, .plot, .probe, graph 中指定的变量。默认情况下，Spice 输出所有的电压、电流数据，再加上输出命令中指定的数据。用 probe 可以显著减小输出文件大小
    probe: bool,

    /*
     * 设置参数优先级，应用于不同层级电路中的重名参数。选项有：
        local 较低层级的电路参数具有高优先级
        global 较高层级的电路参数具有高优先级
    */
    parhier: PARHIER,
}

pub enum PARHIER {
    LOCAL,
    GLOBAL,
}

pub enum NUM {
    ZERO,
    ONE,
    TWO,
    THREE,
}

impl NUM {
    pub fn get(num: &str) -> NUM {
        match num {
            "0" => NUM::ZERO,
            "1" => NUM::ONE,
            "2" => NUM::TWO,
            "3" => NUM::THREE,
            _ => {
                panic!("This is an unspecified number! -> num: {}", num);
            }
        }
    }
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            ACCT: NUM::ONE,
            brief: false,
            CO: 80,
            ingold: NUM::ZERO,
            list: String::new(),
            node: String::new(),
            nomod: true,
            search: String::new(),
            post: NUM::ONE,
            probe: false,
            parhier: PARHIER::LOCAL,
        }
    }

    pub fn option_analysis(&mut self, bit: Vec<&str>) {
        trace!("*INFO* Parsing control '{}'", bit[0]);
        match bit[1] {
            "post" => {
                self.post = NUM::get(bit[3]);
            }
            "search" => self.search = bit[1].to_string(),

            _ => {
                panic!("This is an unspecified parameter! -> {}", bit[1]);
            }
        }
    }
}
