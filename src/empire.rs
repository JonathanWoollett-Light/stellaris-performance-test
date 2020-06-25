use rand::{thread_rng,Rng,rngs::ThreadRng};
use arrayfire::{Array,Dim4,randu,af_print,print_gen};
use std::{sync::Arc,collections::HashMap};
use crate::{
    Job,Species,
    NUMBER_OF_RESOURCES,PLANETS_MIN,PLANETS_MAX,POP_MIN,POP_MAX,JOBS_MIN,JOBS_MAX,SPECIES_MIN,SPECIES_MAX
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

        let empire_jobs:Vec<EmpireJob> = jobs.iter().map(|j|EmpireJob::new(*j)).collect();

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
    pub modifier: Array<f32>
}
impl EmpireSpecies {
    pub fn new(species:*const Species) -> Self {
        Self { species, modifier: randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])) }
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
        let number_of_jobs = rng.gen_range(JOBS_MIN,JOBS_MAX+1);
        let number_of_species = rng.gen_range(SPECIES_MIN,SPECIES_MAX+1);

        let mut planetary_species:Vec<&EmpireSpecies> = Vec::with_capacity(empire_species.len());
        let mut species_indxs:Vec<usize> = (0..JOBS_MAX).collect();
        for _ in 0..empire_species.len() {
            let indx = rng.gen_range(0,species_indxs.len());
            let species_indx = species_indxs.remove(indx);
            planetary_species.push(&empire_species[species_indx]);
        }

        //println!("number_of_pops: {}",number_of_pops);
        //println!("number_of_jobs: {}",number_of_jobs);
        //println!("number_of_species: {}",number_of_species);

        let mut job_positions:Vec<usize> = vec!(0usize;number_of_jobs);
        for &mut position in job_positions.iter_mut() {
            position = rng.gen_range(0,number_of_pops);
            number_of_pops -= position;
        } 

        //println!("pops_per_job: {}",pops_per_job);
        //println!("jobs_per_species: {}",jobs_per_species);

        let mut jobs:HashMap<usize,JobPosition> = HashMap::new();
        let mut job_indxs:Vec<usize> = (0..JOBS_MAX).collect();

        for i in 0..number_of_jobs {

            let indx = rng.gen_range(0,job_indxs.len());
            let job_indx = job_indxs.remove(indx);

            let job = JobPosition::new(rng,&empire_jobs[job_indx],&planetary_species,job_positions[i]);
            jobs.insert(empire_jobs[job_indx].job.id,job);
        }

        // Get population totals
        let population_totals: HashMap<usize,usize> = HashMap::new();
        for job in jobs {
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
    pub fn pops(&self) -> usize {
        let pops:usize = self.population_totals,iter().map(|(_,val)| val).sum();
        //println!("Planet: {}",pops.to_formatted_string(&Locale::en));
        return pops;
    }
}

#[derive(Clone)]
pub struct JobPosition {
    pub positions: usize, // Number of positions to be worked
    pub job: *const EmpireJob,
    pub employees: Vec<SpeciesPosition>
}
impl JobPosition {
    pub fn new(rng:&mut ThreadRng,job_stats:&EmpireJob,species:&Vec<&EmpireSpecies>,positions:usize) -> Self {

        let mut species_assigned:Vec<SpeciesPosition> = Vec::with_capacity(species.len());
        let mut species_indxs:Vec<usize> = (0..species.len()).collect();
        let mut remaining_positions = positions;
        for _ in 0..species.len() {
            let species_indx = species_indxs.remove(rng.gen_range(0,species_indxs.len()));
            species_assigned.push(SpeciesPosition { count: rng.gen_range(0,remaining_positions), species: species[species_indx] });
        }
 
        return Self { 
            positions,
            job: job_stats,
            employees:species_assigned
        };
    }
    pub fn species_counts(&self) -> Vec<(usize,usize)> { // Id, Count
        self.employees.iter().map(|e| ((*(*e.species).species).id,e.count)).collect()
    }
}

#[derive(Clone)]
pub struct SpeciesPosition {
    pub count: usize,
    pub species: *const EmpireSpecies
}