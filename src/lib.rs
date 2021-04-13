#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]

use proc_macro::{TokenStream, Span, Diagnostic, Level};
use syn::{parse_macro_input, LitStr};
use naga::{valid::{ValidationFlags, Validator}, front::wgsl};

#[proc_macro]
pub fn include_wgsl(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr).value();
    let call_site = Span::call_site();
    let mut own_path = call_site.source_file().path();
    // Assert we actually have a valid call site
    assert!(own_path.pop());
    let new_path = own_path.join(&file_path);
    match std::fs::read_to_string(new_path) {
        Ok(wgsl_str) => {
            match wgsl::parse_str(&wgsl_str) {
                Ok(module) => {
                    let mut validator = Validator::new(ValidationFlags::all());
                    match validator.validate(&module) {
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
    format!("&include_str!(\"{}\")", file_path).parse().unwrap()
}
