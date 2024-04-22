
use egui::Color32;

use crate::manifest::{GenreName, SongName};

use super::Gui;

pub struct GuiSongEditor {
    pub is_open: bool,
    pub song: (GenreName, SongName),
    pub ed_url: String,
    pub ed_name: String,
}

impl Gui {
    pub fn draw_song_edit_window(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut save = false;
        
        let (genre, song_name) = self.song_editor.song.clone();

        let Some(song) = self.manifest.get_song(genre.clone(), &song_name) else {
            return;
        };
        let song = song.clone();

        egui::Window::new("Song editor")
            .open(&mut self.song_editor.is_open)
            .show(ctx, 
            |ui| {

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("[");
                ui.hyperlink_to("link", song.get_url().unwrap());
                ui.label("] ");
                ui.colored_label(Color32::LIGHT_BLUE, &genre);
                ui.label(": ");
                ui.label(&song_name)
            });

            ui.horizontal(|ui| {
                ui.label("Type: ");
                ui.label(&song.get_type().to_string());
            });

            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.text_edit_singleline(&mut self.song_editor.ed_name);
            });
            ui.horizontal(|ui| {
                ui.label("Url: ");
                ui.text_edit_singleline(&mut self.song_editor.ed_url);
            });

            if ui.button("Save").clicked() {
                save = true;
            }
        });

        if save {
            {
                let Some(song) = self.manifest.get_song_mut(genre.clone(), &song_name) else {
                    return;
                };
                
                *song.get_url_str_mut() = self.song_editor.ed_url.clone();
            }

            let Some(genre) = self.manifest.get_genre_mut(genre.clone()) else {
                return;
            };


            genre.remove(&song_name);
            genre.insert(self.song_editor.ed_name.clone(), song);
            let _ = self.manifest.save(None);
        }
    }
}