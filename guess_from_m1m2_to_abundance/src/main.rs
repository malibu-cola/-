#[macro_use]
extern crate serde_derive;
extern crate csv;

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, BufRead, prelude::*};
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use regex::Regex;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ConnectData {
    paper   : Option<String>,   ye      : Option<f64>,
    model   : Option<String>,   m_ej    : Option<f64>,
    eos     : Option<String>,   v       : Option<f64>,
    m1      : Option<f64>,      s       : Option<f64>,
    m2      : Option<f64>,
}

impl ConnectData {
    fn read_csv() -> Vec<Self> {
        let path = Path::new("input/connect_data.csv");
        let display = path.display();

        let mut file = File::open(&path).expect("couldn't open connect_data.csv");

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {},
        }

        let mut reader = csv::Reader::from_reader(s.as_bytes());
        let mut ret = vec![];
        for data in reader.deserialize() {
            let mut connect_data: Self = data.expect("can't open with deserialize");
            if connect_data.m1 > connect_data.m2 {
                std::mem::swap(&mut connect_data.m1, &mut connect_data.m2);
            }
            ret.push(connect_data);
        }
        ret
    }

    fn least_square_plane(connect_datas: &Vec<Self>) -> Result<(f64, f64, f64), std::io::Error> {
        let siz = connect_datas.len();
        let mut x = vec![];
        let mut y = vec![];
        let mut z = vec![];

        for i in 0..siz {
            if let Some(m1) = connect_datas[i].m1 {
                if let Some(m2) = connect_datas[i].m2 {
                    if let Some(ye) = connect_datas[i].ye {
                        x.push(m1 + m2);
                        y.push(m1 / m2);
                        z.push(ye);
                    }
                }
            }
        }

        let n = x.len() as f64;
        let x_sum = x.iter().sum::<f64>();
        let y_sum = y.iter().sum::<f64>();
        let z_sum = z.iter().sum::<f64>();
        let x2_sum = x.iter().map(|x| x * x).sum::<f64>();
        let y2_sum = y.iter().map(|y| y * y).sum::<f64>();
        let xy_sum = x.iter().zip(y.iter()).map(|(x, y)| x * y).sum::<f64>();
        let xz_sum = x.iter().zip(z.iter()).map(|(x, z)| x * z).sum::<f64>();
        let yz_sum = y.iter().zip(z.iter()).map(|(y, z)| y * z).sum::<f64>();

        let mut mat = vec![vec![0.0; 3]; 3];
        mat[0][0] = n;      mat[0][1] = x_sum;  mat[0][2] = y_sum;
        mat[1][0] = x_sum;  mat[1][1] = x2_sum; mat[1][2] = xy_sum;
        mat[2][0] = y_sum;  mat[2][1] = xy_sum; mat[2][2] = y2_sum;
        
        let mut inv = vec![vec![0.0; 3]; 3];
        {   // 逆行列を求める
            let mut sweep = vec![vec![0.0; 3 * 2]; 3];
            for i in 0..3 {
                for j in 0..3 {
                    sweep[i][j] = mat[i][j];
                    sweep[i][j + 3] = if i == j {1.} else {0.};
                }
            }
    
            for k in 0..3 {
                let mut a = 1.0 / sweep[k][k];
                for j in 0..3 * 2 {
                    sweep[k][j] *= a;
                }
    
                for i in 0..3 {
                    if i == k {continue;}
                    a = -sweep[i][k];
                    for j in 0..3 * 2 {
                        sweep[i][j] += sweep[k][j] * a;
                    }
                }
            }
    
            for i in 0..3 {
                for j in 0..3 {
                    inv[i][j] = sweep[i][3 + j];
                }
            }
        }

        let a = inv[0][0] * z_sum + inv[0][1] * xz_sum + inv[0][2] * yz_sum;
        let b = inv[1][0] * z_sum + inv[1][1] * xz_sum + inv[1][2] * yz_sum;
        let c = inv[2][0] * z_sum + inv[2][1] * xz_sum + inv[2][2] * yz_sum;
        // println!("{} {} {}", a, b, c);
        // z = a + bx + cyについて
        // 最小二乗法でa,b,cを導出
        // z = Ye, x = M2perM1, y = Msum
        Ok((a, b, c))
    }
}

pub fn step1() -> Result<(f64, f64, f64), std::io::Error> {
    let connect_data = ConnectData::read_csv();

    Ok(ConnectData::least_square_plane(&connect_data).expect("couldn't calculate least_square_plane"))
}


