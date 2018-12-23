use atom::Atom;
use container::*;
use ion::Ion;
use molecule::Molecule;
use reaction::*;
use trait_element::Element;
use trait_properties::Properties;

use std::fmt::*;

macro_rules! fmt {
    ($x:tt) => {
        impl Display for $x {
            fn fmt(&self, formatter: &mut Formatter) -> Result {
                write!(formatter, "{}", self.stringify())
            }
        }
    };
}

macro_rules! fmt_type {
    ($x:tt) => {
        impl<E: Element> Display for $x<E> {
            fn fmt(&self, formatter: &mut Formatter) -> Result {
                write!(formatter, "{}", self.stringify())
            }
        }
    };
}

fmt!(Atom);
fmt!(Ion);
fmt!(Molecule);
fmt_type!(Container);
fmt_type!(ContainerCompound);
fmt_type!(ElemReaction);
fmt_type!(ReactionCompound);
fmt_type!(ReactionSide);
