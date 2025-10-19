use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();

    options.centered = true;
    options.viewport.fullscreen = Some(false);
    options.viewport.resizable = Some(false);

    eframe::run_native(
        "My first App!", 
        options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        })
    )
}

#[derive(Default)]
struct MyApp {
    age: u32,
    check: bool
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Don't click me!").clicked() {
                std::process::exit(0);
            }

            ui.checkbox(&mut self.check, "Do you washh your hands today?");
            
            ui.add(egui::DragValue::new(&mut self.age).speed(0.1));

            ui.hyperlink_to("don't click me", "https://youtu.be/5NQ20M-B7Z8?si=o31ZI1zY8pn2Oz7c");
            ui.add(
                egui::Image::new(egui::include_image!("../assets/ferris.png"))
                .corner_radius(5)
            );


        });
    }
}