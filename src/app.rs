use std::sync::mpsc::SyncSender;

use egui::{Color32, Frame};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Default)]
pub struct Choice {
    pub title: String,
    pub explanation: String,
}

#[derive(Clone,Default, serde::Deserialize, serde::Serialize)]
pub enum Rating {
    #[default] Neutral,
    Up,
    Down,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Default)]
pub struct Criteria {
    pub title: String,
    pub choices: Vec<Choice>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    #[serde(skip)]
    pub criterias: Vec<Criteria>,
    pub selected_criteria: usize,
    pub answers: Vec<Vec<Rating>>,
}

impl Default for App {
    fn default() -> Self {
        let criterias: Vec<Criteria> =
            serde_json::from_slice(include_bytes!("../assets/criteria.json")).unwrap();
        Self {
            criterias,
            selected_criteria: 0,
            answers: vec![],
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.selected_criteria >= self.criterias.len() {
            self.selected_criteria = 0;
        }
        if self.answers.len() != self.criterias.len() {
            self.answers = vec![vec![Rating::Neutral]; self.criterias.len()];
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(criteria) = self.criterias.get(self.selected_criteria) {
                let answers = self.answers.get_mut(self.selected_criteria).unwrap();
                if answers.len() != criteria.choices.len() {
                    *answers = vec![Rating::Neutral; self.criterias.len()];
                }

                ui.label(format!(
                    "question {}/{}",
                    self.selected_criteria,
                    self.criterias.len() - 1
                ));
                ui.heading(&criteria.title);
                for (i, choice) in criteria.choices.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let color = match answers[i] {
                            Rating::Up => Color32::DARK_GREEN,
                            Rating::Down => Color32::DARK_RED,
                            Rating::Neutral => Color32::DARK_GRAY,
                        };
                        let group = Frame::group(ui.style()).fill(color).show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.heading(&choice.title);
                                ui.label(&choice.explanation);
                            });
                        });
                        if ui
                            .interact(group.response.rect, ui.next_auto_id(), egui::Sense::click())
                            .clicked()
                        {
                            let ans = match answers[i] {
                                Rating::Up => Rating::Down,
                                Rating::Down => Rating::Neutral,
                                Rating::Neutral => Rating::Up,
                            };
                            self.answers[self.selected_criteria][i] = ans;
                        }
                    });
                }
            }
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("previous").clicked() {
                    if self.selected_criteria > 1 {
                        self.selected_criteria -= 1;
                    }
                }
                if ui.button("next").clicked() {
                    if self.selected_criteria < self.criterias.len() - 1 {
                        self.selected_criteria += 1;
                    }
                }
            });
        });
    }
}
