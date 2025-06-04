use crate::assets::Assets;
use crate::errors::*;
use db::srcinfo::{self, Pkg};
use serde_json::json;
use std::fmt::Write;

pub struct Html {
    hbs: handlebars::Handlebars<'static>,
}

handlebars::handlebars_helper!(version: |pkg: Pkg| {
    let mut ver = String::new();
    if let Some(epoch) = pkg.epoch {
        write!(ver, "{}:", epoch).unwrap();
    }
    write!(ver, "{}-{}", pkg.pkgver, pkg.pkgrel).unwrap();
    ver
});

handlebars::handlebars_helper!(filename: |pkg: Pkg| {
    pkg.filename()
});

impl Html {
    pub fn new() -> Result<Self> {
        let mut hbs = handlebars::Handlebars::new();
        hbs.set_prevent_indent(true);
        hbs.register_embed_templates::<Assets>()?;
        hbs.register_helper("version", Box::new(version));
        hbs.register_helper("filename", Box::new(filename));
        Ok(Html { hbs })
    }

    fn render<T>(&self, name: &str, data: &T) -> Result<String>
    where
        T: serde::Serialize,
    {
        let out = self
            .hbs
            .render(name, data)
            .context("Failed to render index template")?;
        Ok(out)
    }

    pub fn index(&self, pkgs: &[srcinfo::Pkg]) -> Result<String> {
        self.render(
            "index.html.hbs",
            &json!({
                "pkgs": pkgs,
            }),
        )
    }

    pub fn pkg(&self, pkg: &srcinfo::Pkg) -> Result<String> {
        let sources = pkg.sources();
        self.render(
            "pkg.html.hbs",
            &json!({
                "pkg": pkg,
                "sources": sources,
            }),
        )
    }

    pub fn style(&self) -> Result<String> {
        let file = Assets::get("style.css").context("File not found: style.css")?;
        let style = String::from_utf8(file.data.into_owned())?;
        Ok(style)
    }
}
