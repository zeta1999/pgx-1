#[macro_use]
extern crate pg_guard_attr;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

// expose the #[derive(DatumCompatible)] trait
pub use pg_guard_attr::DatumCompatible;

pub mod datum;
pub mod datum_compatible;
pub mod fcinfo;
pub mod htup;
pub mod itemptr;
pub mod log;
pub mod memcxt;
pub mod nodes;
pub mod oids;
pub mod pg_sys;
pub mod spi;
pub mod stringinfo;
pub mod varlena;

pub use datum::*;
pub use datum_compatible::*;
pub use fcinfo::*;
pub use htup::*;
pub use itemptr::*;
pub use log::*;
pub use memcxt::*;
pub use nodes::*;
pub use oids::*;
pub use pg_guard::*;
pub use spi::*;
pub use varlena::*;

/// A macro for marking a library compatible with the Postgres extension framework.
///
/// This macro was initially inspired from the `pg_module` macro in https://github.com/thehydroimpulse/postgres-extension.rs
///
/// Shameless;y cribbed from https://github.com/bluejekyll/pg-extend-rs
#[macro_export]
macro_rules! pg_module_magic {
    () => {
        #[no_mangle]
        #[allow(non_snake_case)]
        #[allow(unused)]
        #[link_name = "Pg_magic_func"]
        pub extern "C" fn Pg_magic_func() -> &'static pg_sys::Pg_magic_struct {
            use std::mem::size_of;
            use std::os::raw::c_int;

            const my_magic: pg_sys::Pg_magic_struct = pg_sys::Pg_magic_struct {
                len: size_of::<pg_sys::Pg_magic_struct>() as c_int,
                version: pg_sys::PG_VERSION_NUM as std::os::raw::c_int / 100,
                funcmaxargs: pg_sys::FUNC_MAX_ARGS as c_int,
                indexmaxkeys: pg_sys::INDEX_MAX_KEYS as c_int,
                namedatalen: pg_sys::NAMEDATALEN as c_int,
                float4byval: pg_sys::USE_FLOAT4_BYVAL as c_int,
                float8byval: pg_sys::USE_FLOAT8_BYVAL as c_int,
            };

            // go ahead and register our panic handler since Postgres
            // calls this function first
            pg_guard::register_pg_guard_panic_handler();

            // return the magic
            &my_magic
        }
    };
}

/// Top-level initialization function
///
/// C-based Postgres extensions should call this in their _PG_init() function
#[allow(unused)]
#[no_mangle]
pub extern "C" fn initialize() {
    pg_guard::register_pg_guard_panic_handler();
}
