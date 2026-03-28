//! # materia
//!
//! Chemical composition to physical material properties, from first principles.
//!
//! Define materials by their atomic structure — molecules, crystals, polymers,
//! or amorphous compositions — and get all physical properties derived
//! automatically using established physics models.
//!
//! ## Quick start
//!
//! ```rust
//! use materia::prelude::*;
//!
//! // Define iron as a BCC crystal
//! let iron = compile(&Substance::Crystalline(Crystal::iron()));
//! assert_eq!(iron.phase.state, PhaseState::Solid);
//!
//! // Or use a preset
//! let water = materia::presets::water();
//! assert_eq!(water.phase.state, PhaseState::Liquid);
//! ```
//!
//! ## Architecture
//!
//! ```text
//! Element  →  Substance  →  compile()  →  MaterialDef
//! (atoms)     (structure)    (physics)     (properties)
//! ```
//!
//! The [`compile`] function derives every property from the chemical
//! structure: density from crystallography, melting point from cohesive
//! energy, conductivity from Wiedemann-Franz, color from electron
//! structure. No hardcoded material databases.
//!
//! ## Modules
//!
//! - [`element`] — Periodic table: all 118 elements with atomic data
//! - [`substance`] — [`Substance`] enum: molecular, crystalline, polymer, amorphous
//! - [`molecular`] — [`Molecule`], [`Bond`], bond energies
//! - [`crystal`] — [`Crystal`], [`LatticeType`] (BCC, FCC, HCP, Diamond)
//! - [`polymer`] — [`PolymerChain`] with cross-linking
//! - [`compile`] — [`compile()`] and [`compile_as()`]: substance → material properties
//! - [`material`] — [`MaterialDef`] output type with all property groups
//! - [`reaction`] — Chemical [`Reaction`] with energy balance
//! - [`interaction`] — [`InteractionRule`] for runtime reaction dispatch
//! - [`presets`] — 20 ready-to-use world materials

pub mod element;
pub mod substance;
pub mod molecular;
pub mod crystal;
pub mod polymer;
pub mod compile;
pub mod material;
pub mod interaction;
pub mod reaction;
pub mod presets;

// -- Primary re-exports -------------------------------------------------------

pub use element::Element;
pub use substance::Substance;
pub use molecular::{Molecule, Bond, BondKind};
pub use crystal::{Crystal, LatticeType};
pub use polymer::PolymerChain;
pub use compile::{compile, compile_as};
pub use reaction::Reaction;
pub use material::{MaterialDef, PhaseState, MaterialId};

/// Convenience prelude — import everything commonly needed.
///
/// ```rust
/// use materia::prelude::*;
///
/// let iron = compile(&Substance::Crystalline(Crystal::iron()));
/// ```
pub mod prelude {
    pub use crate::element::Element;
    pub use crate::substance::Substance;
    pub use crate::molecular::{Molecule, Bond, BondKind};
    pub use crate::crystal::{Crystal, LatticeType};
    pub use crate::polymer::PolymerChain;
    pub use crate::compile::{compile, compile_as};
    pub use crate::reaction::Reaction;
    pub use crate::material::{
        MaterialDef, PhaseState, MaterialId,
        StructuralProps, ThermalProps, ChemicalProps,
        HydraulicProps, PhaseProps, AcousticProps, OpticalProps,
    };
}
