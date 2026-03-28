//! Periodic table with atomic properties relevant to material science.
//! All 118 elements from Hydrogen (H) to Oganesson (Og).
//!
//! Data sources:
//! - Atomic masses: IUPAC 2021 standard atomic weights
//! - Atomic radii: Empirical/calculated values (pm)
//! - Electronegativity: Pauling scale (0.0 where no data exists)
//! - Cohesive energies: Kittel "Introduction to Solid State Physics" / CRC Handbook
//! - Melting points: CRC Handbook of Chemistry and Physics

/// All 118 elements of the periodic table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    // Period 1
    H, He,
    // Period 2
    Li, Be, B, C, N, O, F, Ne,
    // Period 3
    Na, Mg, Al, Si, P, S, Cl, Ar,
    // Period 4
    K, Ca, Sc, Ti, V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
    // Period 5
    Rb, Sr, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I, Xe,
    // Period 6
    Cs, Ba,
    La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu,
    Hf, Ta, W, Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
    // Period 7
    Fr, Ra,
    Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr,
    Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
}

impl Element {
    /// Atomic mass in daltons (g/mol). IUPAC 2021 standard atomic weights.
    /// For radioactive elements without stable isotopes, the mass number of the
    /// most stable or best-known isotope is used.
    pub fn atomic_mass(&self) -> f32 {
        match self {
            // Period 1
            Element::H  => 1.008,
            Element::He => 4.003,
            // Period 2
            Element::Li => 6.941,
            Element::Be => 9.012,
            Element::B  => 10.81,
            Element::C  => 12.011,
            Element::N  => 14.007,
            Element::O  => 15.999,
            Element::F  => 18.998,
            Element::Ne => 20.180,
            // Period 3
            Element::Na => 22.990,
            Element::Mg => 24.305,
            Element::Al => 26.982,
            Element::Si => 28.086,
            Element::P  => 30.974,
            Element::S  => 32.06,
            Element::Cl => 35.45,
            Element::Ar => 39.948,
            // Period 4
            Element::K  => 39.098,
            Element::Ca => 40.078,
            Element::Sc => 44.956,
            Element::Ti => 47.867,
            Element::V  => 50.942,
            Element::Cr => 51.996,
            Element::Mn => 54.938,
            Element::Fe => 55.845,
            Element::Co => 58.933,
            Element::Ni => 58.693,
            Element::Cu => 63.546,
            Element::Zn => 65.38,
            Element::Ga => 69.723,
            Element::Ge => 72.630,
            Element::As => 74.922,
            Element::Se => 78.971,
            Element::Br => 79.904,
            Element::Kr => 83.798,
            // Period 5
            Element::Rb => 85.468,
            Element::Sr => 87.62,
            Element::Y  => 88.906,
            Element::Zr => 91.224,
            Element::Nb => 92.906,
            Element::Mo => 95.95,
            Element::Tc => 98.0,    // radioactive, most stable isotope
            Element::Ru => 101.07,
            Element::Rh => 102.906,
            Element::Pd => 106.42,
            Element::Ag => 107.868,
            Element::Cd => 112.414,
            Element::In => 114.818,
            Element::Sn => 118.710,
            Element::Sb => 121.760,
            Element::Te => 127.60,
            Element::I  => 126.904,
            Element::Xe => 131.293,
            // Period 6
            Element::Cs => 132.905,
            Element::Ba => 137.327,
            Element::La => 138.905,
            Element::Ce => 140.116,
            Element::Pr => 140.908,
            Element::Nd => 144.242,
            Element::Pm => 145.0,   // radioactive
            Element::Sm => 150.36,
            Element::Eu => 151.964,
            Element::Gd => 157.25,
            Element::Tb => 158.925,
            Element::Dy => 162.500,
            Element::Ho => 164.930,
            Element::Er => 167.259,
            Element::Tm => 168.934,
            Element::Yb => 173.045,
            Element::Lu => 174.967,
            Element::Hf => 178.49,
            Element::Ta => 180.948,
            Element::W  => 183.84,
            Element::Re => 186.207,
            Element::Os => 190.23,
            Element::Ir => 192.217,
            Element::Pt => 195.084,
            Element::Au => 196.967,
            Element::Hg => 200.592,
            Element::Tl => 204.38,
            Element::Pb => 207.2,
            Element::Bi => 208.980,
            Element::Po => 209.0,   // radioactive
            Element::At => 210.0,   // radioactive
            Element::Rn => 222.0,   // radioactive
            // Period 7
            Element::Fr => 223.0,   // radioactive
            Element::Ra => 226.0,   // radioactive
            Element::Ac => 227.0,   // radioactive
            Element::Th => 232.038,
            Element::Pa => 231.036,
            Element::U  => 238.029,
            Element::Np => 237.0,   // radioactive
            Element::Pu => 244.0,   // radioactive
            Element::Am => 243.0,   // radioactive
            Element::Cm => 247.0,   // radioactive
            Element::Bk => 247.0,   // radioactive
            Element::Cf => 251.0,   // radioactive
            Element::Es => 252.0,   // radioactive
            Element::Fm => 257.0,   // radioactive
            Element::Md => 258.0,   // radioactive
            Element::No => 259.0,   // radioactive
            Element::Lr => 266.0,   // radioactive
            Element::Rf => 267.0,   // radioactive
            Element::Db => 268.0,   // radioactive
            Element::Sg => 269.0,   // radioactive
            Element::Bh => 270.0,   // radioactive
            Element::Hs => 277.0,   // radioactive
            Element::Mt => 278.0,   // radioactive
            Element::Ds => 281.0,   // radioactive
            Element::Rg => 282.0,   // radioactive
            Element::Cn => 285.0,   // radioactive
            Element::Nh => 286.0,   // radioactive
            Element::Fl => 289.0,   // radioactive
            Element::Mc => 290.0,   // radioactive
            Element::Lv => 293.0,   // radioactive
            Element::Ts => 294.0,   // radioactive
            Element::Og => 294.0,   // radioactive
        }
    }

