use num_bigint::BigInt;
use num_traits::One;

pub struct Folding {
    m_n: i32,
    m_count: u64,
}

impl Folding {
    pub fn new() -> Self {
        Self {
            m_n: -1,
            m_count: 0,
        }
    }

    fn process(&mut self, _a: &Vec<i32>, _b: &Vec<i32>, n: i32) {
        self.m_count += n as u64;
    }

    pub fn foldings(&mut self, p: Vec<i32>, flag: bool, res: i32, mod_value: i32) {
        let mut n = 1;
        for &pp in &p {
            n *= pp;
        } 

        let mut a = vec![0; n as usize + 1];
        let mut b = vec![0; n as usize + 1];
        let mut count = vec![0; n as usize + 1];
        let mut gapter = vec![0; n as usize + 1];
        let mut gap = vec![0; n as usize * n as usize + 1];

        let dim = p.len();
        let mut big_p = vec![1; dim + 1];
        let mut c = vec![vec![0; n as usize + 1]; dim + 1];
        let mut d = vec![vec![vec![0; n as usize + 1]; n as usize + 1]; dim + 1];

        for i in 1..dim {
            big_p[i] = big_p[i - 1] * p[i - 1];
        }

        for i in 1..=dim {
            for m in 1..=n {
                c[i][m as usize] = (m - 1) / big_p[i - 1] - ((m - 1) / big_p[i]) * p[i - 1] + 1;
            }
        }

        for i in 1..dim {
            for l in 1..=n {
                for m in 1..=l {
                    let delta = c[i][l as usize] - c[i][m as usize];
                    d[i][l as usize][m as usize] = if delta % 2 == 0 {
                        if c[i][m as usize] == 1 {
                            m
                        } else {
                            m - big_p[i - 1]
                        }
                    } else if c[i][m as usize] == p[i - 1] || m + big_p[i - 1] > l {
                        m
                    } else {
                        m + big_p[i - 1]
                    };
                }
            }
        }
        
        let mut g = 0;
        let mut l = 1;

        while l > 0 {
            if !flag || l <= 1 || b[0] == 1{
                if l > n {
                    self.process(&a, &b, n);
                } else {
                    let mut dd = 0;
                    let mut gg = gapter[l as usize - 1];
                    g = gg;

                    for i in 1..=dim {
                        if d[i][l as usize][l as usize] == l {
                            dd += 1;
                        } else {
                            let mut m = d[i][l as usize][l as usize];
                            while m != l {
                                if mod_value == 0 || l != mod_value || m % mod_value == res {
                                    gap[gg as usize] = m;
                                    if count[m as usize] == 0 {
                                        gg += 1;
                                    }
                                    count[m as usize] += 1;
                                }
                                m = d[i][l as usize][b[m as usize] as usize];
                            }
                        }
                    }
                    
                    if dd == dim {
                        for m in 0..l {
                            gap[gg as usize] = m;
                            gg += 1;
                        }
                    }

                    for j in g..gg {
                        gap[g as usize] = gap[j as usize];
                        if count[gap[j as usize] as usize] == dim - dd {
                            g += 1;
                        }
                        count[gap[j as usize] as usize] = 0;
                    }
                }
            }

            while l > 0 && g == gapter[l as usize - 1] {
                l -= 1;
                b[a[l as usize] as usize] = b[l as usize];
                a[b[l as usize] as usize] = a[l as usize];
            }

            if l > 0 {
                println!("l: {}, a: {:?}, g: {}", l, a, g);
                a[l as usize] = gap[g as usize - 1];
                b[l as usize] = b[a[l as usize] as usize];
                b[a[l as usize] as usize] = l;
                a[b[l as usize] as usize] = l;
                gapter[l as usize] = g - 1;
                l += 1;
            }
        }
    }

    fn get_dimensions(&self, n: i32) -> Vec<i32> {
        vec![n, 2]
    }

    pub fn next(&mut self) -> BigInt {
        if self.m_n == -1 {
            self.m_n += 1;
            BigInt::one();
        }
        self.m_n += 1;
        self.m_count = 0;
        self.foldings(self.get_dimensions(self.m_n), true, 0, 0);
        BigInt::from(self.m_count)
    }
}
