use iced::keyboard;
use iced::widget::{
    button, center_x, center_y, checkbox, column, container, pick_list,
    progress_bar, row, rule, scrollable, slider, space, text, text_input,
    toggler,
};
use iced::{Center, Element, Fill, Shrink, Subscription, Theme};

pub fn main() -> iced::Result {
    iced::application(Styling::default, Styling::update, Styling::view)
        .subscription(Styling::subscription)
        .theme(Styling::theme)
        .run()
}

// Перечисление HTTP методов
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    // Создадим список всех доступных методов
    const ALL: &'static [HttpMethod] = &[
        HttpMethod::GET,
        HttpMethod::POST,
        HttpMethod::PUT,
        HttpMethod::DELETE,
        HttpMethod::PATCH,
    ];
}

// Добавим структуру для Query параметра
#[derive(Debug, Clone)]
struct QueryParam {
    key: String,
    value: String,
}

impl QueryParam {
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}


// Реализуем Display для отображения в pick_list
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default)]
struct Styling {
    theme: Option<Theme>,
    input_value: String,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
    http_method: HttpMethod, // HTTP метод
    url_input: String, // Поле для ввода URL
    // ↓ Добавленные поля ↓
    query_params: Vec<QueryParam>, // Список параметров
    new_query_key: String,         // Поле для нового ключа
    new_query_value: String,       // Поле для нового значения
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
    PreviousTheme,
    NextTheme,
    ClearTheme,
    HttpMethodChanged(HttpMethod), // Изменение HTTP метода
    UrlInputChanged(String), // Изменение URL
    // ↓ Добавленные сообщения для Query Parameters ↓
    NewQueryKeyChanged(String),       // Изменение поля нового ключа
    NewQueryValueChanged(String),     // Изменение поля нового значения
    AddQueryParam,                    // Добавить новый параметр
    RemoveQueryParam(usize),          // Удалить параметр по индексу
    UpdateQueryParamKey(usize, String), // Обновить ключ параметра
    UpdateQueryParamValue(usize, String), // Обновить значение параметра
}

