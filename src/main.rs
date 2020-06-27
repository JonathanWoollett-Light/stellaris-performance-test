use arrayfire::{Array,Dim4,randu,af_print,print_gen,constant,add,lt};
use std::{
    time::{Instant,Duration},
    io::{stdout, Write},
    thread
};
mod empire;
use empire::*;

mod empire_production;
use empire_production::EmpireProduction;

mod empire_optimization;
use empire_optimization::EmpireOptimization;

mod empire_optimization_returns;
use empire_optimization_returns::*;

use crossterm::{cursor, QueueableCommand};
use tokio::task;
use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};

use num_format::{Locale, ToFormattedString};

const NUMBER_OF_RESOURCES:usize = 11;   // Number of pop producible resources.

// https://stellaris.paradoxwikis.com/Trade#The_market
// Resource         Price
// ----------------------
// Energy           1
// Minerals         1
// Food 	        1
// Consumer Goods 	2
// Alloys 	        4
// Exotic Gases 	10
// Rare Crystals 	10
// Volatile Motes 	10
// Dark Matter 	    20
// Living Metal 	20
// Zro 	            20
const MARKET_VALUES:[f32;NUMBER_OF_RESOURCES] = [1.,1.,1.,2.,4.,10.,10.,10.,20.,20.,20.];

const NUMBER_OF_EMPIRES:usize = 10;         // Number of empires.

const PLANETS_MIN:usize = 1;                // Minimum number of planets in an empire.
const PLANETS_MAX:usize = 15;               // Maximum number of planets in an empire.

const POP_MIN:usize = 20;                   // Minimum number of pops on a planet.
const POP_MAX:usize = 200;                  // Maximum number of pops on a planet.

const JOBS_MIN:usize = 1;                   // Minimum number of jobs on a planet.
const JOBS_MAX:usize = 40;                  // Maximum number of jobs on a planet.

const SPECIES_MIN:usize = 1;                // Minimum number of species on a planet.
const SPECIES_MAX:usize = 20;               // Maximum number of species on a planet.

const SPECIES_EMPLOYABILITY:f32 = 0.8;      // Percentage of jobs species can be employed for

const OPTIMISATION_FREQUENCY:usize = 95;    // Every X days every planet in the empire is optimized.

// const traits:HashMap<&str,TraitAffect> = vec![
//     ("agrarian ",2u32),
//     ("asdd",3u32)
// ].into_iter().collect();

