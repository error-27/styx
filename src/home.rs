use eframe::egui::{self, Color32, Frame, Id, TextBuffer, Ui};

use crate::{launch::launch_port, traits::TabScreen, AppSettings, NamedPath};

pub struct HomePage {
    selected_port: usize,
    selected_iwad: usize,
    complevel: isize,
    custom_cl: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PwadInfo {
    col: usize,
    row: usize,
}

const COMPLEVEL_STRINGS: [&str; 22] = [
    "Doom v1.2",
    "Doom v1.666",
    "Doom v1.9 / Doom 2",
    "The Ultimate Doom",
    "Final Doom",
    "DOSDoom",
    "TASDoom",
    "Boom (Broken Vanilla Compat)",
    "Boom v2.01",
    "Boom v2.02",
    "LxDoom",
    "MBF",
    "PrBoom v2.03beta",
    "PrBoom v2.1.0",
    "PrBoom v2.1.1 - 2.2.6",
    "PrBoom v2.3.x",
    "PrBoom v2.4.0",
    "Latest Version",
    "",
    "",
    "",
    "MBF21",
];

impl HomePage {
    fn render_pwad_cols(
        &mut self,
        cols: &mut [Ui],
        pwad_list: &Vec<NamedPath>,
        pwad_selection: &mut [Vec<usize>; 2],
    ) {
        // Most of this is taken from the official egui examples
        cols[0].label("Mod Pool");
        cols[1].label("Active Mods");
        for (col_idx, column) in pwad_selection.clone().into_iter().enumerate() {
            let mut from = None;
            let mut to = None;

            let frame = Frame::default().inner_margin(4.0);

            let (_, dropped_payload) = cols[col_idx].dnd_drop_zone::<PwadInfo, ()>(frame, |ui| {
                // Render placeholder when no mods in this column
                if pwad_selection[col_idx].len() == 0 {
                    ui.label("None");
                }

                // Render each item of a column, and make them interactable
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
            complevel: -1,
            custom_cl: false,
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

            // Mod selection area
            ui.columns(2, |columns| {
                self.render_pwad_cols(columns, &settings.pwads, &mut settings.pwad_selection);
            });

            // Complevel selector
            if !self.custom_cl {
                if self.complevel > 21 || [18, 19, 20].contains(&self.complevel) {
                    self.complevel = -1;
                }
                egui::ComboBox::from_label("Compatibility Level")
                    .selected_text({
                        if self.complevel > -1 {
                            format!(
                                "{}: {}",
                                self.complevel, COMPLEVEL_STRINGS[self.complevel as usize]
                            )
                        } else {
                            "Default".to_string()
                        }
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.complevel, -1, "Default");
                        for (i, c) in COMPLEVEL_STRINGS.into_iter().enumerate() {
                            if c == "" {
                                continue;
                            }
                            ui.selectable_value(
                                &mut self.complevel,
                                i as isize,
                                format!("{}: {}", i, c),
                            );
                        }
                    });
            } else {
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut self.complevel).speed(0.1));
                    ui.label("Compatibility Level");
                });
            }

            ui.checkbox(&mut self.custom_cl, "Custom Comp Level");

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
