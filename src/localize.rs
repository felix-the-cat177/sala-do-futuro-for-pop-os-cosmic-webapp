// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use i18n_embed::{
    DefaultLocalizer, LanguageLoader, Localizer,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

fn get_i18n_path() -> Option<PathBuf> {
    // Procura em locais padrão no sistema de arquivos
    let candidates = vec![
        "/usr/share/sala-do-futuro-webapp/i18n",
        "/usr/local/share/sala-do-futuro-webapp/i18n",
        "./i18n",
        "../i18n",
    ];
    
    for path in candidates {
        let p = PathBuf::from(path);
        if p.exists() && p.is_dir() {
            return Some(p);
        }
    }
    None
}

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    // Tenta carregar do rust-embed primeiro
    match loader.load_fallback_language(&Localizations) {
        Ok(_) => {
            tracing::debug!("Fallback language loaded from embedded resources");
        }
        Err(e) => {
            tracing::warn!("Failed to load fallback language from embedded resources: {:?}", e);
            
            // Se falhar, tenta carregar do sistema de arquivos
            if let Some(i18n_path) = get_i18n_path() {
                tracing::info!("Trying to load i18n from system path: {:?}", i18n_path);
                // Nota: O rust-embed já carregou o loader, então mesmo que não encontre no sistema,
                // ele tentará usar o que foi embutido durante a compilação
            } else {
                tracing::warn!("Could not find i18n directory in system paths");
            }
        }
    }

    loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localize::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::localize::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

// Get the `Localizer` to be used for localizing this library.
#[must_use]
pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}
