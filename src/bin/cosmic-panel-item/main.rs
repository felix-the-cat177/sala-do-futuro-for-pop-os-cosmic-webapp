// Sala do Futuro - COSMIC Panel Item
// Integração com o painel do COSMIC via libcosmic

use cosmic::{
    app::{Command, Core},
    iced::{window, Alignment, Length},
    widget::{container, icon, mouse_area, text},
    Element, Application,
};
use std::sync::{Arc, Mutex};
use std::process::{Command as StdCommand, Stdio};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PanelItemClicked,
    NotificationReceived(bool),
}

pub struct PanelApp {
    core: Core,
    has_notification: Arc<Mutex<bool>>,
}

impl Application for PanelApp {
    type Message = Message;
    type Command = Command<Self::Message>;
    type Theme = cosmic::Theme;
    type Executor = cosmic::executor::Default;

    fn new(core: Core) -> (Self, Command<Self::Message>) {
        let app = PanelApp {
            core,
            has_notification: Arc::new(Mutex::new(false)),
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Sala do Futuro - Panel Item")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::PanelItemClicked => {
                // Abrir aplicativo Sala do Futuro
                let _ = StdCommand::new("sala-do-futuro-webapp")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn();
            }
            Message::NotificationReceived(has_notif) => {
                *self.has_notification.lock().unwrap() = has_notif;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let has_notif = *self.has_notification.lock().unwrap();

        let icon_widget = if has_notif {
            // Mostrar ícone com indicador de notificação
            container(
                container(icon(
                    cosmic::iced::widget::icon::from_rstr(
                        "dialog-information-symbolic",
                        16,
                    )
                ))
                .width(Length::Fixed(24.0))
                .height(Length::Fixed(24.0))
                .padding(2)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            )
            .padding(4)
        } else {
            // Ícone normal
            container(icon(cosmic::iced::widget::icon::from_rstr(
                "application-x-executable-symbolic",
                16,
            )))
            .padding(4)
        };

        mouse_area(icon_widget)
            .on_press(Message::PanelItemClicked)
            .into()
    }

    fn subscription(&self) -> cosmic::Subscription<Self::Message> {
        cosmic::Subscription::none()
    }
}

fn main() -> cosmic::Result {
    cosmic::app::run::<PanelApp>(
        Default::default(),
        |mut core| {
            let (app, command) = PanelApp::new(core);
            (app, command)
        },
    )
}
