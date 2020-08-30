use arrayfire::{Array,Dim4,randu,af_print,print_gen,constant,add,lt};
use std::{
    time::{Instant,Duration},
    io::{stdout, Write},
    thread,
    collections::{HashMap,HashSet}
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

const NUMBER_OF_RESOURCES:usize = 11;   // Number of pop producible resources.

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

const TIERS:usize = 3; // 0=rulers,1=specialists,2=Worker,3=Undesireables


fn main() {
    create_game_data_file();


    let start = Instant::now();
    let jobs:[Arc<Job>;JOBS_MAX] = gen_jobs();
    let species:Vec<Species> = gen_species(false);
    let mut empires = gen_empires(&jobs,&species);
    let market_values = Array::new(&MARKET_VALUES,Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1]));
    println!("Gen time: {}",time(start.elapsed()));

    let pop_sum:usize = empires.iter().map(|empire| empire.pops()).sum();
    println!("pop_sum: {}",pop_sum.to_formatted_string(&Locale::en));

    run(Duration::from_millis(100),&mut empires,1000,10,30,market_values);
}

fn create_game_data_file() {
    // (Name, Value)
    let resources:HashMap<&str,Resource> = vec![
        ("energy",Resource::new(1.,true,true)),                     // Energy
        ("minerals",Resource::new(1.,true,true)),                   // Minerals
        ("food",Resource::new(1.,true,true)),                       // Food
        ("consumer goods",Resource::new(2.,true,true)),             // Consumer Goods
        ("alloys",Resource::new(4.,true,true)),                     // Alloys

        ("exotic gases",Resource::new(10.,true,true)),              // Exotic Gases
        ("rare crystals",Resource::new(10.,true,true)),             // Rare Crystals
        ("volatile motes",Resource::new(10.,true,true)),            // Volatile Motes
        ("dark matter",Resource::new(20.,true,true)),               // Dark Matter
        ("living metal",Resource::new(20.,true,true)),              // Living Metal
        ("nanites",Resource::new(60.,true,true)),                   // Nanites

        ("trade value",Resource::new(1.1,true,false)),              // Trade Value

        ("influence",Resource::new(80.,true,true)),                 // Influence
        ("unity",Resource::new(40.,true,true)),                     // Unity

        ("physics research",Resource::new(20.,true,true)),          // Physics research
        ("society research",Resource::new(20.,true,true)),          // Society research
        ("engineering research",Resource::new(20.,true,true)),      // Engineering research

        ("administrative capacity",Resource::new(2.,true,false)),   // Administrative Capacity
        ("naval capacity",Resource::new(2.,true,false)),            // Naval Capacity

        ("housing",Resource::new(0.,false,false)),                  // Housing
        ("amenities",Resource::new(1.,false,false)),                // Amenities
        ("stability",Resource::new(60.,false,false)),               // Stability
        ("crime",Resource::new(-30.,false,false)),                  // Crime
        ("planetary defense armies",Resource::new(5.,false,false)), // Planetary Defense Armies
        ("pop growth speed",Resource::new(60.,false,false)),        // Pop Growth Speed
        ("monthly pop assembly",Resource::new(60.,false,false)),    // Pop Growth Speed
    ].into_iter().collect();

    // What does `tier` refer to?
    //  `tier` is a better name for 'stratum' in this context, 0=Ruler,1=Specialist/Complex Drone, 2=Worker/Menial Drone, etc. 
    //  The way stratum is described in game cannot be well implemented. It is awkward, inextensible, not generally applicable and inconsistent.
    //  This is simply a result of stratum being strings, using a `tier` as an unsigned integer is nicer.

    // TODO need to add housing
    // TODO need to add functionality for conditional production (e.g. when lithiods produce x else produce x)
    

    let (tiers,jobs) = create_jobs(&resources,&[
        ("administator",0,&[("unity",3.),("amenities",3.)])
    ]);

    println!("{} jobs",jobs.len());
    // TODO job catagories

    let traits = create_traits(&resources,&jobs,&[
        ("agrarian",&[],&[TraitEffect::Res("food",AddOrMul::Mul(0.15))]),
        ("nerve_stapled",&[0,1],&[TraitEffect::All(AddOrMul::Mul(0.05))]),
        ("void_dweller",&[],&[TraitEffect::Tier(0,AddOrMul::Mul(0.15)),TraitEffect::Tier(1,AddOrMul::Mul(0.15))]),
    ]);

    panic!("stop here");

    fn create_jobs(
        resource_namelist: &HashMap<&str,Resource>,
        jobs: &[(&str,usize,&[(&str,f32)])] // (name,tier,[resource,production quantity])
    ) -> ([Vec<String>;TIERS],HashMap<String,Vec<ResourceAffect>>) {

        let mut tiers:[Vec<String>;TIERS] = [Vec::new();TIERS];

        let jobs:HashMap<String,Vec<ResourceAffect>> = jobs.into_iter().map(
            |(name,tier,production)| {
                // Checks tier and adds jobs to tier group
                if *tier > TIERS { panic!("Job assigned to teir which doesn't exist"); }
                tiers[*tier].push(name.to_string());

                // Checks all resources produced create vec
                let produces:Vec<ResourceAffect> = production.iter().map(
                    |(r,p)| {
                        if !resource_namelist.contains_key(r) { panic!("Job produces resource which doesn't exist"); }
                        ResourceAffect { resource:r.to_string(), adjustment: *p }
                    }
                ).collect();

                // Returns
                (name.to_string(),produces)
            }
        ).collect();

        return (tiers,jobs);
    }
    fn create_traits<'a,'b>(
        resource_namelist: &HashMap<&str,Resource>,
        job_namelist: &HashMap<String,Vec<ResourceAffect>>,
        traits: &[(&str,&'b [usize],&'a [TraitEffect],)]
    ) -> HashMap<String,Trait<'a,'b>> {
        traits.iter().map(
            |(name,disallowed_tiers,effects)| {
                // Checks disallowments
                for d in disallowed_tiers.iter() { 
                    if *d > TIERS { panic!("Trait disallowed for teir which doesn't exist"); }
                }
                // Checks `TraitEffect`s
                for e in effects.iter() {
                    match e {
                        TraitEffect::Res(name,_) => {
                            if !resource_namelist.contains_key(name) { panic!("Trait affects resource which doesn't exist"); }
                        },
                        TraitEffect::Job(name,_) => {
                            if !job_namelist.contains_key(&name.to_string()) { panic!("Trait affects job which doesn't exist"); }
                        },
                        TraitEffect::Tier(tier,_) => {
                            if *tier > TIERS { panic!("Trait affects tier which doesn't exist"); }
                        }
                    }
                }

                (name.to_string(), Trait { effects, disallowed_tiers })
            }
        ).collect()
    }
    fn create_techs(

    ) {

    }

    struct Resource {
        value: f32,
        // Is the resource empire wide (shared between planets)?
        //  Energy, research, administrative capacity etc. are.
        //  Crime, stability & amenities are not.
        interplanetary: bool,
        // Does the resource add up each month?
        //  Energy, research, unity etc. are additive.
        //  Crime, stability & administrative capacity are not.
        additive: bool
    }
    impl Resource {
        pub fn new(value: f32, interplanetary: bool, additive: bool) -> Self {
            Self { value, interplanetary, additive  }
        }
    }
    

    #[derive(Clone)]
    struct ResourceAffect {
        resource:String,
        adjustment:f32,
    }
    impl<'a> ResourceAffect {
        pub fn new(namelist:&HashMap<&str,Resource>,resource:&str,adjustment:f32) -> Self {
            if !namelist.contains_key(resource) { panic!("Item name \"{}\" for affect doesn't exist.",resource); }
            Self { resource:resource.to_string(), adjustment }
        }
    }

    struct Trait<'a,'b> {
        effects: &'a [TraitEffect<'a>],
        disallowed_tiers: &'b [usize]
    }
    enum TraitEffect<'a> {
        Res(&'a str,AddOrMul),      // Affects a specific resource
        Job(&'a str,AddOrMul),      // Affects a specific job
        Tier(usize,AddOrMul),       // Affects a tier of jobs
        All(AddOrMul)               // Affects all resources
    }
    enum AddOrMul {
        Add(f32),Mul(f32)
    }

    struct TraitAffect<'a> {
        production: Vec<ResourceAffect<'a>>,
        disallowed_tiers: Vec<usize> // Job tiers this traits disallows the pop to work (e.g. nerve stapled dissallows 0 and 1 (ruler and specialist))
    }

    #[derive(Clone)]
    enum Affects<'a> {
        Item(&'a str),
        Catagory(&'a str)
    }
    impl<'a> Affects<'a> {
        pub fn new_item(
            item:&'a str,
            items_namelist:&HashMap<&str,Resource>
        ) -> Self {
            if !items_namelist.contains_key(item) { panic!("Item name \"{}\" for affect doesn't exist.",item); }
            Self::Item(item)
        }
        pub fn new_catagory(
            catagory:&'a str,
            catagories_namelist:&HashMap<&str,Vec<&str>>
        ) -> Self {
            if !catagories_namelist.contains_key(catagory) { panic!("Catagory name \"{}\" for affect doesn't exist.",catagory); }
            Self::Catagory(catagory)
        }
    }
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

fn gen_jobs() -> [Arc<Job>;JOBS_MAX] {
    let mut jobs: Vec<Arc<Job>> = (0..JOBS_MAX).map(|_|Arc::new(Job::default())).collect();
    
    let mut jobs_arr: [Arc<Job>;JOBS_MAX] = [Arc::new(Job::default());JOBS_MAX];
    jobs_arr.clone_from_slice(&jobs[0..JOBS_MAX]);

    return jobs_arr;
}



static JOB_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub struct Job {
    id:usize,
    production:Array<f32>
}
impl Job {
    pub fn new() -> Self {
        Self { id:JOB_COUNTER.fetch_add(1,Ordering::SeqCst), production: randu::<f32>(Dim4::new(&[NUMBER_OF_RESOURCES as u64,1,1,1])) }
    }
}
impl Default for Job {
    fn default() -> Self { 
        Self { id: usize::default(), production:Array::new_empty(Dim4::default()) } 
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
    modifier:Modifier,
    employability:Array<bool>
}
impl Species {
    pub fn new() -> Self {
        Self {
            id: SPECIES_COUNTER.fetch_add(1,Ordering::SeqCst), 
            modifier: Modifier::new(),
            employability: lt(&SPECIES_EMPLOYABILITY,&randu::<f32>(Dim4::new(&[JOBS_MAX as u64,1,1,1])),false)
        }
    }
}

fn gen_empires(job_prods: &[Arc<Job>;JOBS_MAX],species_mods: &Vec<Species>) -> Vec<Empire> {
    let mut empires:Vec<Empire> = Vec::with_capacity(NUMBER_OF_EMPIRES);
    for _ in 0..NUMBER_OF_EMPIRES {
        let mut empire = Empire::new(job_prods,species_mods);
        empire.gen_planets();
        empires.push(empire);
    }
    return empires;
}