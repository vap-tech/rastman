use iced::{keyboard, Task};
use iced::widget::{
    button, center_x, center_y, checkbox, column, container, pick_list,
    progress_bar, row, rule, scrollable, slider, space, text, text_input,
    toggler, Button
};
use iced::{Center, Element, Fill, Shrink, Subscription, Theme, Font};
use iced::highlighter; // Для подсветки синтаксиса
use iced::widget::text_editor;


pub fn main() -> iced::Result {
    iced::application(Styling::default, Styling::update, Styling::view)
        .subscription(Styling::subscription)
        .theme(Styling::theme)
        .run()
}

// Популярные заголовки
const COMMON_HEADERS: &[&str] = &[
    "Accept",
    "Accept-Charset",
    "Accept-Encoding",
    "Accept-Language",
    "Authorization",
    "Cache-Control",
    "Content-Type",
    "Content-Length",
    "Content-Encoding",
    "Cookie",
    "Date",
    "Host",
    "User-Agent",
    "X-API-Key",
    "X-Requested-With",
    "X-CSRF-Token",
    "X-Forwarded-For",
    "X-Forwarded-Proto",
    "If-Modified-Since",
    "If-None-Match",
    "ETag",
    "Location",
    "Referer",
    "Origin",
    "Access-Control-Allow-Origin",
    "Access-Control-Allow-Methods",
    "Access-Control-Allow-Headers",
];

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

// Добавим структуру для Header
#[derive(Debug, Clone)]
struct HeaderParam {
    key: String,
    value: String,
}

impl HeaderParam {
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

// Реализация Display для наших структур
impl std::fmt::Display for QueryParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

impl std::fmt::Display for HeaderParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

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
     // ↓ Добавленные поля для Headers ↓
    headers: Vec<HeaderParam>,
    new_header_key: String,
    new_header_value: String,
    // ↓ Поля для JSON редактора ↓
    json_theme: highlighter::Theme,
    body_content: text_editor::Content,
    json_valid: bool,
    // ↓ Добавляем новые поля ↓
    is_loading: bool,               // Индикатор загрузки
    response_status: Option<u16>,   // Статус ответа
    response_body: String,          // Тело ответа
    response_error: Option<String>, // Ошибка если была
    header_suggestions: Vec<String>,    // Текущие подсказки
}

// 3. Реализуй Default вручную
impl Default for Styling {
    fn default() -> Self {
        Self {
            theme: None,
            input_value: String::new(),
            slider_value: 0.0,
            checkbox_value: false,
            toggler_value: false,
            http_method: HttpMethod::default(),
            url_input: String::new(),
            query_params: Vec::new(),
            new_query_key: String::new(),
            new_query_value: String::new(),
            headers: Vec::new(),
            new_header_key: String::new(),
            new_header_value: String::new(),
            // Инициализируем поля для JSON редактора
            json_theme: highlighter::Theme::SolarizedDark, // или другой вариант
            body_content: text_editor::Content::new(),
            // Пустое тело считаем валидным
            json_valid: true,
            // ↓ Инициализируем поля для запроса ↓
            is_loading: false,
            response_status: None,
            response_body: String::new(),
            response_error: None,
            header_suggestions: Vec::new(),
        }
    }
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
    // ↓ Добавленные сообщения для Headers ↓
    NewHeaderKeyChanged(String),
    NewHeaderValueChanged(String),
    AddHeader,
    RemoveHeader(usize),
    UpdateHeaderKey(usize, String),
    UpdateHeaderValue(usize, String),
    // ↓ Добавляем обработку действий редактора
    BodyActionPerformed(text_editor::Action),
    // ↓ Опционально: для смены темы подсветки
    JsonThemeChanged(highlighter::Theme),
    // ↓ Добавляем ↓
    SendRequest,  // Отправка запроса
    RequestCompleted(Result<(u16, String), String>), // ← По завершении запроса
    ApplyHeaderSuggestion(String),   // Применить подсказку (клик по ней)
}

