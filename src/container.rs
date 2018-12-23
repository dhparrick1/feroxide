use data_sep::*;
use ion::Ion;
use molecule::Molecule;
use reaction::ReactionSide;
use reaction::{ElemReaction, ReactionCompound};
use redox::RedoxReaction;
use trait_element::Element;
use trait_properties::Properties;
use trait_reaction::Reaction;
use types::*;

use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
/// A container for elements
pub struct Container<E: Element> {
    /// A vector with the contents of this container
    pub contents: Vec<ContainerCompound<E>>,

    /// The amount of energy available
    pub available_energy: Energy,
}

#[derive(Debug, Clone)]
/// A compound for containers
pub struct ContainerCompound<E: Element> {
    /// The element it contains
    pub element: E,

    /// The amount of moles of this element
    pub moles: Moles,
}

/// Convert a given `ReactionCompound` into a `ContainerCompound`
pub fn rc_to_cc<E: Element>(rc: ReactionCompound<E>) -> ContainerCompound<E> {
    ContainerCompound {
        element: rc.element,
        moles: Moles::from(Moles_type::from(rc.amount)),
    }
}

pub fn get_redox_reaction(container: &Container<Ion>) -> Option<RedoxReaction> {
    let mut oxidator: Option<(ElemReaction<Ion>, SEP)> = None;
    let mut reductor: Option<(ElemReaction<Ion>, SEP)> = None;

    for reaction in container.get_redox_reactions() {
        // TODO: But what if there exists an oxidator that provides this reductor with its needed molecules?
        if !container.has_enough_compounds_for_reaction(&reaction.0) {
            continue;
        }

        print!("{}           \twith SEP {}  ", reaction.0, reaction.1);

        // Find electrons
        if reaction
            .0
            .lhs
            .total_atoms(true)
            .contains_key(&AtomNumber::from(0))
        {
            println!("[oxi]");

            if let Some(oxi) = oxidator.clone() {
                if reaction.1 > oxi.1 {
                    oxidator = Some(reaction);
                }
            } else {
                oxidator = Some(reaction);
            }
        } else {
            println!("[red]");

            if let Some(red) = reductor.clone() {
                if reaction.1 < red.1 {
                    reductor = Some(reaction);
                }
            } else {
                reductor = Some(reaction);
            }
        }
    }

    if let Some(ref oxi) = oxidator {
        println!("oxidator: {}  \twith SEP {}", oxi.0, oxi.1);
    } else {
        println!("failed to find oxidator");
    }

    if let Some(ref red) = reductor {
        println!("reductor: {}  \twith SEP {}", red.0, red.1);
    } else {
        println!("failed to find reductor");
    }

    if oxidator.is_some() && reductor.is_some() {
        let oxi = oxidator.unwrap();
        let red = reductor.unwrap();

        Some(RedoxReaction {
            oxidator: oxi.0,
            reductor: red.0,
        })
    } else {
        None
    }
}

impl<E: Element> Container<E> {
    /// Applies given `Reaction` to `Container`
    /// Removing the elements on the left-hand side
    /// and adding the elements on the right-hand side.
    /// If there is enough energy for the reaction, that amount will be consumed
    /// otherwise the reaction won't occur.
    /// Returns if the reaction succeeded
    pub fn react<R: Reaction<E>>(&mut self, reaction: &R) -> bool {
        // Get required items
        let required_energy = reaction.energy_cost();
        let mut required_elements = vec![];
        let mut resulting_elements = vec![];

        // Convert lhs compounds into `ContainerCompound`s
        for rc in &reaction.elem_reaction().lhs.compounds {
            let cc = rc_to_cc(rc.clone());

            required_elements.push(cc);
        }

        // Convert rhs compounds into `ContainerCompound`s
        for rc in &reaction.elem_reaction().rhs.compounds {
            let cc = rc_to_cc(rc.clone());

            resulting_elements.push(cc);
        }

        // Check if the container has enough energy
        if self.available_energy < required_energy {
            println!("####    Not enough energy");
            return false;
        }

        // Check if the container has the required elements
        if !self.has_elements(&required_elements) {
            println!("####    Not enough elements");
            return false;
        }

        // Subtract needed energy (or add, in case of an exothermic reaction)
        self.available_energy -= required_energy;

        // Remove required elements
        self.remove_elements(&required_elements);

        // Add reaction results
        self.add_elements(&resulting_elements);

        true
    }

    /// Check if the container contains a container compound
    pub fn contains(&self, element: &ContainerCompound<E>) -> bool {
        // Find element in self.contents
        if let Some(position) = self.contents.iter().position(|comp| comp == element) {
            let compound = &self.contents[position];

            // Check if more elements are required than available
            if element.moles > compound.moles {
                return false;
            }

            return true;
        }

        // Element is not available
        false
    }

    /// Check if the container has all given elements
    pub fn has_elements(&self, elements: &[ContainerCompound<E>]) -> bool {
        for element in elements {
            if !self.contains(element) {
                return false;
            }
        }

        true
    }

    /// Check if the container has all required elements for a reaction to occur
    /// NOTE: Ignores electrons
    pub fn has_enough_compounds_for_reaction(&self, reaction: &ElemReaction<E>) -> bool {
        self.has_elements(
            &reaction
                .lhs
                .compounds
                .iter()
                .filter(|x| {
                    x.element.clone().get_molecule().unwrap().compounds[0]
                        .atom
                        .number
                        != AtomNumber::from(0)
                })
                .map(|x| rc_to_cc(x.clone()))
                .collect::<Vec<ContainerCompound<E>>>(),
        )
    }

