use data_sef::*;
use ion::Ion;
use molecule::Molecule;
use trait_element::Element;
use trait_properties::Properties;
use trait_reaction::Reaction;
use types::*;

use std::collections::HashMap;
use std::hash::*;
use std::mem;
use std::ops::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// An elementair reaction (not containing ions)
pub struct ElemReaction<E: Element> {
    /// The left-hand-side
    pub lhs: ReactionSide<E>,

    /// The right-hand-side
    pub rhs: ReactionSide<E>,

    // If it's an equilibrium
    pub is_equilibrium: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// A side of a reaction
pub struct ReactionSide<E: Element> {
    /// The compounds of this side
    pub compounds: Vec<ReactionCompound<E>>,
}

#[derive(Debug, Eq, Clone)]
/// A reaction compound
pub struct ReactionCompound<E: Element> {
    /// The element it uses
    pub element: E,

    /// The amount of moles needed
    pub amount: u16,
}

impl<E: Element> ElemReaction<E> {
    /// Convert a string representation of an elementary reaction into one
    pub fn ion_from_string(string: &str) -> Option<ElemReaction<Ion>> {
        let mut token = String::new();

        let mut lhs = None;
        let mut rhs = None;
        let mut is_equilibrium = false;

        for c in string.chars() {
            if c == '<' || c == '>' || c == '⇌' || c == '→' {
                if lhs == None {
                    lhs = ReactionSide::<Ion>::ion_from_string(&token);
                    token = String::new();
                }

                // NOTE: No support for backwards-only reactions
                if c == '<' || c == '⇌' {
                    is_equilibrium = true;
                }

                continue;
            }

            token.push(c);
        }

        if !token.is_empty() {
            rhs = ReactionSide::<Ion>::ion_from_string(&token);
        }

        if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
            Some(ElemReaction {
                lhs,
                rhs,
                is_equilibrium,
            })
        } else {
            None
        }
    }

    /// Convert a string representation of a reaction into one
    pub fn molecule_from_string(string: &str) -> Option<ElemReaction<Molecule>> {
        let mut token = String::new();

        let mut lhs = None;
        let mut rhs = None;
        let mut is_equilibrium = false;

        for c in string.chars() {
            if c == '<' || c == '>' || c == '⇌' || c == '→' {
                if lhs == None {
                    lhs = ReactionSide::<Molecule>::molecule_from_string(&token);
                    token = String::new();
                }

                if c == '<' || c == '⇌' {
                    is_equilibrium = true;
                }

                continue;
            }

            token.push(c);
        }

        if !token.is_empty() {
            rhs = ReactionSide::<Molecule>::molecule_from_string(&token);
        }

        if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
            Some(ElemReaction {
                lhs,
                rhs,
                is_equilibrium,
            })
        } else {
            None
        }
    }

    /// Get the sign of the equation ( → or ⇌ ), depending whether it is an equilibrium or not
    #[cfg(not(feature = "no_utf"))]
    pub fn reaction_sign(&self) -> &str {
        if self.is_equilibrium {
            " ⇌ "
        } else {
            " → "
        }
    }

    #[cfg(feature = "no_utf")]
    pub fn reaction_sign(&self) -> &str {
        if self.is_equilibrium {
            " <> "
        } else {
            " -> "
        }
    }

    /// Swap the equation
    pub fn swap(mut self) -> Self {
        mem::swap(&mut self.lhs, &mut self.rhs);
        self
    }
}

impl<E: Element> ReactionSide<E> {
    /// Convert a string representation of a reactionside into one
    pub fn ion_from_string(symbol: &str) -> Option<ReactionSide<Ion>> {
        let mut compounds = vec![];
        let mut token = String::new();
        let mut was_whitespace = false;

        for c in symbol.chars() {
            if is_whitespace!(c) {
                was_whitespace = true;
                continue;
            }

            if c == '+' && was_whitespace {
                if let Some(compound) = ReactionCompound::<Ion>::ion_from_string(&token) {
                    compounds.push(compound);
                } else {
                    println!("Failed to parse ion '{}'", &token);
                }

                token = String::new();
            } else {
                token.push(c);
            }

            was_whitespace = false;
        }

        if !token.is_empty() {
            if let Some(compound) = ReactionCompound::<Ion>::ion_from_string(&token) {
                compounds.push(compound);
            } else {
                println!("Failed to parse ion '{}'", &token);
            }
        }

        if !compounds.is_empty() {
            Some(ReactionSide { compounds })
        } else {
            None
        }
    }

