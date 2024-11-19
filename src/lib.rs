use infinitable::Infinitable::{self, Finite, Infinity, NegativeInfinity};
use std::cmp;

fn calcular_lx(qx: &Vec<f64>, raiz: u16) -> Vec<f64> {
    let mut lx = Vec::new();
    lx.push(raiz as f64);
    for i in 1..(qx.len() + 1) {
        lx.push(lx[i - 1] * (1.0 - qx[i - 1]));
        if lx[i] == 0.0 {
            break;
        }
    }
    return lx;
}

fn min_positive_finite(a: Infinitable<u16>, b: u16) -> u16 {
    match a {
        Finite(a) => cmp::min(a, b),
        Infinity => b,
        NegativeInfinity => panic!("min_positive_finite não deve ser chamado com NegativeInfinity"),
    }
}

pub struct TabuaBase {
    qx: Vec<f64>,
    lx: Vec<f64>,
    pub w: Infinitable<u16>,
}

impl TabuaBase {
    pub fn new(qx: Vec<f64>) -> TabuaBase {
        let lx = calcular_lx(&qx, 10000);
        let mut w = Infinity;

        if lx[lx.len() - 1] == 0.0 {
            w = Finite((lx.len() - 2) as u16);
        }

        return TabuaBase { qx, lx, w };
    }

    pub fn tempo_futuro_maximo(&self, x: u16) -> Infinitable<u16> {
        return match self.w {
            Finite(w) => {
                if (w + 1) > x {
                    Finite(w + 1 - x)
                } else {
                    Finite(0)
                }
            }
            Infinity => Infinity,
            NegativeInfinity => panic!("w deve ser finito ou infinito, obtive NegativeInfinity"),
        };
    }

    pub fn possui_fechamento_plato(&self) -> bool {
        return !self.w.is_finite();
    }

    fn lx(&self, x: Infinitable<u16>) -> f64 {
        let limite_superior_x =
            min_positive_finite(self.tempo_futuro_maximo(0), self.qx.len() as u16);

        let x_trunc = min_positive_finite(x, limite_superior_x);

        let mut lx_ret = self.lx[x_trunc as usize].clone();

        if self.possui_fechamento_plato() && (x > Finite(x_trunc)) {
            match x {
                Infinity => {
                    return 0.0;
                }
                Finite(x) => {
                    let qx_last = self.qx[self.qx.len() - 1];
                    for _ in 0..(x - x_trunc) {
                        lx_ret = lx_ret * (1.0 - qx_last);
                    }
                    return lx_ret;
                }
                NegativeInfinity => {
                    panic!("x deve ser inteiro maior ou igual a 0 (pode ser Inifinity), obtive NegativeInfinity");
                }
            }
        }
        return lx_ret;
    }

    pub fn qx(&self, x: u16, t: Infinitable<u16>) -> f64 {
        let limite_superior_x =
            min_positive_finite(self.tempo_futuro_maximo(0), (self.qx.len() - 1) as u16);
        let x_trunc = cmp::min(x, limite_superior_x);

        let limite_superior_t = min_positive_finite(
            self.tempo_futuro_maximo(x),
            self.qx.len() as u16 - x_trunc - 1,
        );
        let t_trunc = min_positive_finite(t, limite_superior_t);
        return self.qx[(x_trunc + t_trunc) as usize];
    }

    pub fn tpx(&self, x: u16, t: Infinitable<u16>) -> f64 {
        if t == Finite(0) {
            return 1.0;
        }
        let lx = self.lx(Finite(x));
        let lxt = self.lx(Finite(x) + t);
        if lx == 0.0 {
            return 0.0;
        }
        return lxt / lx;
    }

    pub fn t_qx(&self, x: u16, t: Infinitable<u16>) -> f64 {
        return self.qx(x, t) * self.tpx(x, t);
    }
}
