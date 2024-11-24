use crate::interface::{validar_idades_tabuas, TabuaInterface};
use crate::tabua_base::TabuaBase;
use crate::Periodicidade;
use crate::Tabua;
use infinitable::Infinitable;

pub enum StatusVidasConjuntas {
    First,
    Last,
}

pub struct TabuaMultiplasVidas {
    tabuas: Vec<TabuaBase>,
    status_vidas_conjuntas: StatusVidasConjuntas,
    periodicidade: Periodicidade,
}

impl TabuaMultiplasVidas {
    pub fn new(tabuas: Vec<Tabua>, status_vidas_conjuntas: StatusVidasConjuntas) -> Self {
        let periodicidade = tabuas[0].periodicidade().clone();

        let tabuas = tabuas
            .iter()
            .map(|tabua| {
                if tabua.periodicidade() != &periodicidade {
                    panic!("Todas as tabuas devem possuir a mesma periodicidade.");
                }
                tabua.obter_tabua_base().clone()
            })
            .collect();

        return TabuaMultiplasVidas {
            tabuas,
            status_vidas_conjuntas,
            periodicidade,
        };
    }
}

impl TabuaInterface for TabuaMultiplasVidas {
    fn periodicidade(&self) -> &Periodicidade {
        return &self.periodicidade;
    }

    fn numero_decrementos(&self) -> usize {
        return 1;
    }

    fn numero_vidas(&self) -> usize {
        return self.tabuas.len();
    }

    fn tempo_futuro_maximo(&self, x: &Vec<u16>) -> Infinitable<u16> {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());

        let tempos = self
            .tabuas
            .iter()
            .zip(x.iter())
            .map(|(tabua, idade)| tabua.tempo_futuro_maximo(*idade));

        let result = match self.status_vidas_conjuntas {
            StatusVidasConjuntas::First => tempos.min(),
            StatusVidasConjuntas::Last => tempos.max(),
        };

