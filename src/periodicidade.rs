#[derive(Debug, Clone)]
pub enum Periodicidade {
    Mensal,
    Bimestral,
    Trimestral,
    Quadrimestral,
    Semestral,
    Anual,
}

impl Periodicidade {
    pub fn quantidade_periodos_1_ano(&self) -> u16 {
        match self {
            Periodicidade::Mensal => 12,
            Periodicidade::Bimestral => 6,
            Periodicidade::Trimestral => 4,
            Periodicidade::Quadrimestral => 3,
            Periodicidade::Semestral => 2,
            Periodicidade::Anual => 1,
        }
    }

    pub fn quantidade_periodos_1_periodicidade(&self, periodicidade: &Self) -> f32 {
        return self.quantidade_periodos_1_ano() as f32
            / periodicidade.quantidade_periodos_1_ano() as f32;
    }
}

impl PartialEq for Periodicidade {
    fn eq(&self, other: &Self) -> bool {
        return self.quantidade_periodos_1_ano() == other.quantidade_periodos_1_ano();
    }
}

impl PartialOrd for Periodicidade {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return other
            .quantidade_periodos_1_ano()
            .partial_cmp(&self.quantidade_periodos_1_ano());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quantidade_periodos_1_ano() {
        assert_eq!(Periodicidade::Mensal.quantidade_periodos_1_ano(), 12);
        assert_eq!(Periodicidade::Bimestral.quantidade_periodos_1_ano(), 6);
        assert_eq!(Periodicidade::Trimestral.quantidade_periodos_1_ano(), 4);
        assert_eq!(Periodicidade::Quadrimestral.quantidade_periodos_1_ano(), 3);
        assert_eq!(Periodicidade::Semestral.quantidade_periodos_1_ano(), 2);
        assert_eq!(Periodicidade::Anual.quantidade_periodos_1_ano(), 1);
    }

    #[test]
    fn quantidade_periodos_1_periodicidade() {
        assert_eq!(
            Periodicidade::Mensal.quantidade_periodos_1_periodicidade(&Periodicidade::Mensal),
            1.0
        );
        assert_eq!(
            Periodicidade::Mensal.quantidade_periodos_1_periodicidade(&Periodicidade::Bimestral),
            2.0
        );
        assert_eq!(
            Periodicidade::Mensal.quantidade_periodos_1_periodicidade(&Periodicidade::Trimestral),
            3.0
        );
        assert_eq!(
            Periodicidade::Mensal
                .quantidade_periodos_1_periodicidade(&Periodicidade::Quadrimestral),
            4.0
        );
        assert_eq!(
            Periodicidade::Mensal.quantidade_periodos_1_periodicidade(&Periodicidade::Semestral),
            6.0
        );
        assert_eq!(
            Periodicidade::Mensal.quantidade_periodos_1_periodicidade(&Periodicidade::Anual),
            12.0
        );
    }

    #[test]
    fn periodicidade_eq() {
        assert_eq!(Periodicidade::Mensal, Periodicidade::Mensal);
        assert_ne!(Periodicidade::Mensal, Periodicidade::Bimestral);
    }

    #[test]
    fn periodicidade_partial_ord() {
        assert!(Periodicidade::Mensal < Periodicidade::Bimestral);
        assert!(Periodicidade::Bimestral < Periodicidade::Trimestral);
        assert!(Periodicidade::Trimestral < Periodicidade::Quadrimestral);
        assert!(Periodicidade::Quadrimestral < Periodicidade::Semestral);
        assert!(Periodicidade::Semestral < Periodicidade::Anual);
    }
}
