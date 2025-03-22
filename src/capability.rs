use std::hash::{Hash, Hasher};
use crate::object::Object;
use crate::right::Right;

pub struct Capability {
    from: *const Object,
    to: *const Object,
    rights: Right,
}

impl PartialEq for Capability {
    fn eq(&self, other: &Capability) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl Eq for Capability {}

impl Hash for Capability {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}

impl Capability {
    pub fn new_with_rights(from: *const Object, to: *const Object, rights: Right) -> Capability {
        Capability { from, to, rights }
    }

    pub fn new_with_empty(from: *const Object, to: *const Object) -> Capability {
        Capability {
            from,
            to,
            rights: Right::empty(),
        }
    }

    pub fn new_with_full(from: *const Object, to: *const Object) -> Capability {
        Capability {
            from,
            to,
            rights: Right::full(),
        }
    }

    pub fn contains(&self, rights: &Right) -> bool {
        self.rights.contains(rights)
    }

    pub fn from(&self) -> *const Object {
        self.from
    }

    pub fn to(&self) -> *const Object {
        self.to
    }

    pub fn rights(&self) -> &Right {
        &self.rights
    }
}