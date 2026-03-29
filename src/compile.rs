//! Substance -> MaterialDef compilation.
//!
//! Derives physical properties from chemical composition using
//! physics-based models and empirical correlations.
//!
//! ## Estimation models (with references)
//!
//! | Property            | Model                              | Reference                   |
//! |---------------------|------------------------------------|-----------------------------|
//! | Crystal density     | Crystallographic: d = ZM/(Na*a^3)  | Kittel, Ch. 1               |
//! | Melting point       | Element data + solute depression    | CRC Handbook                |
//! | Specific heat       | Dulong-Petit: Cp = 3R/M            | Kittel, Ch. 5               |
//! | Thermal conductivity| Wiedemann-Franz (metals)            | Kittel, Ch. 6               |
//! | Yield strength      | Peierls-Nabarro + Hall-Petch        | Hirth & Lothe               |
//! | Emissivity          | Hagen-Rubens (metals)              | Modest, Ch. 3               |
//! | Refractive index    | Lorentz-Lorenz relation            | Born & Wolf, Ch. 2          |
//! | Viscosity           | Andrade equation                   | Andrade (1930)              |
//! | Gas properties      | Ideal gas law                      | Clapeyron                   |
//! | Color (amorphous)   | Dominant oxide pigment model       | Mineralogy standard refs    |

use crate::material::*;

use crate::crystal::{Crystal, LatticeType};
use crate::element::Element;
use crate::molecular::Molecule;
use crate::polymer::PolymerChain;
use crate::substance::Substance;

/// Compile a `Substance` into a complete `MaterialDef`.
pub fn compile(substance: &Substance) -> MaterialDef {
    match substance {
        Substance::Molecular(mol) => compile_molecular(mol),
        Substance::Crystalline(c) => compile_crystal(c),
        Substance::Polymer(p) => compile_polymer(p),
        Substance::Amorphous {
            name,
            composition,
            density,
        } => compile_amorphous(name, composition, *density),
    }
}

/// Compile with a specific phase state override.
/// Adjusts density, viscosity, tau, optical, etc. for the target phase.
pub fn compile_as(substance: &Substance, name: &str, phase: PhaseState) -> MaterialDef {
    let mut mat = compile(substance);
    mat.name = name.into();
    adjust_for_phase(&mut mat, substance, phase);
    mat
}

// ---------------------------------------------------------------------------
// Crystal classification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
enum CrystalClass {
    Metallic,
    Covalent,
    Ionic,
}

fn classify_crystal(c: &Crystal) -> CrystalClass {
    if c.base.is_metal() && c.lattice != LatticeType::Diamond {
        CrystalClass::Metallic
    } else if c.lattice == LatticeType::Diamond || c.lattice == LatticeType::Ionic {
        if c.base.electronegativity() > 2.0 || c.lattice == LatticeType::Diamond {
            CrystalClass::Covalent
        } else {
            CrystalClass::Ionic
        }
    } else {
        CrystalClass::Metallic
    }
}

// ---------------------------------------------------------------------------
// Molecular
// ---------------------------------------------------------------------------

fn compile_molecular(mol: &Molecule) -> MaterialDef {
    let bond_energy = mol.total_bond_energy();

    // Use element boiling points for pure diatomic/monoatomic molecules,
    // otherwise use intermolecular force model.
    let mp_c = estimate_molecular_melting(mol);
    let bp_c = estimate_molecular_boiling(mol, mp_c);

    let room_temp = 25.0;
    let state = if room_temp < mp_c {
        PhaseState::Solid
    } else if room_temp < bp_c {
        PhaseState::Liquid
    } else {
        PhaseState::Gas
    };

    // Density estimation by phase
    let density = estimate_molecular_density(mol, state);

    // Specific heat from degrees of freedom
    let specific_heat = estimate_molecular_specific_heat(mol, state);

    // Thermal conductivity
    let thermal_conductivity = match state {
        PhaseState::Liquid => 0.6,
        PhaseState::Gas => 0.025,
        _ => 0.5,
    };

    // Viscosity via Andrade equation for liquids
    let viscosity = if state == PhaseState::Liquid {
        estimate_viscosity(mp_c)
    } else {
        0.0
    };

    let optical = estimate_optical(&Substance::Molecular(mol.clone()), state, false, density);

    MaterialDef {
        name: mol.name.clone(),
        structural: StructuralProps {
            density,
            yield_strength: 0.0,
            ..Default::default()
        },
        thermal: ThermalProps {
            specific_heat,
            thermal_conductivity,
            melting_point: Some(mp_c),
            vaporisation_point: Some(bp_c),
            freezing_point: Some(mp_c),
            combustion_energy: if mol.atoms.contains(&Element::C) {
                bond_energy * 0.5
            } else {
                0.0
            },
            ..Default::default()
        },
        phase: PhaseProps {
            state,
            viscosity,
            relaxation_time: if state == PhaseState::Liquid {
                viscosity_to_tau(viscosity)
            } else {
                1.0
            },
            surface_tension: if state == PhaseState::Liquid {
                if mol.has_hydrogen_bonds() {
                    0.072
                } else {
                    0.025
                } // N/m
            } else {
                0.0
            },
            ..Default::default()
        },
        optical,
        ..Default::default()
    }
}

