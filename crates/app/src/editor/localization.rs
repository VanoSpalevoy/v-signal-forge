#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Language {
    English,
    Russian,
}

#[derive(Clone, Copy)]
pub(super) enum Text {
    Language,
    Tools,
    Settings,
    Nodes,
    Debug,
    Ready,
    SelectedSourceSocket,
    Source,
    Gain,
    Sink,
    Input,
    Output,
    // EnableGrid,
    Format,
}

impl Language {
    pub(super) const ALL: [Self; 2] = [Self::English, Self::Russian];

    pub(super) fn native_name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Russian => "Русский",
        }
    }

    pub(super) fn text(self, key: Text) -> &'static str {
        match (self, key) {
            (Self::English, Text::Language) => "Language",
            (Self::English, Text::Tools) => "Tools",
            (Self::English, Text::Settings) => "Settings",
            (Self::English, Text::Nodes) => "Nodes",
            (Self::English, Text::Debug) => "Debug",
            (Self::English, Text::Ready) => "Ready",
            (Self::English, Text::SelectedSourceSocket) => "Selected source socket: node",
            (Self::English, Text::Source) => "Source",
            (Self::English, Text::Gain) => "Gain",
            (Self::English, Text::Sink) => "Sink",
            (Self::English, Text::Input) => "Input",
            (Self::English, Text::Output) => "Output",
            // (Self::English, Text::EnableGrid) => "Enable Grid",
            (Self::English, Text::Format) => "Format",
            (Self::Russian, Text::Language) => "Язык",
            (Self::Russian, Text::Tools) => "Инструменты",
            (Self::Russian, Text::Settings) => "Настройки",
            (Self::Russian, Text::Nodes) => "Ноды",
            (Self::Russian, Text::Debug) => "Отладка",
            (Self::Russian, Text::Ready) => "Готово",
            (Self::Russian, Text::SelectedSourceSocket) => "Выбран выходной сокет ноды",
            (Self::Russian, Text::Source) => "Источник",
            (Self::Russian, Text::Gain) => "Усиление",
            (Self::Russian, Text::Sink) => "Приёмник",
            (Self::Russian, Text::Input) => "Вход",
            (Self::Russian, Text::Output) => "Выход",
            // (Self::Russian, Text::EnableGrid) => "Включить сетку",
            (Self::Russian, Text::Format) => "Форматировать",
        }
    }
}
