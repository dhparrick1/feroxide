use ion::Ion;
use reaction::ElemReaction;
use trait_reaction::Reaction;
use types::*;

use std::collections::HashMap;

// Reference: https://en.wikipedia.org/wiki/Standard_electrode_potential_(data_page)
// In doubt: Reference: Binas 6th edition, table 49

/// Get the Standard Electrode Potential (SEP) of a reaction
pub fn get_sep(elem_reaction: &ElemReaction<Ion>) -> Option<SEP> {
    if let Some(&sep) = SEPMAP.get(&elem_reaction) {
        Some(sep)
    } else if let Some(&sep) = SEPMAP.get(&elem_reaction.clone().swap()) {
        Some(sep)
    } else {
        None
    }
}

// This is mainly used for debugging purposes, to make sure no invalid reaction are added
macro_rules! str_to_reaction {
    ($s:expr) => {
        valid_or_panic(safe_unwrap_reaction(
            ElemReaction::<Ion>::ion_from_string($s),
            $s,
        ))
    };
}

macro_rules! add_str_reaction {
    ($map:expr, $r:expr, $sep:expr) => {
        $map.insert(str_to_reaction!($r), SEP::from($sep))
    };
}

/// Make sure the reaction is valid, panic otherwise
fn valid_or_panic(reaction: ElemReaction<Ion>) -> ElemReaction<Ion> {
    if !reaction.is_valid() {
        panic!("Invalid reaction: {}", reaction);
    }

    reaction
}

/// Check if the reaction is defined, then unwrap. Otherwise: panic!
fn safe_unwrap_reaction(reaction: Option<ElemReaction<Ion>>, s: &str) -> ElemReaction<Ion> {
    if reaction == None {
        panic!("Reaction failed to create: {}", s);
    }

    reaction.unwrap()
}

pub fn get_reactions_with_element(elem: &Ion) -> Vec<(ElemReaction<Ion>, SEP)> {
    let mut reactions = vec![];

    println!("Searching for reactions with element {}", elem);

    for (reaction, &sep) in SEPMAP.iter() {
        let lhs_elements = reaction
            .lhs
            .compounds
            .iter()
            .map(|x| &x.element)
            .collect::<Vec<&Ion>>();
        let rhs_elements = reaction
            .rhs
            .compounds
            .iter()
            .map(|x| &x.element)
            .collect::<Vec<&Ion>>();

        if lhs_elements.contains(&elem) {
            reactions.push((reaction.clone(), sep));
        } else if rhs_elements.contains(&elem) {
            reactions.push((reaction.clone().swap(), sep));
        }
    }

    reactions
}

