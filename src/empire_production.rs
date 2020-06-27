use arrayfire::{Array,Dim4,constant};
use crate::{Empire,Planet,JobPosition,SpeciesPosition,NUMBER_OF_RESOURCES};


pub struct EmpireProduction {
    planets: Vec<PlanetProduction>,
    modifier: Array<f32>
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
        // TODO Submit pull request adding `sum` implementaaaaaaation to arrayfire::Array.
        // Becuase `sum` isn't implemented
        let income:Array<f32> = self.planets.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum,planet| sum + planet.run()
        );
        let modified_income = &self.modifier * income;
        return modified_income;
    }
    
}

struct PlanetProduction {
    modifier: Array<f32>,
    jobs: Vec<JobPositionProduction>
    
}
impl PlanetProduction {
    pub fn news(planets:&[Planet]) -> Vec<Self> {
        planets.iter().map(|planet| PlanetProduction::new(planet)).collect()
    }
    pub fn new(planet:&Planet) -> Self {
        Self { 
            modifier: planet.modifier.clone(),
            jobs: planet.jobs.iter().map(|(_,v)|JobPositionProduction::new(v)).collect()
        }
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

struct JobPositionProduction {
    pub production: Array<f32>,
    pub modifier: Array<f32>,
    pub employees: Vec<SpeciesPositionProduction>
}
impl JobPositionProduction {
    pub fn new(job:&JobPosition) -> Self {
        unsafe { // TODO Is this the best place to put this?
            Self { 
                production: (*job.job).job.production.clone(),
                modifier: (*job.job).modifier.clone(),
                employees: job.employees.iter().map(|(_,v)| SpeciesPositionProduction::new(v)).collect()
            }
        }
    }
    pub fn run(&self) -> Array<f32> {
        let income:Array<f32> = self.employees.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum, species| sum + (species.count as u64 * &species.modifier)
        );
        let actual_income = income * &self.modifier * &self.production; // look at & vs nothing on `self.modifier`
        return actual_income;
    }
}

struct SpeciesPositionProduction {
    count: usize,
    modifier: Array<f32>
}
impl SpeciesPositionProduction {
    pub unsafe fn new(species:&SpeciesPosition) -> Self {
        Self { count: species.count, modifier: (*species.species).modifier.clone() }
    }
}