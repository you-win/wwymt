use anyhow::anyhow;
use deno_ast::{MediaType, ParseParams};
use deno_core::{futures::FutureExt, ModuleLoader as DenoModuleLoader, ModuleSource, ModuleType};

pub struct ModuleLoader;

impl DenoModuleLoader for ModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _is_main: bool,
    ) -> Result<deno_core::ModuleSpecifier, anyhow::Error> {
        Ok(deno_core::resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
    ) -> std::pin::Pin<Box<deno_core::ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();

        async move {
            let path = module_specifier
                .to_file_path()
                .map_err(|_| anyhow!("Bad path"))?;

            let media_type = MediaType::from(&path);
            let (module_type, should_transpile) = match MediaType::from(&path) {
                MediaType::JavaScript => (ModuleType::JavaScript, false),
                MediaType::TypeScript => (ModuleType::JavaScript, true),
                _ => panic!("Unhandled media type!"),
            };

            let code = std::fs::read_to_string(&path)?;
            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.to_string(),
                    text_info: deno_ast::SourceTextInfo::from_string(code),
                    media_type: media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })?;
                parsed.transpile(&Default::default())?.text
            } else {
                code
            };
            let module = ModuleSource {
                code: code.into_bytes().into_boxed_slice(),
                module_type: module_type,
                module_url_specified: module_specifier.to_string(),
                module_url_found: module_specifier.to_string(),
            };

            Ok(module)
        }
        .boxed_local()
    }
}