impl Styling {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = Some(theme);
                Task::none()
            }
            Message::InputChanged(value) => {
                self.input_value = value;
                Task::none()
            }
            Message::ButtonPressed => Task::none(),
            Message::SliderChanged(value) => {
                self.slider_value = value;
                Task::none()
            }
            Message::CheckboxToggled(value) => {
                self.checkbox_value = value;
                Task::none()
            }
            Message::TogglerToggled(value) => {
                self.toggler_value = value;
                Task::none()
            }
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
                Task::none()
            }
            Message::ClearTheme => {
                self.theme = None;
                Task::none()
            }
            Message::HttpMethodChanged(method) => {
                self.http_method = method;
                Task::none()
            }
            Message::UrlInputChanged(url) => {
                self.url_input = url;
                Task::none()
            }
            Message::NewQueryKeyChanged(key) => {
                self.new_query_key = key;
                Task::none()
            }
            Message::NewQueryValueChanged(value) => {
                self.new_query_value = value;
                Task::none()
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
                Task::none()
            }
            Message::RemoveQueryParam(index) => {
                if index < self.query_params.len() {
                    self.query_params.remove(index);
                }
                Task::none()
            }
            Message::UpdateQueryParamKey(index, key) => {
                if let Some(param) = self.query_params.get_mut(index) {
                    param.key = key;
                }
                Task::none()
            }
            Message::UpdateQueryParamValue(index, value) => {
                if let Some(param) = self.query_params.get_mut(index) {
                    param.value = value;
                }
                Task::none()
            }
            Message::NewHeaderKeyChanged(key) => {

                // Показываем подсказки если ввели хотя бы 2 символа
                self.new_header_key = key.clone();

                // Всегда обновляем подсказки при изменении текста
                if key.len() >= 2 {
                    self.header_suggestions = self.get_header_suggestions(&key);
                } else {
                    self.header_suggestions.clear();
                }

                Task::none()
            }
            // ↓ Новые обработчики для автодополнения ↓
            Message::ApplyHeaderSuggestion(header) => {
                self.new_header_key = header;
                self.header_suggestions.clear(); // ← ОЧИЩАЕМ подсказки
                Task::none()
            }
            Message::NewHeaderValueChanged(value) => {
                self.new_header_value = value;
                Task::none()
            }
            Message::AddHeader => {
                if !self.new_header_key.trim().is_empty() {
                    let header = HeaderParam::new(
                        self.new_header_key.trim().to_string(),
                        self.new_header_value.trim().to_string(),
                    );
                    self.headers.push(header);
                    self.new_header_key.clear();
                    self.new_header_value.clear();
                }
                Task::none()
            }
            Message::RemoveHeader(index) => {
                if index < self.headers.len() {
                    self.headers.remove(index);
                }
                Task::none()
            }
            Message::UpdateHeaderKey(index, key) => {
                if let Some(header) = self.headers.get_mut(index) {
                    header.key = key;
                }
                Task::none()
            }
            Message::UpdateHeaderValue(index, value) => {
                if let Some(header) = self.headers.get_mut(index) {
                    header.value = value;
                }
                Task::none()
            }
            Message::BodyActionPerformed(action) => {
                self.body_content.perform(action);

                // Проверяем валидность JSON
                let text = self.body_content.text();
                self.json_valid = if text.trim().is_empty() {
                    true // Пустое тело - валидно
                } else {
                    serde_json::from_str::<serde_json::Value>(&text).is_ok()
                };

                Task::none()
            }
            Message::JsonThemeChanged(theme) => {
                self.json_theme = theme;
                Task::none()
            }
            // 3. ОБНОВЛЯЕМ SendRequest для асинхронной работы
            Message::SendRequest => {
                // Проверяем URL
                if self.url_input.trim().is_empty() {
                    self.response_error = Some("URL is empty".to_string());
                    self.response_status = None;
                    self.is_loading = false;
                    return Task::none();
                }

                // 1. Сразу показываем индикатор загрузки
                self.is_loading = true;
                self.response_error = None;

                // Клонируем данные для передачи в async задачу
                let method = self.http_method;
                let url = self.url_input.clone();
                let query_params = self.query_params.clone();
                let headers = self.headers.clone();
                let body_text = self.body_content.text();

                // 2. Запускаем асинхронную задачу
                Task::perform(
                    async move {
                        // Вызываем асинхронную функцию
                        send_http_request(method, url, query_params, headers, body_text).await
                    },
                    // 3. Когда задача завершится, Iced вызовет это
                    Message::RequestCompleted
                )
            }
            // 4. ДОБАВЛЯЕМ обработчик для RequestCompleted
            Message::RequestCompleted(result) => {
                self.is_loading = false;

                match result {
                    Ok((status, body)) => {
                        self.response_status = Some(status);
                        self.response_body = body;
                        self.response_error = None;
                    }
                    Err(error) => {
                        self.response_status = None;
                        self.response_body.clear();
                        self.response_error = Some(error);
                    }
                }

                Task::none()
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

        // Функция для Query Parameters таблицы
        let query_params_table = {
            let title = text("Query Parameters:").size(16);
            
            let items_table: Element<Message> = if self.query_params.is_empty() {
                container(text("No query parameters added yet").style(text::secondary))
                    .padding(10)
                    .center_x(Shrink)
                    .into()
            } else {
                let rows = self.query_params.iter().enumerate().map(|(index, param)| {
                    row![
                        text_input("Key", &param.key)
                            .on_input(move |key| Message::UpdateQueryParamKey(index, key))
                            .width(140)
                            .padding(5),
                        text_input("Value", &param.value)
                            .on_input(move |value| Message::UpdateQueryParamValue(index, value))
                            .width(140)
                            .padding(5),
                        button(text("🗑️").size(14))
                            .on_press(Message::RemoveQueryParam(index))
                            .padding(5)
                            .style(button::danger),
                    ]
                    .spacing(8)
                    .align_y(Center)
                });

                let rows_vec: Vec<Element<Message>> = rows.map(|row| row.into()).collect();
                column(rows_vec).spacing(5).into()
            };

            let add_form = row![
                text_input("Key...", &self.new_query_key)
                    .on_input(Message::NewQueryKeyChanged)
                    .width(140)
                    .padding(5),
                text_input("Value...", &self.new_query_value)
                    .on_input(Message::NewQueryValueChanged)
                    .width(140)
                    .padding(5),
                button(text("+ Add").size(14))
                    .on_press(Message::AddQueryParam)
                    .padding(5)
                    .style(button::success),
            ]
            .spacing(8)
            .align_y(Center);

            container(column![
                title,
                space().height(5),
                items_table,
                space().height(10),
                add_form,
            ]
            .spacing(5)
            .padding(10))
            .style(container::bordered_box)
        };

        // Функция для Headers таблицы
        let headers_table = {
            let title = text("Headers:").size(16);

            let items_table: Element<Message> = if self.headers.is_empty() {
                container(text("No headers added yet").style(text::secondary))
                    .padding(10)
                    .center_x(Shrink)
                    .into()
            } else {
                let rows = self.headers.iter().enumerate().map(|(index, header)| {
                    row![
                text_input("Key", &header.key)
                    .on_input(move |key| Message::UpdateHeaderKey(index, key))
                    .width(140)
                    .padding(5),
                text_input("Value", &header.value)
                    .on_input(move |value| Message::UpdateHeaderValue(index, value))
                    .width(140)
                    .padding(5),
                button(text("🗑️").size(14))
                    .on_press(Message::RemoveHeader(index))
                    .padding(5)
                    .style(button::danger),
            ]
                        .spacing(8)
                        .align_y(Center)
                });

                let rows_vec: Vec<Element<Message>> = rows.map(|row| row.into()).collect();
                column(rows_vec).spacing(5).into()
            };

            // ↓ ОБНОВЛЯЕМ форму добавления с автодополнением ↓
            let header_key_input = text_input("Key...", &self.new_header_key)
                .on_input(Message::NewHeaderKeyChanged)
                .width(140)
                .padding(5);

            // Виджет с подсказками
            let suggestions_widget: Element<Message> =
                if self.new_header_key.len() >= 2 && !self.header_suggestions.is_empty() {
                    let suggestions: Vec<Element<Message>> = self.header_suggestions
                        .clone()
                        .into_iter()
                        .map(|suggestion_text| {
                            let text_for_display = suggestion_text.clone(); // Клон для отображения

                            button(text(text_for_display).size(12))
                                .on_press(Message::ApplyHeaderSuggestion(suggestion_text)) // Передаем владение
                                .padding(8)
                                .width(Fill)
                                .style(button::secondary)
                                .into()
                        })
                        .collect();

                    container(column(suggestions).spacing(2))
                        .padding(5)
                        .style(container::bordered_box)
                        .into()
                } else {
                    // Пустой элемент когда нет подсказок
                    Element::from(space().height(0))
                };

            let add_form = column![
        row![
            header_key_input,
            text_input("Value...", &self.new_header_value)
                .on_input(Message::NewHeaderValueChanged)
                .width(140)
                .padding(5),
            button(text("+ Add").size(14))
                .on_press(Message::AddHeader)
                .padding(5)
                .style(button::success),
        ]
        .spacing(8)
        .align_y(Center),
        suggestions_widget, // ← Добавляем подсказки под полем ввода
    ]
                .spacing(5);

            container(column![
        title,
        space().height(5),
        items_table,
        space().height(10),
        add_form,
    ]
                .spacing(5)
                .padding(10))
                .style(container::bordered_box)
        };

        // 4. Собираем таблицы рядом
        let params_tables = row![
            query_params_table,
            space().width(20),
            headers_table,
        ]
        .spacing(10)
        .align_y(Center);   

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

        // Создадим секцию Body с text_editor
        let body_section = {
            let title = row![
                text("Body (JSON):").size(16),
                space().width(10),
                if self.json_valid {
                    text("✅ Valid JSON").size(12).style(text::success)
                } else {
                    text("❌ Invalid JSON").size(12).style(text::danger)
                }
            ]
                .align_y(Center);
            
            // Опционально: выбор темы подсветки
            let theme_selector = row![
                text("Syntax theme:").size(14),
                pick_list(
                    highlighter::Theme::ALL,
                    Some(self.json_theme),
                    Message::JsonThemeChanged
                )
                .width(200)
                .padding(5)
            ]
            .spacing(10)
            .align_y(Center);
            
            // Редактор JSON с подсветкой
            let json_editor = text_editor(&self.body_content)
                .height(150)
                .on_action(Message::BodyActionPerformed)
                .highlight("json", self.json_theme) // Подсветка JSON
                .wrapping(text::Wrapping::Word);
            
            container(column![
                title,
                space().height(5),
                theme_selector, // можно убрать, если не нужен
                space().height(5),
                json_editor,
            ]
            .spacing(5)
            .padding(10))
            .style(if self.json_valid {
                container::bordered_box // обычная рамка
            } else {
                // Красная рамка для невалидного JSON
                |theme: &Theme| container::Style {
                    border: iced::border::Border {
                        color: theme.palette().danger,
                        width: 1.5,
                        radius: 5.0.into(),
                    },
                    ..container::bordered_box(theme)
                }
            })
        };

        // Кнопка запроса
        let send_button: Button<Message> = if self.is_loading {
            // Состояние загрузки
            button(
                row![
            text("⏳").size(20),
            space().width(10),
            text("Sending...").size(16),
        ]
                    .align_y(Center)
            )
                .style(button::secondary)
                .padding(15)
                .width(Fill)
        } else {
            // Обычное состояние
            button(
                row![
            text("🚀").size(20),
            space().width(10),
            text("Send Request").size(16),
        ]
                    .align_y(Center)
            )
                .on_press(Message::SendRequest)  // ← Важно: вызываем SendRequest
                .style(button::primary)
                .padding(15)
                .width(Fill)
        };

        // Секция ответа
        let response_section = {
            let title = text("Response:").size(16);

            // Явно указываем тип Element<Message>
            let content: Element<Message> = if self.is_loading {
                // Показываем индикатор загрузки
                Element::from(
                    container(
                        column![
                    text("Request in progress...").style(text::secondary),
                    space().height(10),
                    progress_bar(0.0..=100.0, 50.0),
                ]
                            .align_x(Center)
                    )
                        .padding(20)
                        .center_x(Shrink)
                )
            } else if let Some(error) = &self.response_error {
                // Показываем ошибку
                Element::from(
                    container(
                        column![
                    text("❌ Error").size(18).style(text::danger),
                    space().height(5),
                    text(error).size(14),
                ]
                            .spacing(5)
                    )
                        .padding(15)
                        .style(container::bordered_box)
                )
            } else if let Some(status) = self.response_status {
                // Показываем успешный ответ
                let status_style = match status {
                    200..=299 => text::success,
                    400..=499 => text::warning,
                    500..=599 => text::danger,
                    _ => text::default,
                };

                Element::from(
                    container(
                        column![
                    row![
                        text(format!("Status: {}", status))
                            .size(18)
                            .style(status_style),
                        space().width(20),
                        text(if status == 200 { "✅ Success" } else { "⚠️ Warning" })
                            .size(14),
                    ]
                    .align_y(Center),
                    space().height(10),
                    text("Response Body:").size(14),
                    container(
                        scrollable(
                            text(&self.response_body)
                                .size(12)
                                .font(Font::MONOSPACE)
                        )
                        //.height(200) высота тела ответа
                    )
                    .padding(10)
                    .style(container::bordered_box),
                ]
                            .spacing(10)
                    )
                        .padding(15)
                        .style(container::bordered_box)
                )
            } else {
                // Нет ответа (начальное состояние)
                Element::from(
                    container(
                        text("No response yet. Click 'Send Request' to make a call.")
                            .style(text::secondary)
                    )
                        .padding(20)
                        .center_x(Shrink)
                )
            };

            container(column![
                title,
                //space().height(10),
                content,
            ]
                .spacing(5)
                .padding(10))
                .width(Fill)
                .style(container::bordered_box)
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
            params_tables, // ← Таблички параметров
            body_section, // ← Редактор Body
            send_button,      // ← Добавляем кнопку
            response_section, // ← Добавляем ответ
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
            .max_width(810);

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

    fn get_header_suggestions(&self, input: &str) -> Vec<String> {
        if input.is_empty() {
            return Vec::new();
        }

        let input_lower = input.to_lowercase();

        COMMON_HEADERS
            .iter()
            .filter(|header| header.to_lowercase().contains(&input_lower))
            .map(|s| s.to_string())
            .take(5) // Показываем до 5 подсказок
            .collect()
    }

}

// ДОБАВЛЯЕМ асинхронную функцию (обязательно вне impl, чтоб токио её видел)
async fn send_http_request(
    method: HttpMethod,
    url: String,
    query_params: Vec<QueryParam>,
    headers: Vec<HeaderParam>,
    body_text: String,
) -> Result<(u16, String), String> {
    // Используем обычный (не blocking) клиент
    let client = reqwest::Client::new();

    // Создаем запрос в зависимости от метода
    let mut request = match method {
        HttpMethod::GET => client.get(&url),
        HttpMethod::POST => client.post(&url),
        HttpMethod::PUT => client.put(&url),
        HttpMethod::DELETE => client.delete(&url),
        HttpMethod::PATCH => client.patch(&url),
    };

    // 1. Сначала проверяем headers ДО их перемещения на наличие "content-type"
    let has_content_type = headers.iter()
        .any(|h| h.key.to_lowercase() == "content-type");

    // 2. Потом перемещаем в map query параметры
    let params_map: std::collections::HashMap<String, String> = query_params
        .into_iter()
        .map(|p| (p.key, p.value))
        .collect();

    if !params_map.is_empty() {
        request = request.query(&params_map);
    }

    // Добавляем заголовки в запрос
    for header in headers {
        request = request.header(&header.key, &header.value);
    }

    // Добавляем тело если есть и метод не GET
    // Попытка to JSON, если не получается - отправляем как текст
    if !body_text.trim().is_empty() && method != HttpMethod::GET {
        match serde_json::from_str::<serde_json::Value>(&body_text) {
            Ok(json_value) => {
                // Это валидный JSON - отправляем как JSON
                request = request.json(&json_value);

                // Автоматически добавляем Content-Type если его нет
                if !has_content_type {
                    request = request.header("Content-Type", "application/json");
                }

            }
            Err(_) => {
                // Не JSON - отправляем как обычный текст
                request = request.body(body_text);
            }
        }

    }

    // Отправляем запрос АСИНХРОННО (не блокируя UI)
    match request.send().await {
        Ok(response) => {
            let status = response.status().as_u16();
            // Тоже асинхронно читаем тело
            let body = response.text().await.unwrap_or_default();
            Ok((status, body))
        }
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}