use ion::Ion;
use types::*;

use std::collections::HashMap;

// Reference: https://en.wikipedia.org/wiki/Standard_enthalpy_of_formation
// In doubt: Reference: Binas 6th edition, table 57
// Missing values from http://www.mrbigler.com/misc/energy-of-formation.html
// and http://www.conradnaleway.net/ThermoData.PDF

/// Get the Standard Enthalpy of Formation (SEF) of a ion
pub fn get_sef(ion: &Ion) -> Option<SEF> {
    if let Some(&sef) = SEFMAP.get(&ion) {
        Some(sef)
    } else {
        None
    }
}

// This is mainly used for debugging purposes, to make sure no invalid reaction are added
macro_rules! str_to_ion {
    ($s:expr) => {
        safe_unwrap_ion(Ion::from_string($s), $s)
    };
}

macro_rules! add_str_ion {
    ($map:expr, $s:expr, $sef:expr) => {
        $map.insert(str_to_ion!($s), SEF::from($sef))
    };
}

/// Check if the ion is defined, then unwrap. Otherwise: panic!
fn safe_unwrap_ion(ion: Option<Ion>, s: &str) -> Ion {
    if ion == None {
        panic!("Ion failed to create: {}", s);
    }

    ion.unwrap()
}

lazy_static! {
    pub static ref SEFMAP: HashMap<Ion, SEF> = {
        let mut map = HashMap::new();

        // NOTE: Conditions: T = 298K, p = p0

        add_str_ion!(map, "AlCl3", -705.63);
        add_str_ion!(map, "Al2O3", -1669.8);
        // add_str_ion!(map, "Al(OH)3", -1277.0);
        // add_str_ion!(map, "Al2(SO4)3", -3440.0);

        add_str_ion!(map, "BaCl2", -858.6);
        add_str_ion!(map, "BaCO3", -1213.0);
        // add_str_ion!(map, "Ba(OH)2", -944.7);
        add_str_ion!(map, "BaO", -548.1);
        add_str_ion!(map, "BaSO4", -1473.2);

        add_str_ion!(map, "Be", 0.0);
        // add_str_ion!(map, "Be(OH)2", -902.9999);
        add_str_ion!(map, "BeO", -609.4);

        add_str_ion!(map, "BCl3", -402.96);

        add_str_ion!(map, "Br;-", -121.0); // (aq)
        add_str_ion!(map, "Br", 111.884); //(g)
        add_str_ion!(map, "Br2", 30.91);
        add_str_ion!(map, "BrF3", -255.6);
        add_str_ion!(map, "HBr", -36.29);

        add_str_ion!(map, "CdO", -258.0);
        // add_str_ion!(map, "Cd(OH)2", -561.0);
        add_str_ion!(map, "CdS", -162.0);
        add_str_ion!(map, "CdSO4", -935.0);

        add_str_ion!(map, "Ca", 178.2); // (g)
        add_str_ion!(map, "Ca;2+", 1925.9); // (g)
        add_str_ion!(map, "CaC2", -59.8);
        add_str_ion!(map, "CaCO3", -1206.9);
        add_str_ion!(map, "CaCl2", -795.8);
        add_str_ion!(map, "CaCl2", -877.3);
        // add_str_ion!(map, "Ca3(PO4)2", -4132.0);
        add_str_ion!(map, "CaF2", -1219.6);
        add_str_ion!(map, "CaH2", -186.2);
        //add_str_ion!(map, "Ca(OH)2", -986.09);
        //add_str_ion!(map, "Ca(OH)2", -1002.82);
        add_str_ion!(map, "CaO", -635.09);
        add_str_ion!(map, "CaSO4", -1434.52);
        add_str_ion!(map, "CaS", -482.4);
        add_str_ion!(map, "CaSiO3", -1630.0);

        add_str_ion!(map, "Cs", 76.5); // (g)
        add_str_ion!(map, "Cs", 2.09); // (l)
        add_str_ion!(map, "Cs;+", 457.964); // (aq)
        add_str_ion!(map, "CsCl", -443.04);

        add_str_ion!(map, "C", 0.0); // graphite
        add_str_ion!(map, "C", 1.9); // diamond
        add_str_ion!(map, "C", 716.67); // (g)
        add_str_ion!(map, "CO2", -393.509);
        add_str_ion!(map, "CS2", 89.41);
        add_str_ion!(map, "CS2", 116.7);
        add_str_ion!(map, "CO", -110.525);
        add_str_ion!(map, "COCl2", -218.8);

        add_str_ion!(map, "CO2", -419.26); // (aq)

        add_str_ion!(map, "HCO3;-", -689.93);
        add_str_ion!(map, "CO3;2-", -675.23);

        add_str_ion!(map, "Cl", 121.7);
        add_str_ion!(map, "Cl;-", -167.2); // (aq)

        add_str_ion!(map, "CuO", -155.2);
        add_str_ion!(map, "CuSO4", -769.98);

        // add_str_ion!(map, "H", 218.0); // (g)
        add_str_ion!(map, "H2O", -241.818);
        add_str_ion!(map, "H2O", -285.8);
        add_str_ion!(map, "H;+", 0.0); // (aq)
        add_str_ion!(map, "OH;-", -230.0); // (aq)

        add_str_ion!(map, "H2O2", -187.8);
        add_str_ion!(map, "H3PO4", -1288.0);
        add_str_ion!(map, "HCN", 130.5);
        add_str_ion!(map, "HBr", -36.3);
        add_str_ion!(map, "HCl", -92.3);
        add_str_ion!(map, "HCl", -167.2);
        add_str_ion!(map, "HF", -273.3);
        add_str_ion!(map, "HI", 26.5);

        add_str_ion!(map, "I2", 62.438); // (g)
        add_str_ion!(map, "I2", 23.0); // (aq)
        add_str_ion!(map, "I;-", -55.0); // (aq)


        add_str_ion!(map, "Fe;2+", -89.1); // From conradnaleway
        add_str_ion!(map, "Fe;3+", -48.5); // From conradnaleway
        add_str_ion!(map, "Fe3C", 5.4);
        add_str_ion!(map, "FeCO3", -750.6);
        add_str_ion!(map, "FeCl3", -399.4);
        add_str_ion!(map, "FeO", -272.0);
        add_str_ion!(map, "Fe3O4", -1118.4);
        add_str_ion!(map, "Fe2O3", -824.2);
        //add_str_ion!(map, "Fe(OH)3", -823.0); // From mrbigler
        //add_str_ion!(map, "Fe(OH)2", -569.0); // From conradnaleway
        // NOTE: Temporary until the parentheses are implemented
        add_str_ion!(map, "FeO2H2;", -569.0); // From conradnaleway

        add_str_ion!(map, "FeSO4", -929.0);
        //add_str_ion!(map, "Fe2(SO4)3", -2583.0);
        add_str_ion!(map, "FeS", -102.0);
        add_str_ion!(map, "FeS2", -178.0);

        add_str_ion!(map, "PbO2", -277.0);
        add_str_ion!(map, "PbS", -100.0);
        add_str_ion!(map, "PbSO4", -920.0);
        //add_str_ion!(map, "Pb(NO3)2", -452.0);
        add_str_ion!(map, "PbSO4", -920.0);

        add_str_ion!(map, "Mg;2", -466.85); // (aq)

        add_str_ion!(map, "MgCO3", -1095.797);
        add_str_ion!(map, "MgCl2", -641.8);
        //add_str_ion!(map, "Mg(OH)2", -924.54); // (s)
        //add_str_ion!(map, "Mg(OH)2", -926.8); // (aq)
        add_str_ion!(map, "MgO", -601.6);
        add_str_ion!(map, "MgSO4", -1278.2);

        add_str_ion!(map, "MnO", -384.9);
        add_str_ion!(map, "MnO2", -519.7);
        add_str_ion!(map, "Mn2O3", -971.0);
        add_str_ion!(map, "Mn3O4", -1387.0);
        add_str_ion!(map, "MnO4;-", -543.0);

        add_str_ion!(map, "HgO", 90.83);
        add_str_ion!(map, "HgS", -58.2);

        add_str_ion!(map, "NH3", -80.8); // (aq)

        // NOTE: The Wikipedia page has 2 different values, but I used the one from the Binas
        // NOTE: [Atkins - Physical Chemistry] has -46.11, like the other Wikipedia value
        // NOTE: According to https://webbook.nist.gov/cgi/cbook.cgi?ID=C7664417&Mask=1 it has to be -45.94
        add_str_ion!(map, "NH3", -45.94); // (g)

        add_str_ion!(map, "NH4Cl", -314.55);
        add_str_ion!(map, "NH4NO3", -365.6);
        add_str_ion!(map, "NO2", 33.2);
        add_str_ion!(map, "N2O", 82.05);
        add_str_ion!(map, "NO", 90.29);
        add_str_ion!(map, "N2O4", 9.16);
        add_str_ion!(map, "N2O5", -43.1);
        add_str_ion!(map, "N2O5", 11.3);

        add_str_ion!(map, "O", 249.0);
        add_str_ion!(map, "O3", 143.0);

        add_str_ion!(map, "P4", 0.0); // white
        add_str_ion!(map, "P", -17.4); // red
        add_str_ion!(map, "P", -39.3); // black

        add_str_ion!(map, "PCl3", -319.7);
        add_str_ion!(map, "PCl3", -278.0);
        add_str_ion!(map, "PCl5", -440.0);
        add_str_ion!(map, "KBr", -392.2);
        add_str_ion!(map, "K2CO3", -1150.0);
        add_str_ion!(map, "KClO3", -391.4);
        add_str_ion!(map, "KCl", -436.68);
        add_str_ion!(map, "KF", -562.6);
        add_str_ion!(map, "K2O", -363.0);
        add_str_ion!(map, "KClO4", -430.12);

        add_str_ion!(map, "Si", 368.2); // (g)
        add_str_ion!(map, "SiC", -73.22);
        add_str_ion!(map, "SiCl4", -640.1);
        add_str_ion!(map, "SiO2", -910.86);

        add_str_ion!(map, "AgBr", -99.5);
        add_str_ion!(map, "AgCl", -127.01);
        add_str_ion!(map, "AgI", -62.4);
        add_str_ion!(map, "Ag2O", -31.1);
        add_str_ion!(map, "Ag2S", -31.8);

        add_str_ion!(map, "Na", 107.5); // (g)
        add_str_ion!(map, "NaHCO3", -950.8);
        add_str_ion!(map, "Na2CO3", -1130.77);
        add_str_ion!(map, "NaCl", -407.27);
        add_str_ion!(map, "NaCl", -411.12);
        add_str_ion!(map, "NaCl", -385.92);
        add_str_ion!(map, "NaCl", -181.42);
        add_str_ion!(map, "NaF", -569.0);
        add_str_ion!(map, "NaOH", -469.15);
        add_str_ion!(map, "NaOH", -425.93);
        add_str_ion!(map, "NaNO3", -446.2);
        add_str_ion!(map, "NaNO3", -424.8);
        add_str_ion!(map, "Na2O", -414.2);

        add_str_ion!(map, "H2S", -20.63);
        add_str_ion!(map, "SO2", -296.84);
        add_str_ion!(map, "SO3", -395.7);
        add_str_ion!(map, "H2SO4", -814.0);

        add_str_ion!(map, "Ti", 468.0); // (g)
        add_str_ion!(map, "TiCl4", -763.2);
        add_str_ion!(map, "TiCl4", -804.2);
        add_str_ion!(map, "TiO2", -944.7);

        add_str_ion!(map, "Zn", 130.7); // (g)
        add_str_ion!(map, "ZnCl2", -415.1);
        add_str_ion!(map, "ZnO", -348.0);
        add_str_ion!(map, "ZnSO4", -980.14);



        map
    };
}
