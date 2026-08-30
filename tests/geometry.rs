use approx::assert_relative_eq;
use exo_planets::constants::{AU_M, REARTH_M, RSUN_M};
use exo_planets::geometry::{
    a_from_kepler3, compute_geometry, depth_from_radii, impact_parameter, rp_over_rstar_from_depth,
};

#[test]
fn depth_is_radius_ratio_squared() {
    let rp = 11.2 * REARTH_M;
    let rs = 1.0 * RSUN_M;
    let d = depth_from_radii(rp, rs);
    assert_relative_eq!(d, (rp / rs).powi(2), epsilon = 1e-12);
    assert_relative_eq!(rp_over_rstar_from_depth(d), rp / rs, epsilon = 1e-12);
}

#[test]
fn impact_parameter_edge_on_is_zero() {
    let a = 0.1 * AU_M;
    let rs = RSUN_M;
    assert_relative_eq!(impact_parameter(a, rs, 90.0), 0.0, epsilon = 1e-12);
    let b = impact_parameter(a, rs, 89.0);
    assert!(b > 0.0);
    assert_relative_eq!(b, a * 89.0_f64.to_radians().cos() / rs, epsilon = 1e-10);
}

#[test]
fn kepler3_earth_is_about_one_au() {
    let p = 365.25 * 86400.0;
    let a = a_from_kepler3(p, exo_planets::constants::MSUN_KG);
    assert_relative_eq!(a / AU_M, 1.0, epsilon = 0.01);
}

#[test]
fn catalog_geometry_does_not_invent_mass() {
    let g = compute_geometry(
        10.0,
        Some(2.0),
        Some(1.0),
        Some(0.09),
        Some(1.0),
        Some(89.5),
        Some(0.2),
        Some(4.0),
        Some(400.0),
    );
    assert!(g.depth > 0.0);
    assert!(g.t14_hr.is_some());
    assert!(!g.a_from_kepler3);
    let derived = compute_geometry(
        10.0,
        Some(2.0),
        Some(1.0),
        None,
        Some(1.0),
        Some(89.5),
        None,
        None,
        None,
    );
    assert!(derived.a_from_kepler3);
    assert!(derived.a_over_rstar.is_some());
}
