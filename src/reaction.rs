//! Chemical reactions: energy balance from bond breaking/formation.
//!
//! Each reaction is defined by reactants and products as `Substance`s.
//! The reaction energy is computed from bond energies:
//!   energy = total_bond_energy(products) - total_bond_energy(reactants)
//!   positive = exothermic (releases heat), negative = endothermic (absorbs heat)

use crate::interaction::*;
use crate::material::MaterialId;

use crate::crystal::Crystal;
use crate::element::Element;
use crate::molecular::Molecule;
use crate::substance::Substance;

/// A chemical reaction described by reactants and products.
#[derive(Debug, Clone)]
pub struct Reaction {
    pub name: String,
    pub reactants: Vec<Substance>,
    pub products: Vec<Substance>,
    /// Activation energy in kJ/mol (energy barrier to start reaction).
    pub activation_energy: f32,
    /// Optional catalyst element.
    pub catalyst: Option<Element>,
}

impl Reaction {
    /// Reaction energy (kJ/mol): positive = exothermic, negative = endothermic.
    pub fn energy(&self) -> f32 {
        let products_energy: f32 = self.products.iter().map(|s| s.total_bond_energy()).sum();
        let reactants_energy: f32 = self.reactants.iter().map(|s| s.total_bond_energy()).sum();
        products_energy - reactants_energy
    }

    /// Is the reaction exothermic (releases heat)?
    pub fn is_exothermic(&self) -> bool {
        self.energy() > 0.0
    }

    /// Compile this reaction into an InteractionRule for hyle-core's runtime.
    ///
    /// The rule triggers when reactant materials are adjacent and temperature
    /// exceeds the activation energy threshold.
    ///
    /// `reactant_ids`: registered MaterialIds corresponding to `self.reactants`.
    /// `product_ids`: registered MaterialIds corresponding to `self.products`.
    pub fn to_interaction_rule(
        &self,
        reactant_ids: &[MaterialId],
        product_ids: &[MaterialId],
    ) -> InteractionRule {
        let temp_threshold = activation_energy_to_celsius(self.activation_energy);

        let trigger = InteractionTrigger::Adjacency {
            a: MaterialMatcher::Exact(reactant_ids[0]),
            b: if reactant_ids.len() > 1 {
                MaterialMatcher::Exact(reactant_ids[1])
            } else {
                MaterialMatcher::Exact(reactant_ids[0])
            },
        };

        let conditions = vec![InteractionCondition {
            target: ConditionTarget::A,
            property: MaterialProperty::Temperature,
            op: CompareOp::GreaterThan,
            threshold: temp_threshold,
        }];

        let heat_effect = if self.is_exothermic() {
            InteractionEffect::AddHeat {
                target: EffectTarget::Both,
                delta_celsius_per_tick: self.energy() * 0.01, // scale factor
            }
        } else {
            InteractionEffect::AddHeat {
                target: EffectTarget::Both,
                delta_celsius_per_tick: -self.energy().abs() * 0.01,
            }
        };

        let mut effects = vec![
            InteractionEffect::TransformA {
                into: MaterialTransform::Into(product_ids[0]),
            },
            heat_effect,
        ];

        // If there is a second product and a second reactant, transform B as well
        if product_ids.len() > 1 && reactant_ids.len() > 1 {
            effects.push(InteractionEffect::TransformB {
                into: MaterialTransform::Into(product_ids[1]),
            });
        }

        InteractionRule {
            id: self.name.clone(),
            trigger,
            conditions,
            effects,
            probability: 1.0,
        }
    }

    // =====================================================================
    // COMBUSTION REACTIONS
    // =====================================================================

    /// CH4 + 2O2 → CO2 + 2H2O  (methane combustion, natural gas burning)
    pub fn combustion_methane() -> Self {
        Self {
            name: "combustion_methane".into(),
            reactants: vec![
                Substance::Molecular(Molecule::methane()),
                Substance::Molecular(Molecule::oxygen_gas()),
                Substance::Molecular(Molecule::oxygen_gas()),
            ],
            products: vec![
                Substance::Molecular(Molecule::carbon_dioxide()),
                Substance::Molecular(Molecule::water()),
                Substance::Molecular(Molecule::water()),
            ],
            activation_energy: 150.0,
            catalyst: None,
        }
    }

    /// 2H2 + O2 → 2H2O  (hydrogen combustion, rocket fuel)
    pub fn combustion_hydrogen() -> Self {
        Self {
            name: "combustion_hydrogen".into(),
            reactants: vec![
                Substance::Molecular(Molecule::hydrogen_gas()),
                Substance::Molecular(Molecule::hydrogen_gas()),
                Substance::Molecular(Molecule::oxygen_gas()),
            ],
            products: vec![
                Substance::Molecular(Molecule::water()),
                Substance::Molecular(Molecule::water()),
            ],
            activation_energy: 75.0,
            catalyst: None,
        }
    }

