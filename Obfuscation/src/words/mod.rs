// Creator: https://github.com/fdx-xdf

pub fn deobfuscate_words(words: Vec<&str>, dataset: Vec<&str>) -> Vec<u8> {
    let mut shellcode: Vec<u8> = vec![0; words.len()];
    for sc_index in 0..shellcode.len() {
        for tt_index in 0..256 {
            if dataset[tt_index] == words[sc_index] {
                shellcode[sc_index] = tt_index as u8;
                break;
            }
        }
    }
    shellcode
}

pub fn obfuscate_words(shellcode: &mut Vec<u8>) {
    print!("let shellcode = vec![");
    let dataset: Vec<&str> = vec!["ironside", "chyliferous", "obediential", "brasil", "antalgics", "predisregard", "hennery", "lobellated", "bordelaise", "methylpentose", "maws", "chapacura", "gowns", "byron", "purport", "puritans", "fandangos", "crioceris", "dystonia", "intoxicator", "ascii", "nobling", "canoe", "paleomammologist", "nonalgebraical", "althorns", "enteralgia", "latimeria", "cannalling", "modular", "malleableize", "rassled", "imaginer", "pholcus", "negligency", "paintpot", "onomatope", "complementaries", "mandom", "outforth", "dyscrased", "missuade", "punctate", "radiating", "lengthens", "preconfession", "galvanomagnetic", "marrams", "compatibly", "eastlings", "ambier", "liquifying", "hontish", "entomion", "lepta", "befiddle", "clammyweed", "numerologists", "flatways", "powwowism", "juristical", "lontar", "centrals", "patroness", "co", "pleater", "overoptimist", "overreacted", "milliluxes", "isazoxy", "adonidin", "hinger", "despecification", "katakinetic", "dynamometric", "overgracious", "quartering", "allotting", "prereadiness", "overkick", "boma", "floorthrough", "mudland", "mimine", "katakinetomer", "nonliberal", "lignaloes", "hyperlogicalness", "entelechial", "posticous", "acicularity", "lagopode", "nontyphoidal", "llanberisslate", "loculose", "heterostatic", "yowt", "preadherence", "moravid", "keacorn", "protosphargis", "aurine", "incursions", "garfield", "mgr", "kilometre", "plastically", "physiotherapeutic", "andrewartha", "picine", "polytrichous", "limbmeal", "oxidable", "overfilter", "divisi", "cafuso", "asonant", "frumentaceous", "neurovisceral", "individuating", "enticeful", "coppaelite", "conformator", "gonophore", "lakie", "cerotic", "arracks", "expenditures", "rebellow", "myelomalacia", "belvedere", "plunger", "microporphyritic", "popularized", "priestless", "electropotential", "mistic", "lupines", "adroitly", "miasmas", "purpurize", "hipflask", "hakea", "fonnish", "growable", "pentaerythrite", "anapaganize", "metregram", "evidential", "onotogenic", "marque", "baled", "geometrid", "moule", "bugweed", "caretakers", "nonabstention", "yuletide", "photosynthetically", "collegiums", "ninepence", "gableended", "advisees", "axon", "overfemininely", "madrid", "chemotherapeuticness", "piloti", "dabba", "nonsolids", "laevorotation", "filarian", "recalcitrance", "psittacomorphic", "mournfullest", "pseudoankylosis", "abaser", "heartiness", "levigating", "labourism", "dizzied", "quernstone", "amphibia", "quinhydrone", "hookcheck", "combflower", "ecphorize", "madded", "yerb", "capitative", "onless", "picturers", "calina", "macrocosm", "codpitchings", "photojournalist", "frondigerous", "grassman", "polytope", "jingodom", "quinoidin", "pompiloid", "delicacies", "radiocalcium", "pimplous", "expressionable", "morphrey", "outstatistic", "musterer", "glebous", "ozonospheric", "phylloid", "ferngale", "promisees", "organicismal", "pneumatophoric", "brinded", "clouters", "micrurus", "computernik", "mermaid", "mitigates", "ombudsman", "hatchway", "broadsword", "decontaminations", "doctrinality", "forspread", "hypersubtlety", "naevus", "consortial", "cherogril", "fungify", "hood", "pimpleback", "joual", "prejudicator", "coleophora", "architecture", "conchies", "benzeneazobenzene", "numerously", "posadaship", "microweber", "padshahs", "cotyliform", "yirth", "nondiplomacy", "priori", "levitant", "eurypelma", "discrowned", "nasitis", "antelabium", "obtainably", "penlike"];
    let length = shellcode.len();
    let mut words: Vec<&str> = vec![""; length];

    for index in 0..length {
        words[index] = dataset[shellcode[index] as usize];
    }

    for index in 0..length {
        print!("\"{}\",", words[index]);
    }
    println!("];")
}