    /// Convert a string representation of a reactionside into one
    pub fn molecule_from_string(symbol: &str) -> Option<ReactionSide<Molecule>> {
        let mut compounds = vec![];

        let mut token = String::new();
        for c in symbol.chars() {
            if is_whitespace!(c) {
                continue;
            }

            if c == '+' {
                if let Some(compound) = ReactionCompound::<Molecule>::molecule_from_string(&token) {
                    compounds.push(compound);
                }
                token = String::new();
                continue;
            }

            token.push(c);
        }

        if !token.is_empty() {
            if let Some(compound) = ReactionCompound::<Molecule>::molecule_from_string(&token) {
                compounds.push(compound);
            }
        }

        if !compounds.is_empty() {
            Some(ReactionSide { compounds })
        } else {
            None
        }
    }

    /// Calculate the total charge of this reaction side
    pub fn total_charge(&self) -> AtomCharge {
        let mut total_charge = AtomCharge::from(0);

        for compound in &self.compounds {
            if let Some(charge) = compound.element.get_charge() {
                total_charge += charge;
            }
        }

        total_charge
    }

    /// Calculate the energy this side has
    pub fn energy(&self) -> Energy {
        let mut energy = 0.0;

        for compound in &self.compounds {
            let sef = get_sef(&compound.element.clone().get_ion().unwrap());

            if sef.is_some() {
                energy += Energy_type::from(SEF_type::from(sef.unwrap()))
                    * Energy_type::from(compound.amount);
            } else {
                let mol = compound.element.clone().get_molecule().unwrap();
                let is_diatomic = mol.is_diatomic();
                let is_monoatomic = mol.compounds.len() == 1 && mol.compounds[0].amount == 1;

                if !is_diatomic && !is_monoatomic {
                    println!(
                        "Failed to get SEF for compound {}, assuming 0",
                        compound.symbol()
                    );
                }
            }
        }

        Energy::from(energy)
    }

    /// Calculate the total amount of atoms this side contains
    pub fn total_atoms(&self, include_electrons: bool) -> HashMap<AtomNumber, u16> {
        let mut atoms: HashMap<AtomNumber, u16> = HashMap::new();

        // for molecule_compound in self.compounds:
        for reaction_compound in &self.compounds {
            if let Some(molecule) = reaction_compound.element.clone().get_molecule() {
                for molecule_compound in &molecule.compounds {
                    let atom_number = molecule_compound.atom.number.clone();

                    if !include_electrons && atom_number == AtomNumber::from(0) {
                        // Ignore electrons in the atom count
                        continue;
                    }

                    let mut amount;
                    if let Some(&old_amount) = atoms.get(&atom_number) {
                        amount = old_amount;
                    } else {
                        amount = 0;
                    }

                    amount += u16::from(molecule_compound.amount) * reaction_compound.amount;

                    atoms.insert(atom_number, amount);
                }
            }
        }

        atoms
    }
}

impl<E: Element> ReactionCompound<E> {
    /// Convert a string representation of a reaction compound into one
    pub fn ion_from_string(symbol: &str) -> Option<ReactionCompound<Ion>> {
        let mut amount: u16 = 0;
        let mut element = None;

        let mut set_amount = true;
        let mut token = String::new();

        for c in symbol.chars() {
            if set_amount && is_number!(c) {
                amount *= 10;
                amount += u16::from(to_number!(c));
                continue;
            } else {
                set_amount = false;
            }

            token.push(c);
        }

        if !token.is_empty() {
            element = Ion::from_string(&token);
        }

        if amount == 0 {
            amount = 1;
        }

        if let Some(element) = element {
            Some(ReactionCompound { amount, element })
        } else {
            None
        }
    }

    /// Convert a string representation of a reaction compound into one
    pub fn molecule_from_string(symbol: &str) -> Option<ReactionCompound<Molecule>> {
        let mut amount: u16 = 0;
        let mut element = None;

        let mut set_amount = true;
        let mut token = String::new();

        for c in symbol.chars() {
            if set_amount && is_number!(c) {
                amount *= 10;
                amount += u16::from(to_number!(c));
                continue;
            } else {
                set_amount = false;
            }

            token.push(c);
        }

        if !token.is_empty() {
            element = Molecule::from_string(&token);
        }

        if amount == 0 {
            amount = 1;
        }

        if let Some(element) = element {
            Some(ReactionCompound { amount, element })
        } else {
            None
        }
    }
}

