use egui_l10n::{ContextExt, Localization, langid};
use jiff::civil::date;
use metadata::{
    Parameters, Version,
    r#const::{AUTHOR, DATE, DESCRIPTION, IDENTIFIER, NAME, VERSION},
    egui::writable::Writable,
    l10n,
    rule::SPECIAL_ALPHABETICAL_RULES,
};

const GVK: &str = "Giorgi Vladimirovich Kazakov";
const RAS: &str = "Roman Alexandrovich Sidorov";

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Writable metadata UI example",
        eframe::NativeOptions::default(),
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
        parameters.push_name_value(IDENTIFIER, Some(1954));
        parameters.push_name_value(NAME, Some("VIR-2699"));
        parameters.push_name_value(DATE, Some(date(2026, 09, 11)));
        parameters.push_name_value(DATE, Some(date(2026, 09, 09)));
        parameters.push_name_value(DESCRIPTION, Some("Cat. No. 2699, Прогресс, Россия"));
        parameters.push_name_value(AUTHOR, Some(RAS));
        parameters.push_name_value(AUTHOR, Some(GVK));
        parameters.push_name_value(VERSION, Some(Version(1, 2, 3)));
        parameters.push_name_value(VERSION, Some(Version(0, 1, 2)));
        parameters.push_name_value("CustomName", Some("CustomValue"));
        parameters.filter_and_sort(&*SPECIAL_ALPHABETICAL_RULES);
        Self { parameters }
    }
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let mut writable = Writable::new(&mut self.parameters);
            writable.show(ui);
        });
    }
}
