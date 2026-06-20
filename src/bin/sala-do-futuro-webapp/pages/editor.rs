use cosmic::{
    Element, Task,
    action::Action,
    iced::{Length, alignment::Vertical},
    style, task,
    widget::{self},
};
use rand::{RngExt as _, rng};
use strum::IntoEnumIterator as _;
use sala_do_futuro_webapp::{
    fl, Category, Icon, handle_icon, generate_icon,
    launcher::{webapp_icon_valid, WebappIcon},
};

use crate::pages;

#[derive(Debug, Clone)]
pub struct AppEditor {
    pub app_browser: Option<sala_do_futuro_webapp::browser::Browser>,
    pub app_title: String,
    pub app_url: String,
    pub app_icon: Option<WebappIcon>,
    pub app_category: sala_do_futuro_webapp::Category,
    pub app_window_width: String,
    pub app_window_height: String,
    pub app_window_size: sala_do_futuro_webapp::WindowSize,
    pub app_isolated: bool,
    pub app_simulate_mobile: bool,
    pub selected_icon: Option<Icon>,
    pub categories: Vec<String>,
    pub category_idx: Option<usize>,
    pub is_installed: bool,
}

impl Default for AppEditor {
    fn default() -> Self {
        let categories = sala_do_futuro_webapp::Category::iter()
            .map(|c| c.name())
            .collect::<Vec<String>>();

        AppEditor {
            app_browser: None,
            app_title: String::from("Sala do Futuro"),
            app_url: String::from("https://saladofuturo.educacao.sp.gov.br/escolha-de-perfil"),
            app_icon: None,
            app_category: sala_do_futuro_webapp::Category::default(),
            app_window_width: String::from(sala_do_futuro_webapp::DEFAULT_WINDOW_WIDTH.to_string()),
            app_window_height: String::from(sala_do_futuro_webapp::DEFAULT_WINDOW_HEIGHT.to_string()),
            app_window_size: sala_do_futuro_webapp::WindowSize::default(),
            app_isolated: true,
            app_simulate_mobile: false,
            selected_icon: None,
            categories,
            category_idx: sala_do_futuro_webapp::Category::iter().position(|c| c == Category::Utility),
            is_installed: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Category(usize),
    Done,
    LaunchApp,
    OpenIconPicker,
    Title(String),
    Url(String),
    WindowWidth(String),
    WindowHeight(String),
    AppIsolated(bool),
    AppSimulateMobile(bool),
    GenerateIcon,
    ResetIcon,
}

impl AppEditor {
    pub fn from(webapp_launcher: sala_do_futuro_webapp::launcher::WebAppLauncher) -> Self {
        let entry = sala_do_futuro_webapp::launcher::installed_webapps()
            .into_iter()
            .find(|webapp| webapp.browser.app_id == webapp_launcher.browser.app_id);

        if let Some(launcher) = entry {
            let window_size = launcher.browser.window_size.clone().unwrap_or_default();
            let simulate_mobile = launcher.browser.try_simulate_mobile.unwrap_or_default();

            let mut editor = AppEditor::default();

            editor.app_browser = Some(launcher.browser.clone());
            editor.app_title = launcher.name.clone();
            editor.app_url = launcher.browser.url.clone().unwrap_or_default();
            editor.app_category = launcher.category.clone();
            editor.app_window_width = window_size.0.to_string();
            editor.app_window_height = window_size.1.to_string();
            editor.app_window_size = window_size.clone();
            editor.app_simulate_mobile = simulate_mobile;
            editor.category_idx = editor
                .categories
                .iter()
                .position(|c| c == &launcher.category.name());
            editor.is_installed = true;

            editor.update_icon(Some(launcher.icon.to_icon()));

            editor
        } else {
            AppEditor::default()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Action<crate::pages::Message>> {
        match message {
            Message::AppIsolated(flag) => {
                self.app_isolated = flag;
            }
            Message::AppSimulateMobile(flag) => {
                self.app_simulate_mobile = flag;
            }
            Message::Category(idx) => {
                self.app_category = sala_do_futuro_webapp::Category::from_index(idx as u8);
                self.category_idx = Some(idx);
            }
            Message::Done => {
                let browser = if let Some(browser) = &self.app_browser {
                    browser.clone()
                } else {
                    let app_id = self.app_title.replace(' ', "");
                    let app_id = app_id + &rng().random_range(1000..10000).to_string();

                    let mut browser = sala_do_futuro_webapp::browser::Browser::new(&app_id);
                    browser.window_title = Some(self.app_title.clone());
                    browser.url = Some(self.app_url.clone());
                    browser.window_size = Some(self.app_window_size.clone());
                    browser.try_simulate_mobile = Some(self.app_simulate_mobile);
                    browser
                };

                if sala_do_futuro_webapp::launcher::webapplauncher_is_valid(&self.app_title, &browser.url) {
                    if let Some(icon) = &self.app_icon {
                        let launcher = sala_do_futuro_webapp::launcher::WebAppLauncher {
                            browser: browser.clone(),
                            name: self.app_title.clone(),
                            icon: icon.clone(),
                            category: self.app_category.clone(),
                        };

                        return task::future(async move {
                            if let Ok(success) = launcher.create().await {
                                if success {
                                    return crate::pages::Message::SaveLauncher(launcher);
                                }
                            }
                            crate::pages::Message::None
                        });
                    }
                } else {
                    return Task::none();
                }
            }
            Message::GenerateIcon => {
                if self.app_title.len() > 1 {
                    let icon = generate_icon(&self.app_title.split_at(1).0);

                    if let Some(icon) = icon {
                        self.update_icon(Some(icon.to_icon()));

                        if webapp_icon_valid(&icon) {
                            let ico = sala_do_futuro_webapp::handle_icon(icon.path.into());

                            return task::future(async {
                                Action::App(pages::Message::SetIcon(Some(ico)))
                            });
                        };
                    }
                }
            }
            Message::LaunchApp => {
                if let Some(browser) = &self.app_browser {
                    let arg_id = browser.app_id.clone();

                    return task::future(async { crate::pages::Message::Launch(arg_id) });
                }
            }
            Message::OpenIconPicker => {
                return task::future(async { pages::Message::OpenIconPicker });
            }
            Message::ResetIcon => {
                self.app_icon = None;
                self.selected_icon = None;
            }
            Message::Title(title) => {
                self.app_title = title;
            }
            Message::Url(url) => {
                self.app_url = url;
            }
            Message::WindowWidth(width) => {
                self.app_window_width = width;
                self.app_window_size.0 = self.app_window_width.parse().unwrap_or_default();
            }
            Message::WindowHeight(height) => {
                self.app_window_height = height;
                self.app_window_size.1 = self.app_window_height.parse().unwrap_or_default();
            }
        }
        Task::none()
    }

    pub fn update_icon(&mut self, icon: Option<Icon>) {
        if let Some(icon) = icon {
            if let Some(webapp_icon) = icon.to_launcher_icon() {
                self.selected_icon = Some(handle_icon(icon.path.clone().into()));
                self.app_icon = Some(webapp_icon);
            }
        }
    }

    fn icon_element(&self, icon: Option<Icon>) -> Element<'_, Message> {
        let ico = if let Some(ico) = icon {
            match ico.icon {
                sala_do_futuro_webapp::IconType::Raster(data) => widget::button::custom(widget::image(data))
                    .width(Length::Fixed(92.0))
                    .height(Length::Fixed(92.0))
                    .class(style::Button::Icon),

                sala_do_futuro_webapp::IconType::Svg(data) => widget::button::custom(widget::svg(data))
                    .width(Length::Fixed(92.0))
                    .height(Length::Fixed(92.0))
                    .class(style::Button::Icon),
            }
        } else {
            widget::button::custom(widget::icon::from_name("dev.heppen.webapps").size(256))
                .class(style::Button::Icon)
        };

        widget::container(ico).into()
    }

    pub fn view(&self) -> Element<'_, Message> {
        widget::container(
            widget::Column::new()
                .spacing(24)
                .push(
                    widget::container(
                        widget::Row::new()
                            .spacing(12)
                            .push(
                                widget::container(self.icon_element(self.selected_icon.clone()))
                                    .width(96.)
                                    .height(96.)
                                    .align_y(Vertical::Center),
                            )
                            .push(
                                widget::container(
                                    widget::Column::new()
                                        .spacing(12)
                                        .push(widget::text::title3(format!(
                                            "{}: {}",
                                            fl!("title"),
                                            if self.app_title.is_empty() {
                                                fl!("new-webapp-title")
                                            } else {
                                                self.app_title.clone()
                                            }
                                        )))
                                        .push(widget::text::title4(format!(
                                            "{}: {}",
                                            fl!("category"),
                                            self.app_category.name()
                                        ))),
                                )
                                .height(Length::Fixed(96.))
                                .align_y(Vertical::Center),
                            ),
                    )
                    .padding(12)
                    .width(Length::Fill)
                    .class(style::Container::Card),
                )
                .push(
                    widget::Row::new()
                        .spacing(8)
                        .push(
                            widget::text_input(fl!("title"), &self.app_title)
                                .on_input(Message::Title),
                        )
                        .push(
                            widget::button::standard(fl!("generate-icon")).on_press_maybe(
                                if self.app_title.len() > 1 {
                                    Some(Message::GenerateIcon)
                                } else {
                                    None
                                },
                            ),
                        )
                        .push(
                            widget::button::standard(fl!("icon-selector"))
                                .on_press_maybe(Some(Message::OpenIconPicker)),
                        )
                        .push(widget::button::standard(fl!("reset-icon")).on_press_maybe(
                            if self.selected_icon.is_some() {
                                Some(Message::ResetIcon)
                            } else {
                                None
                            },
                        )),
                )
                .push(widget::text_input(fl!("url"), &self.app_url).on_input(Message::Url))
                .push(
                    widget::settings::section()
                        .add(widget::settings::item(
                            fl!("select-category"),
                            widget::dropdown(
                                &self.categories,
                                self.category_idx,
                                Message::Category,
                            ),
                        ))
                        .add(widget::settings::item(
                            fl!("window-size"),
                            widget::Row::new()
                                .spacing(8)
                                .push(
                                    widget::text_input(
                                        format!("{}", sala_do_futuro_webapp::DEFAULT_WINDOW_WIDTH),
                                        &self.app_window_width,
                                    )
                                    .on_input(Message::WindowWidth),
                                )
                                .push(
                                    widget::text_input(
                                        format!("{}", sala_do_futuro_webapp::DEFAULT_WINDOW_HEIGHT),
                                        &self.app_window_height,
                                    )
                                    .on_input(Message::WindowHeight),
                                ),
                        ))
                        .add(widget::settings::item(
                            fl!("isolated-profile"),
                            widget::toggler(self.app_isolated).on_toggle(Message::AppIsolated),
                        ))
                        .add(widget::settings::item(
                            fl!("simulate-mobile"),
                            widget::toggler(self.app_simulate_mobile)
                                .on_toggle(Message::AppSimulateMobile),
                        )),
                )
                .push(
                    widget::Row::new()
                        .spacing(8)
                        .push(widget::Space::with_width(cosmic::iced::Length::Fill))
                        .push_maybe(if !self.is_installed {
                            None
                        } else {
                            Some(
                                widget::button::standard(fl!("run-app"))
                                    .on_press(Message::LaunchApp),
                            )
                        })
                        .push(widget::button::suggested(fl!("create")).on_press_maybe(
                            if sala_do_futuro_webapp::launcher::webapplauncher_is_valid(
                                &self.app_title,
                                &Some(self.app_url.clone()),
                            ) {
                                Some(Message::Done)
                            } else {
                                None
                            },
                        )),
                ),
        )
        .padding(cosmic::iced::Padding::new(0.).left(30.0).right(30.0))
        .max_width(1000)
        .into()
    }
}
