use eframe::{egui, epi};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum LeftPanel {
    Page,
    Component,
    Element,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    left_panel: LeftPanel,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            left_panel: LeftPanel::Page,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Design Master"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        setup_custom_fonts(_ctx);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { left_panel } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("side_choose_panel")
                    .resizable(false)
                    .min_height(0.0)
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.selectable_value(left_panel, LeftPanel::Page, "页面");
                            ui.selectable_value(left_panel, LeftPanel::Component, "组件");
                            ui.selectable_value(left_panel, LeftPanel::Element, "元素");
                        });
                    });

                match left_panel {
                    LeftPanel::Page => {
                        ui.heading("页面列表");
                    }
                    LeftPanel::Component => {
                        ui.heading("组件列表");
                    }
                    LeftPanel::Element => {
                        ui.heading("元素列表");
                    }
                }
            });

        egui::SidePanel::right("right_bar")
            .default_width(350.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("组件属性");
            });


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("设计大师");
            ui.end_row();
            ui.label("像大师一样设计：");
            ui.end_row();
            ui.label("高复用 -- 直接使用他人写好的组件/自定义组件");
            ui.end_row();
            ui.label("原型即代码 -- 直接导出Web/桌面/小程序/手机组件代码");
            ui.end_row();
            ui.label("高性能 -- 立即响应");
            ui.end_row();
            ui.label("低存储 -- 导出文件只需几十K而不是几个G");
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/wqy-microhei.ttc")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}