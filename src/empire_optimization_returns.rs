use std::collections::HashMap;

pub struct EmpireOptimizationReturn {
    pub planets: Vec<PlanetOptimizationReturn>
}
pub struct PlanetOptimizationReturn {
    pub jobs: HashMap<usize,JobPositionOptimizationReturn>
}
pub struct JobPositionOptimizationReturn {
    pub employees: Vec<SpeciesPositionOptimization>
}
pub struct SpeciesPositionOptimization {
    pub count: usize,
    pub species_id: usize
}