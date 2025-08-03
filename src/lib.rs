stageleft::stageleft_no_entry_crate!();

pub mod first_ten;
pub mod first_ten_cluster;
pub mod first_ten_distributed;
pub mod legacy_hello_world;
pub mod syn_hello_world;
pub mod transformer;
pub mod syn_transformer;
pub mod legacy;

#[cfg(test)]
mod test_init {
    #[ctor::ctor]
    fn init() {
        hydro_lang::deploy::init_test();
    }
}