impl<E: Element> Reaction<E> for ElemReaction<E> {
    /// NOTE: This function is still a WIP!
    fn equalise(&self) -> bool {
        println!("####    The equalise function is not yet ready.");

        let total_left = self.lhs.total_atoms(false);
        let total_right = self.rhs.total_atoms(false);

        // If both sides are already equal, do nothing
        if total_left == total_right {
            return true;
        }

        for (atom_number, l_amount) in total_left {
            let r_amount: u16;

            match total_right.get(&atom_number) {
                Some(&x) => r_amount = x,
                None => r_amount = 0,
            }

            if r_amount == 0 {
                println!("It's impossible to make this reaction work: {}", self);
                return false;
            }

            if l_amount != r_amount {
                let difference = {
                    if l_amount > r_amount {
                        l_amount - r_amount
                    } else {
                        r_amount - l_amount
                    }
                };

                if difference > 0 {
                    // Increase right side
                    println!("[right] We know what to do, but it's just not implemented yet.");
                } else {
                    // Increase left side
                    println!("[left] We know what to do, but it's just not implemented yet.");
                }
            }
        }

        // NOTE: true only for debugging
        true
    }

    fn is_valid(&self) -> bool {
        self.lhs.total_atoms(false) == self.rhs.total_atoms(false)
            && self.lhs.total_charge() == self.lhs.total_charge()
    }

    fn energy_cost(&self) -> Energy {
        self.rhs.energy() - self.lhs.energy()
    }

    fn elem_reaction(&self) -> ElemReaction<E> {
        self.clone()
    }
}

impl<E: Element> Add for ReactionSide<E> {
    type Output = ReactionSide<E>;

    /// Adding two ReactionSide's adds their compounds
    fn add(self, mut rhs: ReactionSide<E>) -> ReactionSide<E> {
        let mut compounds = self.compounds.clone();
        compounds.append(&mut rhs.compounds);

        ReactionSide { compounds }
    }
}

impl<E: Element> Mul<u16> for ReactionSide<E> {
    type Output = ReactionSide<E>;

    /// Multiplying a ReactionSide with a number
    /// multiplies the amount of all compounds of that side
    fn mul(self, rhs: u16) -> ReactionSide<E> {
        let mut compounds = self.compounds.clone();

        for compound in &mut compounds {
            compound.amount *= rhs;
        }

        ReactionSide { compounds }
    }
}

impl<E: Element> PartialEq for ReactionCompound<E> {
    /// Two ReactionCompound's are equal if their elements are equal
    fn eq(&self, rhs: &ReactionCompound<E>) -> bool {
        self.element == rhs.element
    }
}

impl<E: Element> Hash for ReactionCompound<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.element.hash(state);
    }
}

impl<E: Element> Properties for ElemReaction<E> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += &self.lhs.symbol();
        symbol += self.reaction_sign();
        symbol += &self.rhs.symbol();
        symbol += "    [";
        symbol += &format!("{:.3}", &self.energy_cost());
        symbol += " J]";

        symbol
    }

    fn name(&self) -> String {
        let mut name = String::new();

        name += &self.lhs.name();
        name += self.reaction_sign();
        name += &self.rhs.name();
        name += "    [";
        name += &format!("{:.3}", &self.energy_cost());
        name += " J]";

        name
    }

    fn mass(&self) -> AtomMass {
        // Law of Conservation of Mass
        AtomMass::from(0.0)
    }

    fn is_diatomic(&self) -> bool {
        // Reactions can't be diatomic
        false
    }
}

impl<E: Element> Properties for ReactionSide<E> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        let mut first = true;
        for reaction_compound in &self.compounds {
            if !first {
                symbol += " + ";
            }
            first = false;

            symbol += &reaction_compound.symbol();
        }

        symbol
    }

    fn name(&self) -> String {
        let mut name = String::new();

        let mut first = true;
        for reaction_compound in &self.compounds {
            if !first {
                name += " + ";
            }
            first = false;

            name += &reaction_compound.name();
        }

        name
    }

    fn mass(&self) -> AtomMass {
        let mut mass = AtomMass::from(0.0);

        for reaction_compound in &self.compounds {
            mass += reaction_compound.mass();
        }

        mass
    }

    fn is_diatomic(&self) -> bool {
        // Reactions can't be diatomic
        false
    }
}

impl<E: Element> Properties for ReactionCompound<E> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        if self.amount > 1 {
            symbol += &self.amount.to_string();
        }

        symbol += &self.element.symbol();

        symbol
    }

    fn name(&self) -> String {
        let mut name = String::new();

        if self.amount > 1 {
            name += &self.amount.to_string();
            name += " ";
        }

        name += &self.element.name();

        name
    }

    fn mass(&self) -> AtomMass {
        self.element.mass() * (AtomMass_type::from(self.amount))
    }

    fn is_diatomic(&self) -> bool {
        self.element.is_diatomic()
    }
}

impl<E: Element> Element for ReactionCompound<E> {
    fn get_charge(&self) -> Option<AtomCharge> {
        self.element.get_charge()
    }

    fn get_molecule(self) -> Option<Molecule> {
        self.element.get_molecule()
    }

    fn get_ion(self) -> Option<Ion> {
        self.element.get_ion()
    }
}