struct YePaper {
    index       : usize,
    paper_name  : String,
    fig_num     : String,
    // fig_position: Option<String>,
    fig_position: String,
    condition   : String,
    yedistro    : YeDistro,
    yebar       : f64,
}

impl YePaper {
    fn new(
        index: usize,    paper_name: String,     fig_num : String,           fig_position: String,
        condition   : String,   yebar     : f64,        yedistro: YeDistro,
    ) -> Self {
        Self {index, paper_name, fig_num, fig_position, condition, yebar, yedistro}
    }

    fn init() -> Vec<Self> {
        // ./input/YePaper.csvを読み込み、構造体YePaperの配列として返す。
        let path = Path::new("./input/YePaper.csv");
        let display = path.display();
    
        let mut file = File::open(&path).expect("couldn't open YePaper.csv");
    
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(_) => {},
        };
    
        let mut ret = vec![];
        let mut reader = csv::Reader::from_reader(s.as_bytes());
        
        for result in reader.records() {
            let record = result.expect("couldn't unwrap reader");
            
            let index       :usize    = record[0].parse().unwrap();
            let paper_name  :String   = record[1].to_string(); 
            let fig_num     :String   = record[2].to_string();
            let fig_position:String   = record[3].to_string();// Option<String>にしたい
            let condition   :String   = record[4].to_string();
            let yedistro    :YeDistro = YeDistro::read_csv(index);
            let yebar       :f64     = yedistro.yebar();
                
            ret.push(YePaper::new(index, paper_name,fig_num,fig_position,condition, yebar, yedistro));
        }
        ret
    }

    fn search_yedistro_from_yebar(yepapers: &Vec<Self>, yebar: f64) -> usize {
        // 与えられたyebarに最も近いyebarを持つyedistroのindexを返す。
        let mut yebar_vs_index: Vec<(f64, usize)> = yepapers.iter().map(|x| (x.yebar, x.index )).collect();
        yebar_vs_index.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let siz = yebar_vs_index.len();
        if yebar <= yebar_vs_index[0].0 {
            return yebar_vs_index[0].1;
        } else if yebar_vs_index[siz - 1].0 <= yebar {
            return yebar_vs_index[siz - 1].1;
        } else {
            // 二分探索でindexを探す。
            let mut left = 0;
            let mut right = yebar_vs_index.len();

            while left + 1 < right {
                let mid = (left + right) / 2;
                if yebar_vs_index[mid].0 < yebar {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            // println!("yedistro_yebar = {}", yebar_vs_index[left].0);
            return yebar_vs_index[left].1;
        }
    }
}

struct YeDistro {
    ye      : Vec<f64>,
    mass    : Vec<f64>,
}

impl YeDistro {
    fn sample() -> Self {
        let ye = vec![0.0];
        let mass = vec![0.0];
        Self{ye, mass}
    }

    fn new(ye: Vec<f64>, mass: Vec<f64>) -> Self {
        Self{ye, mass}
    }

    fn read_csv(index: usize) -> Self {
        let input_path = format!("./input/YePapers/YePaper_{}.csv", index);
        let path = Path::new(&input_path);
        let display = path.display();

        let mut file = File::open(&path).expect("couldn't open YePapers");

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!(),
            Ok(_) => {},
        }

        let mut ye_ret  : Vec<f64> = vec![];
        let mut mass_ret: Vec<f64> = vec![];

        let mut reader = csv::Reader::from_reader(s.as_bytes());
        for result in reader.records() {
            let data = result.expect("");
            let ye: f64 = match data[0].parse() {
                Ok(num) => num,
                Err(_) => 0.0,
            };
            let mass: f64 = match data[1].parse(){
                Ok(num) => num,
                Err(_) => 0.0,
            };
            if !(0. <= ye && ye <= 10. && 0. <= mass && mass <= 100.) {
                continue;
            }
            ye_ret.push(ye);
            mass_ret.push(mass);
        }
        Self::new(ye_ret, mass_ret)
    }

    fn yebar(&self) -> f64 {
        self.ye.iter().zip(self.mass.iter()).map(|(&ye, &mass)| ye * mass).sum::<f64>() / self.mass.iter().sum::<f64>()
    }
}

fn step2() -> Result<Vec<YePaper>, std::io::Error> {
    let yepaper: Vec<YePaper> = YePaper::init();
    Ok(yepaper)
}

