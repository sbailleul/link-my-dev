pub trait Entity<Id>{
    fn id(&self) -> &Id;
}