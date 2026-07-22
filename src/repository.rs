mod in_memory;

pub use in_memory::InMemoryDatabase;

pub trait Repository<T> {
    type Id;

    fn add(&mut self, entity: T) -> Self::Id;
    fn get(&self, id: Self::Id) -> Option<&T>;
    fn all<'a>(&'a self) -> impl Iterator<Item = (&'a Self::Id, &'a T)>
    where
        Self::Id: 'a,
        T: 'a;
}
