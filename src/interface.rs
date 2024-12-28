use crate::Periodicidade;
use infinitable::Infinitable;

pub fn validar_idades_tabuas(x: &Vec<u16>, numero_decrementos: usize, numero_vidas: usize) {
    if x.len() != (numero_decrementos * numero_vidas) {
        panic!("O vetor de idades é incompatível com o número de decrementos/vidas");
    }
}

pub trait TabuaInterface {
    fn periodicidade(&self) -> &Periodicidade;
    fn numero_decrementos(&self) -> usize;
    fn numero_vidas(&self) -> usize;
    fn tempo_futuro_maximo(&self, x: &Vec<u16>) -> Infinitable<u16>;
    fn qx(&self, x: &Vec<u16>, t: u16) -> f64;
    fn tpx(&self, x: &Vec<u16>, t: u16) -> f64;
    fn t_qx(&self, x: &Vec<u16>, t: u16) -> f64 {
        return self.qx(x, t) * self.tpx(x, t);
    }
    fn alterar_periodicidade(&self, nova_periodicidade: Periodicidade) -> Self;
}

pub trait JurosInterface {
    fn periodicidade(&self) -> &Periodicidade;
    fn taxa_juros(&self, t: u16) -> f64;
    fn taxa_desconto(&self, t: u16) -> f64;
}
