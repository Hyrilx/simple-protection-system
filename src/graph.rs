use crate::capability::Capability;
use crate::object::Object;
use crate::right::Right;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

pub struct Graph {
    objects: HashSet<*const Object>,
    capabilities: HashSet<Capability>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            objects: Default::default(),
            capabilities: Default::default(),
        }
    }

    pub fn contains_object(&self, object: &Rc<Object>) -> bool {
        self.objects.contains(&Rc::as_ptr(&object))
    }

    pub fn new_object(&mut self) -> Rc<Object> {
        let new = Rc::new(Object::new());
        self.objects.insert(new.as_ref());
        new
    }

    pub fn have_operation(&self, from: &Rc<Object>, to: &Rc<Object>, right: &Right) -> bool {
        let cap = self
            .capabilities
            .get(&Capability::new_with_empty(from.as_ref(), to.as_ref()));

        cap.is_some() && cap.unwrap().contains(right)
    }

    pub fn create(&mut self, object: &Rc<Object>) -> Rc<Object> {
        let new = Rc::new(Object::new());

        self.capabilities
            .insert(Capability::new_with_full(object.as_ref(), new.as_ref()));
        self.objects.insert(new.as_ref());
        new
    }

    pub fn remove(&mut self, from: &Rc<Object>, to: &Rc<Object>) {
        self.capabilities
            .remove(&Capability::new_with_empty(from.as_ref(), to.as_ref()));
    }

    fn insert_or_merge_capability(
        &mut self,
        from: &Rc<Object>,
        to: &Rc<Object>,
        mut rights: Right,
    ) {
        if let Some(cap) = self
            .capabilities
            .take(&Capability::new_with_full(from.as_ref(), to.as_ref()))
        {
            rights.turn_on(&cap.rights())
        }
        self.capabilities.insert(Capability::new_with_rights(
            from.as_ref(),
            to.as_ref(),
            rights,
        ));
    }

    fn collect_rights_to_object(&self, from: &HashSet<*const Object>, to: *const Object) -> Right {
        self.capabilities
            .iter()
            .filter(|cap| from.contains(&cap.from()) && cap.to() == to)
            .fold(Right::empty(), |acc, cap| acc | cap.rights().clone())
    }

    pub fn take(&mut self, from: &Rc<Object>, to: &Rc<Object>) {
        let taking_objects: HashSet<_> = self
            .capabilities
            .iter()
            .filter(|c| c.from() == from.as_ref() && c.contains(&Right::read()))
            .map(|c| c.to())
            .collect();

        let new_rights = self.collect_rights_to_object(&taking_objects, to.as_ref());

        if new_rights.is_empty() {
            return;
        }

        self.insert_or_merge_capability(from, to, new_rights);
    }

    pub fn grant(&mut self, from: &Rc<Object>, to: &Rc<Object>) {
        let granting_objects: HashSet<*const Object> = self
            .capabilities
            .iter()
            .filter(|cap| cap.to() == from.as_ref() && cap.contains(&Right::write()))
            .map(|cap| cap.from())
            .collect();

        let new_rights = self.collect_rights_to_object(&granting_objects, to.as_ref());

        if new_rights.is_empty() {
            return;
        }

        self.insert_or_merge_capability(from, to, new_rights);
    }

    pub fn call(&mut self, from: *const Object, to: *const Object, para: *const Object) {
        let process = Rc::new(Object::new());
        let Some(rights) = self
            .capabilities
            .get(&Capability::new_with_full(from, para))
        else {
            return;
        };

        self.capabilities.insert(Capability::new_with_rights(
            process.as_ref(),
            para,
            rights.rights().clone(),
        ));
        self.capabilities.insert(Capability::new_with_rights(
            process.as_ref(),
            to,
            Right::read(),
        ));
        self.objects.insert(process.as_ref());
    }

    pub fn objects(&self) -> &HashSet<*const Object> {
        &self.objects
    }

    pub fn capabilities(&self) -> &HashSet<Capability> {
        &self.capabilities
    }

    fn is_connected(&self, a: *const Object, b: *const Object) -> bool {
        if a == b {
            return true;
        }

        let mut visited = HashSet::from([a]);
        let mut queue = VecDeque::from([a]);

        while let Some(current) = queue.pop_front() {
            for neighbor in self.neighbors(current) {
                if visited.insert(neighbor) {
                    if neighbor == b {
                        return true;
                    }
                    queue.push_back(neighbor);
                }
            }
        }

        false
    }

    fn neighbors(&self, node: *const Object) -> impl Iterator<Item = *const Object> + '_ {
        self.capabilities.iter().filter_map(move |cap| {
            if cap.from() == node {
                Some(cap.to())
            } else if cap.to() == node {
                Some(cap.from())
            } else {
                None
            }
        })
    }

    fn all_rights_to_object(&self, to: *const Object) -> Right {
        self.collect_rights_to_object(&self.objects, to)
    }

    pub fn can_operation(&self, from: &Rc<Object>, to: &Rc<Object>) -> bool {
        if !self.is_connected(from.as_ref(), to.as_ref()) {
            return false;
        }

        self.all_rights_to_object(to.as_ref()).is_any()
    }
}
