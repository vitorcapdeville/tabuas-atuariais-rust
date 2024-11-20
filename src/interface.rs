use infinitable::Infinitable;

pub fn validar_idades_tabuas(x: &Vec<u16>, numero_decrementos: usize, numero_vidas: usize) {
    if x.len() != (numero_decrementos * numero_vidas) {
        panic!("O vetor de idades é incompatível com o número de decrementos/vidas");
    }
}

pub trait TabuaBiometrica {
    fn numero_decrementos(&self) -> usize;
    fn numero_vidas(&self) -> usize;
    fn tempo_futuro_maximo(&self, x: &Vec<u16>) -> Infinitable<u16>;
    fn qx(&self, x: &Vec<u16>, t: Infinitable<u16>) -> f64;
    fn tpx(&self, x: &Vec<u16>, t: Infinitable<u16>) -> f64;
    fn t_qx(&self, x: &Vec<u16>, t: Infinitable<u16>) -> f64 {
        return self.qx(x, t) * self.tpx(x, t);
    }
}
