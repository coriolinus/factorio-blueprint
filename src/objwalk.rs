use std::any::Any;

pub trait Objwalk {
    fn walk_structure<F>(&self, visit: F)
    where
        F: Fn(&dyn Any);
}