    /// C + O2 → CO2  (coal/charcoal burning)
    pub fn combustion_carbon() -> Self {
        Self {
            name: "combustion_carbon".into(),
            reactants: vec![
                Substance::Crystalline(Crystal::diamond()), // using diamond as pure C
                Substance::Molecular(Molecule::oxygen_gas()),
            ],
            products: vec![Substance::Molecular(Molecule::carbon_dioxide())],
            activation_energy: 200.0,
            catalyst: None,
        }
    }

    // =====================================================================
    // OXIDATION / CORROSION
    // =====================================================================

    /// 4Fe + 3O2 → 2Fe2O3  (iron rusting)
    pub fn rusting() -> Self {
        Self {
            name: "rusting".into(),
            reactants: {
                let mut r: Vec<Substance> = (0..4)
                    .map(|_| Substance::Crystalline(Crystal::iron()))
                    .collect();
                for _ in 0..3 {
                    r.push(Substance::Molecular(Molecule::oxygen_gas()));
                }
                r
            },
            products: vec![
                Substance::amorphous(
                    "iron_oxide",
                    &[(Element::Fe, 0.70), (Element::O, 0.30)],
                    5240.0,
                ),
                Substance::amorphous(
                    "iron_oxide",
                    &[(Element::Fe, 0.70), (Element::O, 0.30)],
                    5240.0,
                ),
            ],
            activation_energy: 50.0,
            catalyst: None,
        }
    }

    /// 2Cu + O2 → 2CuO  (copper tarnishing/patina formation)
    pub fn copper_oxidation() -> Self {
        Self {
            name: "copper_oxidation".into(),
            reactants: vec![
                Substance::Crystalline(Crystal::copper()),
                Substance::Crystalline(Crystal::copper()),
                Substance::Molecular(Molecule::oxygen_gas()),
            ],
            products: vec![
                Substance::amorphous(
                    "copper_oxide",
                    &[(Element::Cu, 0.80), (Element::O, 0.20)],
                    6310.0,
                ),
                Substance::amorphous(
                    "copper_oxide",
                    &[(Element::Cu, 0.80), (Element::O, 0.20)],
                    6310.0,
                ),
            ],
            activation_energy: 40.0,
            catalyst: None,
        }
    }

    /// 4Al + 3O2 → 2Al2O3  (aluminum oxidation — instant protective layer)
    pub fn aluminum_oxidation() -> Self {
        Self {
            name: "aluminum_oxidation".into(),
            reactants: {
                let mut r: Vec<Substance> = (0..4)
                    .map(|_| Substance::Crystalline(Crystal::aluminum()))
                    .collect();
                for _ in 0..3 {
                    r.push(Substance::Molecular(Molecule::oxygen_gas()));
                }
                r
            },
            products: vec![
                Substance::amorphous(
                    "alumina",
                    &[(Element::Al, 0.53), (Element::O, 0.47)],
                    3950.0,
                ),
                Substance::amorphous(
                    "alumina",
                    &[(Element::Al, 0.53), (Element::O, 0.47)],
                    3950.0,
                ),
            ],
            activation_energy: 10.0, // very low — happens spontaneously
            catalyst: None,
        }
    }

    // =====================================================================
    // WATER / ACID-BASE
    // =====================================================================

    /// 2H2 + O2 → 2H2O  (water formation)
    pub fn water_formation() -> Self {
        Self::combustion_hydrogen() // same reaction
    }

    /// H2O → H2 + ½O2  (water electrolysis — endothermic)
    pub fn water_electrolysis() -> Self {
        Self {
            name: "water_electrolysis".into(),
            reactants: vec![
                Substance::Molecular(Molecule::water()),
                Substance::Molecular(Molecule::water()),
            ],
            products: vec![
                Substance::Molecular(Molecule::hydrogen_gas()),
                Substance::Molecular(Molecule::hydrogen_gas()),
                Substance::Molecular(Molecule::oxygen_gas()),
            ],
            activation_energy: 300.0, // needs electricity
            catalyst: None,
        }
    }

    // =====================================================================
    // THERMAL DECOMPOSITION
    // =====================================================================

    /// CaCO3 → CaO + CO2  (limestone calcination, cement production)
    pub fn calcination() -> Self {
        Self {
            name: "calcination".into(),
            reactants: vec![Substance::amorphous(
                "limestone",
                &[(Element::Ca, 0.40), (Element::C, 0.12), (Element::O, 0.48)],
                2710.0,
            )],
            products: vec![
                Substance::amorphous(
                    "quickite",
                    &[(Element::Ca, 0.71), (Element::O, 0.29)],
                    3340.0,
                ),
                Substance::Molecular(Molecule::carbon_dioxide()),
            ],
            activation_energy: 180.0, // ~900°C needed
            catalyst: None,
        }
    }

