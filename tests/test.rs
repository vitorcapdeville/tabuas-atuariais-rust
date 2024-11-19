mod tests {
    use approx::assert_abs_diff_eq;
    use infinitable::Infinitable::{Finite, Infinity};
    use tabatu::*;

    fn criar_tabua_plato() -> TabuaBase {
        TabuaBase::new(vec![0.1, 0.3, 0.5, 0.7, 0.9])
    }

    fn criar_tabua_completa() -> TabuaBase {
        TabuaBase::new(vec![0.1, 0.2, 0.4, 0.8, 1.0])
    }

    #[test]
    fn quando_idade_maxima_eh_infinita_tempo_futuro_max_eh_infinito() {
        let tabua = criar_tabua_plato();

        assert_eq!(tabua.tempo_futuro_maximo(0), Infinity);
        assert_eq!(tabua.tempo_futuro_maximo(15), Infinity);
        assert_eq!(tabua.tempo_futuro_maximo(100), Infinity);
    }

    #[test]
    fn quando_idade_maxima_nao_eh_infinita_tempo_futuro_max_eh_finito() {
        let tabua = criar_tabua_completa();

        assert_eq!(tabua.tempo_futuro_maximo(0), Finite(5));
        assert_eq!(tabua.tempo_futuro_maximo(3), Finite(2));
        assert_eq!(tabua.tempo_futuro_maximo(100), Finite(0));
    }

    #[test]
    fn qx_eh_igual_a_1_quando_x_mais_t_for_maior_ou_igual_ao_tempo_futuro_max_e_a_tabua_eh_completa(
    ) {
        let tabua = criar_tabua_completa();

        assert_eq!(tabua.qx(0, Finite(10)), 1.0);
        assert_eq!(tabua.qx(0, Finite(50)), 1.0);
        assert_eq!(tabua.qx(0, Finite(100)), 1.0);
    }

    #[test]
    fn qx_eh_igual_ao_ultimo_qx_quando_x_mais_t_for_maior_ou_igual_ao_tempo_futuro_max_e_a_tabua_eh_plato(
    ) {
        let tabua = criar_tabua_plato();

        assert_eq!(tabua.qx(0, Finite(10)), 0.9);
        assert_eq!(tabua.qx(0, Finite(50)), 0.9);
        assert_eq!(tabua.qx(0, Finite(100)), 0.9);
    }

    #[test]
    fn tpx_eh_igual_a_1_quando_t_for_igual_a_0_plato() {
        let tabua = criar_tabua_plato();

        assert_eq!(tabua.tpx(0, Finite(0)), 1.0);
        assert_eq!(tabua.tpx(1, Finite(0)), 1.0);
        assert_eq!(tabua.tpx(2, Finite(0)), 1.0);
    }

    #[test]
    fn tpx_eh_igual_a_1_quando_t_for_igual_a_0_completo() {
        let tabua = criar_tabua_completa();

        assert_eq!(tabua.tpx(0, Finite(0)), 1.0);
        assert_eq!(tabua.tpx(1, Finite(0)), 1.0);
        assert_eq!(tabua.tpx(2, Finite(0)), 1.0);
    }

    #[test]
    fn tpx_eh_igual_a_0_quando_t_for_maior_ou_igual_ao_tempo_futuro_max_e_a_tabua_completa() {
        let tabua = criar_tabua_completa();
        let tempo_futuro_max = tabua.tempo_futuro_maximo(3);

        assert_eq!(tabua.tpx(3, tempo_futuro_max + Finite(0)), 0.0);
        assert_eq!(tabua.tpx(3, tempo_futuro_max + Finite(1)), 0.0);
        assert_eq!(tabua.tpx(3, tempo_futuro_max + Finite(2)), 0.0);
    }

    #[test]
    fn tpx_eh_igual_a_zero_quando_x_maior_ou_igual_ao_tempo_futuro_max_t_maior_que_zero_e_tabua_completa(
    ) {
        let tabua = criar_tabua_completa();
        let x = tabua.tempo_futuro_maximo(0) + Finite(1);
        match x {
            Finite(x) => {
                assert_eq!(tabua.tpx(x, Finite(1)), 0.0);
                assert_eq!(tabua.tpx(x, Finite(2)), 0.0);
                assert_eq!(tabua.tpx(x, Finite(3)), 0.0);
            }
            _ => panic!("x deveria ser finito numa tabua completa"),
        }
    }

    #[test]
    fn tpx_termina_com_zero_plato() {
        let tabua = criar_tabua_plato();
        let x = 2;
        let t = Finite(100);

        assert_abs_diff_eq!(tabua.tpx(x, t), 0.0);
    }

    #[test]
    fn tpx_termina_com_zero_completa() {
        let tabua = criar_tabua_completa();
        let x = 2;
        let t = tabua.tempo_futuro_maximo(0);

        assert_eq!(tabua.tpx(x, t), 0.0);
    }

    #[test]
    fn t_qx_soma_1_quando_t_sao_todos_os_tempos_futuros_plato() {
        let tabua = criar_tabua_plato();
        let x = 2;
        let limite = 100;
        let t = (0..=limite).map(|i| Finite(i)).collect::<Vec<_>>();

        let result = t.iter().map(|&t| tabua.t_qx(x, t)).sum::<f64>();

        assert_abs_diff_eq!(result, 1.0);
    }

    #[test]
    fn t_qx_soma_1_quando_t_sao_todos_os_tempos_futuros_completa() {
        let tabua = criar_tabua_completa();
        let x = 2;
        let limite = tabua
            .tempo_futuro_maximo(x)
            .finite()
            .expect("Tempo futuro maximo de uma tabua completa deveria ser finito!");
        let t = (0..=limite).map(|i| Finite(i)).collect::<Vec<_>>();

        let result = t.iter().map(|&t| tabua.t_qx(x, t)).sum::<f64>();

        assert_abs_diff_eq!(result, 1.0);
    }

    #[test]
    fn t_qx_eh_igual_a_1_quando_t_eh_zero_e_x_eh_superior_ao_tempo_futuro_maximo_e_tabua_completa()
    {
        let tabua = criar_tabua_completa();
        let x = tabua
            .tempo_futuro_maximo(0)
            .finite()
            .expect("Tempo futuro maximo de uma tabua completa deveria ser finito!");

        assert_abs_diff_eq!(tabua.t_qx(x - 2, Finite(0)), 0.8);
        assert_abs_diff_eq!(tabua.t_qx(x, Finite(0)), 1.0);
        assert_abs_diff_eq!(tabua.t_qx(x + 2, Finite(0)), 1.0);
    }

    #[test]
    fn t_qx_eh_igual_a_0_quando_x_eh_superior_ao_tempo_futuro_max_e_t_eh_maior_que_zero_e_tabua_completa(
    ) {
        let tabua = criar_tabua_completa();
        let x = tabua
            .tempo_futuro_maximo(0)
            .finite()
            .expect("Tempo futuro maximo de uma tabua completa deveria ser finito!")
            + 1;

        assert_eq!(tabua.t_qx(x, Finite(1)), 0.0);
        assert_eq!(tabua.t_qx(x, Finite(2)), 0.0);
        assert_eq!(tabua.t_qx(x, Finite(3)), 0.0);
    }
}
