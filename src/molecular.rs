//! Molecular structures: atoms connected by bonds.

use crate::element::Element;

/// A molecule: a named collection of atoms and bonds.
#[derive(Debug, Clone)]
pub struct Molecule {
    pub name: String,
    pub atoms: Vec<Element>,
    pub bonds: Vec<Bond>,
}

/// A bond between two atoms in a molecule.
#[derive(Debug, Clone)]
pub struct Bond {
    /// Indices into `Molecule::atoms`.
    pub between: (usize, usize),
    pub kind: BondKind,
    /// Bond dissociation energy in kJ/mol.
    pub energy: f32,
}

/// Classification of chemical bonds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BondKind {
    Single,
    Double,
    Triple,
    Hydrogen,
    Ionic,
    VanDerWaals,
    Metallic,
}

impl BondKind {
    /// Typical bond dissociation energy (kJ/mol) for a pair of elements.
    pub fn typical_energy(a: Element, b: Element, kind: BondKind) -> f32 {
        match kind {
            BondKind::Single => {
                // Common single-bond energies
                match (a, b) {
                    (Element::H, Element::H) => 436.0,
                    (Element::O, Element::H) | (Element::H, Element::O) => 463.0,
                    (Element::C, Element::H) | (Element::H, Element::C) => 413.0,
                    (Element::C, Element::C) => 347.0,
                    (Element::C, Element::O) | (Element::O, Element::C) => 358.0,
                    (Element::C, Element::N) | (Element::N, Element::C) => 305.0,
                    (Element::N, Element::H) | (Element::H, Element::N) => 391.0,
                    (Element::O, Element::O) => 146.0,
                    _ => 350.0, // reasonable default for unknown pairs
                }
            }
            BondKind::Double => {
                match (a, b) {
                    (Element::C, Element::C) => 614.0,
                    (Element::C, Element::O) | (Element::O, Element::C) => 799.0,
                    (Element::O, Element::O) => 498.0,
                    (Element::C, Element::N) | (Element::N, Element::C) => 615.0,
                    _ => 600.0,
                }
            }
            BondKind::Triple => {
                match (a, b) {
                    (Element::C, Element::C) => 839.0,
                    (Element::N, Element::N) => 946.0,
                    (Element::C, Element::N) | (Element::N, Element::C) => 891.0,
                    _ => 800.0,
                }
            }
            BondKind::Hydrogen => 20.0,
            BondKind::Ionic => 700.0,
            BondKind::VanDerWaals => 5.0,
            BondKind::Metallic => {
                // Use average cohesive energy of the two elements
                let avg = (a.cohesive_energy() + b.cohesive_energy()) * 0.5;
                avg * 96.485 // eV → kJ/mol
            }
        }
    }
}

impl Molecule {
    /// Total molecular mass in daltons.
    pub fn molecular_mass(&self) -> f32 {
        self.atoms.iter().map(|a| a.atomic_mass()).sum()
    }

    /// Sum of all bond dissociation energies (kJ/mol).
    pub fn total_bond_energy(&self) -> f32 {
        self.bonds.iter().map(|b| b.energy).sum()
    }

    /// Whether this molecule can form hydrogen bonds.
    /// Checks for O-H, N-H, or F-H bonds.
    pub fn has_hydrogen_bonds(&self) -> bool {
        self.bonds.iter().any(|b| {
            let a = self.atoms[b.between.0];
            let b_elem = self.atoms[b.between.1];
            (a == Element::H && matches!(b_elem, Element::O | Element::N | Element::F))
                || (b_elem == Element::H && matches!(a, Element::O | Element::N | Element::F))
        })
    }

    /// Water: H-O-H
    pub fn water() -> Self {
        let atoms = vec![Element::O, Element::H, Element::H];
        let bonds = vec![
            Bond {
                between: (0, 1),
                kind: BondKind::Single,
                energy: BondKind::typical_energy(Element::O, Element::H, BondKind::Single),
            },
            Bond {
                between: (0, 2),
                kind: BondKind::Single,
                energy: BondKind::typical_energy(Element::O, Element::H, BondKind::Single),
            },
        ];
        Self { name: "water".into(), atoms, bonds }
    }

    /// Methane: CH4
    pub fn methane() -> Self {
        let atoms = vec![Element::C, Element::H, Element::H, Element::H, Element::H];
        let bonds = (1..5)
            .map(|i| Bond {
                between: (0, i),
                kind: BondKind::Single,
                energy: BondKind::typical_energy(Element::C, Element::H, BondKind::Single),
            })
            .collect();
        Self { name: "methane".into(), atoms, bonds }
    }

    /// Carbon dioxide: O=C=O
    pub fn carbon_dioxide() -> Self {
        let atoms = vec![Element::C, Element::O, Element::O];
        let bonds = vec![
            Bond {
                between: (0, 1),
                kind: BondKind::Double,
                energy: BondKind::typical_energy(Element::C, Element::O, BondKind::Double),
            },
            Bond {
                between: (0, 2),
                kind: BondKind::Double,
                energy: BondKind::typical_energy(Element::C, Element::O, BondKind::Double),
            },
        ];
        Self { name: "carbon_dioxide".into(), atoms, bonds }
    }

