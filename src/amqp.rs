///
///
pub trait Exchange {
    fn declare_exchange();
    fn bind_exchange();
}

///
///
///
pub trait Queue {
    fn declare_queue();
    fn bind_queue();
}


///
///
///
pub trait Transaction {
}
