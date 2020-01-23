#[macro_use]
pub mod u64_macros;

pub mod constants;
pub mod cross_link;
pub mod execution_environment;
pub mod newtypes;
pub mod beacon_state;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
