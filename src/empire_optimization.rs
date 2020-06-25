use arrayfire::{Array,Dim4,mul,MatProp};

use crate::{
    Empire,EmpireJob,EmpireSpecies,Planet,Species,
    EmpireOptimizationReturn,PlanetOptimizationReturn,JobPositionOptimizationReturn,
    NUMBER_OF_RESOURCES
};
use std::{
    cmp,
    collections::HashMap
};

pub struct EmpireOptimization {
    pub planets: Vec<PlanetOptimization>,
    pub modifier: Array<f32>,
    pub species: Vec<EmpireSpeciesOptimization>,
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
            modifier:empire.modifier.clone(),
            species: empire.species.iter().map(|s|EmpireSpeciesOptimization::new(s)).collect(),
            jobs: empire.jobs.clone()
        }
    }
    pub fn intraplanetary_optimization(&mut self,market_values:Array<f32>) -> EmpireOptimizationReturn {
        // Convert 2d vec to array
        let collected:Vec<f32> = self.jobs.iter().flat_map(|j|to_vec(&j.job.production)).collect();
        let job_prod_arr = Array::new(&collected,Dim4::new(&[NUMBER_OF_RESOURCES as u64,self.jobs.len() as u64,1,1]));
        //af_print!("job_prod_arr",job_prod_arr);

        // Convert 2d vec to array
        let collected:Vec<f32> = self.jobs.iter().flat_map(|j|to_vec(&j.modifier)).collect();
        let job_mod_arr = Array::new(&collected,Dim4::new(&[NUMBER_OF_RESOURCES as u64,self.jobs.len() as u64,1,1]));
        //af_print!("job_mod_arr",job_mod_arr);

        // Convert 2d vec to array
        let collected:Vec<f32> = self.species.iter().flat_map(|s|to_vec(&s.modifier)).collect();
        let species_mods_arr = Array::new(&collected,Dim4::new(&[NUMBER_OF_RESOURCES as u64,self.species.len() as u64,1,1]));
        //af_print!("species_mods_arr",transpose(&species_mods_arr,false));

        let job_adjusted = &job_prod_arr * &job_mod_arr;
        let imerial_market_values = self.modifier * market_values;
        let market_adjusted = mul(&job_adjusted,&imerial_market_values,true);

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

        let mut actual_ids:Vec<EOrd> = (0..self.jobs.len() * self.species.len()).map(
            |indx| EOrd { 
                job_id: self.jobs[indx % self.jobs.len()].job.id,
                species_id: self.species[indx / self.jobs.len()].species.id
            }
        ).collect();

        let planets:Vec<PlanetOptimizationReturn> = self.planets.iter_mut().map(
            |p|p.empire_intraplanetary_optimization(&imperial_species_job_priorities,&actual_ids)
        ).collect();

        return EmpireOptimizationReturn { planets };
    }
}
 
#[derive(Clone)]
pub struct EmpireSpeciesOptimization {
    pub species: Species,
    pub modifier: Array<f32>
}
impl EmpireSpeciesOptimization {
    pub fn new(empire_species:&EmpireSpecies) -> Self {
        Self { species: *empire_species.species, modifier: empire_species.modifier }
    }
}

struct PlanetOptimization {
    unemployed_pops: HashMap<usize,usize>, // Id, Count
    modifier: Array<f32>,
    jobs: HashMap<usize,usize> // Job ID, Positions
}
impl PlanetOptimization {
    fn news(planets:&[Planet]) -> Vec<Self> {
        planets.iter().map(|planet| PlanetOptimization::new(planet)).collect()
    }
    fn new(planet:&Planet) -> Self {
        let jobs:HashMap<usize,usize> = planet.jobs.iter().map(|(key,val)| (*key,val.positions)).collect();
        Self { unemployed_pops:planet.population_totals.clone(),modifier: planet.modifier.clone(), jobs }
    }
    fn empire_intraplanetary_optimization(&mut self,imperial_species_job_priorities:&Array<f32>,ids:&Vec<EOrd>) -> PlanetOptimizationReturn {
        let planet_job_priorities = mul(imperial_species_job_priorities,&self.modifier,true);

        let vec = to_vec(&planet_job_priorities);
        let mut labelled_vec:Vec<(f32,&EOrd)> = vec.iter().zip(ids.iter()).map(
            |(v,eord)| (*v,eord)
        ).collect(); // Gets job priorities zipped with their job & species IDs

        labelled_vec.sort_by(|(a,_),(b,_)| a.partial_cmp(b).unwrap()); // Sorts into descending

        let mut return_planet = PlanetOptimizationReturn { 
            jobs:self.jobs.iter().map(|(key,val)|(*key,JobPositionOptimizationReturn { employees:Vec::new()})).collect() 
        };

        // Iterates over priorities assigning species to jobs starting descending priorities
        for assignment in labelled_vec {
            if let Some(&mut pop_count) = self.unemployed_pops.get_mut(&assignment.1.species_id) {
                if let Some(&mut open_positions) = self.jobs.get_mut(&assignment.1.job_id) {
                    // TODO Can an `if let` be used here?
                    let pops_assinged = cmp::min(pop_count,open_positions);
                    if pops_assinged > 0 { // >0 === !=0
                        // Since `return_planet.jobs` contains all keys of `self.jobs` we can use `unwrap()`.
                        return_planet.jobs.get_mut(&assignment.1.job_id).unwrap().employees.push(
                            SpeciesPositionOptimization { count: pops_assinged, species_id: assignment.1.species_id }
                        );

                        // Decrease the number of open positions in job by number of pops just assigned to the job.
                        open_positions -= pops_assinged;
                        // Decrease the number of available pops of the species by the number of pops of this species just assigned to a job.
                        pop_count -= pops_assinged;

                        // Prevents future runs of checking job, then checking min when pops are 0.
                        if pop_count == 0 { self.unemployed_pops.remove(&assignment.1.species_id); }
                    }
                }
            }
        }

        return return_planet;
    }
}
struct EOrd {
    job_id: usize,
    species_id:usize
}
fn to_vec<T: arrayfire::HasAfEnum + Default + Clone>(array: &arrayfire::Array<T>) -> Vec<T> {
    let mut vec = vec![T::default(); array.elements()];
    array.host(&mut vec);
    return vec;
}