impl Styling {
    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = Some(theme);
            }
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {}
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::TogglerToggled(value) => self.toggler_value = value,
            Message::PreviousTheme | Message::NextTheme => {
                let current = Theme::ALL.iter().position(|candidate| {
                    self.theme.as_ref() == Some(candidate)
                });

                self.theme = Some(if matches!(message, Message::NextTheme) {
                    Theme::ALL[current.map(|current| current + 1).unwrap_or(0)
                        % Theme::ALL.len()]
                        .clone()
                } else {
                    let current = current.unwrap_or(0);

                    if current == 0 {
                        Theme::ALL
                            .last()
                            .expect("Theme::ALL must not be empty")
                            .clone()
                    } else {
                        Theme::ALL[current - 1].clone()
                    }
                });
            }
            Message::ClearTheme => {
                self.theme = None;
            }
            // ↓ Добавленная обработка ↓
            Message::HttpMethodChanged(method) => {
                self.http_method = method;
            }
            // ↓ Добавленная обработка ↓
            Message::UrlInputChanged(url) => {
                self.url_input = url;
            }
            // ↓ Обработка Query Parameters ↓
            Message::NewQueryKeyChanged(key) => {
                self.new_query_key = key;
            }
            Message::NewQueryValueChanged(value) => {
                self.new_query_value = value;
            }
            Message::AddQueryParam => {
                if !self.new_query_key.trim().is_empty() {
                    let param = QueryParam::new(
                        self.new_query_key.trim().to_string(),
                        self.new_query_value.trim().to_string(),
                    );
                    self.query_params.push(param);
                    self.new_query_key.clear();
                    self.new_query_value.clear();
                }
            }
            Message::RemoveQueryParam(index) => {
                if index < self.query_params.len() {
                    self.query_params.remove(index);
                }
            }
            Message::UpdateQueryParamKey(index, key) => {
                if let Some(param) = self.query_params.get_mut(index) {
                    param.key = key;
                }
            }
            Message::UpdateQueryParamValue(index, value) => {
                if let Some(param) = self.query_params.get_mut(index) {
                    param.value = value;
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(Theme::ALL, self.theme.as_ref(), Message::ThemeChanged)
                .width(Fill)
                .placeholder("System"),
        ]
            .spacing(10);

        // ↓ Добавленный выбор HTTP метода ↓
        let choose_http_method = column![
            text("HTTP Method:"),
            pick_list(
                HttpMethod::ALL,
                Some(&self.http_method),
                Message::HttpMethodChanged
            )
                .width(Fill)
                .placeholder("Select method"),
        ]
            .spacing(10);

        // ↓ Добавленное текстовое поле для URL ↓
        let url_input = text_input("Enter API URL...", &self.url_input)
            .on_input(Message::UrlInputChanged)
            .padding(10)
            .size(16) // Чуть меньше шрифт для URL
            .width(Fill); // Заполняет всю доступную ширину


        // ↓ Создаем таблицу Query Parameters ↓
        let query_params_section = {
            // Заголовок
            let header = text("Query Parameters:").size(16);

            // Таблица существующих параметров
            let params_table: Element<Message> = if self.query_params.is_empty() {
                // Если параметров нет - показываем сообщение
                container(text("No query parameters added yet").style(text::secondary))
                    .padding(10)
                    .center_x(Shrink)
                    .into()
            } else {
                // Создаем таблицу с параметрами
                let rows = self.query_params.iter().enumerate().map(|(index, param)| {
                    row![
                        // Поле для ключа
                        text_input("Key", &param.key)
                            .on_input(move |key| Message::UpdateQueryParamKey(index, key))
                            .width(150)
                            .padding(5),
                        // Поле для значения
                        text_input("Value", &param.value)
                            .on_input(move |value| Message::UpdateQueryParamValue(index, value))
                            .width(150)
                            .padding(5),
                        // Кнопка удаления
                        button(text("🗑️").size(14))
                            .on_press(Message::RemoveQueryParam(index))
                            .padding(5)
                            .style(button::danger),
                    ]
                    .spacing(10)
                    .align_y(Center)
                });

                // Собираем строки в колонку и конвертируем в Element
                let rows_vec: Vec<Element<Message>> = rows.map(|row| row.into()).collect();
                column(rows_vec).spacing(5).into() // ← Исправлено

            };

            // Форма добавления нового параметра
            let add_form = row![
                text_input("New key...", &self.new_query_key)
                    .on_input(Message::NewQueryKeyChanged)
                    .width(150)
                    .padding(5),
                text_input("New value...", &self.new_query_value)
                    .on_input(Message::NewQueryValueChanged)
                    .width(150)
                    .padding(5),
                button(text("+ Add").size(14))
                    .on_press(Message::AddQueryParam)
                    .padding(5)
                    .style(button::success),
            ]
            .spacing(10)
            .align_y(Center);

            // Собираем все вместе
            container(column![
                header,
                space().height(5),
                params_table,
                space().height(10),
                add_form,
            ]
                .spacing(5)
                .padding(10))
                .style(container::bordered_box) // ← Переместили style на container
        };

        let text_input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let buttons = {
            let styles = [
                ("Primary", button::primary as fn(&Theme, _) -> _),
                ("Secondary", button::secondary),
                ("Success", button::success),
                ("Warning", button::warning),
                ("Danger", button::danger),
            ];

            let styled_button =
                |label| button(text(label).width(Fill).center()).padding(10);

            column![
                row(styles.into_iter().map(|(name, style)| styled_button(
                    name
                )
                .on_press(Message::ButtonPressed)
                .style(style)
                .into()))
                .spacing(10)
                .align_y(Center),
                row(styles.into_iter().map(|(name, style)| styled_button(
                    name
                )
                .style(style)
                .into()))
                .spacing(10)
                .align_y(Center),
            ]
                .spacing(10)
        };

        let slider =
            || slider(0.0..=100.0, self.slider_value, Message::SliderChanged);

        let progress_bar = || progress_bar(0.0..=100.0, self.slider_value);

        let scroll_me = scrollable(column![
            "Scroll me!",
            space().height(800),
            "You did it!"
        ])
            .width(Fill)
            .height(Fill)
            .auto_scroll(true);

        let check = checkbox(self.checkbox_value)
            .label("Check me!")
            .on_toggle(Message::CheckboxToggled);

        let check_disabled = checkbox(self.checkbox_value).label("Disabled");

        let toggle = toggler(self.toggler_value)
            .label("Toggle me!")
            .on_toggle(Message::TogglerToggled)
            .spacing(10);

        let disabled_toggle =
            toggler(self.toggler_value).label("Disabled").spacing(10);

        let card = {
            container(
                column![
                    text("Card Example").size(24),
                    slider(),
                    progress_bar(),
                ]
                    .spacing(20),
            )
                .width(Fill)
                .padding(20)
                .style(container::bordered_box)
        };

        let content = column![
            choose_theme,
            choose_http_method, // ← Выбор метода
            url_input, // ← Строка адреса
            query_params_section, // ← Табличка параметров
            rule::horizontal(1),
            text_input,
            buttons,
            slider(),
            progress_bar(),
            row![
                scroll_me,
                rule::vertical(1),
                column![check, check_disabled, toggle, disabled_toggle]
                    .spacing(10)
            ]
            .spacing(10)
            .height(Shrink)
            .align_y(Center),
            card
        ]
            .spacing(20)
            .padding(20)
            .max_width(800);

        center_y(scrollable(center_x(content)).spacing(10))
            .padding(10)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            let keyboard::Event::KeyPressed {
                modified_key: keyboard::Key::Named(modified_key),
                repeat: false,
                ..
            } = event
            else {
                return None;
            };

            match modified_key {
                keyboard::key::Named::ArrowUp
                | keyboard::key::Named::ArrowLeft => {
                    Some(Message::PreviousTheme)
                }
                keyboard::key::Named::ArrowDown
                | keyboard::key::Named::ArrowRight => Some(Message::NextTheme),
                keyboard::key::Named::Space => Some(Message::ClearTheme),
                _ => None,
            }
        })
    }

    fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;

    use iced_test::{Error, simulator};

    #[test]
    #[ignore]
    fn it_showcases_every_theme() -> Result<(), Error> {
        Theme::ALL
            .par_iter()
            .cloned()
            .map(|theme| {
                let mut styling = Styling::default();
                styling.update(Message::ThemeChanged(theme.clone()));

                let mut ui = simulator(styling.view());
                let snapshot = ui.snapshot(&theme)?;

                assert!(
                    snapshot.matches_hash(format!(
                        "snapshots/{theme}",
                        theme = theme
                            .to_string()
                            .to_ascii_lowercase()
                            .replace(" ", "_")
                    ))?,
                    "snapshots for {theme} should match!"
                );

                Ok(())
            })
            .collect()
    }
}