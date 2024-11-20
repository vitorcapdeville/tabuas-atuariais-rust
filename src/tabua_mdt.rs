// use crate::{Tabua, TabuaBiometrica};
// use infinitable::{Infinitable, Infinity};

// fn qx2qxj(qx1: f64, qx2: f64, qx3: f64) -> f64 {
//     return qx1 * (1.0 - 0.5 * (qx2 + qx3) + 1.0 / 3.0 * (qx2 * qx3));
// }

// #[derive(Debug)]
// pub struct TabuaMDT {
//     tabuas: Vec<Tabua>,
//     numero_decrementos: usize,
//     numero_vidas: usize,
// }

// impl TabuaMDT {
//     pub fn new(tabuas: Vec<Tabua>) -> Self {
//         let numero_decrementos = tabuas.len();
//         return TabuaMDT {
//             tabuas,
//             numero_decrementos,
//             numero_vidas: 0,
//         };
//     }
// }

// impl TabuaBiometrica for TabuaMDT {
//     fn tempo_futuro_maximo(&self, x: u16) -> Infinitable<u16> {
//         return Infinity;
//     }
//     fn possui_fechamento_plato(&self) -> bool {
//         return true;
//     }
//     fn qx(&self, x: u16, t: Infinitable<u16>) -> f64 {
//         return 0.0;
//     }
//     fn tpx(&self, x: u16, t: Infinitable<u16>) -> f64 {
//         return self.tabuas.iter().fold(1.0, |acc, tabua| {
//             return acc * tabua.tpx(x, t);
//         });
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use approx;
//     // use infinitable::Infinitable::{Finite, Infinity};

//     #[test]
//     fn calculo_qxj_funciona_com_1_componente() {
//         let qx = 0.4;

//         let result = qx2qxj(qx, 0.0, 0.0);

//         assert_eq!(result, qx);
//     }

//     #[test]
//     fn calculo_qxj_funciona_com_2_componentes() {
//         let qx1 = 0.4;
//         let qx2 = 0.8;
//         let result = qx2qxj(qx1, qx2, 0.0);

//         assert_eq!(result, qx1 * (1.0 - 0.5 * qx2));
//     }

//     #[test]
//     fn calculo_qxj_funciona_com_3_componentes() {
//         let qx1 = 0.4;
//         let qx2 = 0.8;
//         let qx3 = 0.2;

//         let result = qx2qxj(qx1, qx2, qx3);

//         assert_eq!(
//             result,
//             qx1 * (1.0 - 0.5 * (qx2 + qx3) + 1.0 / 3.0 * (qx2 * qx3))
//         );
//     }
// }
