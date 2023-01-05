use deno_core::Extension;

mod debug;
mod net;
mod utils;

pub fn debug() -> Extension {
    Extension::builder()
        .ops(vec![debug::ping::decl(), debug::sum::decl()])
        .build()
}

pub fn net() -> Extension {
    Extension::builder().ops(vec![net::get::decl()]).build()
}

pub fn utils() -> Extension {
    Extension::builder()
        .ops(vec![
            utils::info::decl(),
            utils::debug::decl(),
            utils::warn::decl(),
            utils::error::decl(),
            utils::read_input::decl(),
        ])
        .build()
}
