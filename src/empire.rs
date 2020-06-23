use rand::{thread_rng,Rng,rngs::ThreadRng};
use arrayfire::{Array,Dim4,randu};
use std::sync::Arc;
use num_format::{Locale, ToFormattedString};
use crate::{gen_jobs,NUMBER_OF_RESOURCES,PLANETS_MIN,PLANETS_MAX,POP_MIN,POP_MAX,JOBS_MIN,JOBS_MAX,SPECIES_MIN,SPECIES_MAX};

#[derive(Clone)]
pub struct Empire {
    pub planets: Vec<Planet>,
    pub job_modifiers: Vec<Arc<Array<f32>>>,
    pub empire_mod: Array<f32>
}
impl Empire {
    pub fn new() -> Self {
        let job_modifiers:Vec<Arc<Array<f32>>> = gen_jobs(false);
        let empire_mod = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));

        return Self { planets:Vec::new(), job_modifiers, empire_mod };
    }
    pub fn gen_planets(&mut self,job_prods: &Vec<Arc<Array<f32>>>,species_mods: &Vec<Arc<Array<f32>>>) {
        let mut rng = thread_rng();
        let number_of_planets:usize = rng.gen_range(PLANETS_MIN,PLANETS_MAX+1);
        let mut planets:Vec<Planet> = Vec::with_capacity(number_of_planets);
        for _ in 0..number_of_planets {
            let planet = Planet::new(&mut rng,job_prods,&self.job_modifiers,species_mods);
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
pub struct Planet {
    pub modifier: Array<f32>,
    pub jobs: Vec<Job>
}
impl Planet {
    pub fn new(rng:&mut ThreadRng,job_prods: &Vec<Arc<Array<f32>>>,job_mods: &Vec<Arc<Array<f32>>>,species: &Vec<Arc<Array<f32>>>) -> Self {
        let modifier = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));

        let number_of_pops = rng.gen_range(POP_MIN,POP_MAX+1);
        let number_of_jobs = rng.gen_range(JOBS_MIN,JOBS_MAX+1);
        let number_of_species = rng.gen_range(SPECIES_MIN,SPECIES_MAX+1);

        //println!("number_of_pops: {}",number_of_pops);
        //println!("number_of_jobs: {}",number_of_jobs);
        //println!("number_of_species: {}",number_of_species);

        let pops_per_job = number_of_pops / number_of_jobs;
        let jobs_per_species = (pops_per_job as f32 / number_of_species as f32).ceil() as usize;

        //println!("pops_per_job: {}",pops_per_job);
        //println!("jobs_per_species: {}",jobs_per_species);

        let mut jobs:Vec<Job> = Vec::with_capacity(number_of_jobs);
        let mut job_indxs:Vec<usize> = (0..JOBS_MAX).collect();

        while jobs.len() != number_of_jobs {

            let indx = rng.gen_range(0,job_indxs.len());
            let job_indx = job_indxs[indx];

            let job = Job::new(job_prods[job_indx].clone(),job_mods[job_indx].clone(),species,jobs_per_species);
            jobs.push(job);

            job_indxs.remove(indx);
        }

        return Self { modifier, jobs };
    }
    pub fn pops(&self) -> usize {
        let pops:usize = self.jobs.iter().map(|job| job.pops()).sum();
        //println!("Planet: {}",pops.to_formatted_string(&Locale::en));
        return pops;
    }
}

#[derive(Clone)]
pub struct Job {
    pub modifier: Arc<Array<f32>>,
    pub production: Arc<Array<f32>>,
    pub species: Vec<Species>
}
impl Job {
    pub fn new(production:Arc<Array<f32>>,modifier:Arc<Array<f32>>,species:&Vec<Arc<Array<f32>>>,pops_per_species:usize) -> Self {
        let mut species_assigned:Vec<Species> = Vec::with_capacity(species.len());
        for spec in species.iter() {
            species_assigned.push(Species { count: pops_per_species, modifier: spec.clone() });
        }

        return Self { modifier:modifier, production:production, species:species_assigned };
    }
    pub fn pops(&self) -> usize {
        let pops:usize = self.species.iter().map(|species| species.count).sum();
        //println!("Job: {}",pops.to_formatted_string(&Locale::en));
        return pops;
    }
}

#[derive(Clone)]
pub struct Species {
    pub count: usize,
    pub modifier: Arc<Array<f32>>
}