    /// Molecular oxygen: O=O
    pub fn oxygen_gas() -> Self {
        let atoms = vec![Element::O, Element::O];
        let bonds = vec![Bond {
            between: (0, 1),
            kind: BondKind::Double,
            energy: BondKind::typical_energy(Element::O, Element::O, BondKind::Double),
        }];
        Self { name: "oxygen".into(), atoms, bonds }
    }

    /// Molecular hydrogen: H-H
    pub fn hydrogen_gas() -> Self {
        let atoms = vec![Element::H, Element::H];
        let bonds = vec![Bond {
            between: (0, 1),
            kind: BondKind::Single,
            energy: BondKind::typical_energy(Element::H, Element::H, BondKind::Single),
        }];
        Self { name: "hydrogen".into(), atoms, bonds }
    }

    /// Carbon monoxide: C≡O
    pub fn carbon_monoxide() -> Self {
        let atoms = vec![Element::C, Element::O];
        let bonds = vec![Bond {
            between: (0, 1),
            kind: BondKind::Triple,
            energy: 1072.0, // C≡O is one of the strongest bonds
        }];
        Self { name: "carbon_monoxide".into(), atoms, bonds }
    }

    /// Sulfur dioxide: O=S=O
    pub fn sulfur_dioxide() -> Self {
        let atoms = vec![Element::O, Element::S, Element::O];
        let e = BondKind::typical_energy(Element::S, Element::O, BondKind::Double);
        let bonds = vec![
            Bond { between: (0, 1), kind: BondKind::Double, energy: e },
            Bond { between: (1, 2), kind: BondKind::Double, energy: e },
        ];
        Self { name: "sulfur_dioxide".into(), atoms, bonds }
    }

    /// Glucose: C6H12O6 (simplified — total bond energy approximated)
    pub fn glucose() -> Self {
        // Full glucose has 24 atoms and ~23 bonds. We approximate with
        // the major bonds: 5 C-C, 7 C-H, 5 C-O, 5 O-H
        let mut atoms = Vec::new();
        for _ in 0..6 { atoms.push(Element::C); }
        for _ in 0..12 { atoms.push(Element::H); }
        for _ in 0..6 { atoms.push(Element::O); }

        let cc = BondKind::typical_energy(Element::C, Element::C, BondKind::Single);
        let ch = BondKind::typical_energy(Element::C, Element::H, BondKind::Single);
        let co = BondKind::typical_energy(Element::C, Element::O, BondKind::Single);
        let oh = BondKind::typical_energy(Element::O, Element::H, BondKind::Single);

        // Approximate bond list (not structurally exact, but energy-correct)
        let mut bonds = Vec::new();
        // 5 C-C bonds (ring + chain)
        for i in 0..5 {
            bonds.push(Bond { between: (i, i + 1), kind: BondKind::Single, energy: cc });
        }
        // 7 C-H bonds
        for i in 0..7 {
            bonds.push(Bond { between: (i % 6, 6 + i), kind: BondKind::Single, energy: ch });
        }
        // 5 C-O bonds
        for i in 0..5 {
            bonds.push(Bond { between: (i, 18 + i), kind: BondKind::Single, energy: co });
        }
        // 5 O-H bonds
        for i in 0..5 {
            bonds.push(Bond { between: (18 + i, 13 + i), kind: BondKind::Single, energy: oh });
        }

        Self { name: "glucose".into(), atoms, bonds }
    }

    /// Ethanol: C2H5OH (simplified)
    pub fn ethanol() -> Self {
        let atoms = vec![
            Element::C, Element::C,     // 0, 1
            Element::H, Element::H, Element::H, // 2, 3, 4
            Element::H, Element::H,     // 5, 6
            Element::O,                  // 7
            Element::H,                  // 8
        ];

        let cc = BondKind::typical_energy(Element::C, Element::C, BondKind::Single);
        let ch = BondKind::typical_energy(Element::C, Element::H, BondKind::Single);
        let co = BondKind::typical_energy(Element::C, Element::O, BondKind::Single);
        let oh = BondKind::typical_energy(Element::O, Element::H, BondKind::Single);

        let bonds = vec![
            Bond { between: (0, 1), kind: BondKind::Single, energy: cc },
            Bond { between: (0, 2), kind: BondKind::Single, energy: ch },
            Bond { between: (0, 3), kind: BondKind::Single, energy: ch },
            Bond { between: (0, 4), kind: BondKind::Single, energy: ch },
            Bond { between: (1, 5), kind: BondKind::Single, energy: ch },
            Bond { between: (1, 6), kind: BondKind::Single, energy: ch },
            Bond { between: (1, 7), kind: BondKind::Single, energy: co },
            Bond { between: (7, 8), kind: BondKind::Single, energy: oh },
        ];

        Self { name: "ethanol".into(), atoms, bonds }
    }
}
