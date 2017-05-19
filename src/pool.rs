use futures::future::Future;

pub trait NamedPool {
    type Index : 'static;
    type Instance : 'static + Drop + Send + Sync;
    type Error;

    fn fetch(&self, index: Index) -> Future<Result<Instance, Error>>;
}

pub trait Pool {
    type Instance : 'static + Drop + Send + Sync;
    type Error;

    fn fetch(&self) -> Future<Result<Instance, Error>>;
}
