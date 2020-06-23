use arrayfire::{Array,Dim4,constant};
use std::sync::Arc;
use crate::{Empire,Planet,Job,Species,NUMBER_OF_RESOURCES};


pub struct EmpireProduction {
    planets: Vec<PlanetProduction>,
    job_modifiers: Vec<Arc<Array<f32>>>,
    empire_mod: Array<f32>
}
impl EmpireProduction {
    pub fn news(empires:&[Empire]) -> Vec<Self> {
        empires.iter().map(|empire| EmpireProduction::new(empire)).collect()
    }
    pub fn new(empire:&Empire) -> Self {
        Self { planets:PlanetProduction::news(&empire.planets), job_modifiers:empire.job_modifiers.clone(), empire_mod:empire.empire_mod.clone() }
    }
    pub fn run(&self) -> Array<f32> {
        // TODO Submit pull request adding `sum` implementation to arrayfire::Array.
        // Becuase `sum` isn't implemented
        let income:Array<f32> = self.planets.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum,planet| sum + planet.run()
        );
        return income;
    }
}

struct PlanetProduction {
    modifier: Array<f32>,
    jobs: Vec<JobProduction>
}
impl PlanetProduction {
    pub fn news(planets:&[Planet]) -> Vec<Self> {
        planets.iter().map(|planet| PlanetProduction::new(planet)).collect()
    }
    pub fn new(planet:&Planet) -> Self {
        Self { modifier: planet.modifier.clone(), jobs: JobProduction::news(&planet.jobs) }
    }
    pub fn run(&self) -> Array<f32> {
        //println!("plan start");
        let income:Array<f32> = self.jobs.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum,job| sum + job.run()
        );
        let modified_income = income * &self.modifier;
        return modified_income;
    }
}

struct JobProduction {
    modifier: Arc<Array<f32>>,
    production: Arc<Array<f32>>,
    species: Vec<SpeciesProduction>
}
impl JobProduction {
    pub fn news(jobs:&[Job]) -> Vec<Self> {
        jobs.iter().map(|job| JobProduction::new(job)).collect()
    }
    pub fn new(job:&Job) -> Self {
        Self { modifier: job.modifier.clone(), production:job.production.clone(), species:SpeciesProduction::news(&job.species) }
    }
    pub fn run(&self) -> Array<f32> {
        let income:Array<f32> = self.species.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum, species| sum + (species.count as u64 * &*species.modifier)
        );
        let actual_income = income * &*self.modifier * &*self.production; // TODO Should * be &*?
        return actual_income;
        
    }
}

struct SpeciesProduction {
    count: usize,
    modifier: Arc<Array<f32>>
}
impl SpeciesProduction {
    pub fn news(species:&[Species]) -> Vec<Self> {
        species.iter().map(|s| SpeciesProduction::new(s)).collect()
    }
    pub fn new(species:&Species) -> Self {
        Self { count: species.count.clone(), modifier: species.modifier.clone() }
    }
}