#[macro_use]
extern crate serde_derive;
extern crate csv;

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::fmt;
use regex::Regex;

use glob::glob;

#[derive(Deserialize)]
struct ConnectData {
    Paper   : Option<String>,
    Model   : Option<String>,
    EOS     : Option<String>,
    M1      : Option<f64>,
    M2      : Option<f64>,
    Ye      : Option<f64>,
    M_ej    : Option<f64>,
    v       : Option<f64>,
    s       : Option<f64>,
    M2perM1 : Option<f64>,
    Msum    : Option<f64>,
}

impl ConnectData {
    fn read_csv() -> Vec<Self> {
        let path = Path::new("input/connect_data.csv");
        let display = path.display();
        
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {
                // print!("{} contains: \n{}", display, s),
            }
        }
        let mut ret = vec![];

        let mut reader = csv::Reader::from_reader(s.as_bytes());
        for data in reader.deserialize() {
            let connect_data: ConnectData = data.expect("can't open with deserialize");
            // m2 > m1だったらswapしてm1 > m2にする。
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
            if let Some(m1) = connect_datas[i].M1 {
                if let Some(m2) = connect_datas[i].M2 {
                    if let Some(ye) = connect_datas[i].Ye {
                        x.push(m1 + m2);
                        let m_ratio = if m1 >= m2 {m2 / m1} else {m1 / m2};
                        y.push(m_ratio);
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

        // println!("{}\t{}\t{}\n{}\t{}\t{}\n{}\t{}\t{}", n, x_sum, y_sum, x_sum, x2_sum, xy_sum, y_sum, xy_sum, y2_sum);
        // println!("{}\n{}\n{}", z_sum, xz_sum, yz_sum);

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
        println!("{} {} {}", a, b, c);
        // z = a + bx + cyについて
        // 最小二乗法でa,b,cを導出
        // z = Ye, x = M2perM1, y = Msum
        Ok((a, b, c))
    }
}

impl fmt::Display for ConnectData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Paper: {:?}, Model = {:?}, EOS: {:?}, M1: {:?}, M2: {:?}, Ye: {:?}, M_ej: {:?}, v: {:?}, s: {:?}, M2/M1: {:?}, Msum:{:?}",
                 self.Paper, self.Model, self.EOS, self.M1, self.M2, self.Ye, self.M_ej, self.v, self.s, self.M2perM1, self.Msum)
    }
}

#[derive(Deserialize, Debug, Clone)]
struct YedistroToYebar {
    filename    : String,
    condition   : String,
    Yebar       : f64,
}

impl YedistroToYebar {
    fn read_csv() -> Vec<Self> {
        let path = Path::new("input/YedistroToYebar.csv");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {},
        }

        let mut ret = vec![];
        let mut reader = csv::Reader::from_reader(s.as_bytes());
        for data in reader.deserialize() {
            let data: YedistroToYebar = data.expect("cannot open");
            ret.push(data);
        }
        ret
    }

    fn search_Yedistro(yebar_vs_yedisto: &Vec<(f64, String, String)>, ye: f64) -> (f64, String, String) {
        let mut v = yebar_vs_yedistro;
        let n = v.len()
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        if ye <= v[0].0 {
            return (v[0].0, v[0].1, v[0].2);
        }
        if v[n - 1].0 <= ye {
            return (v[n - 1].0, v[n - 1].1, v[n - 1].2);
        }

        let mut left = 0;
        let mut right = n;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if ye < v[mid] {
                right = mid;
            } else {
                left = mid;
            }
        }
        return (v[left].0, v[left].1, v[left].2);
    }
}


fn step1() -> Result<(f64, f64, f64), std::io::Error>{
    // 3dplot(M2/M1, Msumに対するYe)をプロットし、最小二乗法で平面を求める。

    // connect_data(論文のデータ)読み込み
    let connect_datas = ConnectData::read_csv();

    // connect_datasから最小二乗法の平面を導出。
    // z = a + bx + cyについて
    match ConnectData::least_square_plane(&connect_datas){
        Err(why) => panic!("couldn't caluculate least_square_plane, because: \n{}", why),
        Ok((a, b, c)) => {return Ok((a, b, c));}
    }
}

fn step2() -> Result<Vec<(f64, String, String)>, std::io::Error>{
    // Yedistroの(論文名、計算条件)とそのYebarが書かれたcsvを読み込む。
    let yedistro_to_yebars = YedistroToYebar::read_csv();
    let mut v = yedistro_to_yebars.iter().map(|a| (a.Yebar, a.filename.to_string(), a.condition.to_string())).collect::<Vec<(f64, String,String)>>();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    Ok(v)
}

fn decaytime_init() -> Vec<f64> {
    vec![1e15, 1e16, 1e17]
}

struct AtomicInfo {
    Name        : String,
    Z           : f64,
    A           : f64,
    N           : f64,
    mass_amu    : f64,
    Y           : f64,
}

