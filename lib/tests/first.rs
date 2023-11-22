#[cfg(test)]

extern crate lib;
use lib::module_ttags::*;

mod tests {

#[test]
fn pokus() -> Result<(), String> {
        assert_eq!(21, 21);
        Ok(())
}
}



