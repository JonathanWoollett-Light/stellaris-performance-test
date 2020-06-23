use rand::{thread_rng,Rng,rngs::ThreadRng};
use arrayfire::{Array,Dim4,randu,af_print,print_gen,constant,add};
use std::{
    time::{Instant,Duration},
    io::{stdout, Write},
    thread,
};
mod empire;
use empire::*;

mod empire_production;
use empire_production::EmpireProduction;

use crossterm::{cursor, QueueableCommand};
use tokio::task;
use std::sync::Arc;

use num_format::{Locale, ToFormattedString};

const NUMBER_OF_RESOURCES:usize = 10;   // Number of pop producible resources.

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
const MARKET_VALUES:[f32;11] = [1.,1.,1.,2.,4.,10.,10.,10.,20.,20.,20.];

const NUMBER_OF_EMPIRES:usize = 10;     // Number of empires.

const PLANETS_MIN:usize = 1;            // Minimum number of planets in an empire.
const PLANETS_MAX:usize = 15;           // Maximum number of planets in an empire.

const POP_MIN:usize = 20;               // Minimum number of pops on a planet.
const POP_MAX:usize = 200;              // Maximum number of pops on a planet.

const JOBS_MIN:usize = 1;               // Minimum number of jobs on a planet.
const JOBS_MAX:usize = 40;              // Maximum number of jobs on a planet.

const SPECIES_MIN:usize = 1;            // Minimum number of species on a planet.
const SPECIES_MAX:usize = 20;           // Maximum number of species on a planet.

fn main() {
    let start = Instant::now();
    let jobs:Vec<Arc<Array<f32>>> = gen_jobs(false);
    let species = gen_species(false);
    let empires = gen_empires(&jobs,&species);
    println!("Gen time: {}",time(start.elapsed()));

    let pop_sum:usize = empires.iter().map(|empire| empire.pops()).sum();
    println!("pop_sum: {}",pop_sum.to_formatted_string(&Locale::en));

    run(Duration::from_millis(100),&empires,1000,10);
}
#[tokio::main]
async fn run(step_wait:Duration,empires:&Vec<Empire>,days:usize,grace:usize) {
    
    //let mut total_calc_time = Duration::new(0,0);

    let mut empire_resources:Vec<Array<f32>> = vec!(constant(0f32,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])));

    let mut pending = None;

    let mut stdout = stdout();
    stdout.queue(cursor::SavePosition).unwrap();

    for i in 0..days {
        stdout.write(format!("Day {:#04}",i).as_bytes()).expect("log error");
        
        if i % 30 == 0 {
            let start = Instant::now();

            let future = calculate_incomes(EmpireProduction::news(&empires));
            let deadline = i+grace;
            pending = Some((deadline,future));

            if start.elapsed() < step_wait { thread::sleep(step_wait-start.elapsed()); }
            
        } else {
            thread::sleep(step_wait);
        }
        if let Some((deadline,future)) = pending.take() {
            if i == deadline {
                let income = future.await.unwrap();
                for (r,i) in empire_resources.iter_mut().zip(income.iter()) {
                    *r = add(r,i,false);
                }
            } else {
                pending = Some((deadline,future));
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

fn time(elapsed: Duration) -> String {
    let mut millis = elapsed.as_millis();
    let seconds = (millis as f32 / 1000f32).floor();
    millis = millis % 1000;
    let time = format!("{:#02}:{:#03}", seconds, millis);
    return time;
}

fn gen_jobs(print:bool) -> Vec<Arc<Array<f32>>> {
    let mut jobs:Vec<Arc<Array<f32>>> = Vec::with_capacity(JOBS_MAX);
    for _ in 0..JOBS_MAX {
        let prod = randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));
        jobs.push(Arc::new(prod));
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

fn gen_empires(job_prods: &Vec<Arc<Array<f32>>>,species_mods: &Vec<Array<f32>>) -> Vec<Empire> {
    let mut empires:Vec<Empire> = Vec::with_capacity(NUMBER_OF_EMPIRES);
    for _ in 0..NUMBER_OF_EMPIRES {
        let mut empire = Empire::new();
        empire.gen_planets(job_prods,species_mods);
        empires.push(empire);
    }
    return empires;
}