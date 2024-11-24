use crate::interface::{validar_idades_tabuas, TabuaInterface};
use crate::tabua::extrair_tabua_base_e_periodicidade;
use crate::tabua_base::TabuaBase;
use crate::Periodicidade;
use crate::Tabua;
use infinitable::Infinitable;

fn qx2qxj(qx1: &f64, qx2: &f64, qx3: &f64) -> f64 {
    return qx1 * (1.0 - 0.5 * (qx2 + qx3) + 1.0 / 3.0 * (qx2 * qx3));
}

fn converter_mdt(qx: Vec<f64>) -> Vec<f64> {
    let tamanho = qx.len();

    if tamanho == 0 {
        panic!("A quantidade de decrementos não pode ser zero.");
    }

    if tamanho > 3 {
        panic!("A quantidade de decrementos não pode ser maior que 3, obtido = {tamanho}.");
    }

    let mut qx_mdt = vec![0.0; 3];

    for i in 0..3 {
        qx_mdt[i] = qx2qxj(
            qx.get(i % 3).unwrap_or(&0.0),
            qx.get((i + 1) % 3).unwrap_or(&0.0),
            qx.get((i + 2) % 3).unwrap_or(&0.0),
        );
    }

    return qx_mdt;
}

#[derive(Debug)]
pub struct TabuaMDT {
    tabuas: Vec<TabuaBase>,
    periodicidade: Periodicidade,
}

impl TabuaMDT {
    pub fn new(tabuas: Vec<Tabua>) -> Self {
        if tabuas.len() < 1 {
            panic!("Tabua MDT deve possuir pelo menos uma tabua.");
        }
        if tabuas.len() > 3 {
            panic!("Tabua MDT deve possuir no máximo três tabuas.");
        }

        let (tabuas, periodicidade) = extrair_tabua_base_e_periodicidade(tabuas);

        return TabuaMDT {
            tabuas,
            periodicidade,
        };
    }

    fn qx_j(&self, x: &Vec<u16>, t: u16, j: usize) -> f64 {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        if j > self.numero_decrementos() {
            panic!("j não pode ser maior que o número de decrementos. j = {j}, número de decrementos = {}", self.numero_decrementos());
        }

        let qx: Vec<f64> = (0..self.numero_decrementos())
            .map(|k| self.tabuas[k].qx(x[k], t))
            .collect();

        return converter_mdt(qx)[j];
    }
}

impl TabuaInterface for TabuaMDT {
    //TODO: Eu não gosto da validação rodando no lugar que ela tá rodando.
    // Se eu quiser calcular qualquer quantidade para um vetor de tempos, com o mesmo x,
    // eu vou ter q validar para cada tempo o x sendo que o x não mudou.
    // É estranho por que parece q a solução é não ter essa validação, e criar uma função que recebe uma tábua qlqr, um x, um t e faz a conta.
    // E ai eu teria uma função dessa para qx, tpx e t_qx. Não sei, ainda parece estranho.
    // Outra solução que talvez seja mais clean mais ainda sounds weird seria ter um qx_unsafe e ter um outro qx, talvez sempre recebendo t como vetor, que usa o unsafe.
    // e obviamente o unsafe ficaria privado.
    // Na real isso tá começando a parecer fazível, eu poderia implementar sempre a versão unsafe e a versão safe viria de graça como um wrapper definido no trait,
    // fazendo o map.
    // Pera, se eu definir no trait, a versão unsafe vai precisar ficar definida lá, o que significa q se eu fizer o trait pub, ela tb vai ser pub, oh no.
    // Talvez ser pub não seja uma má ideia, se o usuário quiser usar a versão escalar unsafe ele possa mas sabendo que ele tem q controlar o tamanho de x.
    fn periodicidade(&self) -> &Periodicidade {
        return &self.periodicidade;
    }

    fn numero_decrementos(&self) -> usize {
        return self.tabuas.len();
    }

    fn numero_vidas(&self) -> usize {
        return 1;
    }

    fn tempo_futuro_maximo(&self, x: &Vec<u16>) -> Infinitable<u16> {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        return self
            .tabuas
            .iter()
            .zip(x.iter())
            .map(|(tabua, x)| tabua.tempo_futuro_maximo(*x))
            .min()
            .expect("Tabua MTD deveria possuir uma ou mais tábuas.");
    }

    fn qx(&self, x: &Vec<u16>, t: u16) -> f64 {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        return (0..self.numero_decrementos()).fold(0.0, |acc, j| acc + self.qx_j(x, t, j));
    }

    fn tpx(&self, x: &Vec<u16>, t: u16) -> f64 {
        validar_idades_tabuas(x, self.numero_decrementos(), self.numero_vidas());
        return self
            .tabuas
            .iter()
            .zip(x.iter())
            .fold(1.0, |acc, (tabua, x)| acc * tabua.tpx(*x, t));
    }
}

#[cfg(test)]
mod tests {
    use std::cmp;

    use super::*;
    use approx;

    fn criar_tabua_1dt_1() -> Tabua {
        Tabua::new(vec![0.0, 0.1, 0.5, 0.8, 1.0], Periodicidade::Mensal)
    }

    fn criar_tabua_1dt_2() -> Tabua {
        Tabua::new(vec![0.0, 0.2, 0.4, 0.7, 1.0], Periodicidade::Mensal)
    }

    fn criar_tabua_1dt_plato() -> Tabua {
        Tabua::new(vec![0.0, 0.2, 0.4, 0.7, 0.8], Periodicidade::Mensal)
    }

