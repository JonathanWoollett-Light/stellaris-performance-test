use arrayfire::{add,reorder,Array,Dim4,mul,MatProp,join_many,tile,reorder_v2};

use crate::{
    Modifier,
    Empire,EmpireJob,EmpireSpecies,Planet,Species,SpeciesPositionOptimization,
    EmpireOptimizationReturn,PlanetOptimizationReturn,JobPositionOptimizationReturn,
    NUMBER_OF_RESOURCES,JOBS_MAX
};
use std::{
    cmp,
    collections::HashMap
};

pub struct EmpireOptimization {
    pub planets: Vec<PlanetOptimization>,
    pub modifier: Modifier,
    pub empire_species: Vec<EmpireSpeciesOptimization>,
    pub empire_jobs: [EmpireJob;JOBS_MAX]
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
            empire_species: empire.species.iter().map(|s|EmpireSpeciesOptimization::new(s)).collect(),
            empire_jobs: empire.jobs.clone()
        }
    }
    // TODO Convert all the code under `Convert 2d vec to array` comments to using `join_many` when next arrayfire release is out
    pub fn intraplanetary_optimization(&mut self,market_values:&Array<f32>) -> EmpireOptimizationReturn {
        // Stage 0: Definitions
        // --------------------------------------------------------------------------------

        // Sets job matricies
        // ----------------------------------------
        // Set job production
        let job_prod_arr = join_many(
            1,
            self.empire_jobs.iter().map(|j|&j.job.production).collect()
        );

        // Set job multiplier matrix
        let job_mul_arr = join_many(
            1,
            self.empire_jobs.iter().map(|j|&j.modifier.multiplier).collect()
        );

        // Set job addend matrix
        let job_add_arr = join_many(
            1,
            self.empire_jobs.iter().map(|j|&j.modifier.addend).collect()
        );
        

        // Sets species matricies
        // ----------------------------------------
        // Set species multiplier
        let species_mul_arr = join_many(
            1,
            self.empire_species.iter().map(|es|&es.species.modifier.multiplier).collect()
        );

        // Set species addend
        let species_add_arr = join_many(
            1,
            self.empire_species.iter().map(|es|&es.species.modifier.addend).collect()
        );

        // Set empire species multiplier
        let empire_species_mul_arr = join_many(
            1,
            self.empire_species.iter().map(|es|&es.modifier.multiplier).collect()
        );
        // Set empire species addend
        let empire_species_add_arr = join_many(
            1,
            self.empire_species.iter().map(|es|&es.modifier.addend).collect()
        );
        

        // Sets employability masks
        // ----------------------------------------
        // Sets species employability masks
        let species_employability_mask = join_many(
            1,
            self.empire_species.iter().map(|es|&es.species.employability).collect()
        );
        // Sets empire species employability masks
        let empire_species_employability_mask = join_many(
            1,
            self.empire_species.iter().map(|es|&es.employability).collect()
        );
        
        // Stage 1: Intermediary arrays
        // --------------------------------------------------------------------------------

        // Job productions before addition of species addend and before all mulipliers
        let pre_mul_job_prods = job_prod_arr + job_add_arr;

        // Compressed empire species and species modifiers into single matricies
        let compressed_species_muls = species_mul_arr + empire_species_mul_arr - 1;
        let compressed_species_adds = species_add_arr + empire_species_add_arr;

        // Species and job multiplier compressed into 1 matrix
        let compressed_multiplier = compressed_species_muls + 

        // Compressed species employability and empire species employability into single matrix
        let compressed_employability_mask = species_employability_mask * empire_species_employability_mask;

        let tiled_job_production = tile(&pre_mul_job_prods,Dim4::new(&[0,0,self.empire_species.len() as u64,0]));
        let reorderd_job_production = reorder(&tiled_job_production,Dim4::new(&[0,2,1,3]));
        let summed_production = add(&reorderd_job_production,&compressed_species_adds,true);
        let multiplied_production = mul(&summed_production,compressed_species_muls,true)

        // Market values adjusted by empire modifier
        let imerial_market_values = &self.modifier.multiplier * market_values;

        // Stage 2: Job priorities
        // --------------------------------------------------------------------------------

        let market_adjusted = mul(&job_adjusted,&imerial_market_values,true);

        let dims = Dim4::new(&[self.jobs.len() as u64, self.species.len() as u64, 1, 1]);
        let mut imperial_species_job_priorities: Array<f32> = Array::<f32>::new_empty(dims);
        arrayfire::gemm(
            &mut imperial_species_job_priorities,
            MatProp::TRANS,
            MatProp::NONE,
            vec![1.],
            &market_adjusted,
            &compressed_species_mods_arr,
            vec![0.],
        );

        let employable_imperial_species_job_priorities = imperial_species_job_priorities * compressed_employability_mask;

        let ids:Vec<EOrd> = (0..self.jobs.len() * self.species.len()).map(
            |indx| EOrd { 
                job_id: self.jobs[indx % self.jobs.len()].job.id,
                species_id: self.species[indx / self.jobs.len()].species.id
            }
        ).collect();

        let planets:Vec<PlanetOptimizationReturn> = self.planets.iter_mut().map(
            |p|p.empire_intraplanetary_optimization(&employable_imperial_species_job_priorities,&ids)
        ).collect();

        return EmpireOptimizationReturn { planets };
    }
}
 
pub struct EmpireSpeciesOptimization {
    pub species: Species,
    pub modifier: Modifier,
    pub employability: Array<bool>
}
impl EmpireSpeciesOptimization {
    pub fn new(empire_species:&EmpireSpecies) -> Self {
        unsafe {
            Self { 
                species: (*empire_species.species).clone(),
                modifier: empire_species.modifier.clone(),
                employability: empire_species.employability.clone()
            }
        }
    }
}

pub struct PlanetOptimization {
    unemployed_pops: HashMap<usize,usize>, // Id, Count
    modifier: Modifier,
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
            jobs: self.jobs.iter().map(|(key,_)|(*key,JobPositionOptimizationReturn { employees:Vec::new()})).collect() 
        };

        // Iterates over priorities assigning species to jobs starting descending priorities
        for assignment in labelled_vec {
            if assignment.0 == 0f32 { break; } // 0 = cannot be employed, since its ordered after 1st 0 all vals will be 0.
            if let Some(pop_count) = self.unemployed_pops.get_mut(&assignment.1.species_id) {
                if let Some(open_positions) = self.jobs.get_mut(&assignment.1.job_id) {
                    // TODO Can an `if let` be used here?
                    let pops_assinged = cmp::min(*pop_count,*open_positions);
                    if pops_assinged > 0 { // >0 === !=0
                        // Since `return_planet.jobs` contains all keys of `self.jobs` we can use `unwrap()`.
                        return_planet.jobs.get_mut(&assignment.1.job_id).unwrap().employees.push(
                            SpeciesPositionOptimization { count: pops_assinged, species_id: assignment.1.species_id }
                        );

                        // Decrease the number of open positions in job by number of pops just assigned to the job.
                        *open_positions -= pops_assinged;
                        // Decrease the number of available pops of the species by the number of pops of this species just assigned to a job.
                        *pop_count -= pops_assinged;

                        // Prevents future runs of checking job, then checking min when pops are 0.
                        if *pop_count == 0 { self.unemployed_pops.remove(&assignment.1.species_id); }
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