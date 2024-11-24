pub use crate::interface::TabuaInterface;
pub use crate::periodicidade::Periodicidade;
pub use crate::periodicidade::Periodicidade::{
    Anual, Bimestral, Mensal, Quadrimestral, Semestral, Trimestral,
};
pub use crate::tabua::Tabua;
pub use crate::tabua_mdt::TabuaMDT;
pub use crate::tabua_multiplas_vidas::StatusVidasConjuntas::{First, Last};
pub use crate::tabua_multiplas_vidas::{StatusVidasConjuntas, TabuaMultiplasVidas};

pub mod interface;
pub mod periodicidade;
pub mod tabua;
mod tabua_base;
pub mod tabua_mdt;
pub mod tabua_multiplas_vidas;
