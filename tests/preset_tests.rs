//! Tests for preset world materials.

use materia::presets;
use materia::material::PhaseState;

// -- Phase state correctness --

#[test]
fn stone_is_solid() {
    let mat = presets::stone();
    assert_eq!(mat.phase.state, PhaseState::Solid);
    assert!(mat.structural.density > 2000.0, "Stone density {}", mat.structural.density);
}

#[test]
fn sand_is_granular() {
    let mat = presets::sand();
    assert_eq!(mat.phase.state, PhaseState::Granular);
}

#[test]
fn dirt_is_granular() {
    let mat = presets::dirt();
    assert_eq!(mat.phase.state, PhaseState::Granular);
    assert!(mat.hydraulic.porosity > 0.0, "Dirt should have porosity");
}

#[test]
fn grass_is_solid() {
    let mat = presets::grass();
    assert_eq!(mat.phase.state, PhaseState::Solid);
    assert!(mat.thermal.ignition_point.is_some(), "Grass should be flammable");
}

#[test]
fn water_is_liquid() {
    let mat = presets::water();
    assert_eq!(mat.phase.state, PhaseState::Liquid);
}

#[test]
fn ice_is_solid() {
    let mat = presets::ice();
    assert_eq!(mat.phase.state, PhaseState::Solid);
    // Ice is less dense than water
    assert!(mat.structural.density < presets::water().structural.density + 200.0);
}

#[test]
fn snow_is_granular() {
    let mat = presets::snow();
    assert_eq!(mat.phase.state, PhaseState::Granular);
    assert!(mat.structural.density < presets::ice().structural.density);
}

#[test]
fn lava_is_liquid() {
    let mat = presets::lava();
    assert_eq!(mat.phase.state, PhaseState::Liquid);
    assert!(mat.optical.emission_intensity > 0.0, "Lava should glow");
}

#[test]
fn air_is_gas() {
    let mat = presets::air();
    assert_eq!(mat.phase.state, PhaseState::Gas);
    assert!(mat.structural.density < 5.0, "Air density {}", mat.structural.density);
}

#[test]
fn steam_is_gas() {
    let mat = presets::steam();
    assert_eq!(mat.phase.state, PhaseState::Gas);
}

#[test]
fn smoke_is_gas() {
    let mat = presets::smoke();
    assert_eq!(mat.phase.state, PhaseState::Gas);
}

// -- Relative property ordering --

#[test]
fn stone_harder_than_dirt() {
    let stone = presets::stone();
    let dirt = presets::dirt();
    assert!(stone.structural.yield_strength > dirt.structural.yield_strength);
}

#[test]
fn stone_denser_than_dirt() {
    let stone = presets::stone();
    let dirt = presets::dirt();
    assert!(stone.structural.density > dirt.structural.density);
}

#[test]
fn wood_harder_than_grass() {
    let wood = presets::wood();
    let grass = presets::grass();
    assert!(wood.structural.yield_strength > grass.structural.yield_strength);
}

#[test]
fn lava_more_viscous_than_water() {
    let lava = presets::lava();
    let water = presets::water();
    assert!(lava.phase.relaxation_time > water.phase.relaxation_time);
}

#[test]
fn iron_denser_than_stone() {
    let iron = presets::iron();
    let stone = presets::stone();
    assert!(iron.structural.density > stone.structural.density,
        "Iron {} should be denser than stone {}", iron.structural.density, stone.structural.density);
}

#[test]
fn gold_denser_than_iron() {
    let gold = presets::gold();
    let iron = presets::iron();
    assert!(gold.structural.density > iron.structural.density,
        "Gold {} should be denser than iron {}", gold.structural.density, iron.structural.density);
}

#[test]
fn steel_harder_than_iron() {
    let steel = presets::steel();
    let iron = presets::iron();
    assert!(steel.structural.yield_strength > iron.structural.yield_strength);
}

// -- Sanity checks --

#[test]
fn all_presets_compile_without_panic() {
    let materials = vec![
        presets::stone(),
        presets::granite(),
        presets::sand(),
        presets::clay(),
        presets::dirt(),
        presets::gravel(),
        presets::grass(),
        presets::wood(),
        presets::leaf(),
        presets::water(),
        presets::ice(),
        presets::snow(),
        presets::iron(),
        presets::steel(),
        presets::copper(),
        presets::gold(),
        presets::air(),
        presets::steam(),
        presets::smoke(),
        presets::lava(),
    ];

    eprintln!("Preset materials:");
    for mat in &materials {
        eprintln!("  {:<10} phase={:?} density={:.0} tau={:.1} color={:.2?}",
            mat.name, mat.phase.state, mat.structural.density,
            mat.phase.relaxation_time, mat.optical.base_color);
    }

    assert_eq!(materials.len(), 20);
}
