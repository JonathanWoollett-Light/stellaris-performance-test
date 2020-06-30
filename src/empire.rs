use rand::{thread_rng,Rng,rngs::ThreadRng};
use arrayfire::{Array,Dim4,randu,constant};
use std::{sync::Arc,collections::HashMap};
use crate::{
    Job,Species,EmpireOptimizationReturn,PlanetOptimizationReturn,JobPositionOptimizationReturn,
    NUMBER_OF_RESOURCES,PLANETS_MIN,PLANETS_MAX,POP_MIN,POP_MAX,JOBS_MIN,JOBS_MAX,SPECIES_MIN
};

#[derive(Clone)]
pub struct Empire {
    pub planets: Vec<Planet>,
    pub modifier: Array<f32>,
    pub species: Vec<EmpireSpecies>,
    pub jobs: Vec<EmpireJob>
}
impl Empire {
    pub fn new(jobs: &Vec<Arc<Job>>,species: &Vec<Species>) -> Self {
        let empire_modifier = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));

        let empire_jobs:Vec<EmpireJob> = jobs.iter().map(|j|EmpireJob::new(j.clone())).collect();

        let empire_species:Vec<EmpireSpecies> = species.iter().map(|s|EmpireSpecies::new(s)).collect();

        return Self { planets:Vec::new(), modifier:empire_modifier, species:empire_species, jobs:empire_jobs };
    }
    pub fn gen_planets(&mut self) {
        let mut rng = thread_rng();
        let number_of_planets:usize = rng.gen_range(PLANETS_MIN,PLANETS_MAX+1);
        let mut planets:Vec<Planet> = Vec::with_capacity(number_of_planets);
        for _ in 0..number_of_planets {
            let planet = Planet::new(&mut rng,&self.jobs,&self.species);
            planets.push(planet);
        }

        self.planets = planets;
        //panic!("finished creation of 1 empire");
    }
    pub fn intraplanetary_optimize(&mut self,optimised_empire:EmpireOptimizationReturn) {
        for (planet,optimised_planet) in self.planets.iter_mut().zip(optimised_empire.planets.into_iter()) {
            planet.intraplanetary_optimize(optimised_planet);
        }
    }
    pub fn pops(&self) -> usize {
        let pops:usize = self.planets.iter().map(|planet| planet.pops()).sum();
        //println!("Empire: {}",pops.to_formatted_string(&Locale::en));
        //panic!("empire panic");
        return pops;
    }
}

#[derive(Clone)]
pub struct EmpireSpecies {
    pub species: *const Species,
    pub modifier: Array<f32>,
    pub employability: Array<bool>
}
impl EmpireSpecies {
    pub fn new(species:*const Species) -> Self {
        Self { 
            species, 
            modifier: randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])), // Randomly gen empire species modifiers (affect of species policies)
            employability: constant(true,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])) // Assume policies allow species to work all jobs
        }
    }
}