    /// Atomic radius in picometers (empirical / calculated).
    /// For elements without experimental data, calculated or estimated values are used.
    pub fn atomic_radius(&self) -> f32 {
        match self {
            // Period 1
            Element::H  => 25.0,
            Element::He => 31.0,
            // Period 2
            Element::Li => 145.0,
            Element::Be => 105.0,
            Element::B  => 85.0,
            Element::C  => 70.0,
            Element::N  => 65.0,
            Element::O  => 60.0,
            Element::F  => 50.0,
            Element::Ne => 38.0,
            // Period 3
            Element::Na => 180.0,
            Element::Mg => 150.0,
            Element::Al => 143.0,
            Element::Si => 110.0,
            Element::P  => 100.0,
            Element::S  => 100.0,
            Element::Cl => 100.0,
            Element::Ar => 71.0,
            // Period 4
            Element::K  => 220.0,
            Element::Ca => 180.0,
            Element::Sc => 160.0,
            Element::Ti => 140.0,
            Element::V  => 135.0,
            Element::Cr => 128.0,
            Element::Mn => 140.0,
            Element::Fe => 126.0,
            Element::Co => 125.0,
            Element::Ni => 124.0,
            Element::Cu => 128.0,
            Element::Zn => 135.0,
            Element::Ga => 130.0,
            Element::Ge => 125.0,
            Element::As => 115.0,
            Element::Se => 115.0,
            Element::Br => 115.0,
            Element::Kr => 88.0,
            // Period 5
            Element::Rb => 235.0,
            Element::Sr => 200.0,
            Element::Y  => 180.0,
            Element::Zr => 155.0,
            Element::Nb => 145.0,
            Element::Mo => 145.0,
            Element::Tc => 135.0,  // estimated
            Element::Ru => 130.0,
            Element::Rh => 135.0,
            Element::Pd => 140.0,
            Element::Ag => 144.0,
            Element::Cd => 155.0,
            Element::In => 155.0,
            Element::Sn => 145.0,
            Element::Sb => 145.0,
            Element::Te => 140.0,
            Element::I  => 140.0,
            Element::Xe => 108.0,
            // Period 6
            Element::Cs => 260.0,
            Element::Ba => 215.0,
            Element::La => 195.0,
            Element::Ce => 185.0,
            Element::Pr => 185.0,
            Element::Nd => 185.0,
            Element::Pm => 185.0,  // estimated
            Element::Sm => 185.0,
            Element::Eu => 185.0,
            Element::Gd => 180.0,
            Element::Tb => 175.0,
            Element::Dy => 175.0,
            Element::Ho => 175.0,
            Element::Er => 175.0,
            Element::Tm => 175.0,
            Element::Yb => 175.0,
            Element::Lu => 175.0,
            Element::Hf => 155.0,
            Element::Ta => 145.0,
            Element::W  => 137.0,
            Element::Re => 135.0,
            Element::Os => 130.0,
            Element::Ir => 135.0,
            Element::Pt => 135.0,
            Element::Au => 144.0,
            Element::Hg => 150.0,
            Element::Tl => 190.0,
            Element::Pb => 180.0,
            Element::Bi => 160.0,
            Element::Po => 190.0,  // estimated
            Element::At => 150.0,  // estimated
            Element::Rn => 120.0,  // estimated
            // Period 7
            Element::Fr => 260.0,  // estimated
            Element::Ra => 215.0,  // estimated
            Element::Ac => 195.0,
            Element::Th => 180.0,
            Element::Pa => 180.0,
            Element::U  => 175.0,
            Element::Np => 175.0,
            Element::Pu => 175.0,
            Element::Am => 175.0,
            Element::Cm => 176.0,  // estimated
            Element::Bk => 176.0,  // estimated
            Element::Cf => 176.0,  // estimated
            Element::Es => 176.0,  // estimated
            Element::Fm => 176.0,  // estimated
            Element::Md => 176.0,  // estimated
            Element::No => 176.0,  // estimated
            Element::Lr => 176.0,  // estimated
            // Transactinides (104-118) — estimated/theoretical
            Element::Rf => 157.0,
            Element::Db => 149.0,
            Element::Sg => 143.0,
            Element::Bh => 141.0,
            Element::Hs => 134.0,
            Element::Mt => 129.0,
            Element::Ds => 128.0,
            Element::Rg => 121.0,
            Element::Cn => 122.0,
            Element::Nh => 136.0,
            Element::Fl => 143.0,
            Element::Mc => 162.0,
            Element::Lv => 175.0,
            Element::Ts => 165.0,
            Element::Og => 157.0,
        }
    }

