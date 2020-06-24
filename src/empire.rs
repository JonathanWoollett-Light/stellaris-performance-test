use rand::{thread_rng,Rng,rngs::ThreadRng};
use arrayfire::{Array,Dim4,randu,af_print,print_gen};
use std::sync::Arc;
use crate::{NUMBER_OF_RESOURCES,PLANETS_MIN,PLANETS_MAX,POP_MIN,POP_MAX,JOBS_MIN,JOBS_MAX,SPECIES_MIN,SPECIES_MAX};

#[derive(Clone)]
pub struct Empire {
    pub planets: Vec<Planet>,
    pub empire_mod: Array<f32>,
    pub species: Vec<*const Array<f32>>,
    pub jobs: Vec<EmpireJob>
}
impl Empire {
    pub fn new(job_prods: &Vec<Arc<Array<f32>>>,species_mods: &Vec<Array<f32>>) -> Self {
        let job_modifiers:Vec<Array<f32>> = gen_job_mods(false);
        let empire_mod = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));

        let jobs:Vec<EmpireJob> = job_modifiers.into_iter().zip(job_prods.iter()).map(
            |(modifier,production)| EmpireJob { modifier, production: production.clone() }
        ).collect();

        let species:Vec<*const Array<f32>> = species_mods.iter().map(|s|s as *const Array<f32>).collect();

        return Self { planets:Vec::new(), empire_mod, species, jobs };

        fn gen_job_mods(print:bool) -> Vec<Array<f32>> {
            let mut jobs:Vec<Array<f32>> = Vec::with_capacity(JOBS_MAX);
            for _ in 0..JOBS_MAX {
                let prod = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));
                jobs.push(prod);
            }
            if print {
                println!("job mods:");
                for job_prod in &jobs {
                    af_print!("",job_prod);
                }
            }
            return jobs;
        }
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
pub struct EmpireJob {
    pub production: Arc<Array<f32>>,
    pub modifier: Array<f32>
}
#[derive(Clone)]
pub struct Planet {
    pub modifier: Array<f32>,
    pub jobs: Vec<Job>
}
impl Planet {
    pub fn new(rng:&mut ThreadRng,job_stats: &Vec<EmpireJob>,species: &Vec<*const Array<f32>>) -> Self {
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

        for _ in 0..number_of_jobs {

            let indx = rng.gen_range(0,job_indxs.len());
            let job_indx = job_indxs.remove(indx);

            let job = Job::new(rng,&job_stats[job_indx],species,job_indx,number_of_species,jobs_per_species);
            jobs.push(job);
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
    pub indx_label:usize,
    pub positions: usize, // Number of positions to be worked
    pub modifier: *const Array<f32>,
    pub production: Arc<Array<f32>>,
    pub species: Vec<Species>
}
impl Job {
    pub fn new(rng:&mut ThreadRng,job_stats:&EmpireJob,species:&Vec<*const Array<f32>>,job_indx:usize,number_of_species:usize,pops_per_species:usize) -> Self {
        let mut species_assigned:Vec<Species> = Vec::with_capacity(species.len());
        let mut unselected_species:Vec<usize> = (0..species.len()).collect();
        for _ in 0..number_of_species {
            let species_indx = unselected_species.remove(rng.gen_range(0,unselected_species.len()));
            species_assigned.push(Species { count: pops_per_species, modifier: species[species_indx] });
        }

        return Self { 
            indx_label: job_indx,
            positions: number_of_species * pops_per_species,
            modifier:&job_stats.modifier,
            production:job_stats.production,
            species:species_assigned
        };
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
    pub modifier: *const Array<f32>
}