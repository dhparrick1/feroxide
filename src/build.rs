#[macro_use]
extern crate serde_derive;
extern crate toml;

mod types;
use types::{AtomGroup_type, AtomMass_type, AtomNumber_type};

use std::collections::HashMap;
use std::fs::File;
use std::io::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    pub atoms: HashMap<String, Atom>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Atom {
    pub number: AtomNumber_type,
    pub group: AtomGroup_type,
    pub symbol: String,
    pub name: String,
    pub mass: AtomMass_type,
    pub diatomic: bool,
}

#[allow(dead_code)]
/// Write an example TOML file to the data_atoms.rs file
fn write(mut atoms_rs_file: &File) {
    let mut atoms: HashMap<String, Atom> = HashMap::new();

    // Add example atom
    atoms.insert(
        "hydrogen".to_owned(),
        Atom {
            number: 1,
            symbol: "H".to_owned(),
            name: "hydrogen".to_owned(),
            group: 1,
            mass: 1.008,
            diatomic: true,
        },
    );

    // Generate config
    let config = Config { atoms: atoms };

    // Convert to TOML
    let config_string = toml::to_string(&config).unwrap();

    // Write TOML to file
    atoms_rs_file.write_all(config_string.as_bytes()).ok();
}

#[allow(dead_code)]
/// Reads the data_atoms.toml file, converts it to the appropriate format
/// and writes it to the atoms.rs file
fn read_and_write(mut atoms_toml_file: &File, mut atoms_rs_file: &File) {
    // Read TOML file
    let mut atoms_toml = String::new();
    atoms_toml_file.read_to_string(&mut atoms_toml).ok();

    // Convert to config struct
    let config: Config;
    match toml::from_str(&atoms_toml) {
        Ok(x) => config = x,
        Err(e) => panic!("{:?}", e),
    }

    // Write header to file
    atoms_rs_file.write_all(b"use atom::Atom;\n").ok();
    atoms_rs_file
        .write_all(b"use types::{ AtomNumber, AtomMass, AtomGroup };\n")
        .ok();

    // Convert items from TOML file to RS syntax
    for (capsname, atom) in config.atoms.clone().into_iter() {
        let Atom {
            number,
            symbol,
            name,
            group,
            mass,
            diatomic,
        } = atom;

        let rust_atom = format!(
            "
pub const {capsname}: Atom = Atom {{
    number: AtomNumber{{0:{number}}}, mass: AtomMass{{0:{mass:.5}}}, symbol: \"{symbol}\",
    name: \"{name}\", group: AtomGroup{{0:{group:?}}}, diatomic: {diatomic} }};
",
            capsname = capsname,
            name = name,
            number = number,
            mass = mass,
            symbol = symbol,
            group = group,
            diatomic = diatomic
        );

        // Append to file
        atoms_rs_file.write_all(rust_atom.as_bytes()).ok();
    }

    atoms_rs_file
        .write_all(b"\npub const ALL_ATOMS: &[Atom] = &[")
        .ok();

    for (i, (capsname, _)) in config.atoms.iter().enumerate() {
        if i > 0 {
            atoms_rs_file.write_all(b", ").ok();
        }
        atoms_rs_file.write_all(capsname.as_bytes()).ok();
    }

    atoms_rs_file.write_all(b"];").ok();
}

fn main() {
    let mut atoms_toml_file = File::open("src/data_atoms.toml").unwrap();
    let mut atoms_rs_file = File::create("src/data_atoms.rs").unwrap();

    // NOTE: For debugging only:
    // write(&mut atoms_rs_file);

    read_and_write(&mut atoms_toml_file, &mut atoms_rs_file);
}