    /// Pauling electronegativity.
    /// Returns 0.0 for noble gases and elements without measured values.
    pub fn electronegativity(&self) -> f32 {
        match self {
            // Period 1
            Element::H  => 2.20,
            Element::He => 0.0,
            // Period 2
            Element::Li => 0.98,
            Element::Be => 1.57,
            Element::B  => 2.04,
            Element::C  => 2.55,
            Element::N  => 3.04,
            Element::O  => 3.44,
            Element::F  => 3.98,
            Element::Ne => 0.0,
            // Period 3
            Element::Na => 0.93,
            Element::Mg => 1.31,
            Element::Al => 1.61,
            Element::Si => 1.90,
            Element::P  => 2.19,
            Element::S  => 2.58,
            Element::Cl => 3.16,
            Element::Ar => 0.0,
            // Period 4
            Element::K  => 0.82,
            Element::Ca => 1.00,
            Element::Sc => 1.36,
            Element::Ti => 1.54,
            Element::V  => 1.63,
            Element::Cr => 1.66,
            Element::Mn => 1.55,
            Element::Fe => 1.83,
            Element::Co => 1.88,
            Element::Ni => 1.91,
            Element::Cu => 1.90,
            Element::Zn => 1.65,
            Element::Ga => 1.81,
            Element::Ge => 2.01,
            Element::As => 2.18,
            Element::Se => 2.55,
            Element::Br => 2.96,
            Element::Kr => 0.0,
            // Period 5
            Element::Rb => 0.82,
            Element::Sr => 0.95,
            Element::Y  => 1.22,
            Element::Zr => 1.33,
            Element::Nb => 1.60,
            Element::Mo => 2.16,
            Element::Tc => 1.90,  // estimated
            Element::Ru => 2.20,
            Element::Rh => 2.28,
            Element::Pd => 2.20,
            Element::Ag => 1.93,
            Element::Cd => 1.69,
            Element::In => 1.78,
            Element::Sn => 1.96,
            Element::Sb => 2.05,
            Element::Te => 2.10,
            Element::I  => 2.66,
            Element::Xe => 0.0,
            // Period 6
            Element::Cs => 0.79,
            Element::Ba => 0.89,
            Element::La => 1.10,
            Element::Ce => 1.12,
            Element::Pr => 1.13,
            Element::Nd => 1.14,
            Element::Pm => 1.13,  // estimated
            Element::Sm => 1.17,
            Element::Eu => 1.20,
            Element::Gd => 1.20,
            Element::Tb => 1.10,  // estimated
            Element::Dy => 1.22,
            Element::Ho => 1.23,
            Element::Er => 1.24,
            Element::Tm => 1.25,
            Element::Yb => 1.10,  // estimated
            Element::Lu => 1.27,
            Element::Hf => 1.30,
            Element::Ta => 1.50,
            Element::W  => 2.36,
            Element::Re => 1.90,
            Element::Os => 2.20,
            Element::Ir => 2.20,
            Element::Pt => 2.28,
            Element::Au => 2.54,
            Element::Hg => 2.00,
            Element::Tl => 1.62,
            Element::Pb => 2.33,
            Element::Bi => 2.02,
            Element::Po => 2.00,  // estimated
            Element::At => 2.20,  // estimated
            Element::Rn => 0.0,
            // Period 7
            Element::Fr => 0.70,  // estimated
            Element::Ra => 0.90,
            Element::Ac => 1.10,
            Element::Th => 1.30,
            Element::Pa => 1.50,
            Element::U  => 1.38,
            Element::Np => 1.36,
            Element::Pu => 1.28,
            Element::Am => 1.30,  // estimated
            Element::Cm => 1.30,  // estimated
            Element::Bk => 1.30,  // estimated
            Element::Cf => 1.30,  // estimated
            Element::Es => 1.30,  // estimated
            Element::Fm => 1.30,  // estimated
            Element::Md => 1.30,  // estimated
            Element::No => 1.30,  // estimated
            Element::Lr => 1.30,  // estimated
            // Transactinides (104-118) — estimated/theoretical
            Element::Rf => 0.0,
            Element::Db => 0.0,
            Element::Sg => 0.0,
            Element::Bh => 0.0,
            Element::Hs => 0.0,
            Element::Mt => 0.0,
            Element::Ds => 0.0,
            Element::Rg => 0.0,
            Element::Cn => 0.0,
            Element::Nh => 0.0,
            Element::Fl => 0.0,
            Element::Mc => 0.0,
            Element::Lv => 0.0,
            Element::Ts => 0.0,
            Element::Og => 0.0,
        }
    }

