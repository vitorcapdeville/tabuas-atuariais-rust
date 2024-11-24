use crate::interface::{validar_idades_tabuas, TabuaInterface};
use crate::tabua_base::TabuaBase;
use crate::Periodicidade;
use infinitable::Infinitable;

#[derive(Debug, Clone)]
pub struct Tabua {
    tabua: TabuaBase,
    periodicidade: Periodicidade,
}

impl Tabua {
    pub fn obter_tabua_base(&self) -> &TabuaBase {
        return &self.tabua;
    }

    pub fn new(qx: Vec<f64>, periodicidade: Periodicidade) -> Self {
        return Tabua {
            tabua: TabuaBase::new(qx),
            periodicidade,
        };
    }
}

impl TabuaInterface for Tabua {
    fn periodicidade(&self) -> &Periodicidade {
        return &self.periodicidade;
    }

    fn numero_decrementos(&self) -> usize {
        return 1;
    }

    fn numero_vidas(&self) -> usize {
        return 1;
    }

    fn tempo_futuro_maximo(&self, x: &Vec<u16>) -> Infinitable<u16> {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        return self.tabua.tempo_futuro_maximo(x[0]);
    }

    fn qx(&self, x: &Vec<u16>, t: u16) -> f64 {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        return self.tabua.qx(x[0], t);
    }

    fn tpx(&self, x: &Vec<u16>, t: u16) -> f64 {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        return self.tabua.tpx(x[0], t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn criar_tabua() -> Tabua {
        return Tabua::new(vec![0.0, 0.1, 0.5, 0.8, 1.0], Periodicidade::Mensal);
    }

    #[test]
    fn tabua_pode_ser_criada_a_partir_do_qx_e_periodicidade() {
        criar_tabua();
        assert!(true)
    }

    #[test]
    #[should_panic]
    fn qx_da_erro_se_vetor_de_idades_tiver_mais_que_1_entrada() {
        let tabua = criar_tabua();

        tabua.qx(&vec![0, 1], 1);
    }

    #[test]
    #[should_panic]
    fn tpx_da_erro_se_vetor_de_idades_tiver_mais_que_1_entrada() {
        let tabua = criar_tabua();

        tabua.tpx(&vec![0, 1], 1);
    }

    #[test]
    #[should_panic]
    fn t_qx_da_erro_se_vetor_de_idades_tiver_mais_que_1_entrada() {
        let tabua = criar_tabua();

        tabua.t_qx(&vec![0, 1], 1);
    }

    #[test]
    #[should_panic]
    fn tempo_futuro_maximo_da_erro_se_vetor_de_idades_tiver_mais_que_1_entrada() {
        let tabua = criar_tabua();

        tabua.tempo_futuro_maximo(&vec![0, 1]);
    }
}