impl AtomicInfo {
    fn sample() -> Self {
        Self{
            Name: "Hoge".to_string(), 
            mass_amu: 0.0,
            A: 0.0, 
            Z: 0.0, 
            N: 0.0, 
            Y: 0.0
        }
    }
    fn new(Name: String, Z: f64, A: f64,  N: f64, mass_amu: f64,Y: f64)-> Self {
        Self {Name,mass_amu,A, Z, N, Y}
    }
}

impl fmt::Display for AtomicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Z = {}, A = {}, N = {}, mass_amu = {:.5}, Y = {:.3e}",
                 self.Name,  self.Z, self.A, self.N, self.mass_amu,   self.Y)
    }
}


struct Abundance {
    Ye0_str          : String,
    Decaytime_str    : String,
    abundance_distro : Vec<AtomicInfo>
}

impl Abundance {
    fn new(Ye0_str: String, Decaytime_str: String, v: Vec<AtomicInfo>) -> Self{
        Self{Ye0_str, Decaytime_str, abundance_distro: v}
    }
}

fn read_initial_abundance() -> Result<Vec<Abundance>, std::io::Error> {
    let files = glob("input/rjava_output/Ye_*_Decaytime_*.txt").expect("cannot open");

    fn take_para_from_str(s: String) -> (String, String) {
        let t = s.split('/').collect::<Vec<_>>();
        let t1 = t[2];
        let t2 = t1.split('_').collect::<Vec<_>>();
        let t3 = t2.clone();
        let ye = t2[1].to_string();
        let t4 = t3[3];
        let decaytime = t4.split(".txt").collect::<Vec<_>>();
        (ye, decaytime[0].to_string())
    }

    let mut abundances = vec![];
    for file in files {
        let filepath = file.expect("Error with filepath");
        let (ye0, decaytime) = {
            let t = filepath.clone();
            let hoge = t.into_os_string().into_string().unwrap();
            take_para_from_str(hoge)
        };
        println!("{}, {}", ye0, decaytime);
        let path = Path::new(&filepath);
        let display = path.display();
        let lines = read_lines(&path).expect("Error with read_lines");

        let mut atomics = vec![];
        for line in lines {
            let line = line.expect("Error with unwrapping line");
            let v = line.split_whitespace().collect::<Vec<_>>();
            println!("{:?}", v);
            let atomic = AtomicInfo::new(
                            v[0].to_string(), 
                            v[1].parse::<f64>().unwrap(), 
                            v[2].parse::<f64>().unwrap(),
                            v[3].parse::<f64>().unwrap(),
                            v[4].parse::<f64>().unwrap(),
                            v[6].parse::<f64>().unwrap()
                        );
            atomics.push(atomic);
        }
        abundances.push(Abundance::new(ye0.to_string(), decaytime.to_string(), atomics));
    }
    Ok(abundances)
}


fn calc_abundance_from_yedistro(yedistro: &Vec<(f64, String, String)>, decaytime: f64) -> Vec<AtomicInfo> {
    let yedistro: Vec<f64, f64> = YeDistro::read_csv(yedistro);
    let mut abu = vec![vec![0.0; N_MAX+ 1]; Z_MAX + 1];

    for (ye, weight) in yedistro {
        let input_distro: Vec<AtomicInfo> = read_rjavaoutput(ye, f64);
        for atomic in input_distro {
            abu[atomic.Z][atomic.N] += atomic.Y * weight;
        }
    }

    let mut ret = vec![];
    for z in 0..Z_MAX + 1 {
        for n in 0..N_MAX + 1 {
            let now = AtomicInfo::new
        }
    }

}

fn main(){
    // Yeの平面(z = a + bx + cy)のパラメータをstep1で定義
    let (a, b, c) = match step1(){
        Err(why)        => panic!("couldn't do step1, because:\n{}", why),
        Ok((a, b, c))   => (a, b, c)
    };

    // YebarとYedistro(の名前)を結びつける配列をstep2で定義。
    let yebar_vs_yedistro = match step2() {
        Err(why)    => panic!("couldn't do step2, because:\n{}", why),
        Ok(v)       => v,
    };

    // r-javaで計算した最終的な存在量を保持しておく。
    let ye0_vs_abundance = match read_initial_abundance() {
        Err(why)    => panic!("couldn't read initial_aubundance:\n{}", why),
        Ok(x)       => x
    };


    let mut M_sum = 1.0;
    let mut M2parM1 = 0.01;
    let decaytimes = decaytime_init();

    while M_sum <= 3.0 {
        while M2parM1 <= 1.0 {
            for &decaytime in decaytimes.iter() {
                let Yebar   : f64                    = a + M_sum * b + M2perM1 * c;
                let Yedistro: (f64, String, String)  = Yedistro_to_Yebar::search_Yedistro(&yebar_vs_yedistro, Yebar);
                let abundance: Vec<AtomicInfo> = calc_abundance_from_Yedistro(&Yedistro, decaytime);
            }
            M2perM1 += 0.01;
        }
        M_sum += 0.1;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> , {
    let file = match File::open(filename) {
        Err(why) => panic!("{}", why),
        Ok(file) => file,
    };
    Ok(io::BufReader::new(file).lines())
}

