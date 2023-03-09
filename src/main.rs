use std::{iter::Map, collections::HashMap};

struct Mapping {
    from: u16,
    to: u16
}

trait Remappable {
    fn nextMapper(self) -> Box<dyn Remappable>;

    fn remap(&self, x: u16) -> u16;
    fn shift(&self);
    fn shift_by(&self, value: u16);
    fn reset(&self);
}

struct Reflector {
    mappingsDict: HashMap<u16, u16>,
    nextMapper: Box<dyn Remappable>,
}

impl Reflector {
    fn createMappingsDict(&mut self, mappings: &HashMap<u16, u16>) {
        for (from, to) in &*mappings {
            self.mappingsDict.insert(*from, *to);
        }
    }
}

impl Remappable for Reflector {
    fn shift(&self) {}
    fn shift_by(&self, _: u16) {}
    fn reset(&self) {}
    fn remap(&self, x: u16) -> u16 {
        return *self.mappingsDict.get(&x).unwrap_or(&0);
    }

    fn nextMapper(self) -> Box<dyn Remappable> {
        return self.nextMapper;
    }
}

fn main() {
    println!("Hello, world!");
}
