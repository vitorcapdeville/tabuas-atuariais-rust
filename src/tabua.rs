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

#[derive(Debug)]
pub struct Tabua {
    qx: Vec<f64>,
    lx: Vec<f64>,
    w: Infinitable<u16>,
}

impl Tabua {
    pub fn new(qx: Vec<f64>) -> Tabua {
        let lx = calcular_lx(&qx, 10000);
        let mut w = Infinity;

        if lx[lx.len() - 1] == 0.0 {
            w = Finite((lx.len() - 2) as u16);
        }

        return Tabua { qx, lx, w };
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx;
    use infinitable::Infinitable::{Finite, Infinity};

    fn criar_tabua_plato() -> Tabua {
        Tabua::new(vec![0.1, 0.3, 0.5, 0.7, 0.9])
    }

    fn criar_tabua_completa() -> Tabua {
        Tabua::new(vec![0.1, 0.2, 0.4, 0.8, 1.0])
    }

    #[test]
    fn quando_idade_maxima_eh_infinita_tempo_futuro_max_eh_infinito() {
        let tabua = criar_tabua_plato();

        assert_eq!(tabua.tempo_futuro_maximo(0), Infinity);
        assert_eq!(tabua.tempo_futuro_maximo(15), Infinity);
        assert_eq!(tabua.tempo_futuro_maximo(100), Infinity);
    }

    #[test]
    fn quando_idade_maxima_nao_eh_infinita_tempo_futuro_max_eh_finito() {
        let tabua = criar_tabua_completa();

        assert_eq!(tabua.tempo_futuro_maximo(0), Finite(5));
        assert_eq!(tabua.tempo_futuro_maximo(3), Finite(2));
        assert_eq!(tabua.tempo_futuro_maximo(100), Finite(0));
    }

    #[test]
    fn qx_eh_igual_a_1_quando_x_mais_t_for_maior_ou_igual_ao_tempo_futuro_max_e_a_tabua_eh_completa(
    ) {
        let tabua = criar_tabua_completa();

        assert_eq!(tabua.qx(0, Finite(10)), 1.0);
        assert_eq!(tabua.qx(0, Finite(50)), 1.0);
        assert_eq!(tabua.qx(0, Finite(100)), 1.0);
    }

    #[test]
    fn qx_eh_igual_ao_ultimo_qx_quando_x_mais_t_for_maior_ou_igual_ao_tempo_futuro_max_e_a_tabua_eh_plato(
    ) {
        let tabua = criar_tabua_plato();

        assert_eq!(tabua.qx(0, Finite(10)), 0.9);
        assert_eq!(tabua.qx(0, Finite(50)), 0.9);
        assert_eq!(tabua.qx(0, Finite(100)), 0.9);
    }

    #[test]
    fn tpx_eh_igual_a_1_quando_t_for_igual_a_0_plato() {
        let tabua = criar_tabua_plato();

        assert_eq!(tabua.tpx(0, Finite(0)), 1.0);
        assert_eq!(tabua.tpx(1, Finite(0)), 1.0);
        assert_eq!(tabua.tpx(2, Finite(0)), 1.0);
    }

    #[test]
    fn tpx_eh_igual_a_1_quando_t_for_igual_a_0_completo() {
        let tabua = criar_tabua_completa();

        assert_eq!(tabua.tpx(0, Finite(0)), 1.0);
        assert_eq!(tabua.tpx(1, Finite(0)), 1.0);
        assert_eq!(tabua.tpx(2, Finite(0)), 1.0);
    }

    #[test]
    fn tpx_eh_igual_a_0_quando_t_for_maior_ou_igual_ao_tempo_futuro_max_e_a_tabua_completa() {
        let tabua = criar_tabua_completa();
        let tempo_futuro_max = tabua.tempo_futuro_maximo(3);

        assert_eq!(tabua.tpx(3, tempo_futuro_max + Finite(0)), 0.0);
        assert_eq!(tabua.tpx(3, tempo_futuro_max + Finite(1)), 0.0);
        assert_eq!(tabua.tpx(3, tempo_futuro_max + Finite(2)), 0.0);
    }

    #[test]
    fn tpx_eh_igual_a_zero_quando_x_maior_ou_igual_ao_tempo_futuro_max_t_maior_que_zero_e_tabua_completa(
    ) {
        let tabua = criar_tabua_completa();
        let x = tabua.tempo_futuro_maximo(0) + Finite(1);
        match x {
            Finite(x) => {
                assert_eq!(tabua.tpx(x, Finite(1)), 0.0);
                assert_eq!(tabua.tpx(x, Finite(2)), 0.0);
                assert_eq!(tabua.tpx(x, Finite(3)), 0.0);
            }
            _ => panic!("x deveria ser finito numa tabua completa"),
        }
    }

    #[test]
    fn tpx_termina_com_zero_plato() {
        let tabua = criar_tabua_plato();
        let x = 2;
        let t = Finite(100);

        approx::assert_abs_diff_eq!(tabua.tpx(x, t), 0.0);
    }

    #[test]
    fn tpx_termina_com_zero_completa() {
        let tabua = criar_tabua_completa();
        let x = 2;
        let t = tabua.tempo_futuro_maximo(0);

        assert_eq!(tabua.tpx(x, t), 0.0);
    }

    #[test]
    fn t_qx_soma_1_quando_t_sao_todos_os_tempos_futuros_plato() {
        let tabua = criar_tabua_plato();
        let x = 2;
        let limite = 100;
        let t = (0..=limite).map(|i| Finite(i)).collect::<Vec<_>>();

        let result = t.iter().map(|&t| tabua.t_qx(x, t)).sum::<f64>();

        approx::assert_abs_diff_eq!(result, 1.0);
    }

    #[test]
    fn t_qx_soma_1_quando_t_sao_todos_os_tempos_futuros_completa() {
        let tabua = criar_tabua_completa();
        let x = 2;
        let limite = tabua
            .tempo_futuro_maximo(x)
            .finite()
            .expect("Tempo futuro maximo de uma tabua completa deveria ser finito!");
        let t = (0..=limite).map(|i| Finite(i)).collect::<Vec<_>>();

        let result = t.iter().map(|&t| tabua.t_qx(x, t)).sum::<f64>();

        approx::assert_abs_diff_eq!(result, 1.0);
    }

    #[test]
    fn t_qx_eh_igual_a_1_quando_t_eh_zero_e_x_eh_superior_ao_tempo_futuro_maximo_e_tabua_completa()
    {
        let tabua = criar_tabua_completa();
        let x = tabua
            .tempo_futuro_maximo(0)
            .finite()
            .expect("Tempo futuro maximo de uma tabua completa deveria ser finito!");

        approx::assert_abs_diff_eq!(tabua.t_qx(x - 2, Finite(0)), 0.8);
        approx::assert_abs_diff_eq!(tabua.t_qx(x, Finite(0)), 1.0);
        approx::assert_abs_diff_eq!(tabua.t_qx(x + 2, Finite(0)), 1.0);
    }

    #[test]
    fn t_qx_eh_igual_a_0_quando_x_eh_superior_ao_tempo_futuro_max_e_t_eh_maior_que_zero_e_tabua_completa(
    ) {
        let tabua = criar_tabua_completa();
        let x = tabua
            .tempo_futuro_maximo(0)
            .finite()
            .expect("Tempo futuro maximo de uma tabua completa deveria ser finito!")
            + 1;

        assert_eq!(tabua.t_qx(x, Finite(1)), 0.0);
        assert_eq!(tabua.t_qx(x, Finite(2)), 0.0);
        assert_eq!(tabua.t_qx(x, Finite(3)), 0.0);
    }
}
