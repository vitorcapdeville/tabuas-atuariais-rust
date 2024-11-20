// use infinitable::Finite;
use tabatu::Tabua;
// use tabatu::{Tabua, TabuaBiometrica, TabuaMDT};

fn main() {
    let tabua = Tabua::new(vec![0.0, 0.1, 0.5, 0.8, 1.0]);
    dbg!(&tabua);
    // dbg!(&tabua.possui_fechamento_plato());
    // dbg!(&tabua.t_qx(2, Finite(1)));

    // let tabua_mdt = TabuaMDT::new(vec![tabua]);
    // dbg!(&tabua_mdt);

    // let idades = vec![20, 30];
    // let tabuas = vec![Tabua::new(vec![0.01, 0.02]), Tabua::new(vec![0.03, 0.04]), Tabua::new(vec![0.05, 0.06])];

    // for (idade, tabua) in idades.iter().zip(tabuas.iter()) {
    //     println!("Idade: {}, Tabua: {:?}", idade, tabua);
    // }
}
