// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("jesterd20", options, Box::new(|_cc| Box::<App>::default()))
}

struct Player {
    name: String,
    atk_bonus: i32,
    def_bonus: i32,
}

struct App {
    player1: Player,
    player2: Player,
    dice: i32,
    // last_roll: Option<i32>,
    loops: i32,
    results: Vec<i32>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            player1: Player {
                name: "P1".to_string(),
                atk_bonus: 0,
                def_bonus: 0,
            },
            player2: Player {
                name: "P2".to_string(),
                atk_bonus: 0,
                def_bonus: 0,
            },
            dice: 20,
            loops: 5,
            // vector of results, 1 for player 1 win, -1 for player 2 win, 0 for tie
            results: vec![],
        }
    }
}

fn roll(dice: i32) -> i32 {
    let mut rng = fastrand::Rng::new();
    rng.i32(1..=dice)
}

fn calculate(player1: &Player, player2: &Player, dice: i32, loops: i32) -> Vec<i32> {
    // both players attack and defend
    // we keep track of successfully attacks and defends
    // ties favor the defender
    let mut results = vec![];
    for _ in 0..loops {
        let p1_roll = roll(dice);
        let p2_roll = roll(dice);
        let p1_atk = p1_roll + player1.atk_bonus;
        let p2_atk = p2_roll + player2.atk_bonus;
        let p1_def = p1_roll + player1.def_bonus;
        let p2_def = p2_roll + player2.def_bonus;

        // player 1 attacks
        match p1_atk.cmp(&p2_def) {
            std::cmp::Ordering::Less => results.push(-1),
            std::cmp::Ordering::Equal => results.push(1),
            std::cmp::Ordering::Greater => results.push(1),
        }

        // player 2 attacks
        match p2_atk.cmp(&p1_def) {
            std::cmp::Ordering::Less => results.push(1),
            std::cmp::Ordering::Equal => results.push(-1),
            std::cmp::Ordering::Greater => results.push(-1),
        }
    }
    results
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let name_label = ui.label("Player 1");
                ui.text_edit_singleline(&mut self.player1.name)
                    .labelled_by(name_label.id);
                ui.add(egui::Slider::new(&mut self.player1.atk_bonus, 0..=20).text("ATK"));
                ui.add(egui::Slider::new(&mut self.player1.def_bonus, 0..=20).text("DEF"));
            });
            ui.vertical(|ui| {
                let name_label = ui.label("Player 2");
                ui.text_edit_singleline(&mut self.player2.name)
                    .labelled_by(name_label.id);
                ui.add(egui::Slider::new(&mut self.player2.atk_bonus, 0..=20).text("ATK"));
                ui.add(egui::Slider::new(&mut self.player2.def_bonus, 0..=20).text("DEF"));
            });

            ui.horizontal(|ui| {
                let loops_label = ui.label("Loops");
                ui.add(egui::widgets::DragValue::new(&mut self.loops))
                    .labelled_by(loops_label.id);

                if ui.button("Calculate").clicked() {
                    self.results = calculate(&self.player1, &self.player2, self.dice, self.loops);
                }
            });

            ui.separator();
            if !self.results.is_empty() {
                // show how often P1 and P2 won, attack and defend, %
                ui.heading("Stats");
                ui.vertical(|ui| {
                    let mut p1_atk_wins = 0;
                    let mut p1_def_wins = 0;
                    let mut p2_atk_wins = 0;
                    let mut p2_def_wins = 0;

                    for result in self.results.chunks(2) {
                        match result[0] {
                            1 => p1_atk_wins += 1,
                            -1 => p2_atk_wins += 1,
                            _ => (),
                        }
                        match result[1] {
                            1 => p1_def_wins += 1,
                            -1 => p2_def_wins += 1,
                            _ => (),
                        }
                    }

                    ui.label(format!(
                        "{} won {}% of the time attacking",
                        &self.player1.name,
                        (p1_atk_wins as f32 / self.loops as f32) * 100.0
                    ));

                    ui.label(format!(
                        "{} won {:.1}% of the time defending",
                        &self.player1.name,
                        (p1_def_wins as f32 / self.loops as f32) * 100.0
                    ));

                    ui.label(format!(
                        "{} won {:.1}% of the time attacking",
                        &self.player2.name,
                        (p2_atk_wins as f32 / self.loops as f32) * 100.0
                    ));

                    ui.label(format!(
                        "{} won {:.1}% of the time defending",
                        &self.player2.name,
                        (p2_def_wins as f32 / self.loops as f32) * 100.0
                    ));
                });

                ui.heading("Results");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("results_grid")
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Iteration");
                            ui.label("P1 Attack");
                            ui.label("P2 Attack");
                            ui.end_row();

                            for (i, result) in self.results.chunks(2).enumerate() {
                                ui.label((i + 1).to_string());
                                ui.label(match result[0] {
                                    1 => format!("{} wins", &self.player1.name),
                                    -1 => format!("{} wins", &self.player2.name),
                                    _ => "Tie".to_string(),
                                });
                                ui.label(match result[1] {
                                    1 => format!("{} wins", &self.player1.name),
                                    -1 => format!("{} wins", &self.player2.name),
                                    _ => "Tie".to_string(),
                                });
                                ui.end_row();
                            }
                        });
                });
            }
        });
    }
}
