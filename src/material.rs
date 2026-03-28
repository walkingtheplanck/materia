//! Material property definitions — the full physical description of every material type.
//!
//! Copied from hyle-core/props.rs to make materia fully standalone.

/// Stable identifier for a material type.
pub type MaterialId = u16;

// -- Optical ------------------------------------------------------------------

/// Electromagnetic / optical properties. Physically derived from chemistry.
#[derive(Debug, Clone)]
pub struct OpticalProps {
    /// Room-temperature reflectance spectrum (simplified to linear RGBA).
    pub base_color: [f32; 4],
    /// Conductor (1.0) vs dielectric (0.0) — affects Fresnel reflectance.
    pub metallic: f32,
    /// Surface microfacet roughness: 0=mirror, 1=fully diffuse.
    pub roughness: f32,
    /// Opacity: 0=transparent, 1=opaque.
    pub opacity: f32,
    /// Index of refraction (vacuum=1.0, water=1.33, glass=1.5, diamond=2.42).
    pub refractive_index: f32,
    /// Chromatic dispersion (Abbe number proxy — higher = more rainbow effect).
    pub dispersion: f32,
    /// Spectral absorption inside volume (Beer-Lambert law), linear RGB.
    pub absorption_color: [f32; 3],
    /// Extinction coefficient — how quickly light is absorbed per unit depth.
    pub absorption_density: f32,
    /// Blackbody emissivity (0=perfect mirror, 1=ideal radiator).
    pub emissivity: f32,
    /// Non-thermal emission color (fluorescence, chemiluminescence), linear RGB.
    pub emission_color: [f32; 3],
    /// Self-emission intensity (0 = none).
    pub emission_intensity: f32,
    /// Mean free path for subsurface scattering (wax, skin, marble, milk).
    pub scatter_distance: f32,
    /// Subsurface scatter color shift, linear RGB.
    pub scatter_color: [f32; 3],
}

impl Default for OpticalProps {
    fn default() -> Self {
        Self {
            base_color: [0.5, 0.5, 0.5, 1.0],
            metallic: 0.0,
            roughness: 0.8,
            opacity: 1.0,
            refractive_index: 1.5,
            dispersion: 0.0,
            absorption_color: [1.0, 1.0, 1.0],
            absorption_density: 0.0,
            emissivity: 0.9,
            emission_color: [0.0, 0.0, 0.0],
            emission_intensity: 0.0,
            scatter_distance: 0.0,
            scatter_color: [1.0, 1.0, 1.0],
        }
    }
}

// -- Structural ---------------------------------------------------------------

/// Mechanical / structural properties.
#[derive(Debug, Clone)]
pub struct StructuralProps {
    /// Mass per unit volume at full density (kg/m^3).
    pub density: f32,
    /// Maximum sustained tensile/shear stress (Pa) before a voxel detaches.
    pub yield_strength: f32,
    /// Maximum instantaneous impulse (N*s/m^2) before fracture propagates.
    pub impact_toughness: f32,
    /// Permanent-deformation ratio before fracture.
    pub plasticity: f32,
    /// Resistance to surface wear from friction over time.
    pub abrasion_resistance: f32,
    /// Optional compaction behaviour.
    pub compaction: Option<CompactionDef>,
}

impl Default for StructuralProps {
    fn default() -> Self {
        Self {
            density: 1000.0,
            yield_strength: 0.0,
            impact_toughness: 0.5,
            plasticity: 0.0,
            abrasion_resistance: 0.5,
            compaction: None,
        }
    }
}

/// Describes how a material compacts under sustained pressure.
#[derive(Debug, Clone)]
pub struct CompactionDef {
    /// Minimum pressure (Pa) needed to begin compaction.
    pub pressure_threshold: f32,
    /// Density increase rate (kg/m^3 per tick per Pa above threshold).
    pub compaction_rate: f32,
    /// Material this voxel becomes once fully compacted (`None` = stays same type).
    pub product: Option<MaterialId>,
}

// -- Thermal ------------------------------------------------------------------

/// Heat and phase-transition properties.
#[derive(Debug, Clone)]
pub struct ThermalProps {
    pub specific_heat: f32,
    pub thermal_conductivity: f32,
    pub ignition_point: Option<f32>,
    pub melting_point: Option<f32>,
    pub vaporisation_point: Option<f32>,
    pub freezing_point: Option<f32>,
    pub melt_product: Option<MaterialId>,
    pub freeze_product: Option<MaterialId>,
    pub burn_product: Option<MaterialId>,
    pub combustion_energy: f32,
    pub emission_onset_temperature: Option<f32>,
}

