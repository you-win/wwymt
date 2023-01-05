use std::rc::Rc;

use anyhow::Error;
use deno_core::{JsRuntime, RuntimeOptions};
use minifb::{Key, KeyRepeat, Window, WindowOptions};

use crate::{ops, typescript};

const WIDTH: usize = 360;
const HEIGHT: usize = 640;

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(&self) -> Result<(), Error> {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut window = Window::new("Testing", WIDTH, HEIGHT, WindowOptions::default())
            .unwrap_or_else(|e| panic!("{}", e));

        // 16600 micro == 60 fps
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            if window.is_key_pressed(Key::A, KeyRepeat::No) {
                self.execute_script(&"./example.ts".into()).await?;
            }

            for i in buffer.iter_mut() {
                *i = 0;
            }

            window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
        }

        Ok(())
    }

    pub async fn execute_script(&self, path: &String) -> Result<(), Error> {
        let main_module = deno_core::resolve_path(path.as_str())?;

        let mut runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(typescript::ModuleLoader)),
            extensions: vec![ops::debug(), ops::net(), ops::utils()],
            ..Default::default()
        });

        let core_module = runtime
            .load_side_module(&deno_core::resolve_path("./src/runtime.ts")?, None)
            .await?;
        let result = runtime.mod_evaluate(core_module);
        runtime.run_event_loop(false).await?;
        result.await??;

        let main_module = runtime.load_main_module(&main_module, None).await?;
        let result = runtime.mod_evaluate(main_module);
        runtime.run_event_loop(false).await?;

        result.await?
    }
}
