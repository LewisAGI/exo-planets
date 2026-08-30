use approx::assert_relative_eq;
use exo_planets::constants::{AU_M, HEK_I_MAXDEV_PREFACTOR_MIN, MJUP_OVER_MEARTH, YEAR_DAYS};
use exo_planets::tdv::{eta_velocity, moon_period_from_eta, predict_tdv, MoonSense};
use exo_planets::ttv::{hek_i_maxdev_minutes, moon_period_days, predict_ttv};

#[test]
fn ttv_scales_with_ms_and_as_when_moon_light() {
    let base = predict_ttv(10.0, 0.09, MJUP_OVER_MEARTH, 1.0, 1.0, 0.25);
    let twice_ms = predict_ttv(10.0, 0.09, MJUP_OVER_MEARTH, 1.0, 2.0, 0.25);
    let twice_d = predict_ttv(10.0, 0.09, MJUP_OVER_MEARTH, 1.0, 1.0, 0.50);
    // Finite-mass barycenter: a_W ∝ MS/(MP+MS), not exactly MS.
    let mp = MJUP_OVER_MEARTH;
    let exact_ms = (2.0 * (mp + 1.0)) / (mp + 2.0);
    assert_relative_eq!(
        twice_ms.dttv_rms_min / base.dttv_rms_min,
        exact_ms,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        twice_d.dttv_rms_min / base.dttv_rms_min,
        2.0,
        epsilon = 1e-9
    );
    assert!(base.dttv_rms_min > 0.0);
}

#[test]
fn hek_i_maxdev_unit_case_is_36_minutes() {
    // D=1, MS=1 M⊕, PB=1 yr, MP=1 MJ, M*=1 M☉ → 36 min
    let dt = hek_i_maxdev_minutes(1.0, 1.0, YEAR_DAYS, MJUP_OVER_MEARTH, 1.0);
    assert_relative_eq!(dt, HEK_I_MAXDEV_PREFACTOR_MIN, epsilon = 1e-9);
}

#[test]
fn moon_period_at_hill_sphere_is_pb_over_sqrt3() {
    let pb = 300.0;
    assert_relative_eq!(
        moon_period_days(pb, 1.0),
        pb / 3.0_f64.sqrt(),
        epsilon = 1e-12
    );
    assert!(moon_period_days(pb, 0.4895) < moon_period_days(pb, 1.0));
}

#[test]
fn tdv_v_over_ttv_matches_2pi_t_over_ps() {
    let period = 20.0;
    let d = 0.3;
    let ttv = predict_ttv(period, 0.15, MJUP_OVER_MEARTH, 1.0, 2.0, d);
    let t14_hr = 5.0;
    let tdv = predict_tdv(
        &ttv,
        t14_hr,
        period,
        0.15 * AU_M,
        exo_planets::constants::RSUN_M,
        0.1,
        0.2,
        Some(0.4),
        MoonSense::Prograde,
    );
    let eta_closed = eta_velocity(t14_hr, ttv.moon_period_days);
    assert_relative_eq!(tdv.eta_v, eta_closed, epsilon = 1e-6);
    let ps_back = moon_period_from_eta(t14_hr, tdv.eta_v).unwrap();
    assert_relative_eq!(ps_back, ttv.moon_period_days, epsilon = 1e-6);
}

#[test]
fn tdv_v_scales_as_ms_over_sqrt_as() {
    // a_S ∝ D, so doubling D multiplies TDV-V by 1/sqrt(2) at fixed MS
    // when M_S << M_P (a_W ∝ MS a_S, v_W ∝ MS a_S^{-1/2}).
    let a = 0.15;
    let t1 = predict_ttv(20.0, a, MJUP_OVER_MEARTH, 1.0, 2.0, 0.20);
    let t2 = predict_ttv(20.0, a, MJUP_OVER_MEARTH, 1.0, 2.0, 0.40);
    let d1 = predict_tdv(
        &t1,
        5.0,
        20.0,
        a * AU_M,
        exo_planets::constants::RSUN_M,
        0.1,
        0.0,
        None,
        MoonSense::Prograde,
    );
    let d2 = predict_tdv(
        &t2,
        5.0,
        20.0,
        a * AU_M,
        exo_planets::constants::RSUN_M,
        0.1,
        0.0,
        None,
        MoonSense::Prograde,
    );
    // b=0 ⇒ TIP=0, combined = V
    let ratio = d2.tdv_v_rms_min / d1.tdv_v_rms_min;
    assert_relative_eq!(ratio, (0.20_f64 / 0.40).sqrt(), epsilon = 0.02);
}

#[test]
fn tip_adds_prograde_subtracts_retrograde() {
    let ttv = predict_ttv(20.0, 0.15, MJUP_OVER_MEARTH, 1.0, 5.0, 0.3);
    let pro = predict_tdv(
        &ttv,
        6.0,
        20.0,
        0.15 * AU_M,
        exo_planets::constants::RSUN_M,
        0.1,
        0.5,
        None,
        MoonSense::Prograde,
    );
    let ret = predict_tdv(
        &ttv,
        6.0,
        20.0,
        0.15 * AU_M,
        exo_planets::constants::RSUN_M,
        0.1,
        0.5,
        None,
        MoonSense::Retrograde,
    );
    assert!(pro.tdv_tip_rms_min > 0.0);
    assert_relative_eq!(
        pro.tdv_combined_rms_min,
        pro.tdv_v_rms_min + pro.tdv_tip_rms_min,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        ret.tdv_combined_rms_min,
        (ret.tdv_v_rms_min - ret.tdv_tip_rms_min).abs(),
        epsilon = 1e-9
    );
}
