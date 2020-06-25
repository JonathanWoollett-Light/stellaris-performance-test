use std::collections::HashMap;

pub struct EmpireOptimizationReturn {
    planets: Vec<PlanetOptimizationReturn>
}
pub struct PlanetOptimizationReturn {
    pub jobs: HashMap<usize,JobPositionOptimizationReturn>
}
pub struct JobPositionOptimizationReturn {
    pub employees: Vec<SpeciesPositionOptimization>
}
struct SpeciesPositionOptimization {
    count: usize,
    species_id: usize
}