//! Preset world materials — chemical definitions only.
//!
//! Each preset defines ONLY the chemical substance and physical form.
//! ALL properties (density, color, strength, thermal, optical) are
//! derived by `compile()`. If a compiled value is wrong, fix the
//! estimation model in `compile.rs`, not here.
//!
//! The only override allowed is `PhaseState` — because the same
//! substance can exist in different physical forms (quartz crystal
//! vs sand grains, water vs ice vs snow).

use crate::material::{MaterialDef, PhaseState};

use crate::crystal::{Crystal, LatticeType};
use crate::element::Element;
use crate::molecular::Molecule;
use crate::polymer::PolymerChain;
use crate::substance::Substance;

/// Compile a substance with full phase-state-aware property adjustment.
fn compile_as(substance: &Substance, name: &str, phase: PhaseState) -> MaterialDef {
    crate::compile::compile_as(substance, name, phase)
}

// =========================================================================
// GEOLOGICAL
// =========================================================================

/// Basalt composition — the most common volcanic rock.
fn basalt_substance() -> Substance {
    Substance::amorphous(
        "basalt",
        &[
            (Element::Si, 0.23), (Element::O, 0.44), (Element::Al, 0.08),
            (Element::Fe, 0.08), (Element::Mg, 0.05), (Element::Ca, 0.07),
            (Element::Na, 0.02), (Element::K, 0.01), (Element::Ti, 0.02),
        ],
        2900.0,
    )
}

/// Granite composition — felsic igneous rock.
fn granite_substance() -> Substance {
    Substance::amorphous(
        "granite",
        &[
            (Element::Si, 0.33), (Element::O, 0.44), (Element::Al, 0.07),
            (Element::K, 0.04), (Element::Na, 0.03), (Element::Ca, 0.02),
            (Element::Fe, 0.03), (Element::Mg, 0.01),
        ],
        2650.0,
    )
}

/// Quartz substance — SiO2 tetrahedral crystal.
fn quartz_substance() -> Substance {
    Substance::Crystalline(Crystal {
        name: "quartz".into(),
        lattice: LatticeType::Diamond,
        base: Element::Si,
        solutes: vec![(Element::O, 0.53)],
    })
}

/// Soil composition — mineral + organic mixture.
fn soil_substance() -> Substance {
    Substance::amorphous(
        "soil",
        &[
            (Element::Si, 0.28), (Element::O, 0.42), (Element::Al, 0.07),
            (Element::Fe, 0.04), (Element::C, 0.05), (Element::Ca, 0.03),
            (Element::Mg, 0.02), (Element::K, 0.01), (Element::N, 0.005),
            (Element::H, 0.03), (Element::P, 0.001),
        ],
        1500.0,
    )
}

/// Kaolinite (clay mineral) composition.
fn clay_substance() -> Substance {
    Substance::amorphous(
        "kaolinite",
        &[
            (Element::Al, 0.21), (Element::Si, 0.22),
            (Element::O, 0.50), (Element::H, 0.02),
        ],
        2600.0,
    )
}

/// Cellulose monomer: C6H10O5 — structural polymer of plants.
fn cellulose_monomer() -> Molecule {
    use crate::molecular::{Bond, BondKind};

    let mut atoms = Vec::new();
    for _ in 0..6 { atoms.push(Element::C); }
    for _ in 0..10 { atoms.push(Element::H); }
    for _ in 0..5 { atoms.push(Element::O); }

    let cc = BondKind::typical_energy(Element::C, Element::C, BondKind::Single);
    let ch = BondKind::typical_energy(Element::C, Element::H, BondKind::Single);
    let co = BondKind::typical_energy(Element::C, Element::O, BondKind::Single);
    let oh = BondKind::typical_energy(Element::O, Element::H, BondKind::Single);

    let mut bonds = Vec::new();
    for i in 0..5 { bonds.push(Bond { between: (i, i + 1), kind: BondKind::Single, energy: cc }); }
    for i in 0..6 { bonds.push(Bond { between: (i % 6, 6 + i), kind: BondKind::Single, energy: ch }); }
    for i in 0..4 { bonds.push(Bond { between: (i, 16 + i), kind: BondKind::Single, energy: co }); }
    for i in 0..3 { bonds.push(Bond { between: (16 + i, 12 + i), kind: BondKind::Single, energy: oh }); }

    Molecule { name: "cellulose_unit".into(), atoms, bonds }
}

