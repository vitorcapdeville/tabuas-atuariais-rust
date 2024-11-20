use crate::interface::{validar_idades_tabuas, TabuaBiometrica};
use crate::tabua_base::TabuaBase;
use infinitable::Infinitable;

#[derive(Debug)]
pub struct Tabua {
    tabuas: Vec<TabuaBase>,
    numero_decrementos: usize,
    numero_vidas: usize,
}

impl Tabua {
    pub fn new(qx: Vec<f64>) -> Self {
        return Tabua {
            tabuas: vec![TabuaBase::new(qx)],
            numero_decrementos: 1,
            numero_vidas: 1,
        };
    }
}

impl TabuaBiometrica for Tabua {
    fn numero_decrementos(&self) -> usize {
        return self.numero_decrementos;
    }
    fn numero_vidas(&self) -> usize {
        return self.numero_vidas;
    }
    fn tempo_futuro_maximo(&self, x: &Vec<u16>) -> Infinitable<u16> {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        return self.tabuas[0].tempo_futuro_maximo(x[0]);
    }
    fn possui_fechamento_plato(&self) -> bool {
        return self.tabuas[0].possui_fechamento_plato();
    }
    fn qx(&self, x: &Vec<u16>, t: Infinitable<u16>) -> f64 {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        return self.tabuas[0].qx(x[0], t);
    }
    fn tpx(&self, x: &Vec<u16>, t: Infinitable<u16>) -> f64 {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        return self.tabuas[0].tpx(x[0], t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use infinitable::Finite;

    #[test]
    fn tabua_pode_ser_criada_a_partir_do_qx() {
        let qx = vec![0.0, 0.1, 0.5, 0.8, 1.0];
        Tabua::new(qx);
        assert!(true)
    }

    #[test]
    #[should_panic]
    fn qx_da_erro_se_vetor_de_idades_tiver_mais_que_1_entrada() {
        let tabua = Tabua::new(vec![0.0, 0.1, 0.5, 0.8, 1.0]);

        tabua.qx(&vec![0, 1], Finite(1));
    }

    #[test]
    #[should_panic]
    fn tpx_da_erro_se_vetor_de_idades_tiver_mais_que_1_entrada() {
        let tabua = Tabua::new(vec![0.0, 0.1, 0.5, 0.8, 1.0]);

        tabua.tpx(&vec![0, 1], Finite(1));
    }

    #[test]
    #[should_panic]
    fn t_qx_da_erro_se_vetor_de_idades_tiver_mais_que_1_entrada() {
        let tabua = Tabua::new(vec![0.0, 0.1, 0.5, 0.8, 1.0]);

        tabua.t_qx(&vec![0, 1], Finite(1));
    }

    #[test]
    #[should_panic]
    fn tempo_futuro_maximo_da_erro_se_vetor_de_idades_tiver_mais_que_1_entrada() {
        let tabua = Tabua::new(vec![0.0, 0.1, 0.5, 0.8, 1.0]);

        tabua.tempo_futuro_maximo(&vec![0, 1]);
    }
}
