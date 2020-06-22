use rand::{thread_rng,Rng,rngs::ThreadRng};
use arrayfire::{Array,Dim4,randu,af_print,print_gen,constant};
use std::time::{Instant,Duration};

use num_format::{Locale, ToFormattedString};

const NUMBER_OF_RESOURCES:usize = 10; // Number of pop producible resources.
const NUMBER_OF_EMPIRES:usize = 20;

const PLANETS_MIN:usize = 1;
const PLANETS_MAX:usize = 50;

const POP_MIN:usize = 100;            // Minimum number of pops on a planet.
const POP_MAX:usize = 1000;           // Maximum number of pops on a planet.

const JOBS_MIN:usize = 1;            // Minimum number of jobs on a planet.
const JOBS_MAX:usize = 50;           // Maximum number of jobs on a planet.

const SPECIES_MIN:usize = 1;         // Minimum number of species on a planet.
const SPECIES_MAX:usize = 30;       // Maximum number of species on a planet.

fn main() {
    println!("gen 0");
    let jobs = gen_jobs(false);
    println!("gen 1");
    let species = gen_species(false);
    println!("gen 2");
    let mut empires = gen_empires(&jobs,&species);
    println!("gen 3");

    let pop_sum:usize = empires.iter().map(|empire| empire.pops()).sum();
    println!("pop_sum: {}",pop_sum.to_formatted_string(&Locale::en));

    let start = Instant::now();
    let mut time_before_iteration = start.elapsed();
    //println!("why does it take so long?");
    for i in 0..10 {
        print!("{}.",i+1);
        for empire in empires.iter_mut() {
            empire.run();
            //println!("progress");
        }
        
        println!(" : {}",time(start.elapsed()-time_before_iteration));
        time_before_iteration = start.elapsed();
    }
    println!("total : {}",time(start.elapsed()));
}

fn time(elapsed: Duration) -> String {
    let mut millis = elapsed.as_millis();
    let seconds = (millis as f32 / 1000f32).floor();
    millis = millis % 1000;
    let time = format!("{:#02}:{:#03}", seconds, millis);
    return time;
}

fn gen_jobs(print:bool) -> Vec<Array<f32>> {
    let mut jobs:Vec<Array<f32>> = Vec::with_capacity(JOBS_MAX);
    for _ in 0..JOBS_MAX {
        let prod = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));
        jobs.push(prod);
    }
    if print {
        println!("jobs:");
        for job_prod in &jobs {
            af_print!("",job_prod);
        }
    }
    return jobs;
}

fn gen_species(print:bool) -> Vec<Array<f32>> {
    let mut species:Vec<Array<f32>> = Vec::with_capacity(SPECIES_MAX);
    for _ in 0..SPECIES_MAX {
        let modifier = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));
        species.push(modifier);
    }
    
    if print {
        println!("species:");
        for modifier in &species {
            af_print!("",modifier);
        }
    }
    return species;
}
struct Empire<'a> {
    planets: Vec<Planet<'a>>,
    job_modifiers: Vec<Array<f32>>,
    empire_mod: Array<f32>,
    resources: Array<f32>
}
impl<'a> Empire<'a> {
    pub fn new() -> Self {
        let job_modifiers = gen_jobs(false);
        let empire_mod = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));

        return Self { planets:Vec::new(), job_modifiers, empire_mod, resources: constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]))};
    }
    pub fn gen_planets(&mut self,job_prods: &'a Vec<Array<f32>>,species_mods: &'a Vec<Array<f32>>) {
        let mut rng = thread_rng();
        let number_of_planets:usize = rng.gen_range(PLANETS_MIN,PLANETS_MAX+1);
        let mut planets:Vec<Planet> = Vec::with_capacity(number_of_planets);
        for _ in 0..number_of_planets {
            let planet = Planet::new(&mut rng,job_prods,&self.job_modifiers,species_mods);
            planets.push(planet);
        }

        self.planets = planets;
    }
    pub fn run(&mut self) {
        // TODO Submit pull request adding `sum` implementation to arrayfire::Array.
        // Becuase `sum` isn't implemented
        let income:Array<f32> = self.planets.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum,planet| sum + planet.run()
        );
        self.resources += income;
    }
    pub fn pops(&self) -> usize {
        let pops = self.planets.iter().map(|planet| planet.pops()).sum();
        return pops;
    }
}
struct Planet<'a> {
    modifier: Array<f32>,
    jobs: Vec<Job<'a>>
}
impl<'a> Planet<'a> {
    pub fn new(rng:&mut ThreadRng,job_prods: &'a Vec<Array<f32>>,job_mods: &Vec<Array<f32>>,species: &'a Vec<Array<f32>>) -> Self {
        let modifier = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));

        let number_of_pops = rng.gen_range(POP_MIN,POP_MIN+1);
        let number_of_jobs = rng.gen_range(JOBS_MIN,JOBS_MAX+1);
        let number_of_species = rng.gen_range(SPECIES_MIN,SPECIES_MAX+1);

        let pops_per_job = number_of_pops / number_of_jobs;
        let jobs_per_species = pops_per_job / number_of_species;

        let mut jobs:Vec<Job> = Vec::with_capacity(number_of_jobs);
        let mut job_indxs:Vec<usize> = (0..JOBS_MAX).collect();

        while jobs.len() != number_of_jobs {

            let indx = rng.gen_range(0,job_indxs.len());
            let job_indx = job_indxs[indx];

            let job = Job::new(&job_prods[job_indx],&job_mods[job_indx],species,jobs_per_species);
            jobs.push(job);

            job_indxs.remove(indx);
        }

        return Self { modifier, jobs };
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
    pub fn pops(&self) -> usize {
        let pops = self.jobs.iter().map(|job| job.pops()).sum();
        return pops;
    }
}
struct Job<'a> {
    modifier: *const Array<f32>,
    production: &'a Array<f32>,
    species: Vec<Species<'a>>
}
impl<'a> Job<'a> {
    pub fn new(production:&'a Array<f32>,modifier:&Array<f32>,species:&'a Vec<Array<f32>>,pops_per_species:usize) -> Self {
        let mut species_assigned:Vec<Species> = Vec::with_capacity(species.len());
        for spec in species.iter() {
            species_assigned.push(Species { count: pops_per_species, modifier: spec });
        }

        return Self { modifier, production, species:species_assigned };
    }
    pub fn run(&self) -> Array<f32> {
        let income:Array<f32> = self.species.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum, species| sum + (species.count as u64 * species.modifier)
        );
        unsafe {
            let actual_income = income * &*self.modifier * self.production;
            return actual_income;
        }
        
    }
    pub fn pops(&self) -> usize {
        let pops = self.species.iter().map(|species| species.count).sum();
        return pops;
    }
}
struct Species<'a> {
    count: usize,
    modifier: &'a Array<f32>
}

fn gen_empires<'a>(job_prods: &'a Vec<Array<f32>>,species_mods: &'a Vec<Array<f32>>) -> Vec<Empire<'a>> {
    let mut empires:Vec<Empire> = Vec::with_capacity(NUMBER_OF_EMPIRES);
    for _ in 0..NUMBER_OF_EMPIRES {
        let mut empire = Empire::new();
        empire.gen_planets(job_prods,species_mods);
        empires.push(empire);
    }
    return empires;
}