    // =====================================================================
    // METALLURGICAL
    // =====================================================================

    /// Fe2O3 + 3CO → 2Fe + 3CO2  (iron smelting from ore)
    pub fn iron_smelting() -> Self {
        Self {
            name: "iron_smelting".into(),
            reactants: vec![
                Substance::amorphous(
                    "iron_ore",
                    &[(Element::Fe, 0.70), (Element::O, 0.30)],
                    5240.0,
                ),
                Substance::Molecular(Molecule::carbon_monoxide()),
                Substance::Molecular(Molecule::carbon_monoxide()),
                Substance::Molecular(Molecule::carbon_monoxide()),
            ],
            products: vec![
                Substance::Crystalline(Crystal::iron()),
                Substance::Crystalline(Crystal::iron()),
                Substance::Molecular(Molecule::carbon_dioxide()),
                Substance::Molecular(Molecule::carbon_dioxide()),
                Substance::Molecular(Molecule::carbon_dioxide()),
            ],
            activation_energy: 250.0, // needs blast furnace temps
            catalyst: None,
        }
    }

    /// Cu2S + O2 → 2Cu + SO2  (copper smelting from sulfide ore)
    pub fn copper_smelting() -> Self {
        Self {
            name: "copper_smelting".into(),
            reactants: vec![
                Substance::amorphous(
                    "copper_sulfide",
                    &[(Element::Cu, 0.80), (Element::S, 0.20)],
                    5600.0,
                ),
                Substance::Molecular(Molecule::oxygen_gas()),
            ],
            products: vec![
                Substance::Crystalline(Crystal::copper()),
                Substance::Crystalline(Crystal::copper()),
                Substance::Molecular(Molecule::sulfur_dioxide()),
            ],
            activation_energy: 200.0,
            catalyst: None,
        }
    }

    // =====================================================================
    // GEOLOGICAL
    // =====================================================================

    /// MgCO3 → MgO + CO2  (magnesite decomposition, volcanic CO2 release)
    pub fn magnesite_decomposition() -> Self {
        Self {
            name: "magnesite_decomposition".into(),
            reactants: vec![Substance::amorphous(
                "magnesite",
                &[(Element::Mg, 0.29), (Element::C, 0.14), (Element::O, 0.57)],
                3000.0,
            )],
            products: vec![
                Substance::amorphous(
                    "magnesia",
                    &[(Element::Mg, 0.60), (Element::O, 0.40)],
                    3580.0,
                ),
                Substance::Molecular(Molecule::carbon_dioxide()),
            ],
            activation_energy: 160.0,
            catalyst: None,
        }
    }

    // =====================================================================
    // ORGANIC / FOOD
    // =====================================================================

    /// C6H12O6 → 2C2H5OH + 2CO2  (sugar fermentation → alcohol + CO2)
    pub fn fermentation() -> Self {
        Self {
            name: "fermentation".into(),
            reactants: vec![Substance::Molecular(Molecule::glucose())],
            products: vec![
                Substance::Molecular(Molecule::ethanol()),
                Substance::Molecular(Molecule::ethanol()),
                Substance::Molecular(Molecule::carbon_dioxide()),
                Substance::Molecular(Molecule::carbon_dioxide()),
            ],
            activation_energy: 30.0, // low — yeast provides enzymes
            catalyst: None,
        }
    }

    /// C6H12O6 + 6O2 → 6CO2 + 6H2O  (sugar combustion / cellular respiration)
    pub fn sugar_combustion() -> Self {
        Self {
            name: "sugar_combustion".into(),
            reactants: {
                let mut r = vec![Substance::Molecular(Molecule::glucose())];
                for _ in 0..6 {
                    r.push(Substance::Molecular(Molecule::oxygen_gas()));
                }
                r
            },
            products: {
                let mut p = Vec::new();
                for _ in 0..6 {
                    p.push(Substance::Molecular(Molecule::carbon_dioxide()));
                }
                for _ in 0..6 {
                    p.push(Substance::Molecular(Molecule::water()));
                }
                p
            },
            activation_energy: 100.0,
            catalyst: None,
        }
    }
}

/// Convert activation energy (kJ/mol) to approximate temperature threshold (Celsius).
///
/// From Arrhenius equation: k ~ A * exp(-Ea/(R*T))
/// At the "onset" temperature, the reaction rate becomes significant.
/// Approximation: T_onset ~ Ea*1000 / R (where R = 8.314 J/(mol*K))
/// This gives the temperature (in Kelvin) at which the Boltzmann factor
/// e^(-Ea/RT) is approximately e^(-1), then convert to Celsius.
/// Ref: Arrhenius (1889), Atkins "Physical Chemistry".
fn activation_energy_to_celsius(ea_kjmol: f32) -> f32 {
    (ea_kjmol * 1000.0 / 8.314) - 273.15
}