impl Default for ThermalProps {
    fn default() -> Self {
        Self {
            specific_heat: 1000.0,
            thermal_conductivity: 1.0,
            ignition_point: None,
            melting_point: None,
            vaporisation_point: None,
            freezing_point: None,
            melt_product: None,
            freeze_product: None,
            burn_product: None,
            combustion_energy: 0.0,
            emission_onset_temperature: None,
        }
    }
}

// -- Chemical -----------------------------------------------------------------

/// Chemical interaction properties.
#[derive(Debug, Clone)]
pub struct ChemicalProps {
    pub ph: f32,
    pub acid_solubility: f32,
    pub base_solubility: f32,
    pub oxidation_resistance: f32,
    pub corrosion_product: Option<MaterialId>,
}

impl Default for ChemicalProps {
    fn default() -> Self {
        Self {
            ph: 7.0,
            acid_solubility: 0.0,
            base_solubility: 0.0,
            oxidation_resistance: 1.0,
            corrosion_product: None,
        }
    }
}

// -- Hydraulic ----------------------------------------------------------------

/// Fluid / moisture properties.
#[derive(Debug, Clone)]
pub struct HydraulicProps {
    pub porosity: f32,
    pub permeability: f32,
    pub saturation_ignition_modifier: f32,
    pub saturation_conductivity_modifier: f32,
}

impl Default for HydraulicProps {
    fn default() -> Self {
        Self {
            porosity: 0.0,
            permeability: 0.0,
            saturation_ignition_modifier: 0.0,
            saturation_conductivity_modifier: 0.0,
        }
    }
}

// -- Phase --------------------------------------------------------------------

/// State-of-matter and flow properties.
#[derive(Debug, Clone)]
pub struct PhaseProps {
    pub state: PhaseState,
    pub repose_angle: f32,
    pub viscosity: f32,
    pub buoyancy: f32,
    /// LBM relaxation time tau. Controls kinematic viscosity:
    /// nu = cs^2 (tau - 0.5).  tau=0.8 -> water, tau=3.0 -> sand, tau=5.0 -> lava.
    /// Solid materials ignore this (they are bounce-back boundaries).
    pub relaxation_time: f32,
    /// Surface tension coefficient. Controls resistance to free surface
    /// area increase in VOF. Higher values = rounder droplets, less
    /// fragmentation. Water=0.072, mercury=0.5, molten iron=1.8 (N/m).
    pub surface_tension: f32,
}

impl Default for PhaseProps {
    fn default() -> Self {
        Self {
            state: PhaseState::Solid,
            repose_angle: 90.0,
            viscosity: 0.0,
            buoyancy: 1.0,
            relaxation_time: 1.0,
            surface_tension: 0.0,
        }
    }
}

/// Fundamental state of matter for a material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhaseState {
    Solid,
    Granular,
    Liquid,
    Gas,
}

// -- Acoustic -----------------------------------------------------------------

/// Acoustic (sound propagation) properties of a material.
#[derive(Debug, Clone)]
pub struct AcousticProps {
    pub propagation_speed: f32,
    pub damping: f32,
    pub reflectance: f32,
}

impl Default for AcousticProps {
    fn default() -> Self {
        Self {
            propagation_speed: 343.0,
            damping: 0.1,
            reflectance: 0.5,
        }
    }
}

// -- MaterialDef --------------------------------------------------------------

/// Complete physical description of a material type.
#[derive(Debug, Clone)]
pub struct MaterialDef {
    pub name: String,
    pub structural: StructuralProps,
    pub thermal: ThermalProps,
    pub chemical: ChemicalProps,
    pub hydraulic: HydraulicProps,
    pub phase: PhaseProps,
    pub acoustic: AcousticProps,
    pub optical: OpticalProps,
}

impl Default for MaterialDef {
    fn default() -> Self {
        Self {
            name: "unnamed".to_string(),
            structural: StructuralProps::default(),
            thermal: ThermalProps::default(),
            chemical: ChemicalProps::default(),
            hydraulic: HydraulicProps::default(),
            phase: PhaseProps::default(),
            acoustic: AcousticProps::default(),
            optical: OpticalProps::default(),
        }
    }
}
