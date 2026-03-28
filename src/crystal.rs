//! Crystalline structures: lattice type, base element, and solute impurities.

use crate::element::Element;

/// A crystalline material defined by lattice geometry and composition.
#[derive(Debug, Clone)]
pub struct Crystal {
    pub name: String,
    pub lattice: LatticeType,
    pub base: Element,
    /// Solute elements and their weight fractions (0.0–1.0).
    pub solutes: Vec<(Element, f32)>,
}

/// Common crystal lattice types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LatticeType {
    /// Body-centered cubic (Fe-alpha, Cr, W, Mo).
    BCC,
    /// Face-centered cubic (Al, Cu, Ni, Au, Ag).
    FCC,
    /// Hexagonal close-packed (Ti-alpha, Mg, Zn, Co).
    HCP,
    /// Body-centered tetragonal (Sn-beta, martensite).
    BCT,
    /// Diamond cubic (C-diamond, Si, Ge).
    Diamond,
    /// Ionic crystal (NaCl, etc.).
    Ionic,
}

impl LatticeType {
    /// Atomic packing fraction.
    pub fn packing_fraction(&self) -> f32 {
        match self {
            LatticeType::BCC => 0.680,
            LatticeType::FCC => 0.740,
            LatticeType::HCP => 0.740,
            LatticeType::BCT => 0.700,
            LatticeType::Diamond => 0.340,
            LatticeType::Ionic => 0.660,
        }
    }

    /// Coordination number (nearest neighbours).
    pub fn coordination_number(&self) -> u32 {
        match self {
            LatticeType::BCC => 8,
            LatticeType::FCC => 12,
            LatticeType::HCP => 12,
            LatticeType::BCT => 8,
            LatticeType::Diamond => 4,
            LatticeType::Ionic => 6,
        }
    }
}

impl Crystal {
    /// Cohesive energy (eV/atom), weighted by composition.
    pub fn cohesive_energy(&self) -> f32 {
        let base_frac: f32 = 1.0 - self.solutes.iter().map(|(_, f)| f).sum::<f32>();
        let mut energy = self.base.cohesive_energy() * base_frac;
        for &(elem, frac) in &self.solutes {
            energy += elem.cohesive_energy() * frac;
        }
        energy
    }

    /// Pure iron: BCC Fe.
    pub fn iron() -> Self {
        Self {
            name: "iron".into(),
            lattice: LatticeType::BCC,
            base: Element::Fe,
            solutes: Vec::new(),
        }
    }

    /// Carbon steel: BCC Fe + interstitial C.
    /// `carbon_pct` is the weight fraction (e.g., 0.008 = 0.8 wt%).
    pub fn steel(carbon_pct: f32) -> Self {
        Self {
            name: format!("steel_{:.1}C", carbon_pct * 100.0),
            lattice: LatticeType::BCC,
            base: Element::Fe,
            solutes: vec![(Element::C, carbon_pct)],
        }
    }

    /// Pure copper: FCC Cu.
    pub fn copper() -> Self {
        Self {
            name: "copper".into(),
            lattice: LatticeType::FCC,
            base: Element::Cu,
            solutes: Vec::new(),
        }
    }

    /// Pure gold: FCC Au.
    pub fn gold() -> Self {
        Self {
            name: "gold".into(),
            lattice: LatticeType::FCC,
            base: Element::Au,
            solutes: Vec::new(),
        }
    }

    /// Pure aluminium: FCC Al.
    pub fn aluminum() -> Self {
        Self {
            name: "aluminum".into(),
            lattice: LatticeType::FCC,
            base: Element::Al,
            solutes: Vec::new(),
        }
    }

    /// Diamond: pure carbon, diamond cubic lattice.
    pub fn diamond() -> Self {
        Self {
            name: "diamond".into(),
            lattice: LatticeType::Diamond,
            base: Element::C,
            solutes: Vec::new(),
        }
    }
}
