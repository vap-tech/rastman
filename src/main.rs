use iced::widget::{
    button, center, checkbox, column, horizontal_rule, pick_list,
    row, scrollable, text, text_input, toggler, vertical_rule,
    vertical_space,
};
use iced::{Center, Element, Fill, Theme};

pub fn main() -> iced::Result {
    iced::application("Rastman - API Client", Rastman::update, Rastman::view)
        .theme(Rastman::theme)
        .run()
}

#[derive(Default)]
struct Rastman {
    theme: Theme,
    input_value: String,
    checkbox_value: bool,
    toggler_value: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    InputChanged(String),
    ButtonPressed,
    CheckboxToggled(bool),
    TogglerToggled(bool),
}

impl Rastman {
    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {}
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::TogglerToggled(value) => self.toggler_value = value,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
                .width(Fill),
        ]
            .spacing(10);

        let text_input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let button = button("Submit")
            .padding(10)
            .on_press(Message::ButtonPressed);

        let scrollable = scrollable(column![
            "Scroll me!",
            vertical_space().height(10),
            "You did it!"
        ])
            .width(Fill)
            .height(100);

        let checkbox = checkbox("Check me!", self.checkbox_value)
            .on_toggle(Message::CheckboxToggled);

        let toggler = toggler(self.toggler_value)
            .label("Toggle me!")
            .on_toggle(Message::TogglerToggled)
            .spacing(10);

        let content = column![
            choose_theme,
            horizontal_rule(38),
            row![text_input, button].spacing(10).align_y(Center),

            row![
                scrollable,
                vertical_rule(38),
                column![checkbox, toggler].spacing(20)
            ]
            .spacing(10)
            .height(100)
            .align_y(Center),
        ]
            .spacing(20)
            .padding(20)
            .max_width(600);

        center(content).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}