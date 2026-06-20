// Sala do Futuro - COSMIC Panel Item Applet
// Integração real com o painel do COSMIC usando libcosmic

use cosmic::iced::{window::Id, Subscription};
use cosmic::prelude::*;
use cosmic::widget;
use std::process::Command;
use std::time::Duration;
use tokio::time::interval;
use zbus::Connection;

#[zbus::proxy(
    interface = "org.saladofuturo.NotificationDaemon",
    default_service = "org.saladofuturo.NotificationDaemon",
    default_path = "/org/saladofuturo/NotificationDaemon"
)]
trait NotificationDaemon {
    #[zbus(property)]
    fn has_notification(&self) -> zbus::Result<bool>;
}

pub struct AppletModel {
    core: cosmic::Core,
    has_notification: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    AppletClicked,
    SetNotificationStatus(bool),
}

impl cosmic::Application for AppletModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "dev.heppen.webapps.applet";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let app = AppletModel {
            core,
            has_notification: false,
        };
        (app, Task::none())
    }

    fn on_close_requested(&self, _id: Id) -> Option<Message> {
        None
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let button = self.core
            .applet
            .icon_button("sala-do-futuro-webapp")
            .on_press(Message::AppletClicked);

        if self.has_notification {
            // Desenhar um pequeno ponto vermelho ao lado para indicar notificações
            let dot = widget::text("●")
                .size(14)
                .style(cosmic::theme::Text::Color(cosmic::iced::Color::from_rgb(1.0, 0.2, 0.2)));

            widget::row::with_children(vec![
                button.into(),
                dot.into(),
            ])
            .align_y(cosmic::iced::alignment::Vertical::Center)
            .spacing(2)
            .into()
        } else {
            button.into()
        }
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        // Applets de cliques diretos como este geralmente não usam popups,
        // mas precisamos definir um elemento vazio por exigência da trait.
        widget::container(widget::text("")).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::run(|| {
            cosmic::iced::stream::channel(10, move |mut channel: cosmic::iced::futures::channel::mpsc::Sender<Message>| async move {
                // Tenta se conectar ao D-Bus de sessão
                let conn = match Connection::session().await {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to connect to session bus: {}", e);
                        return;
                    }
                };

                // Cria o proxy para ler as notificações do daemon
                let proxy = match NotificationDaemonProxy::new(&conn).await {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Failed to create daemon proxy: {}", e);
                        return;
                    }
                };

                // Executa um loop de polling a cada 2 segundos
                let mut ticker = interval(Duration::from_secs(2));
                loop {
                    ticker.tick().await;
                    if let Ok(has_notif) = proxy.has_notification().await {
                        let _ = channel.send(Message::SetNotificationStatus(has_notif)).await;
                    }
                }
            })
        })
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::AppletClicked => {
                if !check_and_focus() {
                    launch_app();
                }
            }
            Message::SetNotificationStatus(status) => {
                self.has_notification = status;
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

fn check_and_focus() -> bool {
    let socket_path = dirs::runtime_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("sala-do-futuro-webview.sock");

    if let Ok(mut stream) = std::os::unix::net::UnixStream::connect(&socket_path) {
        use std::io::Write;
        let _ = stream.write_all(b"focus");
        true
    } else {
        false
    }
}

fn launch_app() {
    let _ = Command::new("sala-do-futuro-webapp")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<AppletModel>(())
}