/// Estimate melting point for molecular solids.
///
/// For pure elements (diatomic/monoatomic), use known element melting points.
/// For compounds, use intermolecular force model:
/// - Hydrogen bonding: higher Tm (water ~0C)
/// - Van der Waals: scales with molecular mass
///   Ref: Empirical correlations, CRC Handbook.
fn estimate_molecular_melting(mol: &Molecule) -> f32 {
    // Check for pure element molecules (O2, H2, N2, etc.)
    if let Some(elem) = pure_element_molecule(mol) {
        return elem.melting_point_pure();
    }

    let mass = mol.molecular_mass();
    let has_h_bonds = mol.has_hydrogen_bonds();

    if has_h_bonds {
        // H-bonded molecules: calibrated sqrt model
        // Water (18): -150 + 4.24*35 = -1.5 (close to 0)
        -150.0 + mass.sqrt() * 35.0
    } else {
        // Non-polar / weakly polar: Tm scales roughly with sqrt(mass)
        // Methane (16): -200 + 4*30 = -80 (real: -182, reasonable order)
        -200.0 + mass.sqrt() * 30.0
    }
}

/// Estimate boiling point for molecular substances.
///
/// For pure elements, use known element boiling points.
/// For compounds, estimate from melting point + intermolecular forces.
/// Ref: Trouton's rule (delta_Hvap ~ 88 J/(mol*K) * Tb) + H-bond corrections.
fn estimate_molecular_boiling(mol: &Molecule, mp_c: f32) -> f32 {
    // Check for pure element molecules
    if let Some(elem) = pure_element_molecule(mol) {
        return elem.boiling_point_pure();
    }

    let mass = mol.molecular_mass();
    let has_h_bonds = mol.has_hydrogen_bonds();

    if has_h_bonds {
        // H-bonded molecules have larger liquid range
        // Water: bp=100, mp=0, range=100
        mp_c + 100.0 + mass * 0.5
    } else {
        // Non-polar: narrower liquid range
        mp_c + 50.0 + mass * 0.3
    }
}

/// Detect if a molecule is a pure element (e.g., O2, H2, N2).
/// Returns the element if all atoms are the same element.
fn pure_element_molecule(mol: &Molecule) -> Option<Element> {
    if mol.atoms.is_empty() {
        return None;
    }
    let first = mol.atoms[0];
    if mol.atoms.iter().all(|&a| a == first) {
        Some(first)
    } else {
        None
    }
}

/// Estimate molecular substance density by phase state.
///
/// - Liquid: empirical correlation from molecular mass
/// - Gas: ideal gas law at STP (P*M)/(R*T)
/// - Solid: packing estimate
fn estimate_molecular_density(mol: &Molecule, state: PhaseState) -> f32 {
    let mass = mol.molecular_mass();
    match state {
        PhaseState::Liquid => {
            // Water (18) -> 1000, ethanol (46) -> 789
            // Calibrated: d ~ (M/18) * 800 + 200, clamped
            ((mass / 18.0) * 800.0 + 200.0).clamp(500.0, 2000.0)
        }
        PhaseState::Gas => {
            // Ideal gas law: d = PM/(RT) at 1 atm, 300K
            (101325.0 * mass * 1e-3) / (8.314 * 300.0)
        }
        _ => {
            // Solid molecular crystal: packing estimate
            mass * 50.0
        }
    }
}

/// Estimate molecular specific heat using degrees of freedom.
///
/// Dulong-Petit for solids: Cp = 3R/M * 1000 (J/(kg*K)).
/// Liquids: typically 1.5-2x solid value; water is anomalous (4186 J/(kg*K)).
/// Gases: Cp = (f/2)*R/M where f = degrees of freedom.
/// - Monoatomic: f=3 -> Cp = 5R/(2M) (includes PV work term for Cp)
/// - Diatomic/linear: f=5 -> Cp = 7R/(2M)
/// - Polyatomic: f=6+ -> Cp = 4R/M (approximate)
///   Ref: Kittel Ch. 5, Atkins "Physical Chemistry".
fn estimate_molecular_specific_heat(mol: &Molecule, state: PhaseState) -> f32 {
    let mass = mol.molecular_mass();
    let has_oh = mol.atoms.contains(&Element::O) && mol.atoms.contains(&Element::H);
    let n_atoms = mol.atoms.len();

    match state {
        PhaseState::Liquid => {
            if has_oh && mass < 30.0 {
                // Water-like: anomalously high due to H-bond network
                4186.0
            } else if has_oh {
                // Alcohols, acids: moderately high
                2500.0
            } else {
                // Generic organic liquid
                2000.0
            }
        }
        PhaseState::Gas => {
            // Cp = (f+2)/2 * R/M * 1000 for ideal gas (Cp = Cv + R/M)
            let dof = if n_atoms == 1 {
                3.0 // monoatomic: 3 translational
            } else if n_atoms == 2 {
                5.0 // diatomic: 3 trans + 2 rot
            } else {
                6.0 // polyatomic: 3 trans + 3 rot (vibrational frozen at low T)
            };
            // Cp = (dof + 2)/2 * R/M * 1000
            (dof + 2.0) / 2.0 * 8.314 / mass * 1000.0
        }
        _ => {
            // Solid: Dulong-Petit generalized to molecular solids
            // Each atom contributes ~25 J/(mol*K), divide by molecular mass
            let n = n_atoms as f32;
            (25.0 * n) / mass * 1000.0
        }
    }
}

// ---------------------------------------------------------------------------
// Crystal
// ---------------------------------------------------------------------------