    /// Remove given elements from container
    pub fn remove_elements(&mut self, elements: &[ContainerCompound<E>]) {
        for element in elements {
            let mut to_remove = None;

            // Find element in self.contents
            if let Some(position) = self.contents.iter().position(|comp| comp == element) {
                let compound = &mut self.contents[position];

                // Check if we have enough
                if compound.moles < element.moles {
                    panic!("Can't remove element {} (not enough)", element.symbol());
                }

                // Remove amount
                compound.moles -= element.moles.clone();

                // If none is available anymore, let element be removed from container
                if compound.moles == Moles::from(0.0) {
                    to_remove = Some(position);
                }
            } else {
                panic!("Can't remove element {} (not found)", element.symbol());
            }

            // Remove element if needed
            if let Some(pos) = to_remove {
                self.contents.remove(pos);
            }
        }
    }

    /// Add given elements to container
    pub fn add_elements(&mut self, elements: &[ContainerCompound<E>]) {
        for element in elements {
            // Find element in self.contents
            if let Some(position) = self.contents.iter().position(|comp| comp == element) {
                let compound = &mut self.contents[position];

                // Add amount
                compound.moles += element.moles.clone();
            } else {
                // If the element is not found in the container, add it
                self.contents.push(element.clone());
            }
        }
    }

    /// Get all possible redox reactions and their SEP's
    pub fn get_redox_reactions(&self) -> Vec<(ElemReaction<Ion>, SEP)> {
        let mut redox_reactions = vec![];

        for container_element in &self.contents {
            redox_reactions.append(&mut get_reactions_with_element(
                &container_element.clone().get_ion().unwrap(),
            ));
        }

        redox_reactions
    }

    /// Convert container to a nice string for displaying
    pub fn stringify(&self) -> String {
        let mut string = String::new();

        let mut first = true;
        for compound in &self.contents {
            if compound.moles > Moles::from(0.0) {
                if !first {
                    string += " + ";
                }
                first = false;

                string += &compound.stringify();
            }
        }

        string += "    [";
        string += &format!("{:.3}", self.available_energy);
        string += " J]";

        string
    }

    pub fn ion_from_string(string: &str) -> Option<Container<Ion>> {
        let mut token = String::new();
        let mut contents = None;
        let mut energy = None;

        for c in string.chars() {
            if c == '[' {
                let rs = ReactionSide::<Ion>::ion_from_string(&token)?;
                let mut _contents = vec![];

                for rc in rs.compounds {
                    _contents.push(rc_to_cc(rc));
                }

                contents = Some(_contents);

                token = String::new();
            } else if c == ']' {
                let mut _energy = 0.0f64;

                for x in token.chars() {
                    if is_number!(x) {
                        _energy *= 10.0;
                        _energy += f64::from(to_number!(x));
                    }
                }

                energy = Some(Energy::from(_energy));
            }

            token.push(c);
        }

        if contents.is_some() && energy.is_some() {
            Some(Container {
                contents: contents.unwrap(),

                available_energy: energy.unwrap(),
            })
        } else {
            None
        }
    }

    pub fn molecule_from_string(string: &str) -> Option<Container<Molecule>> {
        let mut token = String::new();
        let mut contents = None;
        let mut energy = None;

        for c in string.chars() {
            if c == '[' {
                let rs = ReactionSide::<Molecule>::molecule_from_string(&token)?;
                let mut _contents = vec![];

                for rc in rs.compounds {
                    _contents.push(rc_to_cc(rc));
                }

                contents = Some(_contents);

                token = String::new();
            } else if c == ']' {
                let mut _energy = 0.0f64;

                for x in token.chars() {
                    if is_number!(x) {
                        _energy *= 10.0;
                        _energy += f64::from(to_number!(x));
                    }
                }

                energy = Some(Energy::from(_energy));
            }

            token.push(c);
        }

        if contents.is_some() && energy.is_some() {
            Some(Container {
                contents: contents.unwrap(),

                available_energy: energy.unwrap(),
            })
        } else {
            None
        }
    }
}

impl<E: Element> ContainerCompound<E> {
    pub fn ion_from_string(string: &str) -> Option<ContainerCompound<Ion>> {
        let rc = ReactionCompound::<Ion>::ion_from_string(string)?;

        Some(rc_to_cc(rc))
    }

    pub fn molecule_from_string(string: &str) -> Option<ContainerCompound<Molecule>> {
        let rc = ReactionCompound::<Molecule>::molecule_from_string(string)?;

        Some(rc_to_cc(rc))
    }
}

impl<E: Element> Eq for ContainerCompound<E> {}

impl<E: Element> PartialEq for ContainerCompound<E> {
    /// Two container compounds are equal when their elements are equal
    fn eq(&self, rhs: &ContainerCompound<E>) -> bool {
        self.element == rhs.element
    }
}

impl<E: Element> Hash for ContainerCompound<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.element.hash(state)
    }
}

impl<E: Element> Element for ContainerCompound<E> {
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

impl<E: Element> Properties for ContainerCompound<E> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        if self.moles != Moles::from(1.0) {
            symbol += &self.moles.to_string();
            symbol += " ";
        }

        symbol += &self.element.symbol();

        symbol
    }

    fn name(&self) -> String {
        let mut name = String::new();

        if self.moles != Moles::from(1.0) {
            name += &self.moles.to_string();
            name += " ";
        }

        name += &self.element.name();

        name
    }

    fn mass(&self) -> AtomMass {
        self.element.mass() * (self.moles.0 as AtomMass_type)
    }

    fn is_diatomic(&self) -> bool {
        self.element.is_diatomic()
    }
}
