use crate::{interface::JurosInterface, Periodicidade};

pub struct JurosConstante {
    taxa: f64,
    periodicidade: Periodicidade,
}

impl JurosConstante {
    pub fn new(taxa: f64, periodicidade: Periodicidade) -> JurosConstante {
        return JurosConstante {
            taxa,
            periodicidade,
        };
    }
}

impl JurosInterface for JurosConstante {
    fn periodicidade(&self) -> &crate::Periodicidade {
        return &self.periodicidade;
    }

    fn taxa_juros(&self, t: u16) -> f64 {
        let _ = t;
        return self.taxa;
    }

    fn taxa_desconto(&self, t: u16) -> f64 {
        return (1.0 + self.taxa_juros(t)).powi(-(t as i32));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Periodicidade;
    use approx;

    #[test]
    fn test_periodicidade() {
        let juros = JurosConstante::new(0.1, Periodicidade::Mensal);
        assert_eq!(juros.periodicidade(), &Periodicidade::Mensal);
    }

    #[test]
    fn test_taxa_juros() {
        let juros = JurosConstante::new(0.1, Periodicidade::Mensal);
        assert_eq!(juros.taxa_juros(0), 0.1);
        assert_eq!(juros.taxa_juros(1), 0.1);
        assert_eq!(juros.taxa_juros(2), 0.1);
    }

    #[test]
    fn test_taxa_desconto() {
        let juros = JurosConstante::new(0.1, Periodicidade::Mensal);
        approx::assert_relative_eq!(juros.taxa_desconto(0), ((1.0 + 0.1) as f64).powi(0));
        approx::assert_relative_eq!(juros.taxa_desconto(1), ((1.0 + 0.1) as f64).powi(-1));
        approx::assert_relative_eq!(juros.taxa_desconto(2), ((1.0 + 0.1) as f64).powi(-2));
    }
}
