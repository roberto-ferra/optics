pub struct LaserParameters {
    laser_frequency: f64,
    laser_electric_field_amplitude: f64,
}

impl LaserParameters {
    pub fn new(wavelength_in_nanometers: f64, power_per_metre_squared: f64) -> LaserParameters {
        let output_light_frequency = WavelengthToFrequencyConverter::convert_wavelength_to_frequency(wavelength_in_nanometers);
        let laser_electric_field_amplitude = Self::calculate_electric_field_amplitude(power_per_metre_squared);
        LaserParameters {
            laser_frequency: output_light_frequency,
            laser_electric_field_amplitude,
        }
    }

    pub fn calculate_electric_field_amplitude(power_per_metre_squared: f64) -> f64 {
        f64::sqrt(2.0 * PhysicalConstants::SPEED_OF_LIGHT *
            PhysicalConstants::VACUUM_MAGNETIC_PERMEABILITY * power_per_metre_squared)
    }
}

pub struct AtomParameters {
    // simply one parameter for now for this two level atom model
    transition_frequency: f64,
}

impl AtomParameters {
    pub fn new(wavelength_in_nanometers: f64) -> AtomParameters {
        let transition_frequency = WavelengthToFrequencyConverter::convert_wavelength_to_frequency(wavelength_in_nanometers);
        AtomParameters {
            transition_frequency
        }
    }
}

pub struct ExcitedStatePopulationProbabilityCalculator {
    rabi_frequency: f64,
    generalized_rabi_frequency: f64,
}

impl ExcitedStatePopulationProbabilityCalculator {
    pub fn new(laser_parameters: LaserParameters, atom_parameters: AtomParameters) -> ExcitedStatePopulationProbabilityCalculator {
        let rabi_frequency = Self::calculate_rabi_frequency(laser_parameters.laser_electric_field_amplitude);
        let laser_detuning = atom_parameters.transition_frequency - laser_parameters.laser_frequency;
        let generalized_rabi_frequency = Self::calculate_generalized_rabi_frequency(laser_detuning, rabi_frequency);
        ExcitedStatePopulationProbabilityCalculator {
            rabi_frequency,
            generalized_rabi_frequency,
        }
    }

    pub fn calculate_excited_state_population_probability(&self, time: f64) -> f64 {
        // the below solution for the density matrix element p_22 assumes that at time 0 the excited state population is 0.
        f64::powi((self.rabi_frequency / self.generalized_rabi_frequency) * f64::sin(self.generalized_rabi_frequency * 0.5 * time), 2)
    }

    fn calculate_rabi_frequency(laser_electric_field_amplitude: f64) -> f64 {
        laser_electric_field_amplitude * PhysicalConstants::ELECTRON_CHARGE *
            PhysicalConstants::BOHR_RADIUS / PhysicalConstants::REDUCED_PLANCK_CONSTANT
    }

    fn calculate_generalized_rabi_frequency(laser_detuning: f64, rabi_frequency: f64) -> f64 {
        f64::sqrt(f64::powi(laser_detuning, 2) + f64::powi(rabi_frequency, 2))
    }
}

pub struct WavelengthToFrequencyConverter;

impl WavelengthToFrequencyConverter {
    pub fn convert_wavelength_to_frequency(wavelength_in_nanometers: f64) -> f64 {
        PhysicalConstants::SPEED_OF_LIGHT / (wavelength_in_nanometers * 1e-9)
    }
}

pub struct PhysicalConstants;

impl PhysicalConstants {
    const SPEED_OF_LIGHT: f64 = 299792458.0;
    const REDUCED_PLANCK_CONSTANT: f64 = 1.0545718e-34;
    const ELECTRON_CHARGE: f64 = 1.60217663e-19;
    const VACUUM_MAGNETIC_PERMEABILITY: f64 = std::f64::consts::PI * 4.0e-7;
    const BOHR_RADIUS: f64 = 0.529177210903e-10;
}

