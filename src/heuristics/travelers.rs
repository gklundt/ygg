use std::rc::Rc;

#[derive(Debug)]
pub struct Traveler<T> {
    name: String,
    resources: Vec<Rc<T>>,
}


impl<T> Traveler<T> {
    pub fn new(name: String) -> Self {
        Traveler { name, resources: Vec::new() }
    }

    pub fn push_resource(&mut self, resource: Rc<T>) {
        self.resources.push(resource);
    }

    pub fn get_resources(&mut self) -> &Vec<Rc<T>> {
        &self.resources
    }
}
