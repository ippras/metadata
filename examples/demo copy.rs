use egui_l10n::{ContextExt, Localization, langid};
use jiff::civil::date;
use metadata::{
    Parameters, Version,
    r#const::{AUTHOR, DATE, DESCRIPTION, IDENTIFIER, NAME, VERSION},
    egui::readable::{READABLE_RULES, Readable},
    l10n,
};
// egui::writable::Writable,

const GVK: &str = "Giorgi Vladimirovich Kazakov";
const RAS: &str = "Roman Alexandrovich Sidorov";

fn main() -> eframe::Result<()> {
    // Настройки окна
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    // Запускаем приложение
    eframe::run_native(
        "Writable UI Demo",
        options,
        Box::new(|creation_context| Ok(Box::new(DemoApp::new(creation_context)))),
    )
}

// Наше тестовое приложение, которое хранит состояние
struct DemoApp {
    parameters: Parameters,
}

impl DemoApp {
    fn new(creation_context: &eframe::CreationContext) -> Self {
        // Customize style of egui.
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        creation_context.egui_ctx.set_fonts(fonts);
        creation_context.egui_ctx.set_localization(
            langid!("en"),
            Localization::new(langid!("en")).with_sources(l10n::EN),
        );
        creation_context
            .egui_ctx
            .set_language_identifier(langid!("en"));

        let mut parameters = Parameters::new();
        parameters.push(IDENTIFIER, Some(1954));
        parameters.push(NAME, Some("VIR-2699"));
        parameters.push(DATE, Some(date(2026, 09, 11)));
        parameters.push(DATE, Some(date(2026, 09, 09)));
        parameters.push(DESCRIPTION, Some("Cat. No. 2699, Прогресс, Россия"));
        parameters.push(AUTHOR, Some(RAS));
        parameters.push(AUTHOR, Some(GVK));
        parameters.push(VERSION, Some(Version(1, 2, 3)));
        parameters.push(VERSION, Some(Version(0, 1, 2)));
        parameters.push("CustomName", Some("CustomValue"));
        parameters.filter_and_sort(&*READABLE_RULES);
        Self { parameters }
    }
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Тестирование виджета Writable");
            ui.separator();

            // Отрисовываем ваш виджет
            let readable = Readable::new(&mut self.parameters);
            readable.show(ui);
            // let mut writable = Writable::new(&mut self.parameters);
            // writable.show(ui);

            ui.separator();

            // Для отладки: показываем, как выглядят данные под капотом
            ui.heading("Текущее состояние (Debug):");
            ui.label(format!("{:#?}", self.parameters));
        });
    }
}
