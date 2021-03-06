pub type Job = Box<dyn FnBox + Send + 'static>;
pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
         (*self)();
    }
}

pub enum Message {
    NewJob(Job),
    Terminate,
}