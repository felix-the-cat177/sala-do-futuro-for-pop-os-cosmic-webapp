// Notification Daemon para Sala do Futuro
// Monitora notificações HTML5 e custom events
// Se comunica com o COSMIC panel via D-Bus

use std::error::Error;
use zbus::{Connection, dbus_interface};
use tokio::time::Duration;
use tracing::info;

/// Interface D-Bus para notificações
pub struct NotificationService {
    has_notification: bool,
}

#[dbus_interface]
impl NotificationService {
    #[dbus_interface(property)]
    pub fn has_notification(&self) -> bool {
        self.has_notification
    }

    #[dbus_interface(property)]
    pub fn set_has_notification(&mut self, value: bool) {
        self.has_notification = value;
        info!("Notification status changed to: {}", value);
    }

    /// Método chamado quando uma notificação é recebida
    pub fn notify(&mut self, app: &str, title: &str, body: &str) -> zbus::fdo::Result<u32> {
        info!("Notification from {}: {} - {}", app, title, body);
        self.has_notification = true;
        Ok(1)
    }

    /// Limpar notificações
    pub fn close_notification(&mut self, id: u32) -> zbus::fdo::Result<()> {
        info!("Closing notification: {}", id);
        self.has_notification = false;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting Sala do Futuro Notification Daemon");

    // Conectar ao D-Bus
    let connection = Connection::session().await?;

    // Registrar a interface D-Bus
    let service = NotificationService {
        has_notification: false,
    };

    connection
        .object_server()
        .at("/org/saladofuturo/NotificationDaemon", service)
        .await?;

    connection
        .request_name("org.saladofuturo.NotificationDaemon")
        .await?;

    info!("D-Bus service registered: org.saladofuturo.NotificationDaemon");

    // Manter o serviço rodando
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