lazy_static! {
    pub static ref SEPMAP: HashMap<ElemReaction<Ion>, SEP> = {
        let mut map = HashMap::new();

        // NOTE: These are all the common ones

        add_str_reaction!(map, "Ca;1 + e <> Ca", -3.8);
        add_str_reaction!(map, "Li;1 + e <> Li", -3.0401);
        add_str_reaction!(map, "Cs;1 + e <> Cs", -3.026);
        add_str_reaction!(map, "Rb;1 + e <> Rb", -2.98);
        add_str_reaction!(map, "K;1 + e <> K", -2.931);
        add_str_reaction!(map, "Ba;2 + 2e <> Ba", -2.912);
        add_str_reaction!(map, "Fr;1 + e <> Fr", -2.9);
        add_str_reaction!(map, "Ca;2 + 2e <> Ca", -2.899);
        add_str_reaction!(map, "Na;1 + e <> Na", -2.71);
        add_str_reaction!(map, "Mg;1 + e <> Mg", -2.70);
        add_str_reaction!(map, "Mg;2 + 2e <> Mg", -2.372);
        add_str_reaction!(map, "H2 + 2e <> 2H;-", -2.23);
        add_str_reaction!(map, "Be;2 + 2e <> Be", -1.847);
        add_str_reaction!(map, "Al;3 + 3e <> Al", -1.662);
        add_str_reaction!(map, "Mn;2 + 2e <> Mn", -1.185);
        add_str_reaction!(map, "Sn + 4H;1 + 4e <> SnH4", -1.07);
        add_str_reaction!(map, "2H2O + 2e <> H2 + 2OH;-", -0.8277);
        add_str_reaction!(map, "Zn;2 + 2e <> Zn", -0.7618);
        add_str_reaction!(map, "Fe;2 + 2e <> Fe", -0.44);
        add_str_reaction!(map, "Cr;3 + e <> Cr;2", -0.42);
        add_str_reaction!(map, "Sn;2 + 2e <> Sn", -0.13);
        add_str_reaction!(map, "Fe;3 + 3e <> Fe", -0.04);

        add_str_reaction!(map, "2H;1 + 2e <> H2", 0.0000);

        add_str_reaction!(map, "S4O6;2- + 2e <> 2S2O3;2-", 0.08);
        add_str_reaction!(map, "S + 2H;1 + 2e <> H2S", 0.14);
        add_str_reaction!(map, "Sn;4 + 2e <> Sn;2", 0.15);
        add_str_reaction!(map, "Cu;2 + e <> Cu;1", 0.159);
        add_str_reaction!(map, "HSO4;- + 3H;1 + 2e <> SO2 + 2H2O", 0.16);
        add_str_reaction!(map, "SO4;- + 4H;1 + 2e <> SO2 + 2H2O", 0.17);
        add_str_reaction!(map, "Cu;2 + 2e <> Cu", 0.337);
        add_str_reaction!(map, "O2 + 2H2O + 2e <> 4OH;-", 0.40); // From Binas
        add_str_reaction!(map, "SO2 + 4H;1 + 4e <> S + 2H2O", 0.50);
        add_str_reaction!(map, "Cu;1 + e <> Cu", 0.520);
        add_str_reaction!(map, "I3;- + 2e <> 3I;-", 0.53);
        add_str_reaction!(map, "I2 + 2e <> 2I;-", 0.54);
        add_str_reaction!(map, "S2O3;2- + 6H;1 + 4e <> 2S + 3H2O", 0.60);
        add_str_reaction!(map, "O2 + 2H;1 + 2e <> H2O2", 0.70);
        add_str_reaction!(map, "Fe;3 + e <> Fe;2", 0.77);
        add_str_reaction!(map, "Ag;1 + e <> Ag", 0.7996);
        add_str_reaction!(map, "NO3;- + 2H;1 + e <> NO2 + H2O", 0.80);
        add_str_reaction!(map, "MnO4;- + H;1 + e <> HMnO4;-", 0.90);
        add_str_reaction!(map, "MnO2 + 4H;1 + e <> Mn;3 + 2H2O", 0.95);
        add_str_reaction!(map, "NO3;- + 4H;1 + 3e <> NO + 2H2O", 0.958);
        add_str_reaction!(map, "O2 + 4H;1 + 4e <> 2H2O", 1.229);
        add_str_reaction!(map, "MnO2 + 4H;1 + 2e <> Mn;2 + 2H2O", 1.23);
        add_str_reaction!(map, "Cr2O7;2- + 14H;1 + 6e <> 2Cr;3 + 7H2O", 1.33);
        add_str_reaction!(map, "Cl2 + 2e <> 2Cl;-", 1.36);
        add_str_reaction!(map, "MnO4;- + 8H;1 + 5e <> Mn;2 + 4H2O", 1.51);
        add_str_reaction!(map, "Au;3 + 3e <> Au", 1.52);
        add_str_reaction!(map, "Pb;4 + 2e <> Pb;2", 1.69);
        add_str_reaction!(map, "MnO4;- + 4H;1 + 3e <> MnO2 + 2H2O", 1.70);
        add_str_reaction!(map, "H2O2 + 2H;1 + 2e <> 2H2O", 1.78);
        add_str_reaction!(map, "Co;3 + e <> Co;2", 1.82);
        add_str_reaction!(map, "Au;1 + e <> Au", 1.83);
        add_str_reaction!(map, "Ag;2 + e <> Ag;1", 1.98);
        add_str_reaction!(map, "S2O8;2- + 2e <> 2SO4;2-", 2.010);
        add_str_reaction!(map, "HMnO4;- + 3H;1 + 2e <> MnO2 + 2H2O", 2.09);
        add_str_reaction!(map, "FeO4;2- + 3e + 8H;1 <> Fe;3 + 4H2O", 2.20);
        add_str_reaction!(map, "F2 + 2e <> 2F;-", 2.87);
        add_str_reaction!(map, "F2 + 2H;1 + 2e <> 2HF", 3.05);
        add_str_reaction!(map, "KrF2 + 2e <> Kr + 2F;-", 3.27);


        // NOTE: The Wikipedia reference says something different,
        // yet according to the Binas this is the right one.
        // Since the value on the Wikipedia page (1.692) does not look "in-place",
        // I assumed the Binas one is correct here.
        add_str_reaction!(map, "MnO4;- + 2H2O + 3e <> MnO2 + 4OH;-", 0.60);



        map
    };
}