// =========================================================================
// PUBLIC PRESETS — only substance + phase form
// =========================================================================

// -- Rocks --

pub fn stone() -> MaterialDef { compile_as(&basalt_substance(), "stone", PhaseState::Solid) }
pub fn granite() -> MaterialDef { compile_as(&granite_substance(), "granite", PhaseState::Solid) }

// -- Loose materials (same substances, granular form) --

pub fn sand() -> MaterialDef { compile_as(&quartz_substance(), "sand", PhaseState::Granular) }
pub fn clay() -> MaterialDef { compile_as(&clay_substance(), "clay", PhaseState::Granular) }
pub fn dirt() -> MaterialDef { compile_as(&soil_substance(), "dirt", PhaseState::Granular) }
pub fn gravel() -> MaterialDef { compile_as(&basalt_substance(), "gravel", PhaseState::Granular) }

// -- Plants --

pub fn grass() -> MaterialDef {
    compile_as(
        &Substance::Polymer(PolymerChain {
            name: "cellulose".into(),
            monomer: cellulose_monomer(),
            chain_length: 3000,
            cross_link_density: 0.1,
            branching: 0.0,
        }),
        "grass",
        PhaseState::Solid,
    )
}

pub fn wood() -> MaterialDef {
    compile_as(
        &Substance::Polymer(PolymerChain {
            name: "cellulose_lignin".into(),
            monomer: cellulose_monomer(),
            chain_length: 5000,
            cross_link_density: 0.3,
            branching: 0.1,
        }),
        "wood",
        PhaseState::Solid,
    )
}

pub fn leaf() -> MaterialDef {
    compile_as(
        &Substance::Polymer(PolymerChain {
            name: "cellulose".into(),
            monomer: cellulose_monomer(),
            chain_length: 1000,
            cross_link_density: 0.05,
            branching: 0.0,
        }),
        "leaf",
        PhaseState::Solid,
    )
}

// -- Water forms --

pub fn water() -> MaterialDef { compile_as(&Substance::Molecular(Molecule::water()), "water", PhaseState::Liquid) }
pub fn ice() -> MaterialDef { compile_as(&Substance::Molecular(Molecule::water()), "ice", PhaseState::Solid) }
pub fn snow() -> MaterialDef { compile_as(&Substance::Molecular(Molecule::water()), "snow", PhaseState::Granular) }

// -- Metals --

pub fn iron() -> MaterialDef { compile_as(&Substance::Crystalline(Crystal::iron()), "iron", PhaseState::Solid) }
pub fn steel() -> MaterialDef { compile_as(&Substance::Crystalline(Crystal::steel(0.008)), "steel", PhaseState::Solid) }
pub fn copper() -> MaterialDef { compile_as(&Substance::Crystalline(Crystal::copper()), "copper", PhaseState::Solid) }
pub fn gold() -> MaterialDef { compile_as(&Substance::Crystalline(Crystal::gold()), "gold", PhaseState::Solid) }

// -- Gases --

pub fn air() -> MaterialDef { compile_as(&Substance::Molecular(Molecule::oxygen_gas()), "air", PhaseState::Gas) }
pub fn steam() -> MaterialDef { compile_as(&Substance::Molecular(Molecule::water()), "steam", PhaseState::Gas) }
pub fn smoke() -> MaterialDef { compile_as(&Substance::Molecular(Molecule::carbon_dioxide()), "smoke", PhaseState::Gas) }

// -- Phase transitions (same substance, different state) --

pub fn lava() -> MaterialDef { compile_as(&basalt_substance(), "lava", PhaseState::Liquid) }
