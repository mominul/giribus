#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;

pub use auto::*;

mod auto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
