use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates"]
#[include = "*.hbs"]
#[include = "*.css"]
pub struct Assets;
