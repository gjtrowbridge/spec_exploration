#[macro_use]
pub mod u64_macros;

pub mod beacon_state;
pub mod chain_spec;
pub mod constants;
pub mod cross_link;
pub mod eth_spec;
pub mod execution_environment;
pub mod fork;
pub mod newtypes;
pub mod serde_utils;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
