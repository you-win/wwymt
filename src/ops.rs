use std::task::Poll;

use deno_core::Extension;
use tokio::sync::broadcast::{channel, Receiver, Sender};

mod debug;
mod net;
mod utils;

#[derive(Debug, Clone, PartialEq)]
pub enum JsMessage {
    None,
}

pub fn debug() -> Extension {
    Extension::builder()
        .ops(vec![
            debug::ping::decl(),
            debug::sum::decl(),
            debug::echo::decl(),
        ])
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

pub fn communications() -> (Extension, Sender<JsMessage>, Receiver<JsMessage>) {
    let (js_sender, app_receiver) = channel::<JsMessage>(10);
    let (app_sender, js_receiver) = channel::<JsMessage>(10);

    let extension = Extension::builder()
        .event_loop_middleware(|state_rc, ctx| {
            let mut state = state_rc.borrow_mut();
            let receiver = state.borrow_mut::<Receiver<JsMessage>>();
            let mut ref_loop = false;

            match receiver.try_recv() {
                Ok(m) => match m {
                    JsMessage::None => log::debug!("JsMessage::None received!"),
                    _ => log::error!("Unhandled JsMessage: {:?}", m),
                },
                Err(e) => {}
            }

            ref_loop
        })
        .state(move |state| {
            state.put(js_sender.clone());
            state.put(js_receiver.resubscribe());

            Ok(())
        })
        .build();

    (extension, app_sender, app_receiver)
}
