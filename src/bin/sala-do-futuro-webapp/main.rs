use i18n_embed::DesktopLanguageRequester;
use sala_do_futuro_webapp::localize;
use std::env;
use std::process::Command;
use std::path::PathBuf;

pub(crate) mod config;
pub(crate) mod pages;
pub(crate) mod themes;

fn main() -> cosmic::iced::Result {
    init_logging();

    // Verificar se a Sala do Futuro já está rodando
    let socket_path = dirs::runtime_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("sala-do-futuro-webview.sock");

    if let Ok(mut stream) = std::os::unix::net::UnixStream::connect(&socket_path) {
        use std::io::Write;
        tracing::info!("Sala do Futuro já está rodando. Requisitando foco e saindo.");
        let _ = stream.write_all(b"focus");
        return Ok(());
    }

    init_localizer();
    configure_rendering();

    let args: Vec<String> = env::args().collect();
    let background_mode = args.contains(&"--background".to_string());

    if background_mode {
        tracing::info!("Starting Sala do Futuro in background mode with notification support");
    }

    // Se não for solicitado o gerenciador, o background mode ou argumentos especiais,
    // inicia diretamente o WebView para a Sala do Futuro
    if !args.contains(&"--manager".to_string()) && !args.contains(&"-m".to_string()) && !background_mode {
        // Garantir que a configuração existe na base de dados
        if let Some(db_path) = sala_do_futuro_webapp::database_path("sala-do-futuro.ron") {
            if !db_path.exists() {
                let xdg_data = dirs::data_dir().unwrap_or_default();
                let profile_path = xdg_data.join(sala_do_futuro_webapp::APP_ID).join("profiles").join("sala-do-futuro");
                
                let launcher = sala_do_futuro_webapp::launcher::WebAppLauncher {
                    browser: sala_do_futuro_webapp::browser::Browser {
                        app_id: sala_do_futuro_webapp::WebviewArgs {
                            id: "sala-do-futuro".to_string(),
                        },
                        window_title: Some("Sala do Futuro".to_string()),
                        url: Some("https://saladofuturo.educacao.sp.gov.br/escolha-de-perfil".to_string()),
                        profile: profile_path,
                        window_size: Some(sala_do_futuro_webapp::WindowSize(800, 600)),
                        try_simulate_mobile: Some(false),
                    },
                    name: "Sala do Futuro".to_string(),
                    icon: sala_do_futuro_webapp::launcher::WebappIcon {
                        path: PathBuf::from("/usr/share/icons/hicolor/256x256/apps/sala-do-futuro-webapp.svg"),
                        buffer: Vec::new(),
                    },
                    category: sala_do_futuro_webapp::Category::Education,
                };
                
                if let Ok(content) = ron::ser::to_string_pretty(&launcher, ron::ser::PrettyConfig::default()) {
                    let _ = std::fs::write(db_path, content);
                }
            }
        }

        if let Some(cef_path) = sala_do_futuro_webapp::cef_path() {
            let webview_bin = sala_do_futuro_webapp::webview_bin();
            let mut cmd = Command::new(&webview_bin);
            cmd.env("LD_LIBRARY_PATH", cef_path.display().to_string());
            cmd.arg("sala-do-futuro");
            
            match cmd.spawn() {
                Ok(_) => {
                    tracing::info!("WebView da Sala do Futuro iniciado.");
                    return Ok(());
                }
                Err(e) => {
                    tracing::error!("Erro ao iniciar WebView ({}): {}", webview_bin, e);
                }
            }
        } else {
            tracing::error!("Não foi possível localizar o diretório CEF.");
        }
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
