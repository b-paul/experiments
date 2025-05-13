use eframe::egui;

/// A display of a maze, as an egui widget.
#[derive(Debug)]
pub struct BoardDisplay {
    pub size: f32,
    pub grid: Vec<Vec<egui::Color32>>,
    pub n: usize,
}

impl egui::widgets::Widget for BoardDisplay {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(self.size), egui::Sense::click());

        let painter = ui.painter_at(rect);

        for x in 0..self.n {
            for y in 0..self.n {
                let colour = self.grid[y][x];
                let min = rect.min
                    + egui::vec2(
                        self.size / self.n as f32 * x as f32,
                        self.size / self.n as f32 * y as f32,
                    );
                let max = min + egui::vec2(self.size / self.n as f32, self.size / self.n as f32);
                let square = egui::Rect { min, max };
                let shape = egui::Shape::rect_filled(square, 0., colour);
                painter.add(shape);
            }
        }

        response
    }
}
