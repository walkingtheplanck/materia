//! Top-level substance abstraction: molecular, crystalline, polymer, or amorphous.

use crate::crystal::Crystal;
use crate::element::Element;
use crate::molecular::Molecule;
use crate::polymer::PolymerChain;

/// A material described by its chemical/structural nature.
#[derive(Debug, Clone)]
pub enum Substance {
    /// Discrete molecules (water, methane, CO2, etc.).
    Molecular(Molecule),
    /// Crystalline solid (metals, ceramics, salts).
    Crystalline(Crystal),
    /// Polymer chain.
    Polymer(PolymerChain),
    /// Amorphous material (glass, some ceramics).
    Amorphous {
        name: String,
        /// Elemental composition as (element, weight fraction).
        composition: Vec<(Element, f32)>,
        /// Bulk density in kg/m^3.
        density: f32,
    },
}

impl Substance {
    /// Convenience constructor for amorphous materials.
    pub fn amorphous(name: &str, composition: &[(Element, f32)], density: f32) -> Self {
        Substance::Amorphous {
            name: name.into(),
            composition: composition.to_vec(),
            density,
        }
    }

    /// Total bond energy of the substance (kJ/mol or eV/atom depending on variant).
    pub fn total_bond_energy(&self) -> f32 {
        match self {
            Substance::Molecular(mol) => mol.total_bond_energy(),
            Substance::Crystalline(c) => {
                // Convert eV/atom to kJ/mol: 1 eV = 96.485 kJ/mol
                c.cohesive_energy() * 96.485
            }
            Substance::Polymer(p) => {
                // Approximate: monomer bond energy * chain length
                p.monomer.total_bond_energy() * p.chain_length as f32
            }
            Substance::Amorphous { composition, .. } => {
                // Weighted average cohesive energy
                let mut energy = 0.0f32;
                for &(elem, frac) in composition {
                    energy += elem.cohesive_energy() * frac * 96.485;
                }
                energy
            }
        }
    }
}
