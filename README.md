# materia

Chemical composition to physical material properties, from first principles.

[![Crates.io](https://img.shields.io/crates/v/materia.svg)](https://crates.io/crates/materia)
[![Docs.rs](https://docs.rs/materia/badge.svg)](https://docs.rs/materia)
[![CI](https://github.com/walkingtheplanck/materia/actions/workflows/ci.yml/badge.svg)](https://github.com/walkingtheplanck/materia/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/walkingtheplanck/materia/branch/master/graph/badge.svg)](https://codecov.io/gh/walkingtheplanck/materia)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.70-blue.svg)](https://www.rust-lang.org)

## What it does

Define materials by their **chemical structure** — atoms, bonds, crystal lattice, polymer chains — and get all physical properties derived automatically from first principles.

```rust
use materia::prelude::*;

// Define iron as a BCC crystal
let iron = compile(&Substance::Crystalline(Crystal::iron()));

assert_eq!(iron.phase.state, PhaseState::Solid);
println!("Density: {:.0} kg/m³", iron.structural.density);     // ~7528
println!("Melting: {:.0} °C", iron.thermal.melting_point.unwrap()); // 1538
println!("Conductivity: {:.1} W/(m·K)", iron.thermal.thermal_conductivity); // ~75
```

No hardcoded material databases. No magic numbers. Every property is computed from the atomic structure using established physics models.

## Quick start

```toml
[dependencies]
materia = "0.1"
```

### From molecules

```rust
use materia::prelude::*;

let water = compile(&Substance::Molecular(Molecule::water()));
// Liquid at room temp, density ~1000, melting ~0°C, IOR 1.33
```

### From crystals

```rust
let steel = compile(&Substance::Crystalline(Crystal::steel(0.008))); // 0.8% carbon
// Harder than pure iron — carbon solute strengthening
```

### From polymers

```rust
let wood = compile_as(
    &Substance::Polymer(PolymerChain {
        name: "cellulose_lignin".into(),
        monomer: Molecule::water(), // simplified
        chain_length: 5000,
        cross_link_density: 0.3,
        branching: 0.1,
    }),
    "wood",
    PhaseState::Solid,
);
```

### Preset world materials

```rust
use materia::presets;

let stone = presets::stone();     // basalt composition
let sand  = presets::sand();      // quartz, granular form
let dirt  = presets::dirt();      // soil mineral + organic mix
let grass = presets::grass();     // cellulose polymer
let water = presets::water();     // H2O, liquid
let lava  = presets::lava();      // molten basalt, glows orange
let gold  = presets::gold();      // FCC Au crystal
```

20 presets included: stone, granite, sand, clay, dirt, gravel, grass, wood, leaf, water, ice, snow, iron, steel, copper, gold, air, steam, smoke, lava.

### Chemical reactions

```rust
let reaction = Reaction::combustion_methane(); // CH4 + 2O2 → CO2 + 2H2O
println!("Energy: {:.0} kJ/mol", reaction.energy()); // exothermic
assert!(reaction.is_exothermic());

// Compile to a runtime interaction rule
let rule = reaction.to_interaction_rule(&reactant_ids, &product_ids);
```

13 reactions included: methane/hydrogen/carbon/sugar combustion, iron/copper/aluminum oxidation, water electrolysis, calcination, iron/copper smelting, magnesite decomposition, fermentation.

## What it computes

Every field of `MaterialDef` is derived from the chemical structure:

| Property | Model | Accuracy |
|----------|-------|----------|
| Density | Crystallographic d = ZM/(Na·a³) | Iron: 4%, Gold: 0.3% |
| Melting point | Element data + solute depression | Iron: exact, Water: 1.4°C off |
| Specific heat | Dulong-Petit (solids), DOF (gases) | Iron: 0.2% |
| Thermal conductivity | Wiedemann-Franz (metals) | Iron: 6%, Copper: 9% |
| Yield strength | Peierls-Nabarro + solute hardening | Order of magnitude |
| Color | Element-specific + oxide pigments | Qualitative |
| Refractive index | Lorentz-Lorenz density model | Water: exact |
| Emissivity | Hagen-Rubens (metals vs non-metals) | Qualitative |
| Surface tension | H-bond presence model | Water: exact |
| Phase state | Melting/boiling point comparison | All tested elements correct |

## The 118 elements

All elements from hydrogen to oganesson, each with:
- Atomic mass (IUPAC 2021)
- Atomic radius (metallic/covalent, CRC Handbook)
- Electronegativity (Pauling scale)
- Cohesive energy (Kittel)
- Melting and boiling points (CRC Handbook)
- Electrical resistivity (15 common metals)
- Characteristic color

## Substance types

| Type | Description | Example |
|------|-------------|---------|
| `Molecular` | Discrete molecules with explicit bonds | H2O, CH4, glucose |
| `Crystalline` | Periodic lattice (BCC, FCC, HCP, Diamond) | Fe, Cu, Au, Si, diamond |
| `Polymer` | Long-chain with cross-linking | Cellulose, rubber |
| `Amorphous` | Disordered solid, defined by composition | Glass, basite rock, soil |

## Phase-aware compilation

The same substance compiles differently depending on physical form:

```rust
let water  = compile_as(&water_substance, "water", PhaseState::Liquid);
let ice    = compile_as(&water_substance, "ice",   PhaseState::Solid);
let steam  = compile_as(&water_substance, "steam", PhaseState::Gas);
let snow   = compile_as(&water_substance, "snow",  PhaseState::Granular);
```

Each form gets adjusted density, viscosity, optical properties, etc.

## Physics references

All estimation models cite their sources:

- **Dulong-Petit** specific heat: Kittel, *Introduction to Solid State Physics*, Ch. 5
- **Wiedemann-Franz** conductivity: Kittel, Ch. 6
- **Lindemann criterion** melting point: Lindemann (1910)
- **Hagen-Rubens** emissivity: Modest, *Radiative Heat Transfer*, Ch. 3
- **Lorentz-Lorenz** refractive index: Born & Wolf, *Principles of Optics*, Ch. 2
- **Peierls-Nabarro** yield strength: Hull & Bacon, *Dislocations*, Ch. 4
- **Arrhenius** activation energy: Atkins, *Physical Chemistry*
- **Planck's law** blackbody emission: Planck (1901)

## No dependencies

Zero external dependencies. Pure Rust. No `std` features required beyond default.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