        return result.expect("TabuaMultiplasVidas deveria possuir uma ou mais tábuas.");
    }

    fn qx(&self, x: &Vec<u16>, t: u16) -> f64 {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());

        let iter = self.tabuas.iter().zip(x.iter());

        match self.status_vidas_conjuntas {
            StatusVidasConjuntas::First => {
                let iter_fold =
                    iter.fold(1.0, |acc, (tabua, idade)| acc * (1.0 - tabua.qx(*idade, t)));
                return 1.0 - iter_fold;
            }
            StatusVidasConjuntas::Last => {
                return iter.fold(1.0, |acc, (tabua, idade)| acc * tabua.qx(*idade, t));
            }
        }
    }

    fn tpx(&self, x: &Vec<u16>, t: u16) -> f64 {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());

        let iter = self.tabuas.iter().zip(x.iter());

        match self.status_vidas_conjuntas {
            StatusVidasConjuntas::First => {
                return iter.fold(1.0, |acc, (tabua, idade)| acc * tabua.tpx(*idade, t));
            }
            StatusVidasConjuntas::Last => {
                return (1..=t).fold(1.0, |acc, t| acc * 1.0 - self.qx(x, t - 1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //TODO: Incluir testes para casos com t = Infinity.
    // tpx deveria ser sempre zero (probabilidade de sobreviver infinitos anos é zero)
    // qx deveria ser sempre 1.0 (probabiliade de morrer antes de completar infinitos anos é sempre um)
    // t_qx deveria ser sempre 0.0 (pois tpx é zero e qx é um)

    fn criar_tabua_1_vida_1() -> Tabua {
        Tabua::new(vec![0.0, 0.1, 0.5, 0.8, 1.0], Periodicidade::Mensal)
    }

    fn criar_tabua_1_vida_2() -> Tabua {
        Tabua::new(vec![0.0, 0.2, 0.4, 0.7, 0.8], Periodicidade::Mensal)
    }

    #[test]
    fn tabua_multiplas_vidas_pode_ser_criada_a_partir_de_outras_tabuas() {
        let tabua = criar_tabua_1_vida_1();
        TabuaMultiplasVidas::new(
            vec![tabua.clone(), tabua.clone()],
            StatusVidasConjuntas::First,
        );
        assert!(true)
    }

    #[test]
    fn tpx_retorna_produto_acumulado_de_um_menos_qx_quando_status_eh_last() {
        let tabua = criar_tabua_1_vida_1();
        let tabua_multiplas_vidas = TabuaMultiplasVidas::new(
            vec![tabua.clone(), tabua.clone()],
            StatusVidasConjuntas::Last,
        );

        let x = vec![0, 0];
        let t = 2;

        let result = tabua_multiplas_vidas.tpx(&x, t);

        approx::assert_relative_eq!(
            result,
            (1.0 - tabua_multiplas_vidas.qx(&x, 0)) * (1.0 - tabua_multiplas_vidas.qx(&x, 1))
        );
    }

    #[test]
    fn qx_retorna_o_produto_de_qx_de_cada_tabua_quando_status_eh_last() {
        let tabua = criar_tabua_1_vida_1();
        let tabua_multiplas_vidas = TabuaMultiplasVidas::new(
            vec![tabua.clone(), tabua.clone()],
            StatusVidasConjuntas::Last,
        );

        let x = vec![0, 0];
        let t = 2;

        let result = tabua_multiplas_vidas.qx(&x, t);

        approx::assert_relative_eq!(
            result,
            tabua.qx(&vec![x[0]], t) * tabua.qx(&vec![x[1]], t),
            epsilon = f64::EPSILON
        );
    }

    #[test]
    fn qx_retorna_um_menos_o_produto_de_1_menos_qx_quando_status_eh_first() {
        let tabua = criar_tabua_1_vida_1();
        let tabua_multiplas_vidas = TabuaMultiplasVidas::new(
            vec![tabua.clone(), tabua.clone()],
            StatusVidasConjuntas::First,
        );

        let x = vec![0, 0];
        let t = 2;

        let result = tabua_multiplas_vidas.qx(&x, t);

        approx::assert_relative_eq!(
            result,
            1.0 - (1.0 - tabua.qx(&vec![x[0]], t)) * (1.0 - tabua.qx(&vec![x[1]], t)),
            epsilon = f64::EPSILON
        );
    }

    #[test]
    #[should_panic(expected = "O vetor de idades é incompatível")]
    fn qx_falha_quando_tamanho_de_x_eh_incompativel_com_a_qntd_de_tabuas() {
        let tabua = criar_tabua_1_vida_1();
        let tabua_multiplas_vidas = TabuaMultiplasVidas::new(
            vec![tabua.clone(), tabua.clone()],
            StatusVidasConjuntas::First,
        );

        let x = vec![0, 0, 0];
        let t = 2;

        tabua_multiplas_vidas.qx(&x, t);
    }

    #[test]
    fn tempo_futuro_max_retorna_o_menor_dos_tempos_quando_status_eh_first() {
        let tabua1 = criar_tabua_1_vida_1();
        let tabua2 = criar_tabua_1_vida_2();
        let tabua_multiplas_vidas = TabuaMultiplasVidas::new(
            vec![tabua1.clone(), tabua2.clone()],
            StatusVidasConjuntas::First,
        );

        let x = vec![0, 0];
        let result = tabua_multiplas_vidas.tempo_futuro_maximo(&x);

        let result_tabua1 = tabua1.tempo_futuro_maximo(&vec![x[0]]);
        let result_tabua2 = tabua2.tempo_futuro_maximo(&vec![x[1]]);

        assert_eq!(result, result_tabua1.min(result_tabua2));
    }

    #[test]
    fn tempo_futuro_max_retorna_o_maior_dos_tempos_quando_status_eh_last() {
        let tabua1 = criar_tabua_1_vida_1();
        let tabua2 = criar_tabua_1_vida_2();
        let tabua_multiplas_vidas = TabuaMultiplasVidas::new(
            vec![tabua1.clone(), tabua2.clone()],
            StatusVidasConjuntas::Last,
        );

        let x = vec![0, 0];
        let result = tabua_multiplas_vidas.tempo_futuro_maximo(&x);

        let result_tabua1 = tabua1.tempo_futuro_maximo(&vec![x[0]]);
        let result_tabua2 = tabua2.tempo_futuro_maximo(&vec![x[1]]);

        assert_eq!(result, result_tabua1.max(result_tabua2));
    }

    #[test]
    #[should_panic(expected = "O vetor de idades é incompatível")]
    fn tempo_futuro_max_falha_quando_o_tamanho_de_x_eh_incompativel_com_a_qntd_de_tabuas() {
        let tabua1 = criar_tabua_1_vida_1();
        let tabua2 = criar_tabua_1_vida_2();
        let tabua_multiplas_vidas = TabuaMultiplasVidas::new(
            vec![tabua1.clone(), tabua2.clone()],
            StatusVidasConjuntas::Last,
        );

        let x = vec![0, 0, 0];
        tabua_multiplas_vidas.tempo_futuro_maximo(&x);
    }
}
