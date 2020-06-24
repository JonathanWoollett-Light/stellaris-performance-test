use arrayfire::{Array,Dim4,constant,mul,MatProp};
use std::sync::Arc;
use crate::{Empire,EmpireJob,Planet,Job,Species,NUMBER_OF_RESOURCES,JOBS_MAX,SPECIES_MAX};


pub struct EmpireOptimization {
    pub planets: Vec<PlanetOptimization>,
    pub empire_mod: Array<f32>,
    pub species: Vec<Array<f32>>,
    pub jobs: Vec<EmpireJob>
}
impl EmpireOptimization {
    pub fn news(empires:&[Empire]) -> Vec<Self> {
        empires.iter().map(
            |empire| EmpireOptimization::new(empire)
        ).collect()
    }
    pub fn new(empire:&Empire) -> Self {
        Self { 
            planets: PlanetOptimization::news(&empire.planets),
            empire_mod:empire.empire_mod.clone(),
            species: empire.species.iter().map(|&s|(*s).clone()).collect(),
            jobs: empire.jobs.clone()
        }
    }
    pub fn run(&self) -> Array<f32> {
        // TODO Submit pull request adding `sum` implementation to arrayfire::Array.
        // Becuase `sum` isn't implemented
        let income:Array<f32> = self.planets.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum,planet| sum + planet.run()
        );
        let modified_income = &self.empire_mod * income;
        return modified_income;
    }
    pub fn optimize(&mut self,market_values:Array<f32>) {
        let collected:Vec<f32> = self.jobs.iter().flat_map(|j|to_vec(&*j.production)).collect();
        let job_prod_arr = Array::new(&collected,Dim4::new(&[NUMBER_OF_RESOURCES as u64,self.jobs.len() as u64,1,1]));
        //af_print!("job_prod_arr",job_prod_arr);

        let collected:Vec<f32> = self.jobs.iter().flat_map(|j|to_vec(&j.modifier)).collect();
        let job_mod_arr = Array::new(&collected,Dim4::new(&[NUMBER_OF_RESOURCES as u64,self.jobs.len() as u64,1,1]));
        //af_print!("job_mod_arr",job_mod_arr);

        let collected:Vec<f32> = self.species.iter().flat_map(|s|to_vec(s)).collect();
        let species_mods_arr = Array::new(&collected,Dim4::new(&[NUMBER_OF_RESOURCES as u64,self.species.len() as u64,1,1]));
        //af_print!("species_mods_arr",transpose(&species_mods_arr,false));

        let job_adjusted = &job_prod_arr * &job_mod_arr;
        let market_adjusted = mul(&job_adjusted,&market_values,true);

        let dims = Dim4::new(&[self.jobs.len() as u64, self.species.len() as u64, 1, 1]);
        let mut imperial_species_job_priorities: Array<f32> = Array::<f32>::new_empty(dims);
        arrayfire::gemm(
            &mut imperial_species_job_priorities,
            MatProp::TRANS,
            MatProp::NONE,
            vec![1.],
            &market_adjusted,
            &species_mods_arr,
            vec![0.],
        );

        for planet in self.planets.iter_mut() {
            planet.optimise(&imperial_species_job_priorities);
        }
        
    }
}

struct PlanetOptimization {
    modifier: Array<f32>,
    jobs: Vec<JobOptimization>
}
impl PlanetOptimization {
    pub fn news(planets:&[Planet]) -> Vec<Self> {
        planets.iter().map(|planet| PlanetOptimization::new(planet)).collect()
    }
    pub fn new(planet:&Planet) -> Self {
        Self { modifier: planet.modifier.clone(), jobs: JobOptimization::news(&planet.jobs) }
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
    pub fn optimise(&mut self,imperial_species_job_priorities:&Array<f32>) {
        let planet_job_priorities = mul(imperial_species_job_priorities,&self.modifier,true);

        let vec = to_vec(&planet_job_priorities);
        let mut labelled_vec:Vec<(f32,EOrd)> = vec.iter().enumerate().map(|(indx,v)| (*v, EOrd { job: indx % self.jobs.len(),species: indx / self.jobs.len() })).collect();
        labelled_vec.sort_by(|(a,_),(b,_)|a.partial_cmp(b).unwrap());

        fn to_vec<T: arrayfire::HasAfEnum + Default + Clone>(array: &arrayfire::Array<T>) -> Vec<T> {
            let mut vec = vec![T::default(); array.elements()];
            array.host(&mut vec);
            return vec;
        }
        struct EOrd {
            job: usize,
            species:usize
        }
    }
}

struct JobOptimization {
    indx_label: usize,
    positions: usize,
    modifier: Array<f32>,
    production: Arc<Array<f32>>,
    species: Vec<SpeciesOptimization>
}
impl JobOptimization {
    pub fn news(jobs:&[Job]) -> Vec<Self> {
        jobs.iter().map(|job| JobOptimization::new(job)).collect()
    }
    pub fn new(job:&Job) -> Self {
        unsafe { // TODO Is this the best place to put this?
            Self { indx_label: job.indx_label, positions: job.positions, modifier: (*job.modifier).clone(), production:job.production.clone(), species:SpeciesOptimization::news(&job.workers) }
        }
    }
    pub fn run(&self) -> Array<f32> {
        let income:Array<f32> = self.species.iter().fold(
            constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            |sum, species| sum + (species.count as u64 * &species.modifier)
        );
        let actual_income = income * &self.modifier * &*self.production; // look at & vs nothing on `self.modifier`
        return actual_income;
        
    }
}

struct SpeciesOptimization {
    count: usize,
    modifier: Array<f32>
}
impl SpeciesOptimization {
    pub unsafe fn news(species:&[Species]) -> Vec<Self> {
        species.iter().map(|s| SpeciesOptimization::new(s)).collect()
    }
    pub unsafe fn new(species:&Species) -> Self {
        Self { count: species.count.clone(), modifier: (*species.modifier).clone() }
    }
}