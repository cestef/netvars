pub mod prelude {
    pub use super::HasNetvar;
    pub use netvars_derive::netvar;
    pub use netvars_derive::offset_fn;
    pub use netvars_derive::HasNetvar;
}

pub trait HasNetvar {
    unsafe fn get_field<T>(&self, offset: usize) -> T;
}
