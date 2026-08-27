pub trait Owner {
    fn other(&self);
}

pub trait DifferentOwner {
    fn member(&self);
}