    /// Electrical resistivity at 20°C in μΩ·cm.
    /// Used for Wiedemann-Franz thermal conductivity estimation.
    pub fn electrical_resistivity(&self) -> Option<f32> {
        match self {
            Element::Ag => Some(1.59),   // silver — best conductor
            Element::Cu => Some(1.68),   // copper
            Element::Au => Some(2.21),   // gold
            Element::Al => Some(2.65),   // aluminum
            Element::W  => Some(5.28),   // tungsten
            Element::Fe => Some(9.71),   // iron
            Element::Ni => Some(6.99),   // nickel
            Element::Zn => Some(5.90),   // zinc
            Element::Sn => Some(11.0),   // tin
            Element::Pb => Some(20.6),   // lead
            Element::Ti => Some(42.0),   // titanium
            Element::Cr => Some(12.5),   // chromium
            Element::Mn => Some(144.0),  // manganese — poor conductor
            Element::Co => Some(6.24),   // cobalt
            Element::Pt => Some(10.5),   // platinum
            _ => None,  // non-metals or unknown
        }
    }

    /// Whether this element is a metal.
    /// Metals include: alkali, alkaline earth, transition metals, post-transition metals,
    /// lanthanides, and actinides.
    pub fn is_metal(&self) -> bool {
        match self {
            // Non-metals
            Element::H | Element::He
            | Element::C | Element::N | Element::O | Element::F | Element::Ne
            | Element::P | Element::S | Element::Cl | Element::Ar
            | Element::Se | Element::Br | Element::Kr
            | Element::I | Element::Xe
            | Element::At | Element::Rn  // astatine is sometimes metalloid, radon is noble gas
            | Element::Ts | Element::Og  // predicted non-metals/noble gas
            => false,
            // Metalloids (treated as non-metals for this classification)
            Element::B | Element::Si | Element::Ge | Element::As
            | Element::Sb | Element::Te
            => false,
            // Everything else is a metal
            _ => true,
        }
    }

