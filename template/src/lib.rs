stageleft::stageleft_no_entry_crate!();

// Generated modules will be injected here

#[cfg(test)]
mod test_init {
    #[ctor::ctor]
    fn init() {
        hydro_lang::deploy::init_test();
    }
}
pub mod hello_world;
pub mod hello_world_test;
pub mod counter_test;
