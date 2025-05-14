use eframe::egui;

const BBSIZE: f32 = 230.;

#[derive(Default)]
pub struct BitboardDemoApp {
    pub gui_scale: f32,

    bbs: [u64; 3],
}

impl BitboardDemoApp {
    fn draw_bb(&mut self, ui: &mut egui::Ui, index: usize) {
        ui.vertical(|ui| {
            ui.set_width(BBSIZE * self.gui_scale);
            let grid = (0..8)
                .rev()
                .map(|y| {
                    (0..8)
                        .rev()
                        .map(|x| match self.bbs[index] & 1 << (8 * y + x) != 0 {
                            true => egui::Color32::BLACK,
                            false => egui::Color32::WHITE,
                        })
                        .collect()
                })
                .collect();
            let maze_display = crate::board::BoardDisplay {
                size: BBSIZE * self.gui_scale,
                grid,
                n: 8,
            };
            let r = ui.add(maze_display);
            // TODO drag well
            if r.clicked() {
                if let Some(click_pos) = r.interact_pointer_pos() {
                    let rel_pos = (click_pos - r.rect.min) / (BBSIZE * self.gui_scale) * 8.;
                    let (x, y) = (rel_pos.x.floor() as usize, rel_pos.y.floor() as usize);
                    if x <= 7 && y <= 7 {
                        self.bbs[index] ^= 1 << (8 * (7 - y) + (7 - x));
                    }
                }
            }

            ui.horizontal_wrapped(|ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                ui.monospace(format!("{:064b}", self.bbs[index]));
            });

            ui.horizontal_wrapped(|ui| {
                if ui.button("0").clicked() {
                    self.bbs[index] = 0;
                }
                if ui.button("not").clicked() {
                    self.bbs[index] = !self.bbs[index];
                }
                if ui.button("<< 1").clicked() {
                    self.bbs[index] = self.bbs[index] << 1;
                }
                if ui.button("<< 8").clicked() {
                    self.bbs[index] = self.bbs[index] << 8;
                }
                if ui.button(">> 1").clicked() {
                    self.bbs[index] = self.bbs[index] >> 1;
                }
                if ui.button(">> 8").clicked() {
                    self.bbs[index] = self.bbs[index] >> 8;
                }
                if ui.button("left").clicked() {
                    self.bbs[index] = (self.bbs[index] & !0x8080808080808080) << 1;
                }
                if ui.button("right").clicked() {
                    self.bbs[index] = (self.bbs[index] & !0x0101010101010101) >> 1;
                }
                if ui.button("up").clicked() {
                    self.bbs[index] = self.bbs[index] << 8
                }
                if ui.button("down").clicked() {
                    self.bbs[index] = self.bbs[index] >> 8
                }
                if ui.button("|=").clicked() {
                    self.bbs[2] |= self.bbs[index];
                }
                if ui.button("&=").clicked() {
                    self.bbs[2] &= self.bbs[index];
                }
            });
        });
    }
}

impl eframe::App for BitboardDemoApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for i in 0..3 {
                    self.draw_bb(ui, i);
                }
            });
            ui.add_space(10.);
            ui.horizontal_wrapped(|ui| {
                if ui.button("or").clicked() {
                    self.bbs[2] = self.bbs[0] | self.bbs[1];
                }
                if ui.button("and").clicked() {
                    self.bbs[2] = self.bbs[0] & self.bbs[1];
                }
                if ui.button("copy a").clicked() {
                    self.bbs[0] = self.bbs[2];
                }
                if ui.button("copy b").clicked() {
                    self.bbs[1] = self.bbs[2];
                }
            });
        });
    }
}
