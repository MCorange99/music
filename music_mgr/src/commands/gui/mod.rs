mod nav_bar;
mod song_edit_window;

use egui::{Color32, Label, Sense};

use crate::manifest::Manifest;

use self::song_edit_window::GuiSongEditor;



pub struct Gui {
    manifest: Manifest,
    song_editor: GuiSongEditor
}

impl Gui {
    fn new(_: &eframe::CreationContext<'_>, manifest: Manifest) -> Self {
        Self {
            manifest,
            song_editor: GuiSongEditor {
                is_open: false,
                song: Default::default(),
                ed_url: String::new(),
                ed_name: String::new(),
            },
        }
    }

    pub fn start(manifest: Manifest) -> anyhow::Result<()> {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([400.0, 300.0])
                .with_min_inner_size([300.0, 220.0]),
                // .with_icon(
                //     // NOTE: Adding an icon is optional
                //     eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                //         .expect("Failed to load icon"),
                // ),
            ..Default::default()
        };

        if let Err(e) = eframe::run_native(
            "eframe template",
            native_options,
            Box::new(|cc| Box::new(Gui::new(cc, manifest))),
        ) {
            log::error!("Failed to create window: {e}");
        };

        Ok(())
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.draw_nav(ctx, frame);
        self.draw_song_edit_window(ctx, frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(format!("Songs ({})", self.manifest.get_song_count()));

            egui::ScrollArea::vertical()
                .max_width(f32::INFINITY)
                .auto_shrink(false)
                .show(ui, |ui| {
                for (genre, songs) in  self.manifest.get_genres() {
                    for (song_name, song) in songs {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.label("[");
                            ui.hyperlink_to("link", song.get_url().unwrap());
                            ui.label("] ");
                            ui.colored_label(Color32::LIGHT_BLUE, genre);
                            ui.label(": ");
                            if ui.add(Label::new(song_name).sense(Sense::click())).clicked() {
                                self.song_editor.song = (
                                    genre.clone(),
                                    song_name.clone(),
                                );
                                log::debug!("Label pressed");
                                self.song_editor.is_open = true;
                                self.song_editor.ed_name = song_name.clone();
                                self.song_editor.ed_url = song.get_url_str().clone();
                            }
                        });
                        // ui.label(RichText::new(""))
                    }
                }
            });

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}