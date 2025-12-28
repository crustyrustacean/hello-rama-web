// src/templates.rs

// dependencies
use crate::configuration::Settings;
use std::sync::OnceLock;
use tera::{Error, Tera};

// static variable to hold the compiled templates
static COMPILED_TEMPLATES: OnceLock<Tera> = OnceLock::new();

pub fn compile_templates(configuration: &Settings) -> Result<&'static Tera, Error> {
    let templates = Tera::new(&format!(
        "{}/{}",
        &configuration.application.template_dir, &configuration.application.template_pattern
    ))?;
    Ok(COMPILED_TEMPLATES.get_or_init(|| templates))
}
