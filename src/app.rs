use std::rc::Rc;

use anyhow::Error;
use deno_core::{JsRuntime, RuntimeOptions};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::{
    ops::{self, JsMessage},
    typescript,
};

const WIDTH: usize = 360;
const HEIGHT: usize = 640;

pub struct App {
    js: JsRuntime,
    js_sender: Sender<JsMessage>,
    js_receiver: Receiver<JsMessage>,
}

impl App {
    /// Creates a new app with a preconfigured JS runtime.
    pub async fn new() -> Result<Self, Error> {
        let (runtime, sender, receiver) = create_runtime().await?;

        Ok(Self {
            js: runtime,
            js_sender: sender,
            js_receiver: receiver,
        })
    }

    #[cfg(feature = "minifb")]
    pub async fn start(&mut self) -> Result<(), Error> {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut window = Window::new("Testing", WIDTH, HEIGHT, WindowOptions::default())
            .unwrap_or_else(|e| panic!("{}", e));

        // 16600 micro == 60 fps
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            if window.is_key_pressed(Key::A, KeyRepeat::No) {
                self.execute_script(&"./example.ts".into()).await?;
            }
            if window.is_key_pressed(Key::W, KeyRepeat::No) {
                self.js_sender.send(JsMessage::None)?;
            }

            self.run_js_loop().await?;

            for i in buffer.iter_mut() {
                *i = 0;
            }

            window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
        }

        Ok(())
    }

    /// Executes the given script at the given path.
    async fn execute_script(&mut self, path: &String) -> Result<(), Error> {
        let runtime = &mut self.js;

        let main_module = deno_core::resolve_path(path.as_str())?;

        let main_module = runtime.load_main_module(&main_module, None).await?;
        let result = runtime.mod_evaluate(main_module);
        runtime.run_event_loop(false).await?;

        result.await?
    }

    /// Runs the JS event loop. Needed or else middleware won't be processed.
    async fn run_js_loop(&mut self) -> Result<(), Error> {
        let runtime = &mut self.js;

        runtime.run_event_loop(false).await
    }
}

async fn create_runtime() -> Result<(JsRuntime, Sender<JsMessage>, Receiver<JsMessage>), Error> {
    let (comms_ext, sender, receiver) = ops::communications();

    let mut runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(typescript::ModuleLoader)),
        extensions: vec![ops::debug(), ops::net(), ops::utils(), comms_ext],
        ..Default::default()
    });

    let core_module = runtime
        .load_side_module(&deno_core::resolve_path("./src/runtime.ts")?, None)
        .await?;
    let result = runtime.mod_evaluate(core_module);
    runtime.run_event_loop(false).await?;
    result.await??;

    Ok((runtime, sender, receiver))
}