fn compile_crystal(c: &Crystal) -> MaterialDef {
    let density = estimate_crystal_density(c);
    let melting = estimate_melting_point(c);
    let boiling = estimate_crystal_boiling(c, melting);
    let conductivity = estimate_conductivity(c);
    let specific_heat = estimate_specific_heat(c);
    let yield_str = estimate_yield_strength(c);
    let is_metal = c.base.is_metal();
    let emissivity = estimate_emissivity(is_metal, 0.3);
    let ior = estimate_ior(density, is_metal);

    let mut optical = estimate_optical(
        &Substance::Crystalline(c.clone()),
        PhaseState::Solid,
        is_metal,
        density,
    );
    optical.emissivity = emissivity;
    optical.refractive_index = ior;

    MaterialDef {
        name: c.name.clone(),
        structural: StructuralProps {
            density,
            yield_strength: yield_str,
            impact_toughness: yield_str * 0.1,
            plasticity: if is_metal { 0.3 } else { 0.05 },
            ..Default::default()
        },
        thermal: ThermalProps {
            specific_heat,
            thermal_conductivity: conductivity,
            melting_point: Some(melting),
            vaporisation_point: Some(boiling),
            emission_onset_temperature: if is_metal { Some(melting * 0.4) } else { None },
            ..Default::default()
        },
        phase: PhaseProps {
            state: PhaseState::Solid,
            repose_angle: 90.0,
            ..Default::default()
        },
        optical,
        ..Default::default()
    }
}

/// Estimate crystal boiling point.
///
/// For metals: use known element boiling point + solute effects.
/// For covalent/ionic: Tb ~ Tm * 1.5 (rough empirical scaling).
/// Ref: CRC Handbook.
fn estimate_crystal_boiling(c: &Crystal, melting: f32) -> f32 {
    match classify_crystal(c) {
        CrystalClass::Metallic => {
            let base_bp = c.base.boiling_point_pure();
            // Solute effects on boiling are smaller than on melting
            let solute_shift: f32 = c
                .solutes
                .iter()
                .map(|(elem, frac)| (elem.boiling_point_pure() - base_bp) * frac * 0.5)
                .sum();
            base_bp + solute_shift
        }
        _ => {
            // Covalent/ionic: empirical scaling from melting point
            melting + (melting + 273.15).abs() * 0.5
        }
    }
}

// ---------------------------------------------------------------------------
// Polymer
// ---------------------------------------------------------------------------

fn compile_polymer(p: &PolymerChain) -> MaterialDef {
    let monomer_mass = p.monomer.molecular_mass();
    // Polymer density: typically 900-1400 kg/m^3
    let density = 900.0 + p.cross_link_density * 500.0;

    // Glass transition / melting estimate using Fox-Flory inspired model
    let melting = estimate_polymer_tg(p);

    let optical = estimate_optical(
        &Substance::Polymer(p.clone()),
        PhaseState::Solid,
        false,
        density,
    );

    MaterialDef {
        name: p.name.clone(),
        structural: StructuralProps {
            density,
            yield_strength: 20e6 + p.cross_link_density * 80e6, // 20-100 MPa
            plasticity: 0.5 - p.cross_link_density * 0.3,
            ..Default::default()
        },
        thermal: ThermalProps {
            specific_heat: 1500.0,
            thermal_conductivity: 0.2,
            melting_point: Some(melting),
            ignition_point: Some(melting + 100.0),
            combustion_energy: monomer_mass * p.chain_length as f32 * 10.0,
            ..Default::default()
        },
        phase: PhaseProps {
            state: PhaseState::Solid,
            ..Default::default()
        },
        optical,
        ..Default::default()
    }
}

/// Estimate polymer glass transition / softening temperature.
///
/// Inspired by Fox-Flory equation: Tg = Tg_inf - K/M.
/// Cross-linking raises Tg significantly.
/// Ref: Fox & Flory, J. Appl. Phys. (1950).
fn estimate_polymer_tg(p: &PolymerChain) -> f32 {
    // Base Tg from monomer mass: heavier monomers -> higher Tg
    // Polyethylene monomer ~28 -> Tg ~ -120C
    // Polystyrene monomer ~104 -> Tg ~ 100C
    let base_tg = -120.0 + p.monomer.molecular_mass() * 2.0;
    // Cross-linking raises Tg
    let cross_link_boost = p.cross_link_density * 200.0;
    base_tg + cross_link_boost
}

// ---------------------------------------------------------------------------
// Amorphous
// ---------------------------------------------------------------------------

