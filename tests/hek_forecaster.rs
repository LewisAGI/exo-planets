use exo_planets::constants::{
    DMAX_PROGRADE, DMAX_RETROGRADE, FORECASTER_TERRAN_NEPTUNIAN_MEARTH, HEK_VI_ETA_95_UPPER,
    HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION,
};
use exo_planets::forecaster::{assign_prior, class_from_mass_earth, ForecasterClass};
use exo_planets::hek::evaluate_hek;
use exo_planets::tdv::MoonSense;

#[test]
fn domingos_dmax_locked() {
    assert_eq!(DMAX_PROGRADE, 0.4895);
    assert_eq!(DMAX_RETROGRADE, 0.9309);
}

#[test]
fn hek_flags_mark_nulls_and_photometry_caution() {
    let f = evaluate_hek(
        0.3,
        100.0,
        1.0,
        MoonSense::Prograde,
        20.0,
        2.0,
        10,
        0.5,
        true,
    );
    assert!(f.hek_ii_to_v_are_null);
    assert!(f.photometry_only_caution);
    assert_eq!(
        f.hek_v_false_claim_fraction,
        HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION
    );
    assert!(f.eta_above_hek_vi_stack);
    assert!(f.d_within_prograde_dmax);
    assert!(f.bayes_proxy.method.contains("not LUNA"));
    let unstable = evaluate_hek(
        0.8,
        100.0,
        1.0,
        MoonSense::Prograde,
        1.0,
        2.0,
        10,
        HEK_VI_ETA_95_UPPER * 0.5,
        false,
    );
    assert!(!unstable.d_within_prograde_dmax);
    assert!(unstable.d_within_retrograde_dmax);
    assert!(!unstable.eta_above_hek_vi_stack);
}

#[test]
fn forecaster_mass_classes() {
    assert_eq!(class_from_mass_earth(1.0), ForecasterClass::Terran);
    assert_eq!(
        class_from_mass_earth(FORECASTER_TERRAN_NEPTUNIAN_MEARTH),
        ForecasterClass::Neptunian
    );
    assert_eq!(class_from_mass_earth(200.0), ForecasterClass::Jovian);
}

#[test]
fn missing_or_limit_mass_is_labelled_extrapolation() {
    let p = assign_prior(None, false, Some(11.0));
    assert!(p.from_radius_extrapolation);
    assert!(p.hill_mass_is_class_default);
    assert_eq!(p.class, ForecasterClass::Jovian);
    let lim = assign_prior(Some(1462.0), true, Some(10.0));
    assert!(lim.mass_is_upper_limit);
    assert!(lim.from_radius_extrapolation);
}