struct RjavaOutput {
    ye0: f64,
    decaytime: String,
    // tau  : f64,
    abundances: Vec<RjavaAbundance>
}

impl RjavaOutput {
    fn init(decaytime: &String) -> Vec<(f64, Self)> {
        let mut ret = vec![];
        let ye0s = ye0_init();
        for ye0 in &ye0s {
            let abundances = Self::read_txt(decaytime.clone(), ye0.to_string());
            ret.push((ye0.parse().unwrap(), Self{ye0: ye0.parse().unwrap(), decaytime:decaytime.clone(), abundances}));
        }
        ret
    }

    fn read_txt(decaytime: String, ye0: String) -> Vec<RjavaAbundance> {
        let input_path = format!("./input/rjava_output/Ye_{}_Decaytime_{}.txt", ye0, decaytime);
        let path = Path::new(&input_path);
        let display = path.display();

        let mut file = File::open(&path).expect("couldn't read rjavaoutput");
        let lines = std::io::BufReader::new(file).lines();
        
        let mut ret = vec![];
        for line in lines {
            let t = line.unwrap();
            let arr = t.split_whitespace().collect::<Vec<_>>();

            let element         :String = arr[0].to_string();
            let proton_number   :f64    = arr[1].parse().unwrap();
            let neutron_number  :f64    = arr[2].parse().unwrap();
            let mass_number     :f64    = arr[3].parse().unwrap();
            let mass_amu        :f64    = arr[4].parse().unwrap();
            let solar_mf        :f64    = arr[5].parse().unwrap();
            let mf              :f64    = arr[6].parse().unwrap();
            let initial_mf      :f64    = arr[7].parse().unwrap();

            ret.push(RjavaAbundance::new(element, proton_number, neutron_number, mass_number, mass_amu, solar_mf, mf, initial_mf));
        }
        ret
    }
}

struct RjavaAbundance {
    element         : String,
    proton_number   : f64,
    neutron_number  : f64,
    mass_number     : f64,
    mass_amu        : f64,
    solar_mf        : f64,
    mf              : f64,
    initial_mf      : f64,
}

impl RjavaAbundance {
    fn sample() -> Self {
        Self{
            element: "hoge".to_string(), 
            proton_number: 0.0, 
            neutron_number: 0.0, 
            mass_number: 0.0, 
            mass_amu: 0.0, 
            solar_mf: 0.0, 
            mf: 0.0, 
            initial_mf: 0.0}
    }
    fn new(
        element: String,    proton_number: f64, neutron_number: f64,    mass_number: f64,
        mass_amu: f64,      solar_mf:f64,       mf:f64,                 initial_mf: f64,
    ) -> Self {
        Self {element, proton_number, neutron_number, mass_number, mass_amu, solar_mf, mf, initial_mf}
    }
}

struct OutputAbundance {
    // element         : String,
    proton_number   : f64,
    neutron_number  : f64,
    mass_number     : f64,
    mf              : f64,
}

const N_MAX: usize = 500;
const Z_MAX: usize = 500;
const DIFF_MAX: f64 = 0.01;

impl OutputAbundance {
    fn sample() -> Self {
        Self {
            // element         : "Hogehoge".to_string(),
            proton_number   : -1.0,
            neutron_number  : -1.0,
            mass_number     : -2.0,
            mf              : 0.0
        }
    }

    fn new(
        // element: String, 
        proton_number: f64, neutron_number: f64, mass_number: f64, mf: f64
    ) -> Self {
        Self{
            // element, 
            proton_number, neutron_number, mass_number, mf}
    }

