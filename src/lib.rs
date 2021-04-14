#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]

//! A tiny proc macro to include a WGSL file in your binary, and verify that it is valid at compile time.

use proc_macro::{TokenStream, Span, Diagnostic, Level};
use syn::{parse_macro_input, LitStr};
use naga::{valid::{ValidationFlags, Validator}, front::wgsl};

/// Just like `include_str!`, but errors on compile time if the contents are not valid WGSL.
///
/// # Example
///
/// ```
/// let shader_str = include_wgsl!("shader.wgsl");
/// device.create_shader_module(&ShaderModuleDescriptor {
///     source: ShaderSource::Wgsl(Cow::Borrowed(&shader_str)),
///     flags: ShaderFlags::default(),
///     label: None,
/// })
/// ```
#[proc_macro]
pub fn include_wgsl(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr).value();
    let call_site = Span::call_site();
    let mut own_path = call_site.source_file().path();
    // Assert we actually have a valid call site
    assert!(own_path.pop());

    // This is the path relative to the current working directory
    let new_path = own_path.join(&file_path);

    // Load string contents, error if path not found
    match std::fs::read_to_string(new_path) {
        Ok(wgsl_str) => {
            // Attempt to parse WGSL, error if invalid
            match wgsl::parse_str(&wgsl_str) {
                Ok(module) => {
                    // Attempt to validate WGSL, error if invalid
                    match Validator::new(ValidationFlags::all()).validate(&module) {
                        Ok(_) => {},
                        Err(e) => Diagnostic::new(Level::Error, format!("{}: {}", file_path, e)).emit(),
                    }
                },
                Err(e) => {
                    Diagnostic::new(Level::Error, format!("Unable to parse {}:", file_path)).emit();
                    e.emit_to_stderr();
                },
            }
        },
        Err(e) => {
            Diagnostic::spanned(call_site, Level::Error, format!("couldn't read {}: {}", file_path, e));
        },
    };

    // just return the `include_str!` macro with the given file path
    format!("&include_str!(\"{}\")", file_path).parse().unwrap()
}