    /// Characteristic display colour (linear sRGB + alpha).
    pub fn base_color(&self) -> [f32; 4] {
        match self {
            // Alkali metals — soft silvery
            Element::Li | Element::Na | Element::K | Element::Rb
            | Element::Cs | Element::Fr => [0.75, 0.75, 0.73, 1.0],
            // Alkaline earth — light silvery
            Element::Be | Element::Mg | Element::Ca | Element::Sr
            | Element::Ba | Element::Ra => [0.80, 0.80, 0.78, 1.0],
            // Specific well-known element colors
            Element::Fe => [0.56, 0.55, 0.54, 1.0],  // gray steel
            Element::Cu => [0.72, 0.45, 0.20, 1.0],  // copper
            Element::Au => [1.00, 0.84, 0.00, 1.0],  // gold
            Element::Ag => [0.75, 0.75, 0.75, 1.0],  // silver
            Element::Al => [0.77, 0.79, 0.82, 1.0],  // aluminium
            Element::C  => [0.15, 0.15, 0.15, 1.0],  // carbon black
            Element::Ti => [0.62, 0.62, 0.64, 1.0],  // titanium
            Element::Cr => [0.55, 0.56, 0.59, 1.0],  // chrome
            Element::Ni => [0.66, 0.66, 0.60, 1.0],  // nickel
            Element::W  => [0.50, 0.50, 0.52, 1.0],  // tungsten
            Element::Pb => [0.34, 0.35, 0.38, 1.0],  // lead
            Element::Sn => [0.73, 0.73, 0.71, 1.0],  // tin
            Element::Co => [0.44, 0.44, 0.52, 1.0],  // cobalt (bluish gray)
            Element::Mn => [0.48, 0.47, 0.46, 1.0],  // manganese
            Element::Zn => [0.67, 0.69, 0.70, 1.0],  // zinc
            Element::S  => [0.90, 0.85, 0.20, 1.0],  // sulfur yellow
            Element::P  => [0.80, 0.80, 0.80, 1.0],  // white phosphorus
            Element::Bi => [0.62, 0.55, 0.60, 1.0],  // bismuth pinkish
            Element::Os => [0.50, 0.50, 0.58, 1.0],  // osmium bluish
            Element::Pt => [0.70, 0.70, 0.72, 1.0],  // platinum
            Element::Hg => [0.72, 0.72, 0.73, 1.0],  // mercury silvery
            Element::Br => [0.60, 0.20, 0.10, 1.0],  // bromine reddish brown
            Element::I  => [0.40, 0.20, 0.50, 1.0],  // iodine violet
            Element::N  => [0.20, 0.30, 0.90, 1.0],  // nitrogen (gas, bluish for viz)
            Element::O  => [0.90, 0.20, 0.20, 1.0],  // oxygen (red for viz)
            Element::H  => [0.90, 0.90, 0.90, 1.0],  // hydrogen (white for viz)
            Element::He => [0.85, 0.93, 1.00, 1.0],  // helium pale blue
            Element::F  => [0.70, 0.90, 0.70, 1.0],  // fluorine pale green
            Element::Ne => [0.90, 0.50, 0.30, 1.0],  // neon orange glow
            Element::Cl => [0.50, 0.80, 0.30, 1.0],  // chlorine greenish
            Element::Ar => [0.60, 0.40, 0.70, 1.0],  // argon purple glow
            Element::Se => [0.60, 0.40, 0.40, 1.0],  // selenium gray-red
            Element::Kr | Element::Xe | Element::Rn | Element::Og
                => [0.50, 0.50, 0.60, 1.0],           // noble gases grayish
            // Lanthanides — silvery white
            Element::La | Element::Ce | Element::Pr | Element::Nd | Element::Pm
            | Element::Sm | Element::Eu | Element::Gd | Element::Tb | Element::Dy
            | Element::Ho | Element::Er | Element::Tm | Element::Yb | Element::Lu
                => [0.70, 0.70, 0.68, 1.0],
            // Actinides — silvery to dull gray
            Element::Ac | Element::Th | Element::Pa | Element::U | Element::Np
            | Element::Pu | Element::Am | Element::Cm | Element::Bk | Element::Cf
            | Element::Es | Element::Fm | Element::Md | Element::No | Element::Lr
                => [0.55, 0.55, 0.55, 1.0],
            // Generic fallback for remaining elements
            _ => [0.50, 0.50, 0.50, 1.0],
        }
    }

