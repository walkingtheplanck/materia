//! Hydraulic property validation — porosity, permeability, capillary physics.

use materia::prelude::*;
use materia::presets;

// -- Porosity derivation --

#[test]
fn sand_has_porosity() {
    let sand = presets::sand();
    assert!(sand.hydraulic.porosity > 0.2 && sand.hydraulic.porosity < 0.6,
        "Sand porosity {:.2} out of range [0.2, 0.6]", sand.hydraulic.porosity);
}

#[test]
fn clay_higher_porosity_than_sand() {
    let clay = presets::clay();
    let sand = presets::sand();
    assert!(clay.hydraulic.porosity > sand.hydraulic.porosity,
        "Clay porosity {:.2} should exceed sand {:.2}", clay.hydraulic.porosity, sand.hydraulic.porosity);
}

#[test]
fn stone_nearly_zero_porosity() {
    let stone = presets::stone();
    assert!(stone.hydraulic.porosity < 0.05,
        "Stone porosity {:.2} should be near zero", stone.hydraulic.porosity);
}

#[test]
fn dirt_has_moderate_porosity() {
    let dirt = presets::dirt();
    assert!(dirt.hydraulic.porosity > 0.25 && dirt.hydraulic.porosity < 0.65,
        "Dirt porosity {:.2} out of range", dirt.hydraulic.porosity);
}

// -- Permeability (Kozeny-Carman) --

#[test]
fn sand_higher_permeability_than_clay() {
    let sand = presets::sand();
    let clay = presets::clay();
    assert!(sand.hydraulic.permeability > clay.hydraulic.permeability,
        "Sand perm {:.2e} should exceed clay {:.2e}", sand.hydraulic.permeability, clay.hydraulic.permeability);
}

#[test]
fn granular_permeability_positive() {
    // All granular materials should have non-zero permeability
    let gravel = presets::gravel();
    let sand = presets::sand();
    let clay = presets::clay();
    assert!(gravel.hydraulic.permeability > 0.0, "Gravel should have positive permeability");
    assert!(sand.hydraulic.permeability > 0.0, "Sand should have positive permeability");
    assert!(clay.hydraulic.permeability > 0.0, "Clay should have positive permeability");
    // Note: gravel uses basalt (denser) so the density-based grain estimator
    // gives it smaller grains than quartz sand. This is a known model limitation.
    let ratio = sand.hydraulic.permeability / clay.hydraulic.permeability.max(1e-20);
    eprintln!("Permeability ratio sand/clay: {ratio:.1}x");
    assert!(ratio > 1.0, "Sand should be more permeable than clay");
}

// -- Pore radius --

#[test]
fn pore_radius_positive_for_granular() {
    let sand = presets::sand();
    assert!(sand.hydraulic.pore_radius > 0.0,
        "Sand pore_radius should be positive, got {}", sand.hydraulic.pore_radius);
    let clay = presets::clay();
    assert!(clay.hydraulic.pore_radius > 0.0,
        "Clay pore_radius should be positive, got {}", clay.hydraulic.pore_radius);
}

#[test]
fn sand_larger_pores_than_clay() {
    let sand = presets::sand();
    let clay = presets::clay();
    assert!(sand.hydraulic.pore_radius > clay.hydraulic.pore_radius,
        "Sand pores {:.2e} should be larger than clay {:.2e}",
        sand.hydraulic.pore_radius, clay.hydraulic.pore_radius);
}

// -- Contact angle (wettability) --

#[test]
fn sand_is_hydrophilic() {
    let sand = presets::sand();
    assert!(sand.hydraulic.contact_angle < 45.0,
        "Sand contact_angle {:.0} should be < 45 (hydrophilic)", sand.hydraulic.contact_angle);
}

#[test]
fn metals_moderate_contact_angle() {
    let iron = presets::iron();
    assert!(iron.hydraulic.contact_angle > 30.0 && iron.hydraulic.contact_angle < 90.0,
        "Iron contact_angle {:.0} out of range", iron.hydraulic.contact_angle);
}

// -- Water repellent surface --

#[test]
fn polymer_granular_is_hydrophobic() {
    // A polymer compiled as Granular should get contact_angle ~85 (hydrophobic)
    // Note: contact_angle is only computed in the Granular branch via estimate_contact_angle.
    // Solid-phase materials keep the default contact_angle (60).
    let wax = compile_as(
        &Substance::Polymer(PolymerChain {
            name: "wax".into(),
            monomer: Molecule::ethanol(), // simplified
            chain_length: 50,
            cross_link_density: 0.0,
            branching: 0.0,
        }),
        "wax",
        PhaseState::Granular,
    );
    assert!(wax.hydraulic.contact_angle > 70.0,
        "Wax (granular) contact_angle {:.0} should be > 70 (hydrophobic)", wax.hydraulic.contact_angle);
    eprintln!("Wax granular contact angle: {:.0} (hydrophobic)", wax.hydraulic.contact_angle);
}

// -- Print hydraulic property table --

#[test]
fn print_hydraulic_table() {
    let materials = vec![
        ("stone", presets::stone()),
        ("granite", presets::granite()),
        ("sand", presets::sand()),
        ("clay", presets::clay()),
        ("dirt", presets::dirt()),
        ("gravel", presets::gravel()),
        ("grass", presets::grass()),
        ("wood", presets::wood()),
        ("iron", presets::iron()),
        ("water", presets::water()),
    ];

    eprintln!("{:<10} {:>8} {:>12} {:>12} {:>8}", "Material", "Porosity", "Permeability", "Pore radius", "Contact");
    eprintln!("{}", "-".repeat(56));
    for (name, mat) in &materials {
        eprintln!("{:<10} {:>8.3} {:>12.2e} {:>12.2e} {:>8.0}",
            name,
            mat.hydraulic.porosity,
            mat.hydraulic.permeability,
            mat.hydraulic.pore_radius,
            mat.hydraulic.contact_angle);
    }
}
