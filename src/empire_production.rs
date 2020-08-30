use arrayfire::{Array,Dim4,constant};
use crate::{Job,Modifier,Empire,Planet,JobPosition,SpeciesPosition,NUMBER_OF_RESOURCES};
use std::{collections::HashMap,sync::Arc};

pub struct EmpireProduction {
    planets: Vec<PlanetProduction>,
    modifier: Modifier
}
impl EmpireProduction {
    pub fn news(empires:&[Empire]) -> Vec<Self> {
        empires.iter().map(
            |empire| EmpireProduction::new(empire)
        ).collect()
    }
    pub fn new(empire:&Empire) -> Self {
        Self { 
            planets: PlanetProduction::news(&empire.planets),
            modifier:empire.modifier.clone()
        }
    } 
    pub fn run(&self) -> Array<f32> {
        let raw:Array<f32> = self.planets.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum,planet| sum + planet.produce()
        );
        return self.modifier.produce(raw);
    }
    
}

struct PlanetProduction {
    modifier: Modifier,
    jobs: Vec<JobPositionProduction>
    
}
impl PlanetProduction {
    pub fn news(planets:&[Planet]) -> Vec<Self> {
        planets.iter().map(|planet| PlanetProduction::new(planet)).collect()
    }
    pub fn new(planet:&Planet) -> Self {
        Self { 
            modifier: planet.modifier.clone(),
            jobs: JobPositionProduction::news(&planet.jobs)
        }
    }
    pub fn produce(&self) -> Array<f32> {
        //println!("plan start");
        let raw:Array<f32> = self.jobs.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum,job| sum + job.produce()
        );
        return self.modifier.produce(raw);
    }
}

struct JobPositionProduction {
    pub job: Arc<Job>,
    pub modifier: Modifier,
    pub employees: Vec<SpeciesPositionProduction>
}
impl JobPositionProduction {
    fn news(job_positions:&HashMap<usize,JobPosition>) -> Vec<Self> {
        job_positions.iter().map(|(_,jp)| JobPositionProduction::new(jp)).collect()
    }
    unsafe fn new(job_position:&JobPosition) -> Self {
        Self { 
            job: (*job_position.empire_job).job.clone(),
            modifier: (*job_position.empire_job).modifier.clone(),
            employees: SpeciesPositionProduction::news(&job_position.employees)
        }
    }
    fn produce(&self) -> Array<f32> {
        let base = self.job.production + self.modifier.addend;

        return self.employees.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum, species| sum 
                + (
                    species.count as u64 
                    * (species.empire_species_modifier.multiplier + species.species_modifier.multiplier - 1) 
                    * (base + species.empire_species_modifier.addend + species.species_modifier.addend)
                )
        );
    }
}
struct SpeciesPositionProduction {
    count: usize,
    empire_species_modifier: Modifier,
    species_modifier: Modifier
}
impl SpeciesPositionProduction {
    unsafe fn news(species_positions:&HashMap<usize,SpeciesPosition>) -> Vec<Self> {
        species_positions.iter().map(|(_,sp)| SpeciesPositionProduction::new(sp)).collect()
    }
    unsafe fn new(species:&SpeciesPosition) -> Self {
        Self { 
            count: species.count, 
            empire_species_modifier: (*species.empire_species).modifier.clone(), 
            species_modifier: (*(*species.empire_species).species).modifier.clone() 
        }
    }
}