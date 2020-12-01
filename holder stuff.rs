let mut jobs: HashMap<&str,Vec<ResourceAffect>> = vec![
    ("administrator", vec![
        ResourceAffect::new(&resources,"unity", 3.),
        ResourceAffect::new(&resources,"amenities", 8.)
    ]),
    ("executive", vec![
        ResourceAffect::new(&resources,"amenities", 5.),
        ResourceAffect::new(&resources,"unity", 2.),
        ResourceAffect::new(&resources,"trade value", 4.)
    ]),
    ("high priest", vec![
        ResourceAffect::new(&resources,"amenities", 5.),
        ResourceAffect::new(&resources,"society research", 2.),
        ResourceAffect::new(&resources,"unity", 5.) // +1 with 'Exalted Priesthood'
    ]),
    ("merchant", vec![
        ResourceAffect::new(&resources,"trade value", 8.),
        ResourceAffect::new(&resources,"amenities", 5.)
        // +2 unity with 'Merchant Guilds'
    ]),
    ("noble", vec![
        ResourceAffect::new(&resources,"stability", 5.),
        ResourceAffect::new(&resources,"unity", 3.)
    ]),
    ("science director", vec![
        ResourceAffect::new(&resources,"physics research", 5.),
        ResourceAffect::new(&resources,"society research", 5.),
        ResourceAffect::new(&resources,"engineering research", 5.),
        ResourceAffect::new(&resources,"amenities", 3.)
    ]),
    ("artisan", vec![
        ResourceAffect::new(&resources,"consumer goods", 6.),
        ResourceAffect::new(&resources,"minerals", -6.)
    ]),
    ("bureaucrat", vec![
        ResourceAffect::new(&resources,"administrative capacity", 10.),
        ResourceAffect::new(&resources,"crime", -8.),
        ResourceAffect::new(&resources,"consumer goods", -1.)
    ]),
    ("chemist", vec![
        ResourceAffect::new(&resources,"volatile motes", 2.),
        ResourceAffect::new(&resources,"minerals", -10.)
    ]),
    ("colonist", vec![
        ResourceAffect::new(&resources,"amenities", 5.)
        // +1 minerals if 'Lithoid' else +1 food
    ]),
    ("culture worker", vec![
        ResourceAffect::new(&resources,"society research", 5.),
        ResourceAffect::new(&resources,"unity", 3.),
        ResourceAffect::new(&resources,"consumer goods", -2.)
    ]),
    ("duelist", vec![
        ResourceAffect::new(&resources,"unity", 3.),
        ResourceAffect::new(&resources,"amenities", 12.),
        ResourceAffect::new(&resources,"naval capacity", 2.),
        ResourceAffect::new(&resources,"alloys", -1.)
    ]),
    ("enforcer", vec![
        ResourceAffect::new(&resources,"crime", -25.),
        ResourceAffect::new(&resources,"planetary defense armies", 2.),
        ResourceAffect::new(&resources,"unity", 1.) // +1 with 'Police State'
    ]),
    ("entertainer", vec![
        ResourceAffect::new(&resources,"unity", 2.),
        ResourceAffect::new(&resources,"amenities", 10.),
        ResourceAffect::new(&resources,"consumer goods", -1.)
    ]),
    ("gas refiner", vec![
        ResourceAffect::new(&resources,"exotic gases", 2.),
        ResourceAffect::new(&resources,"minerals", -10.),
    ]),
    ("manager", vec![
        ResourceAffect::new(&resources,"society research", 2.),
        ResourceAffect::new(&resources,"unity", 3.),
        ResourceAffect::new(&resources,"trade value", 2.),
        ResourceAffect::new(&resources,"consumer goods", -2.)
    ]),
    ("medical worker", vec![
        ResourceAffect::new(&resources,"amenities", 2.),
        ResourceAffect::new(&resources,"pop growth speed", 0.05),
        ResourceAffect::new(&resources,"consumer goods", -1.)
    ]),
    ("metallurgist", vec![
        ResourceAffect::new(&resources,"alloys", 3.),
        ResourceAffect::new(&resources,"minerals", -6.)
    ]),
    ("priest", vec![
        ResourceAffect::new(&resources,"society research", 2.),
        ResourceAffect::new(&resources,"amenities", 5.),
        ResourceAffect::new(&resources,"unity", 3.), // +1 with 'Exalted Priesthood'
        ResourceAffect::new(&resources,"consumer goods", -2.),
    ]),
    ("researcher", vec![
        ResourceAffect::new(&resources,"physics research", 4.),
        ResourceAffect::new(&resources,"society research", 4.),
        ResourceAffect::new(&resources,"engineering research", 4.),
        // +1 unity with 'Technocracy'
        ResourceAffect::new(&resources,"consumer goods", -2.),
    ]),
    ("roboticist", vec![
        ResourceAffect::new(&resources,"monthly pop assembly", 2.),
        ResourceAffect::new(&resources,"alloys", -2.),
    ]),
    ("telepath", vec![
        ResourceAffect::new(&resources,"crime", -35.),
        ResourceAffect::new(&resources,"unity", 3.), // +1 with 'Police State'
    ]),
    ("translucer", vec![
        ResourceAffect::new(&resources,"rare crystals", 2.),
        ResourceAffect::new(&resources,"minerals", -10.),
    ]),
    ("clerk", vec![
        ResourceAffect::new(&resources,"trade value", 2.),
        ResourceAffect::new(&resources,"amenities", 2.)
    ]),
    ("crystal miner", vec![
        ResourceAffect::new(&resources,"rare crystals", 2.)
    ]),
    ("farmer", vec![
        ResourceAffect::new(&resources,"food", 6.)
    ]),
    ("gas extractor", vec![
        ResourceAffect::new(&resources,"exotic gases", 2.)
    ]),
    ("miner", vec![
        ResourceAffect::new(&resources,"minerals", 4.) // +1 with 'Mining Guilds' // +1 with 'Rockbreakers'
    ]),
    ("mote harvester", vec![
        ResourceAffect::new(&resources,"volatile motes", 2.)
    ]),
    ("prosperity preacher", vec![
        ResourceAffect::new(&resources,"unity", 1.),
        ResourceAffect::new(&resources,"amenities", 3.),
        ResourceAffect::new(&resources,"trade value", 3.)
    ]),
    ("soldier", vec![
        ResourceAffect::new(&resources,"naval capacity", 4.),
        ResourceAffect::new(&resources,"planetary defense armies", 3.)
    ]),
    ("technician", vec![
        ResourceAffect::new(&resources,"energy", 4.) // +2 if 'Machine Intelligence'
    ]),
    ("agri-drone", vec![
        ResourceAffect::new(&resources,"food", 5.) // +1 if 'Hive Mind'
    ]),
    ("maintenance drone", vec![
        ResourceAffect::new(&resources,"amenities", 4.)
        // +1 unity if 'Maintenance Protocols'
    ]),
    ("warrior drone", vec![
        ResourceAffect::new(&resources,"naval capacity", 4.),
        ResourceAffect::new(&resources,"planetary defense armies", 3.),
    ]),
    ("brain drone", vec![
        ResourceAffect::new(&resources,"physics research", 4.),
        ResourceAffect::new(&resources,"society research", 4.),
        ResourceAffect::new(&resources,"engineering research", 4.),
        ResourceAffect::new(&resources,"minerals", -6.),
    ]),
    ("calculator", vec![
        ResourceAffect::new(&resources,"physics research", 4.),
        ResourceAffect::new(&resources,"society research", 4.),
        ResourceAffect::new(&resources,"engineering research", 4.),
        ResourceAffect::new(&resources,"energy", -4.),
    ]),
    ("chem-drone", vec![
        ResourceAffect::new(&resources,"volatile motes", 2.),
        ResourceAffect::new(&resources,"minerals", -10.),
    ]),
    ("coordinator", vec![
        ResourceAffect::new(&resources,"administrative capacity", 15.), // +3 with 'Integrated Preservation'
        ResourceAffect::new(&resources,"energy", -4.),
        // -2 crime with 'Integrated Preservation' ('deviancy' is an alias for 'crime')
    ]),
    ("crystal mining", vec![
        ResourceAffect::new(&resources,"rare crystals", 2.),
        ResourceAffect::new(&resources,"energy", -1.),
    ]),
    ("evaluator", vec![
        ResourceAffect::new(&resources,"unity", 4.),
        ResourceAffect::new(&resources,"energy", -1.),
    ]),
    ("fabricator", vec![ 
        ResourceAffect::new(&resources,"alloys", 4.),
        ResourceAffect::new(&resources,"minerals", -8.),
    ]),
    ("foundry drone", vec![ // Alias for 'metallurgist'
        ResourceAffect::new(&resources,"alloys", 3.),
        ResourceAffect::new(&resources,"minerals", -6.),
    ]),
    ("gas extraction drone", vec![ 
        ResourceAffect::new(&resources,"exotic gases", 2.),
        ResourceAffect::new(&resources,"energy", -1.),
    ]),
    ("hunter-seeler drone", vec![ 
        ResourceAffect::new(&resources,"unity", 1.),
        ResourceAffect::new(&resources,"crime", -20.), // 'deviancy' is alias for 'crime'
        ResourceAffect::new(&resources,"planetary defense armies", 2.)
    ]),
    ("mote harvesting drone", vec![
        ResourceAffect::new(&resources,"volatile motes", 2.),
        ResourceAffect::new(&resources,"energy", -1.)
    ]),
    ("replicator", vec![
        ResourceAffect::new(&resources,"monthly pop assembly", 1.),
        ResourceAffect::new(&resources,"alloys", -1.)
    ]),
    ("spawning drone", vec![
        ResourceAffect::new(&resources,"amenities", 5.),
        ResourceAffect::new(&resources,"pop growth speed", 0.25)
        // -5 minerals if 'Lithoid' else -5 food
    ]),
    ("synapse drone", vec![
        ResourceAffect::new(&resources,"unity", 3.),
        ResourceAffect::new(&resources,"administrative capacity", 5.),
        // +2 amenities with 'Instinctive Synchronization'
        ResourceAffect::new(&resources,"food", -2.)
        // -2 minerals if 'Lithoid' else -2 food
    ]),
    ("artisan drone", vec![
        ResourceAffect::new(&resources,"consumer goods", 8.),
        ResourceAffect::new(&resources,"minerals", -8.)
    ]),
    ("bio-trophy", vec![
        ResourceAffect::new(&resources,"unity", 2.)
        // TODO '+0.25% Complex Drone Output' and '−1 Pop Housing Usage'
    ]),
    // TODO 'Special jobs'
    ("grid amalgamated", vec![
        ResourceAffect::new(&resources,"energy", 6.)
        // TODO '−0.5 Pop Housing Usage'
    ]),
    ("livestock", vec![
        // TODO '−0.5 Pop Housing Usage'
        // +2 minerals if 'Lithioid' else +4 food
    ]),
    ("servant", vec![
        ResourceAffect::new(&resources,"amenities", 4.)
        // TODO '−0.5 Pop Housing Usage'
    ]),
    ("overseer", vec![
        ResourceAffect::new(&resources,"crime", -25.),
        // TODO +25 happiness
        ResourceAffect::new(&resources,"planetary defense armies", 2.),
        // -2 energy with 'Anticrime Campaign'
    ]),
    ("toiler", vec![
        ResourceAffect::new(&resources,"amenities", 2.),
    ]),
    ("unemployed", vec![
        // A whole bunch of modifiers
    ]),
    ("extermination", vec![
        // +2 unity with 'Determined Exterminator'
    ]),
    ("forced labor", vec![
        // A whole bunch of modifiers
        ResourceAffect::new(&resources,"food", 3.),
        ResourceAffect::new(&resources,"minerals", 3.)
    ]),
    ("processing", vec![
        // +4 minerals if 'Lithiod'
        // else +3 alloys if 'Robot'
        // else +6 food

        // +2 society research if 'Devouring Swarm'
    ]),
    ("chemical processing", vec![
        ResourceAffect::new(&resources,"energy", 6.)
    ]),
    ("criminal", vec![
        ResourceAffect::new(&resources,"trade value", -1.)
    ]),
    ("deviant drone", vec![
        ResourceAffect::new(&resources,"energy", -1.)
    ]),
    ("titan hunter", vec![
        ResourceAffect::new(&resources,"food", 8.),
        ResourceAffect::new(&resources,"trade value", 6.)
    ]),
    ("odd factory worker", vec![
        ResourceAffect::new(&resources,"alloys", 4.)
    ]),
    ("subterranean liaison officer", vec![
        ResourceAffect::new(&resources,"trade value", 5.),
        ResourceAffect::new(&resources,"amenities", 3.),
        // TODO -1 pop housing usage
    ]),
    ("subterranean contact drone", vec![
        ResourceAffect::new(&resources,"energy", 3.),
        ResourceAffect::new(&resources,"amenities", 3.),
        // TODO -1 pop housing usage
    ]),
    ("transmuter", vec![
        ResourceAffect::new(&resources,"alloys", 4.),
    ]),
    ("gas plant engineer", vec![
        ResourceAffect::new(&resources,"exotic gases", 3.),
        ResourceAffect::new(&resources,"minerals", -10.),
    ]),
    ("gas plant drone", vec![
        ResourceAffect::new(&resources,"exotic gases", 2.),
        ResourceAffect::new(&resources,"minerals", -10.),
    ]),
    ("cave cleaner", vec![
        ResourceAffect::new(&resources,"minerals", 5.),
        ResourceAffect::new(&resources,"energy", -2.),
    ]),
    ("dimensional portal researcher", vec![
        ResourceAffect::new(&resources,"physics research", 12.),
        // +1 unity if 'Technocracy'
    ]),
    ("feudal noble", vec![
        ResourceAffect::new(&resources,"amenities", 3.),
        ResourceAffect::new(&resources,"unity", 1.),
    ]),
    ("cleric", vec![
        ResourceAffect::new(&resources,"amenities", 2.),
        ResourceAffect::new(&resources,"unity", 1.),
    ]),
    ("scholar", vec![
        ResourceAffect::new(&resources,"physics research", 1.),
        ResourceAffect::new(&resources,"society research", 1.),
        ResourceAffect::new(&resources,"engineering research", 1.),
    ]),
    ("warrior", vec![
        ResourceAffect::new(&resources,"planetary defense armies", 3.),
    ]),
    ("peasant", vec![
        ResourceAffect::new(&resources,"food", 2.),
    ]),
    ("primitive farmer", vec![
        ResourceAffect::new(&resources,"food", 3.),
    ]),
    ("primitive miner", vec![
        ResourceAffect::new(&resources,"food", 3.),
    ]),
    ("primitive technician", vec![
        ResourceAffect::new(&resources,"energy", 1.),
    ]),
    ("laborer", vec![
        ResourceAffect::new(&resources,"consumer goods", 2.),
        ResourceAffect::new(&resources,"minerals", -2.),
    ]),
    ("hunter-gatherer", vec![
        ResourceAffect::new(&resources,"food", 1.),
        ResourceAffect::new(&resources,"amenities", 1.),
    ]),
    ("scavenger", vec![
        ResourceAffect::new(&resources,"food", 1.),
        ResourceAffect::new(&resources,"minerals", 1.),
        ResourceAffect::new(&resources,"consumer goods", 1.),
    ]),
    ("fallen empire overseer", vec![
        ResourceAffect::new(&resources,"stability", 5.),
        ResourceAffect::new(&resources,"amenities", 10.),
    ]),
    ("scavenger", vec![
        ResourceAffect::new(&resources,"food", 1.),
        ResourceAffect::new(&resources,"minerals", 1.),
        ResourceAffect::new(&resources,"consumer goods", 1.),
    ]),
    ("protector", vec![
        ResourceAffect::new(&resources,"crime", -25.),
        ResourceAffect::new(&resources,"planetary defense armies", 3.),
    ]),
    ("archivist", vec![
        ResourceAffect::new(&resources,"physics research", 5.),
        ResourceAffect::new(&resources,"society research", 5.),
        ResourceAffect::new(&resources,"engineering research", 5.),
    ]),
    ("acolyte of the hyperspanner", vec![
        ResourceAffect::new(&resources,"energy", 6.),
    ]),
    ("acolyte of the hammer", vec![
        ResourceAffect::new(&resources,"minerals", 6.),
    ]),
    ("acolyte of the plow", vec![
        ResourceAffect::new(&resources,"food", 8.),
    ]),
    ("augur of the shroud", vec![
        ResourceAffect::new(&resources,"unity", 10.),
    ]),
    ("acolyte of the workshop", vec![
        ResourceAffect::new(&resources,"amenities", 3.),
        ResourceAffect::new(&resources,"consumer goods", 2.)
    ]),
    ("xeno-keeper", vec![
        ResourceAffect::new(&resources,"amenities", 3.),
        ResourceAffect::new(&resources,"crime", -20.),
        ResourceAffect::new(&resources,"planetary defense armies", 2.)
    ]),
    ("xeno-ward", vec![
        ResourceAffect::new(&resources,"unity", 2.)
    ]),
    ("guardian", vec![
        ResourceAffect::new(&resources,"crime", -25.),
        ResourceAffect::new(&resources,"planetary defense armies", 3.)
    ]),
    ("caretaker", vec![
        ResourceAffect::new(&resources,"amenities", 5.)
    ]),
].into_iter().collect();


// Certain jobs are renamed under types of empires.
// Thus we have jobs with identical productions under different names.
// key = original name, value = alias
let job_aliases:HashMap<&str,&str> = vec![
    ("miner","mining drone"),
    ("technician","tech-drone"),
    ("gas refiner","refinery drone"),
    ("translucer","lensing drone"),
    ("deviant drone","corrupt drone"),
    ("odd factory worker","odd factory drone"),
    ("transmuter","transmuter drone"),
    ("dimensional portal researcher","dimensional portal drone"),
    ("feudal noble","bureaucrat"),
    ("cleric","priest"),
    ("scholar","researcher"),
    ("warrior","soldier"),
    ("fallen empire overseer","scavenger"),
    ("xeno-ward","hedonist")
].into_iter().collect();

// Adds job aliases (while this is not the more efficient way to handle this, it is the easiest and avoids adding a whole aliasing system)
for (og,alias) in job_aliases.iter() {
    if let Some(job) = jobs.get(og) {
        jobs.insert(alias,job.clone());
    } else {
        panic!("Job with original name given in alias doesn't exist.");
    }
}
println!("{} jobs with identical productions",job_aliases.len());