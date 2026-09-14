use jiff::civil::date;
use metadata::{
    Parameter, Parameters, Version,
    r#const::{AUTHOR, DATE, DESCRIPTION, NAME, VERSION},
    egui::writable::Writable,
};

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

        // return Default::default();
        // creation_context
        //     .storage
        //     .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
        //     .unwrap_or_default()

        let mut parameters = Parameters::new();
        parameters.push(NAME, Some("VIR-2699"));
        parameters.push(DATE, Some(date(2026, 09, 11)));
        parameters.push(DATE, Some(date(2026, 09, 09)));
        parameters.push(DESCRIPTION, Some("Cat. No. 2699, Прогресс, Россия"));
        parameters.push(AUTHOR, Some(GVK));
        parameters.push(AUTHOR, Some(RAS));
        parameters.push(VERSION, Some("1.2.3"));
        parameters.push(VERSION, Some("0.1.2"));
        parameters.push("CustomName", Some("CustomValue"));
        parameters.sort();
        Self { parameters }
    }

    // fn default() -> Self {
    //     // Создадим пару параметров для наглядности при старте
    //     let mut parameters = Parameters::default();
    //     parameters.push(Parameter {
    //         name: "API_KEY".to_string(),
    //         value: Some("12345-abcde".to_string()),
    //     });
    //     parameters.push(Parameter {
    //         name: "DEBUG_MODE".to_string(),
    //         value: None,
    //     });

    //     Self { parameters }
    // }
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Тестирование виджета Writable");
            ui.separator();

            // Отрисовываем ваш виджет
            let mut writable = Writable::new(&mut self.parameters);
            writable.show(ui);

            ui.separator();

            // Для отладки: показываем, как выглядят данные под капотом
            ui.heading("Текущее состояние (Debug):");
            ui.label(format!("{:#?}", self.parameters));
        });
    }
}