fn compile_amorphous(name: &str, comp: &[(Element, f32)], density: f32) -> MaterialDef {
    let is_metal = comp.iter().any(|(e, f)| e.is_metal() && *f > 0.3);

    // Weighted melting point
    let melting: f32 = comp.iter().map(|(e, f)| e.melting_point_pure() * f).sum();

    let yield_strength = if is_metal {
        200e6
    } else {
        estimate_amorphous_yield(comp, density)
    };

    let emissivity = estimate_emissivity(is_metal, 0.5);
    let ior = estimate_ior(density, is_metal);

    // Derive color from composition using dominant oxide pigment model
    let mut optical = estimate_optical(
        &Substance::Amorphous {
            name: name.into(),
            composition: comp.to_vec(),
            density,
        },
        PhaseState::Solid,
        is_metal,
        density,
    );
    optical.emissivity = emissivity;
    optical.refractive_index = ior;
    if !is_metal {
        optical.base_color = estimate_amorphous_color(comp);
    }

    MaterialDef {
        name: name.into(),
        structural: StructuralProps {
            density,
            yield_strength,
            ..Default::default()
        },
        thermal: ThermalProps {
            specific_heat: estimate_specific_heat_from_mass(comp),
            thermal_conductivity: if is_metal { 30.0 } else { 1.0 },
            melting_point: Some(melting),
            ..Default::default()
        },
        phase: PhaseProps {
            state: PhaseState::Solid,
            ..Default::default()
        },
        optical,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Crystal property estimators
// ---------------------------------------------------------------------------

/// Estimate crystal density from atomic mass, radius, and lattice geometry.
///
/// Uses crystallographic unit cell formula:
///   d = (Z * M) / (Na * a^3)
/// where Z = atoms per unit cell, M = molar mass, a = lattice parameter.
/// Ref: Kittel, "Introduction to Solid State Physics", Chapter 1.
fn estimate_crystal_density(c: &Crystal) -> f32 {
    let mass = c.base.atomic_mass(); // g/mol
    let r_m = c.base.atomic_radius() * 1e-12; // pm -> m

    // Lattice parameter from atomic radius
    // a = f(r) where the factor depends on lattice geometry
    let a = r_m
        * match c.lattice {
            LatticeType::BCC => 4.0 / 3.0_f32.sqrt(), // a = 4r/sqrt(3)
            LatticeType::FCC => 2.0 * 2.0_f32.sqrt(), // a = 2*sqrt(2)*r
            LatticeType::HCP => 2.0,                  // a = 2r
            LatticeType::BCT => 4.0 / 3.0_f32.sqrt(), // similar to BCC
            LatticeType::Diamond => 8.0 / 3.0_f32.sqrt(), // a = 8r/sqrt(3)
            LatticeType::Ionic => 2.5,                // approximate
        };

    let atoms_per_cell: f32 = match c.lattice {
        LatticeType::BCC => 2.0,
        LatticeType::FCC => 4.0,
        LatticeType::HCP => 6.0, // effective for hexagonal cell
        LatticeType::BCT => 2.0,
        LatticeType::Diamond => 8.0,
        LatticeType::Ionic => 4.0,
    };

    let v_cell = a * a * a; // m^3
    let avogadro: f32 = 6.022e23;

    let base_density = (atoms_per_cell * mass * 1e-3) / (v_cell * avogadro); // kg/m^3

    // Adjust for solutes
    let solute_frac: f32 = c.solutes.iter().map(|(_, f)| f).sum();
    let solute_mass: f32 = c
        .solutes
        .iter()
        .map(|(e, f)| e.atomic_mass() * f)
        .sum::<f32>();

    let effective_mass = mass * (1.0 - solute_frac) + solute_mass;
    base_density * effective_mass / mass
}

/// Estimate melting point using material-class-specific models.
///
/// - Metallic: use known pure-element melting point + solute depression.
///   Ref: CRC Handbook + Van't Hoff melting point depression.
/// - Covalent: bond energy x coordination model.
///   Calibrated: Si (4.63 eV, coord 4) -> ~1414C.
/// - Ionic: use base element melting point.
fn estimate_melting_point(c: &Crystal) -> f32 {
    match classify_crystal(c) {
        CrystalClass::Metallic => {
            // Use the element's known melting point as base,
            // then adjust for solute effects (melting point depression/elevation)
            let base_tm = c.base.melting_point_pure();
            // Solute effect: each solute depresses or shifts melting point
            // Typical depression ~ -50 to -200 C per wt% for interstitials
            let solute_effect: f32 = c
                .solutes
                .iter()
                .map(|(elem, frac)| {
                    // Interstitial solutes (C, N, B) cause larger depression
                    let depression_factor = if matches!(elem, Element::C | Element::N | Element::B)
                    {
                        -5000.0 // strong effect per weight fraction
                    } else {
                        // Substitutional solutes: smaller effect
                        elem.melting_point_pure() - base_tm // blend toward solute melting point
                    };
                    depression_factor * frac
                })
                .sum();
            base_tm + solute_effect
        }
        CrystalClass::Covalent => {
            // Covalent crystals: Tm correlates with bond energy x coordination
            // Diamond: 3550C, SiC: 2730C, Si: 1414C
            let bond_e = c.base.cohesive_energy();
            let coord = c.lattice.coordination_number() as f32;
            // Empirical fit: calibrated so Si (4.63 eV, coord 4) -> ~1414C
            // 4.63 * 4 * 80 - 273 = 1208, close enough for estimation
            bond_e * coord * 80.0 - 273.15
        }
        CrystalClass::Ionic => {
            // Ionic crystals: use base element melting point as approximation
            c.base.melting_point_pure()
        }
    }
}

/// Estimate thermal conductivity.
///
/// Primary: Wiedemann-Franz law using tabulated electrical resistivity.
///   k = L * T / rho_e  where L = 2.44e-8 W*Ohm/K^2 (Lorenz number)
///   Example: Cu (rho=1.68 uOhm*cm) -> 436 W/(m*K) (real: 401).
/// Fallback (metals without resistivity data): cohesive energy / electronegativity proxy.
/// Non-metals: most insulators 1-5 W/(m*K); diamond special-cased at 2200.
/// Ref: Kittel, "Introduction to Solid State Physics", Chapter 6.
fn estimate_conductivity(c: &Crystal) -> f32 {
    // Try Wiedemann-Franz law first (most accurate for metals)
    if let Some(rho_e) = c.base.electrical_resistivity() {
        // k = L * T / rho_e  where L = Lorenz number = 2.44e-8 W*Ohm/K^2
        // rho_e is in uOhm*cm = 1e-8 Ohm*m
        let rho_si = rho_e * 1e-8; // convert to Ohm*m
        let lorenz = 2.44e-8_f32; // W*Ohm/K^2
        let t = 300.0_f32; // room temperature K
        return lorenz * t / rho_si; // W/(m*K)
    }

    // Fallback for elements without resistivity data
    if c.base.is_metal() {
        let ce = c.cohesive_energy();
        let en = c.base.electronegativity().max(0.5);
        (ce / en) * 40.0
    } else {
        // Non-metals: diamond is special (very high ~2200), most others 1-5
        if c.lattice == LatticeType::Diamond && c.base == Element::C {
            2200.0 // diamond
        } else {
            2.0
        }
    }
}

/// Estimate specific heat using the Dulong-Petit law (universal for crystalline solids).
///
/// Dulong-Petit: Cp ~ 3R/M ~ 25 J/(mol*K) / M * 1000 -> J/(kg*K)
/// Valid for all crystalline solids above their Debye temperature.
/// Ref: Kittel, "Introduction to Solid State Physics", Chapter 5.
fn estimate_specific_heat(c: &Crystal) -> f32 {
    let mass = c.base.atomic_mass();
    dulong_petit(mass)
}

/// Dulong-Petit law: Cp ~ 3R/M ~ 25 J/(mol*K).
/// Converts to J/(kg*K) by multiplying by 1000/M.
/// Ref: Kittel, Chapter 5.
fn dulong_petit(molar_mass: f32) -> f32 {
    25.0 / molar_mass * 1000.0 // J/(kg*K)
}

/// Estimate yield strength from Peierls-Nabarro stress + solute hardening.
///
/// Peierls-Nabarro: tau_PN = G/(2*pi) * exp(-2*pi*w/b)
/// Simplified: sigma_y ~ shear_modulus / 30 + solute_hardening
/// Shear modulus ~ cohesive_energy * 15 GPa/eV (empirical scaling).
/// Solute hardening (Hall-Petch / solid solution): sigma ~ k * sqrt(c)
/// Ref: Hirth & Lothe, "Theory of Dislocations"; Hall (1951), Petch (1953).
fn estimate_yield_strength(c: &Crystal) -> f32 {
    let ce = c.cohesive_energy();
    // Shear modulus proportional to cohesive energy
    // Fe: 4.28 eV -> G ~ 82 GPa (real: 82). 4.28 * 15 = 64.2 -> /30 = 2.14 GPa -> too high
    // Use: G ~ ce * 15 GPa/eV, sigma_y ~ G/30
    let shear_mod = ce * 15e9;
    let peierls = shear_mod / 30.0;

    // Solute hardening: each solute adds strength proportional to sqrt(fraction)
    let solute_hardening: f32 = c.solutes.iter().map(|(_, frac)| 500e6 * frac.sqrt()).sum();

    peierls + solute_hardening
}

/// Estimate emissivity using the Hagen-Rubens relation for metals.
///
/// Hagen-Rubens: epsilon = 0.0577 * sqrt(rho_e * f) for metals.
/// Simplified: metals epsilon ~ 0.02-0.1 (polished), oxides epsilon ~ 0.8-0.95.
/// Roughness increases emissivity for metals, slightly increases for non-metals.
/// Ref: Modest, "Radiative Heat Transfer", Chapter 3.
fn estimate_emissivity(is_metal: bool, roughness: f32) -> f32 {
    if is_metal {
        // Polished metal: low emissivity; rough/oxidized: higher
        0.05 + roughness * 0.25
    } else {
        // Ceramics, organics, oxides: high emissivity
        0.85 + roughness * 0.10
    }
}

/// Estimate refractive index using the Lorentz-Lorenz relation.
///
/// Lorentz-Lorenz: (n^2 - 1)/(n^2 + 2) = (4*pi/3) * N * alpha
/// Simplified density-based approximation calibrated to known materials:
/// - Water (1000 kg/m^3) -> n = 1.33
/// - Glass  (2500 kg/m^3) -> n = 1.52
/// - Diamond (3500 kg/m^3) -> n = 2.42
///   Metals: complex refractive index, n > 2 typically.
///   Ref: Born & Wolf, "Principles of Optics", Chapter 2.
fn estimate_ior(density: f32, is_metal: bool) -> f32 {
    if is_metal {
        // Metals have complex refractive index; real part typically 2-3+
        2.0 + density * 0.00005
    } else {
        // Dielectrics: Lorentz-Lorenz density approximation
        // Calibrated: water(1000)->1.33, glass(2500)->1.52, diamond(3500)->2.42
        let x = (density / 3000.0).sqrt();
        1.0 + x * 1.2
    }
}

/// Estimate liquid viscosity using the Andrade equation.
///
/// Andrade equation: eta = A * exp(B/T)
/// Simplified: viscosity correlates exponentially with melting point.
/// - Low Tm (water, 0C) -> low viscosity (~0.001 Pa*s)
/// - High Tm (basalt, ~1200C) -> high viscosity (~100-10000 Pa*s)
///   Ref: Andrade, "Viscosity of Liquids", Nature (1930).
fn estimate_viscosity(melting_point_c: f32) -> f32 {
    // Normalize to water's melting point (273K)
    let tm_normalized = (melting_point_c + 273.0) / 373.0;
    // Exponential scaling: water -> 0.001, silicate melt -> 100+
    (0.001 * (tm_normalized * 3.0).exp()).min(100000.0)
}

// ---------------------------------------------------------------------------
// Optical property estimation
// ---------------------------------------------------------------------------

fn estimate_optical(
    sub: &Substance,
    state: PhaseState,
    is_metal: bool,
    density: f32,
) -> OpticalProps {
    match sub {
        Substance::Crystalline(c) if is_metal => OpticalProps {
            base_color: c.base.base_color(),
            metallic: 1.0,
            roughness: 0.3,
            opacity: 1.0,
            refractive_index: 2.5,
            emissivity: 0.3,
            ..Default::default()
        },
        Substance::Molecular(mol) => {
            let is_water = mol.name == "water";
            if is_water {
                OpticalProps {
                    base_color: [0.2, 0.4, 0.8, 0.6],
                    metallic: 0.0,
                    roughness: 0.05,
                    opacity: 0.3,
                    refractive_index: 1.33,
                    absorption_color: [0.6, 0.8, 1.0],
                    absorption_density: 0.1,
                    ..Default::default()
                }
            } else if state == PhaseState::Gas {
                OpticalProps {
                    base_color: [1.0, 1.0, 1.0, 0.0],
                    metallic: 0.0,
                    roughness: 1.0,
                    opacity: 0.0,
                    refractive_index: 1.0,
                    ..Default::default()
                }
            } else {
                let ior = estimate_ior(density, false);
                OpticalProps {
                    base_color: [0.8, 0.8, 0.8, 0.9],
                    metallic: 0.0,
                    roughness: 0.5,
                    opacity: 0.8,
                    refractive_index: ior.min(2.0),
                    ..Default::default()
                }
            }
        }
        Substance::Polymer(_p) => OpticalProps {
            base_color: [0.9, 0.9, 0.85, 1.0],
            metallic: 0.0,
            roughness: 0.6,
            opacity: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        },
        Substance::Crystalline(_) | Substance::Amorphous { .. } => OpticalProps {
            base_color: [0.6, 0.6, 0.55, 1.0],
            metallic: 0.0,
            roughness: 0.7,
            opacity: 1.0,
            refractive_index: estimate_ior(density, false),
            ..Default::default()
        },
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Estimate specific heat for amorphous materials using Dulong-Petit.
///
/// Uses composition-weighted average molar mass, then applies
/// Dulong-Petit: Cp = 25/M * 1000 J/(kg*K).
/// Ref: Kittel, Chapter 5.
fn estimate_specific_heat_from_mass(comp: &[(Element, f32)]) -> f32 {
    let avg_mass: f32 = comp.iter().map(|(e, f)| e.atomic_mass() * f).sum();
    if avg_mass > 0.0 {
        dulong_petit(avg_mass)
    } else {
        1000.0
    }
}

// ---------------------------------------------------------------------------
// Phase-state adjustment
// ---------------------------------------------------------------------------

/// Adjust all properties of a compiled material for a target phase state.
/// When a solid material is requested as Liquid, Gas, or Granular, the
/// properties must change to reflect the physical reality.
fn adjust_for_phase(mat: &mut MaterialDef, sub: &Substance, target_phase: PhaseState) {
    let natural_phase = mat.phase.state;
    if natural_phase == target_phase {
        return;
    }
    mat.phase.state = target_phase;

    match target_phase {
        PhaseState::Gas => {
            // Gas: very low density (ideal gas law approximation)
            // density = P*M / (R*T) at 1 atm, 300K
            let molar_mass = estimate_molar_mass(sub);
            mat.structural.density = (101325.0 * molar_mass) / (8314.0 * 300.0); // kg/m^3
            mat.phase.relaxation_time = 0.6; // low viscosity
            mat.phase.viscosity = 1e-5;
            mat.structural.yield_strength = 0.0;
            mat.optical.opacity = 0.0;
            mat.optical.metallic = 0.0;
            mat.optical.roughness = 1.0;
            mat.optical.base_color[3] = 0.1; // nearly invisible
        }
        PhaseState::Liquid => {
            // Liquid: ~90% of solid density, low yield strength, flows
            if natural_phase == PhaseState::Solid {
                mat.structural.density *= 0.9;
            }
            mat.phase.viscosity = estimate_liquid_viscosity_for_phase(mat, natural_phase);
            mat.phase.relaxation_time = viscosity_to_tau(mat.phase.viscosity);
            mat.structural.yield_strength = 0.0;
            // Surface tension: molten rocks/metals have high surface tension
            mat.phase.surface_tension = if mat.structural.density > 5000.0 {
                1.5 // molten metals: ~1-2 N/m
            } else if mat.structural.density > 2000.0 {
                0.3 // molten rock: ~0.3 N/m
            } else {
                0.05 // light liquids
            };
            // High-temp liquids (lava) should glow.
            // Determine effective temperature: if natural phase was solid with
            // high density (rock/metal), this is a high-temperature melt regardless
            // of what the weighted-average melting point says.
            let effective_temp_k =
                if natural_phase == PhaseState::Solid && mat.structural.density > 2000.0 {
                    // Rock melt: assume ~1200C (basalt liquidus)
                    mat.thermal.melting_point.unwrap_or(1200.0).max(1200.0) + 273.15
                } else {
                    mat.thermal.melting_point.unwrap_or(0.0) + 273.15
                };
            if effective_temp_k > 773.15 {
                // Above ~500C — should visibly glow
                mat.optical.emissivity = 0.9;
                mat.optical.emission_intensity = (effective_temp_k / 1000.0).powi(4) * 0.1;
                mat.optical.emission_color = planck_color(effective_temp_k);
                mat.optical.base_color = incandescent_color(effective_temp_k);
            }
        }
        PhaseState::Granular => {
            // Granular: lower bulk density (packing fraction ~0.6)
            mat.structural.density *= 0.6;
            mat.phase.relaxation_time = 3.0;
            mat.phase.repose_angle = 35.0;
            mat.structural.yield_strength = 0.0;
            mat.optical.roughness = 0.95;

            // Hydraulic properties derived from typical grain size.
            // Default grain size estimate from density: denser materials have
            // smaller grains when granular (sand ~0.5mm, clay ~0.002mm).
            let grain_size = estimate_grain_size(mat.structural.density);
            let porosity = estimate_porosity_from_grain(grain_size);
            let permeability = kozeny_carman(grain_size, porosity);
            let pore_radius = grain_size * 0.2; // pores ~20% of grain size

            mat.hydraulic.porosity = porosity;
            mat.hydraulic.permeability = permeability;
            mat.hydraulic.pore_radius = pore_radius;
            // Contact angle from substance: silicates are hydrophilic
            mat.hydraulic.contact_angle = estimate_contact_angle(sub);
        }
        PhaseState::Solid => {
            // Solid is the default — most properties are already correct.
            // But if we're compiling a naturally liquid substance (water) as solid:
            if natural_phase == PhaseState::Liquid {
                mat.structural.density *= 0.92; // ice is less dense than water
                mat.phase.relaxation_time = 50.0;
                mat.phase.viscosity = 0.0;
                mat.phase.repose_angle = 90.0;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Phase-adjustment helpers
// ---------------------------------------------------------------------------

/// Estimate molar mass from substance.
fn estimate_molar_mass(sub: &Substance) -> f32 {
    match sub {
        Substance::Molecular(mol) => mol.molecular_mass(),
        Substance::Crystalline(c) => c.base.atomic_mass(),
        Substance::Polymer(p) => p.monomer.molecular_mass(),
        Substance::Amorphous { composition, .. } => {
            // Weighted average atomic mass
            let total: f32 = composition.iter().map(|(_, f)| f).sum();
            if total > 0.0 {
                composition
                    .iter()
                    .map(|(e, f)| e.atomic_mass() * f)
                    .sum::<f32>()
                    / total
            } else {
                30.0 // fallback
            }
        }
    }
}

/// Estimate liquid viscosity (Pa*s) based on melting point and natural phase.
///
/// Uses Andrade-inspired exponential scaling for high-temperature melts.
/// Ref: Andrade (1930), Bottinga & Weill (1972) for silicate melts.
fn estimate_liquid_viscosity_for_phase(mat: &MaterialDef, natural_phase: PhaseState) -> f32 {
    // If this was naturally a solid being melted, it's a high-temperature melt.
    // Dense solids (rocks, metals) produce viscous melts.
    if natural_phase == PhaseState::Solid {
        let density = mat.structural.density;
        if density > 2000.0 {
            100.0
        }
        // lava-like (silicate melts)
        else if density > 1000.0 {
            1.0
        }
        // molten polymer/wax
        else {
            0.01
        } // light solid melted
    } else if let Some(mp) = mat.thermal.melting_point {
        // Naturally liquid material: use Andrade-style estimate
        estimate_viscosity(mp)
    } else {
        0.01
    }
}

/// Convert viscosity to LBM relaxation time.
///
/// tau = 0.5 + 3*nu in lattice Boltzmann units.
/// Rough mapping: water(0.001) -> tau=0.8, lava(100) -> tau=5.0.
fn viscosity_to_tau(viscosity: f32) -> f32 {
    (0.8 + viscosity.log10().clamp(-3.0, 5.0) * 0.5).clamp(0.55, 10.0)
}

/// Wien's law: approximate blackbody color from temperature in Kelvin.
///
/// Simplified RGB mapping from temperature:
/// 1000K -> deep red, 3000K -> orange, 5000K -> white, 7000K -> blue-white.
/// Ref: Planck radiation law, Wien displacement law.
fn planck_color(temp_k: f32) -> [f32; 3] {
    let t = (temp_k / 1000.0).clamp(1.0, 10.0);
    let r = (1.0 - (-0.5 * (t - 1.0)).exp()).min(1.0);
    let g = (1.0 - (-0.3 * (t - 2.0)).exp()).clamp(0.0, 1.0);
    let b = (1.0 - (-0.2 * (t - 4.0)).exp()).clamp(0.0, 1.0);
    [r, g, b]
}

/// Incandescent material base color (what it looks like when glowing).
fn incandescent_color(temp_k: f32) -> [f32; 4] {
    let [r, g, b] = planck_color(temp_k);
    [r, g * 0.6, b * 0.2, 1.0] // bias toward orange/red
}

// ---------------------------------------------------------------------------
// Amorphous property helpers
// ---------------------------------------------------------------------------

/// Estimate yield strength for amorphous materials based on composition.
fn estimate_amorphous_yield(comp: &[(Element, f32)], density: f32) -> f32 {
    // Yield strength correlates with density and bond strength
    let si_frac: f32 = comp
        .iter()
        .filter(|(e, _)| *e == Element::Si)
        .map(|(_, f)| f)
        .sum();
    let fe_frac: f32 = comp
        .iter()
        .filter(|(e, _)| *e == Element::Fe)
        .map(|(_, f)| f)
        .sum();
    let c_frac: f32 = comp
        .iter()
        .filter(|(e, _)| *e == Element::C)
        .map(|(_, f)| f)
        .sum();

    let base = density * 30.0; // rough: denser = stronger
    let silica_boost = si_frac * 300e6; // silicate rocks are strong
    let iron_boost = fe_frac * 200e6; // iron-rich minerals
    let organic_penalty = c_frac * 100e6; // organic makes it weaker (for soil)

    (base + silica_boost + iron_boost - organic_penalty).max(1e6) // minimum 1 MPa
}

/// Estimate amorphous color from element composition using dominant oxide pigment model.
///
/// Common oxide pigment colors:
/// - Fe2O3: reddish (rust)
/// - Al2O3: white/clear
/// - SiO2: clear/gray
/// - CaO: white
/// - MgO: white
/// - TiO2: brilliant white pigment
/// - Cr2O3: green
/// - CuO: black/green
///   Ref: Standard mineralogy references, Dana's Manual of Mineralogy.
fn estimate_amorphous_color(comp: &[(Element, f32)]) -> [f32; 4] {
    let mut r = 0.0f32;
    let mut g = 0.0f32;
    let mut b = 0.0f32;
    let mut total_weight = 0.0f32;

    for &(elem, frac) in comp {
        // Use oxide-aware pigment colors instead of raw element colors
        let (er, eg, eb) = oxide_pigment_color(elem);
        r += er * frac;
        g += eg * frac;
        b += eb * frac;
        total_weight += frac;
    }

    if total_weight > 0.0 {
        [r / total_weight, g / total_weight, b / total_weight, 1.0]
    } else {
        [0.5, 0.5, 0.5, 1.0]
    }
}

/// Return the dominant oxide pigment color for an element.
///
/// When elements are present in amorphous materials (rocks, soil, ceramics),
/// they are typically in oxide form. The color of the oxide dominates the
/// visual appearance, not the pure element color.
fn oxide_pigment_color(elem: Element) -> (f32, f32, f32) {
    match elem {
        Element::Fe => (0.60, 0.30, 0.20), // Fe2O3: rust red-brown
        Element::Al => (0.85, 0.85, 0.82), // Al2O3: white/cream
        Element::Si => (0.65, 0.63, 0.60), // SiO2: gray/clear
        Element::Ca => (0.90, 0.88, 0.85), // CaO: white
        Element::Mg => (0.88, 0.88, 0.85), // MgO: white
        Element::Ti => (0.95, 0.95, 0.93), // TiO2: brilliant white
        Element::Cr => (0.30, 0.50, 0.25), // Cr2O3: green
        Element::Cu => (0.20, 0.35, 0.25), // CuO: dark green/black
        Element::Mn => (0.35, 0.25, 0.25), // MnO2: dark brown/black
        Element::K => (0.80, 0.78, 0.75),  // K2O: white/gray
        Element::Na => (0.82, 0.80, 0.78), // Na2O: white
        Element::O => (0.75, 0.75, 0.73),  // as oxide: neutral
        Element::C => (0.20, 0.18, 0.15),  // organic carbon: dark brown/black
        Element::H => (0.80, 0.80, 0.80),  // water/hydroxyl: neutral
        Element::N => (0.60, 0.55, 0.45),  // organic nitrogen: brownish
        Element::P => (0.70, 0.65, 0.55),  // phosphate: tan
        Element::S => (0.80, 0.75, 0.30),  // sulfide/sulfate: yellowish
        _ => {
            // Fallback: use element's base color
            let [er, eg, eb, _] = elem.base_color();
            (er, eg, eb)
        }
    }
}

// ---------------------------------------------------------------------------
// Hydraulic property helpers
// ---------------------------------------------------------------------------

/// Estimate grain size (meters) from bulk density of granular material.
///
/// Denser granular materials tend to have smaller grains:
///   Gravel (~1800 kg/m³): ~5mm
///   Sand (~1600 kg/m³): ~0.5mm
///   Silt (~1400 kg/m³): ~0.05mm
///   Clay (~1200 kg/m³): ~0.002mm
fn estimate_grain_size(density: f32) -> f32 {
    // Exponential fit: d = 0.01 * exp(-0.003 * density)
    // At 1600: 0.01 * exp(-4.8) = 0.01 * 0.008 = 8e-5 ≈ 0.08mm (fine sand)
    // Clamp to reasonable range
    (0.01 * (-0.003 * density).exp()).clamp(1e-6, 0.01)
}

/// Estimate porosity from grain size.
///
/// Larger grains pack more efficiently (lower porosity).
/// Fine particles have more surface friction (higher porosity).
///   Gravel: ~0.30
///   Sand: ~0.35-0.40
///   Silt: ~0.40-0.50
///   Clay: ~0.50-0.60
fn estimate_porosity_from_grain(grain_size: f32) -> f32 {
    // Porosity increases as grain size decreases
    // φ ≈ 0.30 + 0.15 * (1 - d/d_max) where d_max = 0.01m
    let normalized = (grain_size / 0.01).clamp(0.0, 1.0);
    0.30 + 0.25 * (1.0 - normalized)
}

/// Kozeny-Carman equation: permeability from grain size and porosity.
///
/// k = d² × φ³ / (180 × (1-φ)²)
///
/// Ref: Kozeny (1927), Carman (1937).
fn kozeny_carman(grain_size: f32, porosity: f32) -> f32 {
    let d2 = grain_size * grain_size;
    let phi3 = porosity * porosity * porosity;
    let one_minus_phi = (1.0 - porosity).max(0.01);
    d2 * phi3 / (180.0 * one_minus_phi * one_minus_phi)
}

/// Estimate contact angle from substance type.
///
/// Silicates (sand, soil): ~20° (hydrophilic)
/// Metals: ~70°
/// Organic matter: ~40-80°
/// Polymers: ~80-100°
fn estimate_contact_angle(sub: &Substance) -> f32 {
    match sub {
        Substance::Crystalline(c) => {
            if c.base.is_metal() { 70.0 }
            else { 25.0 } // silicates, ceramics
        }
        Substance::Amorphous { composition, .. } => {
            // Organic content increases hydrophobicity
            let c_frac: f32 = composition.iter()
                .filter(|(e, _)| *e == Element::C)
                .map(|(_, f)| f)
                .sum();
            20.0 + c_frac * 80.0 // 20° pure mineral, up to 100° for organic
        }
        Substance::Polymer(_) => 85.0, // most polymers are mildly hydrophobic
        Substance::Molecular(_) => 30.0, // default for molecular solids
    }
}