    /// Cohesive energy in eV/atom.
    /// For crystalline solids: energy to separate all atoms from the bulk.
    /// For non-metals: atomization energy of the standard state.
    /// For synthetic/superheavy elements: estimated values.
    pub fn cohesive_energy(&self) -> f32 {
        match self {
            // Period 1
            Element::H  => 2.26,   // H2 bond: 4.52 eV / 2
            Element::He => 0.0,    // noble gas
            // Period 2
            Element::Li => 1.63,
            Element::Be => 3.32,
            Element::B  => 5.81,
            Element::C  => 7.37,   // diamond
            Element::N  => 4.92,   // N2 bond: 9.84 / 2
            Element::O  => 2.60,   // O2 bond: 5.20 / 2
            Element::F  => 0.82,   // F2 bond: 1.64 / 2
            Element::Ne => 0.002,
            // Period 3
            Element::Na => 1.11,
            Element::Mg => 1.51,
            Element::Al => 3.39,
            Element::Si => 4.63,
            Element::P  => 3.43,
            Element::S  => 2.85,
            Element::Cl => 1.26,
            Element::Ar => 0.08,
            // Period 4
            Element::K  => 0.93,
            Element::Ca => 1.84,
            Element::Sc => 3.90,
            Element::Ti => 4.85,
            Element::V  => 5.31,
            Element::Cr => 4.10,
            Element::Mn => 2.92,
            Element::Fe => 4.28,
            Element::Co => 4.39,
            Element::Ni => 4.44,
            Element::Cu => 3.49,
            Element::Zn => 1.35,
            Element::Ga => 2.81,
            Element::Ge => 3.85,
            Element::As => 2.96,
            Element::Se => 2.46,
            Element::Br => 1.22,
            Element::Kr => 0.012,
            // Period 5
            Element::Rb => 0.85,
            Element::Sr => 1.72,
            Element::Y  => 4.37,
            Element::Zr => 6.25,
            Element::Nb => 7.57,
            Element::Mo => 6.82,
            Element::Tc => 6.85,   // estimated
            Element::Ru => 6.74,
            Element::Rh => 5.75,
            Element::Pd => 3.89,
            Element::Ag => 2.95,
            Element::Cd => 1.16,
            Element::In => 2.52,
            Element::Sn => 3.14,
            Element::Sb => 2.75,
            Element::Te => 2.19,
            Element::I  => 1.11,
            Element::Xe => 0.016,
            // Period 6
            Element::Cs => 0.80,
            Element::Ba => 1.90,
            Element::La => 4.47,
            Element::Ce => 4.32,
            Element::Pr => 3.70,
            Element::Nd => 3.40,
            Element::Pm => 3.30,   // estimated
            Element::Sm => 2.14,
            Element::Eu => 1.86,
            Element::Gd => 4.14,
            Element::Tb => 4.05,
            Element::Dy => 3.04,
            Element::Ho => 3.14,
            Element::Er => 3.29,
            Element::Tm => 2.42,
            Element::Yb => 1.60,
            Element::Lu => 4.43,
            Element::Hf => 6.44,
            Element::Ta => 8.10,
            Element::W  => 8.90,
            Element::Re => 8.03,
            Element::Os => 8.17,
            Element::Ir => 6.94,
            Element::Pt => 5.84,
            Element::Au => 3.81,
            Element::Hg => 0.67,
            Element::Tl => 1.88,
            Element::Pb => 2.03,
            Element::Bi => 2.18,
            Element::Po => 1.50,   // estimated
            Element::At => 1.00,   // estimated
            Element::Rn => 0.02,   // estimated, noble gas
            // Period 7
            Element::Fr => 0.75,   // estimated
            Element::Ra => 1.66,   // estimated
            Element::Ac => 4.25,   // estimated
            Element::Th => 5.93,
            Element::Pa => 5.20,   // estimated
            Element::U  => 5.55,
            Element::Np => 4.73,   // estimated
            Element::Pu => 3.60,   // estimated
            Element::Am => 2.73,   // estimated
            Element::Cm => 3.99,   // estimated
            Element::Bk => 3.30,   // estimated
            Element::Cf => 2.42,   // estimated
            Element::Es => 1.80,   // estimated
            Element::Fm => 1.60,   // estimated
            Element::Md => 1.30,   // estimated
            Element::No => 1.20,   // estimated
            Element::Lr => 3.50,   // estimated
            // Transactinides (104-118) — theoretical estimates
            Element::Rf => 6.00,
            Element::Db => 7.00,
            Element::Sg => 7.50,
            Element::Bh => 7.00,
            Element::Hs => 6.50,
            Element::Mt => 6.00,
            Element::Ds => 5.00,
            Element::Rg => 4.00,
            Element::Cn => 0.50,   // predicted weak bonding
            Element::Nh => 1.50,
            Element::Fl => 0.50,   // predicted weak bonding
            Element::Mc => 1.50,
            Element::Lv => 1.00,
            Element::Ts => 0.80,
            Element::Og => 0.02,   // predicted noble-gas-like
        }
    }