#[derive(Clone)]
pub struct EmpireJob {
    pub job: Arc<Job>,
    pub modifier: Array<f32>
}
impl EmpireJob {
    pub fn new(job:Arc<Job>) -> Self {
        Self { job, modifier: randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])) }
    }
}
#[derive(Clone)]
pub struct Planet {
    pub population_totals: HashMap<usize,usize>, // Id, Count
    pub modifier: Array<f32>,
    pub jobs: HashMap<usize,JobPosition>
}
impl Planet {
    pub fn new(rng:&mut ThreadRng,empire_jobs: &Vec<EmpireJob>,empire_species: &Vec<EmpireSpecies>) -> Self {
        let modifier = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));

        let mut number_of_pops = rng.gen_range(POP_MIN,POP_MAX+1);
        let max_jobs = rng.gen_range(JOBS_MIN,JOBS_MAX+1);

        // Generate species
        // ---------------------
        let number_of_species = rng.gen_range(SPECIES_MIN,empire_species.len()+1);

        let mut planetary_species:Vec<&EmpireSpecies> = Vec::with_capacity(empire_species.len());
        let mut species_indxs:Vec<usize> = (0..empire_species.len()).collect();
        for _ in 0..number_of_species {
            let indx = rng.gen_range(0,species_indxs.len());
            let species_indx = species_indxs.remove(indx);
            planetary_species.push(&empire_species[species_indx]);
        }

        // Generate pops per job
        // ---------------------
        let mut job_positions:Vec<usize> = Vec::with_capacity(max_jobs);
        while !(job_positions.len() == max_jobs || number_of_pops == 0) {
            let positions = rng.gen_range(0,number_of_pops);
            job_positions.push(positions);
            number_of_pops -= positions;
        }

        // Generate jobs
        // ---------------------
        let mut jobs:HashMap<usize,JobPosition> = HashMap::new();
        let mut job_indxs:Vec<usize> = (0..empire_jobs.len()).collect();

        for i in 0..job_positions.len() {
            let indx = rng.gen_range(0,job_indxs.len());
            let job_indx = job_indxs.remove(indx);

            let job = JobPosition::new(rng,&empire_jobs[job_indx],&planetary_species,job_positions[i]);
            jobs.insert(empire_jobs[job_indx].job.id,job);
        }

        // Gets species population totals
        // ---------------------
        let mut population_totals: HashMap<usize,usize> = HashMap::new();
        for (_,job) in jobs.iter() {
            let species_counts = job.species_counts();
            for species in species_counts {
                if let Some(count) = population_totals.get_mut(&species.0) {
                    *count += species.1;
                } else {
                    population_totals.insert(species.0,species.1);
                }
            }
        }
        
        return Self { population_totals, modifier, jobs };
    }
    pub fn intraplanetary_optimize(&mut self,optimised_planet:PlanetOptimizationReturn) {
        for (id,job) in optimised_planet.jobs.into_iter() {
            // `optimised_planet` is created from keys in `self.jobs` so they both have same keys, thus we can use unwrap.
            self.jobs.get_mut(&id).unwrap().intraplanetary_optimize(job);
        }
    }
    pub fn pops(&self) -> usize {
        let pops:usize = self.population_totals.iter().map(|(_,val)| val).sum();
        //println!("Planet: {}",pops.to_formatted_string(&Locale::en));
        return pops;
    }
}

#[derive(Clone)]
pub struct JobPosition {
    pub positions: usize, // Number of positions to be worked
    pub job: *const EmpireJob,
    pub employees: HashMap<usize,SpeciesPosition>
}
impl JobPosition {
    pub fn new(rng:&mut ThreadRng,job_stats:&EmpireJob,species:&Vec<&EmpireSpecies>,positions:usize) -> Self {
        let mut employees:HashMap<usize,SpeciesPosition> = HashMap::new();
        let mut species_indxs:Vec<usize> = (0..species.len()).collect();
        let mut remaining_positions = positions;
        
        unsafe {
            while !(species_indxs.is_empty() || remaining_positions == 0) {
                let species_indx = species_indxs.remove(rng.gen_range(0,species_indxs.len()));
                let employed = rng.gen_range(0,remaining_positions);
                employees.insert((*species[species_indx].species).id,SpeciesPosition { count: employed, species: species[species_indx] });
                remaining_positions -= employed;
            }
        }
        
 
        return Self {
            positions,
            job: job_stats,
            employees
        };
    }
    pub fn intraplanetary_optimize(&mut self, optimized_job:JobPositionOptimizationReturn) {
        unsafe {
            // Update counts if exists in `optimized_job` (new count is non-zero), otherwise removes.
            self.employees.retain(
                |_,employee| {
                    if let Some(sp) = optimized_job.employees.get((*(*employee.species).species).id) {
                        employee.count = sp.count;
                        true
                    } else { false }
                }
            );
        }
    }
    pub fn species_counts(&self) -> Vec<(usize,usize)> { // Id, Count
        self.employees.iter().map(|(id,v)| (*id,v.count)).collect()
    }
}

#[derive(Clone)]
pub struct SpeciesPosition {
    pub count: usize,
    pub species: *const EmpireSpecies
}