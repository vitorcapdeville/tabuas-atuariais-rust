fn aumentar_periodicidade(x: Vec<f64>, fator: usize) -> Vec<f64> {
    return x
        .into_iter()
        .flat_map(|v| std::iter::repeat(v).take(fator))
        .map(|v| alterar_qx(v, fator as f64))
        .collect();
}

fn reduzir_periodicidade(x: Vec<f64>, fator: usize) -> Vec<f64> {
    x.into_iter()
        .step_by(fator)
        .map(|v| alterar_qx(v, 1.0 / (fator as f64)))
        .collect()
}

fn alterar_qx(qx: f64, fator: f64) -> f64 {
    return 1.0 - (1.0 - qx).powf(fator);
}

pub fn alterar_periodicidade(
    qx: Vec<f64>,
    periodicidade_atual: usize,
    nova_periodicidade: usize,
) -> Vec<f64> {
    if nova_periodicidade == periodicidade_atual {
        return qx;
    } else if nova_periodicidade > periodicidade_atual {
        let fator = nova_periodicidade as f64 / periodicidade_atual as f64;
        if fator != (fator as usize) as f64 {
            panic!("A nova periodicidade deve ser múltiplo da atual.");
        }
        return aumentar_periodicidade(qx, fator as usize);
    } else {
        let fator = periodicidade_atual as f64 / nova_periodicidade as f64;
        if fator != (fator as usize) as f64 {
            panic!("A nova periodicidade deve ser múltiplo da atual.");
        }
        return reduzir_periodicidade(qx, fator as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aumentar_periodicidade() {
        let qx_original = vec![0.1, 0.5, 0.7, 1.0];
        let qx_esperado = vec![
            0.18999999999999995,
            0.18999999999999995,
            0.75,
            0.75,
            0.9099999999999999,
            0.9099999999999999,
            1.0,
            1.0,
        ];

        let qx_obtido = aumentar_periodicidade(qx_original, 2);
        assert_eq!(qx_esperado, qx_obtido);
    }

    #[test]
    fn test_reduzir_periodicidade() {
        let qx_original = vec![
            0.18999999999999995,
            0.18999999999999995,
            0.75,
            0.75,
            0.9099999999999999,
            0.9099999999999999,
            1.0,
            1.0,
        ];
        let qx_esperado = vec![0.1, 0.5, 0.7, 1.0];

        let qx_obtido = reduzir_periodicidade(qx_original, 2);

        assert_eq!(
            qx_esperado,
            qx_obtido
                .iter()
                .map(|v| (v * 1000.0).round() / 1000.0)
                .collect::<Vec<f64>>()
        );
    }

    #[test]
    fn test_alterar_periodicidade_funciona_quando_aumenta() {
        let qx_original = vec![0.1, 0.5, 0.7, 1.0];
        let qx_esperado = vec![
            0.18999999999999995,
            0.18999999999999995,
            0.75,
            0.75,
            0.9099999999999999,
            0.9099999999999999,
            1.0,
            1.0,
        ];

        let qx_obtido = alterar_periodicidade(qx_original, 1, 2);
        assert_eq!(qx_esperado, qx_obtido);
    }

    #[test]
    fn test_alterar_periodicidade_funciona_quando_reduz() {
        let qx_original = vec![
            0.18999999999999995,
            0.18999999999999995,
            0.75,
            0.75,
            0.9099999999999999,
            0.9099999999999999,
            1.0,
            1.0,
        ];
        let qx_esperado = vec![0.1, 0.5, 0.7, 1.0];

        let qx_obtido = alterar_periodicidade(qx_original, 2, 1);
        assert_eq!(
            qx_esperado,
            qx_obtido
                .iter()
                .map(|v| (v * 1000.0).round() / 1000.0)
                .collect::<Vec<f64>>()
        );
    }
}