    fn calc_abundances_from_yedistro(index: usize, ye0_vs_rjavaoutput: &Vec<(f64, RjavaOutput)>) -> Vec<Self> {
        fn search_weight_from_ye0(ye0: f64, ye_arr: &Vec<f64>, weight_arr: &Vec<f64>) -> f64 {
            let mut diff = std::f64::MAX;
            let mut ans = 0;
            for i in 0..ye_arr.len() {
                if diff > (ye0 - ye_arr[i]).abs() {
                    diff    = (ye0 - ye_arr[i]).abs();
                    ans     = i;
                }
            }
            // println!("ye0 = {}, ye_arr[i] = {}, weight_arr[i] = {}", ye0, ye_arr[ans], if diff > DIFF_MAX {0.0} else {weight_arr[ans]});
            if diff > DIFF_MAX {
                return 0.0
            } else {
                return weight_arr[ans];
            }
        }

        // fn add_mf(ye0: f64, weight: f64, )
        
        // indexを持つyedistro を読み込む。
        let ye_vs_weight        : YeDistro              = YeDistro::read_csv(index);
        let (ye_arr, weight_arr): (Vec<f64>, Vec<f64>)  = (ye_vs_weight.ye, ye_vs_weight.mass);
        
        // ye0(rjavaの計算の初期条件であるyeの配列)におけるweightをYeDistroから計算する。
        let mut ye0_vs_weight   :Vec<(f64, f64)>        = vec![];
        for ye0 in ye0_init() {
            let ye0 = ye0.parse().unwrap();
            let weight = search_weight_from_ye0(ye0, &ye_arr, &weight_arr);
            ye0_vs_weight.push((ye0, weight));
        }

        let mut mf_element = vec![vec![(0.0, "".to_string()); N_MAX]; Z_MAX];
        let mut ret = vec![];

        for &(ye0, weight) in ye0_vs_weight.iter() {
            let mut ok = false;
            for i in 0..ye0_vs_rjavaoutput.len() {
                if ye0 != ye0_vs_rjavaoutput[i].0 {continue;}
                ok = true;
                let now = &ye0_vs_rjavaoutput[i].1;
                let abundances = &now.abundances;
                for i in 0..abundances.len() {
                    let z = abundances[i].proton_number as usize;
                    let n = abundances[i].neutron_number as usize;
                    mf_element[z][n].0 += abundances[i].mf * weight;
                }
            }
            if !ok {panic!("didn't calculate ye0 = {}", ye0);}
        }
        for z in 0..Z_MAX {
            for n in 0..N_MAX {
                ret.push(Self::new(z as f64, n as f64, (z + n) as f64, mf_element[z][n].0));
            }
        }
        ret
    }
}

impl fmt::Display for OutputAbundance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "{}\t{}\t{}\t{}",
                 self.proton_number, self.neutron_number, self.mass_number, self.mf)
    }
}

fn main() {
    let (a, b, c)   : (f64, f64, f64)   = step1().expect("couldn't do step1");

    let yepapers    : Vec<YePaper>      = step2().expect("couldn't do step2");

    
    const D_MSUM    : f64   = 0.1;
    const D_MRATIO  : f64   = 0.01;
    const BASE      : f64   = 100.;
    const M_SUM_MN  : f64   = 3.0; // デフォルトは1.0
    const M_SUM_MX  : f64   = 5.0; // デフォルトは3.0
    
    let decaytimes = decaytimes_init();
    for decaytime in decaytimes.iter() {
        let ye0_vs_rjavaoutput: Vec<(f64, RjavaOutput)> = RjavaOutput::init(&decaytime);
        let mut m_sum   : f64   = M_SUM_MN;
        while m_sum <= M_SUM_MX {
            let mut m_ratio : f64   = 0.01;
            while m_ratio <= 1.0 {
                let yebar           : f64       = a + b * m_sum + c * m_ratio;
                let yedistro_index  : usize     = YePaper::search_yedistro_from_yebar(&yepapers, yebar);
                let abundances      : Vec<OutputAbundance> = OutputAbundance::calc_abundances_from_yedistro(yedistro_index, &ye0_vs_rjavaoutput);

                println!("#decaytime = {}, m_sum = {}, m_ratio = {}", decaytime, m_sum, m_ratio);
                for abundance in abundances {
                    if abundance.mf != 0. {
                        println!("{}", abundance);
                    }
                }
                println!("\n");
                m_ratio += D_MRATIO;
                m_ratio = (m_ratio * BASE).round() / BASE;
            }
            m_sum += D_MSUM;
            m_sum = (m_sum * BASE).round() / BASE;
        }
    }
}

fn decaytimes_init() -> Vec<String>{
    vec!["1e15", "3e15", "5e15", "7e15", "9e15",
            "1e16", "3e16", "5e16", "7e16", "9e16",
                "1e17", "3e17", "4.354e17"]
        .iter().map(|s| s.to_string()).collect::<Vec<String>>()
}

fn ye0_init() -> Vec<String> {
    vec!["0.02", "0.04", "0.06", "0.08", "0.10",
            "0.12", "0.14", "0.16", "0.18", "0.20",
                "0.22", "0.24", "0.26", "0.28", "0.30",
                    "0.32", "0.34", "0.36", "0.38", "0.40",
                        "0.42", "0.44", "0.46", "0.48", "0.50"]
        .iter().map(|s| s.to_string()).collect::<Vec<String>>()
}