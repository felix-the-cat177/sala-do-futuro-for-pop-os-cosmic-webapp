use i18n_embed::DesktopLanguageRequester;
use sala_do_futuro_webapp::localize;
use std::env;

pub(crate) mod config;
pub(crate) mod pages;
pub(crate) mod themes;

fn main() -> cosmic::iced::Result {
    init_logging();
    init_localizer();
    configure_rendering();

    // Verificar se está rodando em modo background
    let args: Vec<String> = env::args().collect();
    let background_mode = args.contains(&"--background".to_string());

    if background_mode {
        tracing::info!("Starting Sala do Futuro in background mode with notification support");
    }

    cosmic::app::run::<crate::pages::QuickWebApps>(
        cosmic::app::Settings::default()
            .antialiasing(true)
            .client_decorations(true),
        (),
    )
}

fn configure_rendering() {
    // Configurar renderização com Vulkan como prioridade
    // Fallback para OpenGL se Vulkan não estiver disponível
    
    if env::var("ENABLE_VULKAN").is_ok() {
        tracing::info!("Vulkan rendering enabled");
        unsafe {
            env::set_var("OZONE_PLATFORM", "wayland");
        }
    }

    // Habilitar hardware acceleration
    if env::var("LIBGL_ALWAYS_SOFTWARE").is_err() {
        unsafe {
            env::set_var("LIBGL_ALWAYS_SOFTWARE", "0");
        }
    }

    // Drivers GPU disponíveis
    if env::var("LIBGBM_DRIVERS").is_err() {
        unsafe {
            env::set_var("LIBGBM_DRIVERS", "nouveau,radeonsi,iris,amdgpu,swrast");
        }
    }

    tracing::debug!("Rendering configured: OZONE_PLATFORM={:?}", 
        env::var("OZONE_PLATFORM").unwrap_or_default());
}

fn init_localizer() {
    let localizer = localize::localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Err(why) = localizer.select(&requested_languages) {
        tracing::error!(%why, "error while loading fluent localizations");
    }
}

fn init_logging() {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