    /// Melting point of the pure element in degrees Celsius.
    /// For synthetic/radioactive elements: estimated or theoretical values.
    pub fn melting_point_pure(&self) -> f32 {
        match self {
            // Period 1
            Element::H  => -259.16,
            Element::He => -272.20,  // at 26 atm; He has no triple point at 1 atm
            // Period 2
            Element::Li => 180.5,
            Element::Be => 1287.0,
            Element::B  => 2076.0,
            Element::C  => 3550.0,   // sublimation point (diamond)
            Element::N  => -210.0,
            Element::O  => -218.79,
            Element::F  => -219.67,
            Element::Ne => -248.59,
            // Period 3
            Element::Na => 97.794,
            Element::Mg => 650.0,
            Element::Al => 660.32,
            Element::Si => 1414.0,
            Element::P  => 44.15,    // white phosphorus
            Element::S  => 115.21,
            Element::Cl => -101.5,
            Element::Ar => -189.34,
            // Period 4
            Element::K  => 63.38,
            Element::Ca => 842.0,
            Element::Sc => 1541.0,
            Element::Ti => 1668.0,
            Element::V  => 1910.0,
            Element::Cr => 1907.0,
            Element::Mn => 1246.0,
            Element::Fe => 1538.0,
            Element::Co => 1495.0,
            Element::Ni => 1455.0,
            Element::Cu => 1084.62,
            Element::Zn => 419.53,
            Element::Ga => 29.76,
            Element::Ge => 938.25,
            Element::As => 817.0,    // sublimation
            Element::Se => 221.0,
            Element::Br => -7.2,
            Element::Kr => -157.37,
            // Period 5
            Element::Rb => 39.31,
            Element::Sr => 777.0,
            Element::Y  => 1526.0,
            Element::Zr => 1855.0,
            Element::Nb => 2477.0,
            Element::Mo => 2623.0,
            Element::Tc => 2157.0,   // estimated
            Element::Ru => 2334.0,
            Element::Rh => 1964.0,
            Element::Pd => 1554.9,
            Element::Ag => 961.78,
            Element::Cd => 321.07,
            Element::In => 156.6,
            Element::Sn => 231.93,
            Element::Sb => 630.63,
            Element::Te => 449.51,
            Element::I  => 113.7,
            Element::Xe => -111.75,
            // Period 6
            Element::Cs => 28.44,
            Element::Ba => 727.0,
            Element::La => 920.0,
            Element::Ce => 795.0,
            Element::Pr => 935.0,
            Element::Nd => 1024.0,
            Element::Pm => 1042.0,   // estimated
            Element::Sm => 1072.0,
            Element::Eu => 826.0,
            Element::Gd => 1312.0,
            Element::Tb => 1356.0,
            Element::Dy => 1407.0,
            Element::Ho => 1461.0,
            Element::Er => 1529.0,
            Element::Tm => 1545.0,
            Element::Yb => 824.0,
            Element::Lu => 1652.0,
            Element::Hf => 2233.0,
            Element::Ta => 3017.0,
            Element::W  => 3422.0,
            Element::Re => 3186.0,
            Element::Os => 3033.0,
            Element::Ir => 2446.0,
            Element::Pt => 1768.3,
            Element::Au => 1064.18,
            Element::Hg => -38.83,
            Element::Tl => 304.0,
            Element::Pb => 327.46,
            Element::Bi => 271.4,
            Element::Po => 254.0,    // estimated
            Element::At => 302.0,    // estimated
            Element::Rn => -71.0,    // estimated
            // Period 7
            Element::Fr => 27.0,     // estimated
            Element::Ra => 700.0,
            Element::Ac => 1050.0,
            Element::Th => 1750.0,
            Element::Pa => 1572.0,
            Element::U  => 1132.2,
            Element::Np => 644.0,
            Element::Pu => 639.4,
            Element::Am => 1176.0,
            Element::Cm => 1345.0,
            Element::Bk => 986.0,    // estimated
            Element::Cf => 900.0,    // estimated
            Element::Es => 860.0,    // estimated
            Element::Fm => 1527.0,   // estimated
            Element::Md => 827.0,    // estimated
            Element::No => 827.0,    // estimated
            Element::Lr => 1627.0,   // estimated
            // Transactinides (104-118) — theoretical/estimated
            Element::Rf => 2100.0,   // estimated
            Element::Db => 2400.0,   // estimated
            Element::Sg => 2500.0,   // estimated
            Element::Bh => 2400.0,   // estimated
            Element::Hs => 2200.0,   // estimated
            Element::Mt => 2000.0,   // estimated
            Element::Ds => 1800.0,   // estimated
            Element::Rg => 1500.0,   // estimated
            Element::Cn => -40.0,    // predicted, possibly liquid near room temp
            Element::Nh => 430.0,    // estimated
            Element::Fl => -70.0,    // predicted, possibly gas near room temp
            Element::Mc => 400.0,    // estimated
            Element::Lv => 300.0,    // estimated
            Element::Ts => 350.0,    // estimated
            Element::Og => -60.0,    // predicted, noble-gas-like
        }
    }

