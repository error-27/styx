use std::usize;

use eframe::egui::{self, Color32, Frame, Id, Ui};

use crate::{launch::launch_port, traits::TabScreen, AppSettings, NamedPath};

pub struct HomePage {
    selected_port: usize,
    selected_iwad: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PwadInfo {
    col: usize,
    row: usize,
}

impl HomePage {
    fn render_pwad_cols(
        &mut self,
        cols: &mut [Ui],
        pwad_list: &Vec<NamedPath>,
        pwad_selection: &mut [Vec<usize>; 2],
    ) {
        cols[0].label("Mod Pool");
        cols[1].label("Active Mods");
        for (col_idx, column) in pwad_selection.clone().into_iter().enumerate() {
            let mut from = None;
            let mut to = None;

            let frame = Frame::default().inner_margin(4.0);

            let (_, dropped_payload) = cols[col_idx].dnd_drop_zone::<PwadInfo, ()>(frame, |ui| {
                if pwad_selection[col_idx].len() == 0 {
                    ui.label("None");
                }
                for (row_idx, item) in column.iter().enumerate() {
                    let item_id = Id::new(("pwad_list_dnd", col_idx, row_idx));
                    let item_info = PwadInfo {
                        col: col_idx,
                        row: row_idx,
                    };

                    let response = ui
                        .dnd_drag_source(item_id, item_info, |ui| {
                            ui.label(pwad_list[*item].name.clone());
                        })
                        .response;

                    if let (Some(pointer), Some(hovered_payload)) = (
                        ui.input(|i| i.pointer.interact_pos()),
                        response.dnd_hover_payload::<PwadInfo>(),
                    ) {
                        let rect = response.rect;

                        let stroke = egui::Stroke::new(1.0, Color32::WHITE);
                        let insert_row_idx = if *hovered_payload == item_info {
                            ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                            row_idx
                        } else if pointer.y < rect.center().y {
                            ui.painter().hline(rect.x_range(), rect.top(), stroke);
                            row_idx
                        } else {
                            ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                            row_idx + 1
                        };

                        if let Some(dragged_payload) = response.dnd_release_payload::<PwadInfo>() {
                            from = Some(dragged_payload);
                            to = Some(PwadInfo {
                                col: col_idx,
                                row: insert_row_idx,
                            });
                        }
                    }
                }
            });
            if let Some(dragged_payload) = dropped_payload {
                from = Some(dragged_payload);
                to = Some(PwadInfo {
                    col: col_idx,
                    row: usize::MAX,
                });
            }

            if let (Some(from), Some(mut to)) = (from, to) {
                if from.col == to.col {
                    // Adjust index if re-ordering
                    to.row -= (from.row < to.row) as usize;
                }

                let item = pwad_selection[from.col].remove(from.row);

                let c = &mut pwad_selection[to.col];
                to.row = to.row.min(c.len());
                c.insert(to.row, item);
            }
        }
    }
}

impl TabScreen for HomePage {
    fn new() -> Self {
        Self {
            selected_port: 0,
            selected_iwad: 0,
        }
    }

    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, settings: &mut AppSettings) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Selected Port")
                .selected_text({
                    if settings.ports.len() > 0 {
                        settings.ports[self.selected_port].name.clone()
                    } else {
                        "None".to_string()
                    }
                })
                .show_ui(ui, |ui| {
                    for (i, p) in settings.ports.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_port, i, p.name.clone());
                    }
                });

            egui::ComboBox::from_label("Selected Game")
                .selected_text({
                    if settings.iwads.len() > 0 {
                        settings.iwads[self.selected_iwad].name.clone()
                    } else {
                        "None".to_string()
                    }
                })
                .show_ui(ui, |ui| {
                    for (i, p) in settings.iwads.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_iwad, i, p.name.clone());
                    }
                });

            ui.columns(2, |columns| {
                self.render_pwad_cols(columns, &settings.pwads, &mut settings.pwad_selection);
            });

            if ui.button("Play").clicked() {
                launch_port(
                    settings.ports[self.selected_port].path.clone(),
                    settings.iwads[self.selected_iwad].path.clone(),
                    settings.pwad_selection[1]
                        .iter()
                        .map(|f| settings.pwads[*f].path.clone())
                        .collect(),
                );
            }
        });
    }
}
