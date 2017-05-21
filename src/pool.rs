use futures::future::Future;
use std::hash::Hash;
use std::ops::Index;

///
/// Named pool trait based on
/// index on certain type namespace.
///
pub trait NamedPool {
    type Index : 'static + Hash;
    type Instance : 'static + Drop + Send + Sync;
    type Error;

    ///
    ///
    ///
    fn fetch(&self, index: Self::Index) -> Future<Item = Self::Instance, Error = Self::Error>;
}


///
/// Pool trait.
///
pub trait Pool {
    type Instance : 'static + Drop + Send + Sync;
    type Error;

    ///
    ///
    ///
    fn fetch(&self) -> Future<Item = Self::Instance, Error = Self::Error>;
}
