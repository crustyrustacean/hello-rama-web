// src/assets/rs

// dependencies
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "static"]
pub struct Asset;

#[derive(Embed)]
#[folder = "templates"]
pub struct Templates;
