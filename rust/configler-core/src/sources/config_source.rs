use dyn_clone::DynClone;

pub trait ConfigSource: DynClone {
    #![allow(dead_code)]
    fn get_ordinal(&self) -> usize;
    fn get_value(&self, property_name: &str) -> Option<String>;
    fn get_name(&self) -> &str;
}

dyn_clone::clone_trait_object!(ConfigSource);
