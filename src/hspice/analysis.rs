/// Program execution trace macro - prefix `<spice>`
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        println!(concat!("<spice> ", $fmt), $($($arg)*)?);
    };
}

// 基本控制选项
#[derive(Debug)]
pub struct Configuration {
    option: Option,
    dc: DC,
}
impl Configuration {
    pub fn new() -> Self {
        Self {
            option: Option::new(),
            dc: DC::new(),
        }
    }
    // option 写入
    pub fn option_analysis(&mut self, bit: Vec<&str>) {
        trace!("*INFO* Parsing control '{}'", bit[0]);
        // 根据参数值赋值
        match bit[1] {
            "post" => {
                self.option.post = NUM::get(bit[3]);
            }
            "search" => self.option.search = bit[1].to_string(),

            _ => {
                panic!("This is an unspecified parameter! -> {}", bit[1]);
            }
        }
        println!("{:?}", self.option);
    }
    // dc 写入
    pub fn dc_analysis(&mut self, bit: Vec<&str>) {
        trace!("*INFO* Parsing control '{}'", bit[0]);
        let mut vars: Vec<Var> = Vec::new();
        let var_name = bit[1];
        let mut scan = Scan_type::None;
        // 判断开头的 poi
        let mut poi_vec: Vec<u32> = Vec::new();
        if bit[2] == "poi" {
            let num = bit[3].parse::<u32>().unwrap();
            for i in 0..num {
                let value = bit[4 + i as usize].parse::<u32>().unwrap();
                //println!("poi_value: {}", value);
                poi_vec.push(value);
            }
            scan = Scan_type::POI(poi_vec);
        }

        let start = bit[2];
        let stop = bit[3];
        let step = bit[4];
        let mut sweep: Vec<String> = Vec::new();
        // 判断参数输入完是否有其它关联变量
        if bit[5] == "sweep" {
            sweep.push(bit[6].to_string());
        }
        // 判断末尾的 poi
        if bit[7] == "poi" {
            poi_vec = Vec::new();
            let num = bit[8].parse::<u32>().unwrap();
            for i in 0..num {
                let value = bit[9 + i as usize].parse::<u32>().unwrap();
                //println!("poi_value: {}", value);
                poi_vec.push(value);
            }
            scan = Scan_type::POI(poi_vec);
        }

        vars.push(Var {
            var: var_name.to_string(),
            scan: scan,
            start: start.to_string(),
            stop: stop.to_string(),
            step: step.to_string(),
            sweep: sweep,
        });
        self.dc = DC::from(vars);
        println!("{:?}", self.dc);
    }
}
/*
 * ype 设定扫描类型，可以是以下四种：
 DEC — decade variation
 OCT — octave variation
 LIN — linear variation
 POI — list of points
*/
#[derive(Debug)]
pub enum Scan_type {
    DEC,
    OCT,
    LIN(Vec<u32>),
    POI(Vec<u32>),
    None,
}
#[derive(Debug)]
struct DC {
    vars: Vec<Var>,
}
impl DC {
    pub fn new() -> Self {
        Self { vars: Vec::new() }
    }
    pub fn from(Vars: Vec<Var>) -> Self {
        Self { vars: Vars }
    }
}
#[derive(Debug)]
struct Var {
    var: String,
    scan: Scan_type,
    start: String,
    stop: String,
    // 步长
    step: String,
    sweep: Vec<String>,
}
impl Var {
    pub fn new() -> Self {
        Self {
            var: String::new(),
            scan: Scan_type::None,
            start: String::new(),
            stop: String::new(),
            step: String::new(),
            sweep: Vec::new(),
        }
    }
    pub fn from(
        var: String,
        scan: Scan_type,
        start: String,
        stop: String,
        step: String,
        sweep: Vec<String>,
    ) -> Self {
        Self {
            var: var,
            scan,
            start,
            stop,
            step,
            sweep,
        }
    }
}
#[derive(Debug)]
pub enum PARHIER {
    LOCAL,
    GLOBAL,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Option {
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

impl Option {
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
}
