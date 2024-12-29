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

    fn alterar_periodicidade(&self, nova_periodicidade: Periodicidade) -> Self {
        let nova_taxa = (1.0 + self.taxa).powf(
            self.periodicidade
                .quantidade_periodos_1_periodicidade(&nova_periodicidade) as f64,
        ) - 1.0;
        return JurosConstante::new(nova_taxa, nova_periodicidade);
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

    #[test]
    fn alterar_periodicidade_funciona() {
        let juros_anual = JurosConstante::new(0.05, Periodicidade::Anual);
        let juros_mensal = juros_anual.alterar_periodicidade(Periodicidade::Mensal);

        assert_eq!(juros_mensal.periodicidade(), &Periodicidade::Mensal);
        approx::assert_relative_eq!(juros_mensal.taxa_juros(2), 0.004074123905313698);
    }
}
