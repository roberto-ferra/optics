use excited_state_probability::ExcitedStatePopulationProbabilityCalculator;

use crate::excited_state_probability::{AtomParameters, LaserParameters};

mod excited_state_probability;


fn main() {
    let laser_parameters = LaserParameters::new(520.0, 10.0);
    let atom_parameters = AtomParameters::new(520.0);
    let calculator = ExcitedStatePopulationProbabilityCalculator::new(laser_parameters, atom_parameters);

    let mut max_probability = 0.0;
    for i in 0..100 {
        let calculated = calculator.calculate_excited_state_population_probability(i as f64 * 0.1f64);
        if calculated > max_probability {
            max_probability = calculated;
        }
    }

    assert_ne!(0.0, max_probability);
    assert!(max_probability <= 1.0);
}