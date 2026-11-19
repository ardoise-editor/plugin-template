//! A plugin for Ardoise: a command that says hello in the status bar,
//! and a panel that asks your name.

use ardoise_plugin::protocol::{Context, InitializeParams, UiEvent};
use ardoise_plugin::ui::*;
use ardoise_plugin::{run, Host, Plugin};

#[derive(Default)]
struct Hello {
    name: String,
    french: bool,
}

impl Hello {
    fn greeting(&self) -> String {
        let name = match (self.name.is_empty(), self.french) {
            (false, _) => self.name.as_str(),
            (true, true) => "le monde",
            (true, false) => "world",
        };
        if self.french {
            format!("Bonjour, {name} !")
        } else {
            format!("Hello, {name}!")
        }
    }

    fn view(&self) -> Node {
        let (title, hint, label) = if self.french {
            ("Bonjour", "Votre nom", "Dire bonjour")
        } else {
            ("Hello", "Your name", "Say hello")
        };
        column([
            text(title).style(TextStyle::Heading),
            input("name", &self.name, hint).on_submit("greet"),
            button(label, "greet").button_style(ButtonStyle::Primary),
        ])
        .spacing(8.0)
    }
}

impl Plugin for Hello {
    fn initialize(&mut self, _: &Host, params: &InitializeParams) {
        self.french = params.settings.locale.starts_with("fr");
    }

    fn command(&mut self, host: &Host, command: &str, _: &Context) {
        if command == "hello.greet" {
            host.notify(self.greeting());
        }
    }

    fn panel_opened(&mut self, host: &Host, panel: &str) {
        host.render(panel, self.view());
    }

    fn ui_event(&mut self, host: &Host, event: &UiEvent) {
        match event.action.as_str() {
            "name" => self.name = event.text().unwrap_or_default().to_string(),
            "greet" => host.notify(self.greeting()),
            _ => {}
        }
    }

    fn settings_changed(&mut self, host: &Host, settings: &ardoise_plugin::protocol::Settings) {
        self.french = settings.locale.starts_with("fr");
        host.render("hello", self.view());
    }
}

fn main() -> std::io::Result<()> {
    run(Hello::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_greets_by_name_in_the_language_of_the_editor() {
        let mut hello = Hello::default();
        assert_eq!(hello.greeting(), "Hello, world!");
        hello.french = true;
        assert_eq!(hello.greeting(), "Bonjour, le monde !");
        hello.name = "Ada".into();
        hello.french = true;
        assert_eq!(hello.greeting(), "Bonjour, Ada !");
    }
}
