// Sala do Futuro - COSMIC Panel Item
// Integração com o painel do COSMIC - simplified widget
// Para produção completa, use libcosmic com extensão de painel

use std::sync::{Arc, Mutex};
use std::process::{Command, Stdio};
use tracing::{info, debug};

#[derive(Clone)]
pub struct PanelApp {
    has_notification: Arc<Mutex<bool>>,
}

impl PanelApp {
    pub fn new() -> Self {
        PanelApp {
            has_notification: Arc::new(Mutex::new(false)),
        }
    }

    pub fn on_notification(&self, has_notif: bool) {
        debug!("Notification status: {}", has_notif);
        if let Ok(mut notif) = self.has_notification.lock() {
            *notif = has_notif;
        }
    }

    pub fn launch_app(&self) {
        info!("Launching Sala do Futuro");
        let _ = Command::new("sala-do-futuro-webapp")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    }

    pub fn has_notification(&self) -> bool {
        self.has_notification
            .lock()
            .map(|n| *n)
            .unwrap_or(false)
    }
}

fn main() {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Sala do Futuro - COSMIC Panel Widget initialized");
    info!("This is a placeholder for COSMIC panel integration.");
    info!("In production, this would be loaded as a COSMIC panel extension.");
    
    let app = PanelApp::new();
    
    // Simular monitoramento de notificações
    app.on_notification(true);
    println!("Panel status: {}", if app.has_notification() { "●" } else { "○" });
    
    // Teste: tentar iniciar aplicativo
    // app.launch_app();
}