// struct TraitAffect {
//     production:Option<Array<f32>>,
//     employability:Option<Array<f32>>
// }
fn main() {
    let start = Instant::now();
    let jobs:Vec<Arc<Job>> = gen_jobs(false);
    let species:Vec<Species> = gen_species(false);
    let mut empires = gen_empires(&jobs,&species);
    let market_values = Array::new(&MARKET_VALUES,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));
    println!("Gen time: {}",time(start.elapsed()));

    let pop_sum:usize = empires.iter().map(|empire| empire.pops()).sum();
    println!("pop_sum: {}",pop_sum.to_formatted_string(&Locale::en));

    run(Duration::from_millis(100),&mut empires,1000,10,30,market_values);
}
#[tokio::main]
async fn run(step_wait:Duration,empires:&mut Vec<Empire>,days:usize,production_grace:usize,optimization_grace:usize,market_values:Array<f32>) {
    //let mut total_calc_time = Duration::new(0,0);

    let mut empire_resources:Vec<Array<f32>> = vec!(constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])));

    let mut production_pending = None;
    let mut optimisation_pending = None;

    let mut stdout = stdout();
    stdout.queue(cursor::SavePosition).unwrap();

    for i in 0..days {
        stdout.write(format!("Day {:#04}",i).as_bytes()).expect("log error");
        
        if i % 30 == 0 {
            let start = Instant::now();

            let future = calculate_incomes(EmpireProduction::news(&empires));
            production_pending = Some((i+production_grace,future));

            if start.elapsed() < step_wait { thread::sleep(step_wait-start.elapsed()); }

        } else if i % OPTIMISATION_FREQUENCY == 0 {
            let start = Instant::now();

            let future = optimize_pops(EmpireOptimization::news(&empires),market_values.clone());
            optimisation_pending = Some((i+optimization_grace,future));

            if start.elapsed() < step_wait { thread::sleep(step_wait-start.elapsed()); }

        } else {
            thread::sleep(step_wait);
        }
        // Only copies out deadline
        if let Some((deadline,_)) = production_pending {
            // Checks deadline
            if i == deadline {
                // If deadline, then move out future and value
                let future = production_pending.unwrap().1;
                let income = future.await.unwrap();
                for (r,i) in empire_resources.iter_mut().zip(income.iter()) {
                    *r = add(r,i,false);
                }
                production_pending = None;
            }
        }
        // Same as above
        if let Some((deadline,_)) = optimisation_pending {
            if i == deadline {
                let future = optimisation_pending.unwrap().1;
                let optimised_empires = future.await.unwrap();
                for (empire,optimised_empire) in empires.iter_mut().zip(optimised_empires.into_iter()) {
                    empire.intraplanetary_optimize(optimised_empire);
                }
                optimisation_pending = None;
            }
        }

        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
    }

    //println!("Average calculation time: {}",time(total_calc_time / (days / 30) as u32))
}
fn calculate_incomes(empires:Vec<EmpireProduction>) -> task::JoinHandle<Vec<Array<f32>>> {
    task::spawn_blocking(move || {
        return empires.iter().map(|empire|empire.run()).collect();
    })
}
fn optimize_pops(mut empires:Vec<EmpireOptimization>,market_values:Array<f32>) -> task::JoinHandle<Vec<EmpireOptimizationReturn>> {
    task::spawn_blocking(move || {
        return empires.iter_mut().map(|e|e.intraplanetary_optimization(&market_values)).collect();
    })
}

fn time(elapsed: Duration) -> String {
    let mut millis = elapsed.as_millis();
    let seconds = (millis as f32 / 1000f32).floor();
    millis = millis % 1000;
    let time = format!("{:#02}:{:#03}", seconds, millis);
    return time;
}

fn gen_jobs(print:bool) -> Vec<Arc<Job>> {
    let mut jobs:Vec<Arc<Job>> = Vec::with_capacity(JOBS_MAX);
    for _ in 0..JOBS_MAX {
        let prod = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));
        jobs.push(Arc::new(Job::new(prod)));
    }
    if print {
        println!("jobs:");
        for job in jobs.iter() {
            af_print!("",job.production);
        }
    }
    return jobs;
}

static JOB_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub struct Job {
    id:usize,
    production:Array<f32>
}
impl Job {
    pub fn new(production:Array<f32>) -> Self {
        Self { id:JOB_COUNTER.fetch_add(1,Ordering::SeqCst), production }
    }
}

fn gen_species(print:bool) -> Vec<Species> {
    let mut species:Vec<Species> = Vec::with_capacity(SPECIES_MAX);
    for _ in 0..SPECIES_MAX {
        species.push(Species::new());
    }
    
    if print {
        println!("species:");
        for species in species.iter() {
            af_print!("",species.modifier);
        }
    }
    return species;
}

static SPECIES_COUNTER: AtomicUsize = AtomicUsize::new(0);
#[derive(Clone)]
pub struct Species {
    id:usize,
    modifier:Array<f32>,
    employability:Array<bool>
}
impl Species {
    pub fn new() -> Self {
        Self {
            id: SPECIES_COUNTER.fetch_add(1,Ordering::SeqCst), 
            modifier: randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),
            employability: lt(&SPECIES_EMPLOYABILITY,&randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])),false)
        }
    }
}

fn gen_empires(job_prods: &Vec<Arc<Job>>,species_mods: &Vec<Species>) -> Vec<Empire> {
    let mut empires:Vec<Empire> = Vec::with_capacity(NUMBER_OF_EMPIRES);
    for _ in 0..NUMBER_OF_EMPIRES {
        let mut empire = Empire::new(job_prods,species_mods);
        empire.gen_planets();
        empires.push(empire);
    }
    return empires;
}