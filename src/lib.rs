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
//! The [`compile()`] function derives every property from the chemical
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
//! - [`mod@compile`] — [`compile()`] and [`compile_as()`]: substance → material properties
//! - [`material`] — [`MaterialDef`] output type with all property groups
//! - [`reaction`] — Chemical [`Reaction`] with energy balance
//! - [`interaction`] — `InteractionRule` for runtime reaction dispatch
//! - [`presets`] — 20 ready-to-use world materials

pub mod compile;
pub mod crystal;
pub mod element;
pub mod interaction;
pub mod material;
pub mod molecular;
pub mod polymer;
pub mod presets;
pub mod reaction;
pub mod substance;

// -- Primary re-exports -------------------------------------------------------

pub use compile::{compile, compile_as};
pub use crystal::{Crystal, LatticeType};
pub use element::Element;
pub use material::{MaterialDef, MaterialId, PhaseState};
pub use molecular::{Bond, BondKind, Molecule};
pub use polymer::PolymerChain;
pub use reaction::Reaction;
pub use substance::Substance;

/// Convenience prelude — import everything commonly needed.
///
/// ```rust
/// use materia::prelude::*;
///
/// let iron = compile(&Substance::Crystalline(Crystal::iron()));
/// ```
pub mod prelude {
    pub use crate::compile::{compile, compile_as};
    pub use crate::crystal::{Crystal, LatticeType};
    pub use crate::element::Element;
    pub use crate::material::{
        AcousticProps, ChemicalProps, HydraulicProps, MaterialDef, MaterialId, OpticalProps,
        PhaseProps, PhaseState, StructuralProps, ThermalProps,
    };
    pub use crate::molecular::{Bond, BondKind, Molecule};
    pub use crate::polymer::PolymerChain;
    pub use crate::reaction::Reaction;
    pub use crate::substance::Substance;
}
