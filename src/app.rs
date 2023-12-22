/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value_i: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 112f32,
            value_i: 112f32,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("Options", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Mouse size Calculator");
            
            /*
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });
            */
            
            ui.add(egui::Slider::new(&mut self.value, 150.0..=220.0).text("Hand size (mm)"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
            ui.heading(get_text(&self.value));

            ui.separator();

            ui.add(egui::Slider::new(&mut self.value, to_inch(&150.0)..=to_inch(&220.0)).text("Hand size (mm)"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
            ui.heading(get_text(&to_mm(&self.value)));

            ui.separator();


            ui.add(egui::github_link_file!(
                "https://github.com/jovillarrealm/Mouse-size",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

enum MouseSize<T> {
    OutOfBounds,
    ExtraSmall(T),
    Small(T),
    Medium(T),
    Large(T),
    ExtraLarge(T),
}

fn table_mm(hand_size: &f32) -> MouseSize<f32> {
    let xs = 0.0..160.0;
    let s = 160.0..172.7;
    let m = 172.8..195.7;
    let l = 195.7..213.5;
    let xl = 213.6..;

    if xs.contains(hand_size) {
        MouseSize::ExtraSmall(113.0)
    } else if s.contains(hand_size) {
        MouseSize::Small(113.0)
    } else if m.contains(hand_size) {
        dbg!("reached medium");
        MouseSize::Medium(122.5)
    } else if l.contains(hand_size) {
        MouseSize::Large(127.8)
    } else if xl.contains(hand_size) {
        MouseSize::ExtraLarge(127.8)
    } else {
        MouseSize::OutOfBounds
    }
}
fn get_text(hand_size: &f32) -> String {
    let mouse_size = table_mm(&hand_size);

    match mouse_size {
        MouseSize::ExtraSmall(v) => format!(
            "Mouse size is XS:  choose mouse sizes smaller than {} (mm) and {} inches",
            v,
            to_inch(&v)
        ),
        MouseSize::Small(v) => format!(
            "Mouse size is S: choose mouse sizes around {} (mm) and {} inches",
            v,
            to_inch(&v)
        ),
        MouseSize::Medium(v) => format!(
            "Mouse size is M: choose mouse sizes around {} (mm) and {} inches",
            v,
            to_inch(&v)
        ),
        MouseSize::Large(v) => format!(
            "Mouse size is L: choose mouse sizes around {} (mm) and {} inches",
            v,
            to_inch(&v)
        ),
        MouseSize::ExtraLarge(v) => format!(
            "Mouse size is XL:  choose mouse sizes larger than {} (mm) and {} inches",
            v,
            to_inch(&v)
        ),
        MouseSize::OutOfBounds => String::from("This is some Bullshit Value"),
    }
}
fn to_inch(measurement: &f32) -> f32 {
    measurement / 25.4
}

fn to_mm(measurement: &f32) -> f32 {
    measurement * 25.4
}