    /// Boiling point of the pure element in degrees Celsius.
    /// Data: CRC Handbook of Chemistry and Physics, 97th ed.
    /// For synthetic/radioactive elements: estimated or theoretical values.
    pub fn boiling_point_pure(&self) -> f32 {
        match self {
            // Period 1
            Element::H  => -252.87,
            Element::He => -268.93,
            // Period 2
            Element::Li => 1342.0,
            Element::Be => 2469.0,
            Element::B  => 3927.0,
            Element::C  => 4027.0,   // sublimation
            Element::N  => -195.79,
            Element::O  => -182.96,
            Element::F  => -188.11,
            Element::Ne => -246.08,
            // Period 3
            Element::Na => 882.94,
            Element::Mg => 1091.0,
            Element::Al => 2519.0,
            Element::Si => 3265.0,
            Element::P  => 280.5,    // white phosphorus
            Element::S  => 444.6,
            Element::Cl => -34.04,
            Element::Ar => -185.85,
            // Period 4
            Element::K  => 759.0,
            Element::Ca => 1484.0,
            Element::Sc => 2836.0,
            Element::Ti => 3287.0,
            Element::V  => 3407.0,
            Element::Cr => 2671.0,
            Element::Mn => 2061.0,
            Element::Fe => 2862.0,
            Element::Co => 2927.0,
            Element::Ni => 2913.0,
            Element::Cu => 2562.0,
            Element::Zn => 907.0,
            Element::Ga => 2204.0,
            Element::Ge => 2833.0,
            Element::As => 614.0,    // sublimation
            Element::Se => 685.0,
            Element::Br => 58.8,
            Element::Kr => -153.22,
            // Period 5
            Element::Rb => 688.0,
            Element::Sr => 1382.0,
            Element::Y  => 3345.0,
            Element::Zr => 4409.0,
            Element::Nb => 4744.0,
            Element::Mo => 4639.0,
            Element::Tc => 4265.0,   // estimated
            Element::Ru => 4150.0,
            Element::Rh => 3695.0,
            Element::Pd => 2963.0,
            Element::Ag => 2162.0,
            Element::Cd => 767.0,
            Element::In => 2072.0,
            Element::Sn => 2602.0,
            Element::Sb => 1587.0,
            Element::Te => 988.0,
            Element::I  => 184.3,
            Element::Xe => -108.12,
            // Period 6
            Element::Cs => 671.0,
            Element::Ba => 1897.0,
            Element::La => 3464.0,
            Element::Ce => 3443.0,
            Element::Pr => 3520.0,
            Element::Nd => 3074.0,
            Element::Pm => 3000.0,   // estimated
            Element::Sm => 1794.0,
            Element::Eu => 1529.0,
            Element::Gd => 3273.0,
            Element::Tb => 3230.0,
            Element::Dy => 2567.0,
            Element::Ho => 2720.0,
            Element::Er => 2868.0,
            Element::Tm => 1950.0,
            Element::Yb => 1196.0,
            Element::Lu => 3402.0,
            Element::Hf => 4603.0,
            Element::Ta => 5458.0,
            Element::W  => 5555.0,
            Element::Re => 5596.0,
            Element::Os => 5012.0,
            Element::Ir => 4428.0,
            Element::Pt => 3825.0,
            Element::Au => 2856.0,
            Element::Hg => 356.73,
            Element::Tl => 1473.0,
            Element::Pb => 1749.0,
            Element::Bi => 1564.0,
            Element::Po => 962.0,    // estimated
            Element::At => 337.0,    // estimated
            Element::Rn => -61.7,    // estimated
            // Period 7
            Element::Fr => 677.0,    // estimated
            Element::Ra => 1737.0,
            Element::Ac => 3200.0,   // estimated
            Element::Th => 4788.0,
            Element::Pa => 4027.0,   // estimated
            Element::U  => 4131.0,
            Element::Np => 4000.0,   // estimated
            Element::Pu => 3228.0,
            Element::Am => 2011.0,
            Element::Cm => 3110.0,   // estimated
            Element::Bk => 2627.0,   // estimated
            Element::Cf => 1470.0,   // estimated
            Element::Es => 996.0,    // estimated
            Element::Fm => 1800.0,   // estimated
            Element::Md => 1100.0,   // estimated
            Element::No => 1100.0,   // estimated
            Element::Lr => 3500.0,   // estimated
            // Transactinides (104-118) — theoretical/estimated
            Element::Rf => 5500.0,   // estimated
            Element::Db => 5800.0,   // estimated
            Element::Sg => 5800.0,   // estimated
            Element::Bh => 5500.0,   // estimated
            Element::Hs => 5400.0,   // estimated
            Element::Mt => 5000.0,   // estimated
            Element::Ds => 4500.0,   // estimated
            Element::Rg => 3600.0,   // estimated
            Element::Cn => 80.0,     // predicted, low bp
            Element::Nh => 1100.0,   // estimated
            Element::Fl => -60.0,    // predicted, possibly gas
            Element::Mc => 1100.0,   // estimated
            Element::Lv => 800.0,    // estimated
            Element::Ts => 610.0,    // estimated
            Element::Og => -50.0,    // predicted, noble-gas-like
        }
    }
}
