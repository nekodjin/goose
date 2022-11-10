use std::hash::Hash;

pub type Id<T> = internment::Intern<T>;
pub type IString = Id<str>;

pub trait MoveInternable {
    fn intern(self) -> Id<Self>;
}

pub trait RefInternable {
    fn id(&self) -> Id<Self>;
}

impl<T> MoveInternable for T
where
    T: Eq + Hash + Send + Sync,
{
    fn intern(self) -> Id<Self> {
        Id::from(self)
    }
}

impl RefInternable for str {
    fn id(&self) -> IString {
        IString::from(self)
    }
}

impl<T> RefInternable for [T]
where
    T: Eq + Hash + Send + Sync + 'static + Copy,
{
    fn id(&self) -> Id<[T]> {
        Id::from(self)
    }
}