    #[test]
    fn tabua_mdt_pode_ser_criada_a_partir_de_outras_tabuas() {
        let tabua = criar_tabua_1dt_1();
        TabuaMDT::new(vec![tabua.clone(), tabua.clone()]);
        assert!(true)
    }

    #[test]
    #[should_panic(expected = "Tabua MDT deve possuir pelo menos uma tabua.")]
    fn tabua_mdt_precisa_de_pelo_menos_1_tabua() {
        TabuaMDT::new(vec![]);
    }

    #[test]
    #[should_panic(expected = "Tabua MDT deve possuir no máximo três tabuas.")]
    fn tabua_mdt_aceita_no_maximo_3_tabuas() {
        let tabua = criar_tabua_1dt_1();
        TabuaMDT::new(vec![
            tabua.clone(),
            tabua.clone(),
            tabua.clone(),
            tabua.clone(),
        ]);
    }

    #[test]
    #[should_panic(expected = "quantidade de decrementos não pode ser zero")]
    fn converter_mdt_nao_pode_ser_chamada_com_vetor_vazio() {
        converter_mdt(vec![]);
    }

    #[test]
    #[should_panic(expected = "quantidade de decrementos não pode ser maior que 3")]
    fn converter_mdt_nao_pode_ser_chamada_com_vetor_com_mais_que_3_elementos() {
        converter_mdt(vec![0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn converter_mdt_preenche_com_zero_quando_recebe_2_elementos() {
        let result = converter_mdt(vec![0.1, 0.2]);

        assert_eq!(result.len(), 3);
        assert_eq!(result[2], 0.0);
    }

    #[test]
    fn converter_mdt_preenche_com_zero_quando_recebe_1_elemento() {
        let result = converter_mdt(vec![0.1]);

        assert_eq!(result.len(), 3);
        assert_eq!(result[1], 0.0);
        assert_eq!(result[2], 0.0);
    }

    #[test]
    fn calculo_qxj_funciona_com_1_componente() {
        let qx = &0.4;

        let result = qx2qxj(qx, &0.0, &0.0);

        assert_eq!(result, *qx);
    }

    #[test]
    fn calculo_qxj_funciona_com_2_componentes() {
        let qx1 = &0.4;
        let qx2 = &0.8;
        let result = qx2qxj(qx1, qx2, &0.0);

        assert_eq!(result, qx1 * (1.0 - 0.5 * qx2));
    }

    #[test]
    fn calculo_qxj_funciona_com_3_componentes() {
        let qx1 = &0.4;
        let qx2 = &0.8;
        let qx3 = &0.2;

        let result = qx2qxj(qx1, qx2, qx3);

        assert_eq!(
            result,
            qx1 * (1.0 - 0.5 * (qx2 + qx3) + 1.0 / 3.0 * (qx2 * qx3))
        );
    }

    #[test]
    fn tpx_eh_produto_do_tpx_de_cada_tabua() {
        let tabua1 = criar_tabua_1dt_1();
        let tabua2 = criar_tabua_1dt_2();
        let tabua_mdt = TabuaMDT::new(vec![tabua1.clone(), tabua2.clone()]);

        let x = vec![2, 1];
        let t = 2;

        let result = tabua_mdt.tpx(&x, t);

        approx::assert_abs_diff_eq!(
            result,
            tabua1.tpx(&vec![x[0]], t) * tabua2.tpx(&vec![x[1]], t)
        );
    }

    #[test]
    #[should_panic(expected = "O vetor de idades é incompatível")]
    fn tpx_falha_quando_x_nao_tem_tamanho_correto() {
        let tabua = criar_tabua_1dt_1();
        let tabua_mdt = TabuaMDT::new(vec![tabua]);

        let x = vec![2, 1, 3];
        let t = 2;

        tabua_mdt.tpx(&x, t);
    }

    #[test]
    fn qx_retorna_a_soma_de_qxj() {
        let tabua1 = criar_tabua_1dt_1();
        let tabua2 = criar_tabua_1dt_2();
        let tabua_mdt = TabuaMDT::new(vec![tabua1, tabua2]);

        let x = vec![2, 1];
        let t = 2;

        let qx_0 = tabua_mdt.qx_j(&x, t, 0);
        let qx_1 = tabua_mdt.qx_j(&x, t, 1);
        let result = tabua_mdt.qx(&x, t);

        approx::assert_abs_diff_eq!(result, qx_0 + qx_1);
    }

    #[test]
    fn tempo_futuro_maximo_retorna_o_menor_dos_tempos_futuros_de_cada_tabua() {
        let tabua1 = criar_tabua_1dt_1();
        let tabua2 = criar_tabua_1dt_plato();
        let tabua_mdt = TabuaMDT::new(vec![tabua1.clone(), tabua2.clone()]);

        let x = vec![2, 1];
        let result = tabua_mdt.tempo_futuro_maximo(&x);

        let result_tabua1 = tabua1.tempo_futuro_maximo(&vec![x[0]]);
        let result_tabua2 = tabua2.tempo_futuro_maximo(&vec![x[1]]);

        assert_eq!(result, cmp::min(result_tabua1, result_tabua2));
    }

    #[test]
    #[should_panic(expected = "O vetor de idades é incompatível")]
    fn tempo_futuro_max_falha_quando_x_nao_tem_tamanho_correto() {
        let tabua = criar_tabua_1dt_1();
        let tabua_mdt = TabuaMDT::new(vec![tabua]);

        let x = vec![2, 1, 3];

        tabua_mdt.tempo_futuro_maximo(&x);
    }
}
