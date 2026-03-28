//! Polymer chain structures.

use crate::molecular::Molecule;

/// A polymer: a repeated monomer unit with chain properties.
#[derive(Debug, Clone)]
pub struct PolymerChain {
    pub name: String,
    /// The monomer unit that is repeated.
    pub monomer: Molecule,
    /// Number of monomer repeats.
    pub chain_length: u32,
    /// Cross-link density (0.0 = linear, 1.0 = fully cross-linked).
    pub cross_link_density: f32,
    /// Branching factor (0.0 = linear, 1.0 = highly branched).
    pub branching: f32,
}
