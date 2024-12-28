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

pub fn agravar_qx(qx: Vec<f64>, fator: f64) -> Vec<f64> {
    if fator == 0.0 {
        return qx;
    } else if fator < 0.0 {
        panic!("O fator deve ser maior que ou igual a zero.");
    }
    return qx
        .iter()
        .map(|v| if *v == 1.0 { *v } else { (v * fator).min(1.0) })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aumentar_periodicidade_funciona() {
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
    fn reduzir_periodicidade_funciona() {
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
    fn alterar_periodicidade_funciona_quando_aumenta() {
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
    fn alterar_periodicidade_funciona_quando_reduz() {
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

    #[test]
    #[should_panic]
    fn alterar_periodicidade_panic_quando_nova_periodicidade_nao_e_multiplo_da_atual() {
        let qx_original = vec![0.1, 0.5, 0.7, 1.0];
        alterar_periodicidade(qx_original, 2, 3);
    }

    #[test]
    fn agravar_qx_nao_gera_qx_com_valores_acima_de_1() {
        let qx_original = vec![0.1, 0.5, 0.7, 1.0];
        let qx_esperado = vec![0.2, 1.0, 1.0, 1.0];

        let qx_obtido = agravar_qx(qx_original, 2.0);
        assert_eq!(qx_esperado, qx_obtido);
    }

    #[test]
    fn agravar_qx_nao_desagrava_qx_que_sao_1() {
        let qx_original = vec![0.1, 0.5, 0.7, 1.0];
        let qx_esperado = vec![0.1 * 0.8, 0.5 * 0.8, 0.7 * 0.8, 1.0];

        let qx_obtido = agravar_qx(qx_original, 0.8);
        assert_eq!(qx_esperado, qx_obtido);
    }

    #[test]
    #[should_panic]
    fn agravar_qx_panic_quando_fator_negativo() {
        let qx_original = vec![0.1, 0.5, 0.7, 1.0];
        agravar_qx(qx_original, -1.0);
    }
}
