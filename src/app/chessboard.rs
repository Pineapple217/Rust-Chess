const CHESSBOARD_SQUARE_SIZE: f32 = 4.0;

fn square_ui(ui: &mut egui::Ui, selected: &mut bool, color: egui::Color32) -> egui::Response {
    let desired_size =
        ui.spacing().interact_size.y * egui::vec2(CHESSBOARD_SQUARE_SIZE, CHESSBOARD_SQUARE_SIZE);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        *selected = !*selected;
        response.mark_changed(); // report back that the value changed
    }

    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *selected, ""));

    if ui.is_rect_visible(rect) {
        // let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *selected);
        let rect = rect.expand(visuals.expansion);
        ui.painter().rect_filled(rect, 0.0, color);
        // let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        // let center = egui::pos2(circle_x, rect.center().y);
        // ui.painter()
        //     .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }
    response
}

// fn square(selected: &mut bool) -> impl egui::Widget + '_ {
//     move |ui: &mut egui::Ui| square_ui(ui, selected)
// }

fn chessboard_ui(ui: &mut egui::Ui, selected: &mut bool) -> egui::Response {
    egui::Grid::new("some_unique_id")
        .spacing(egui::vec2(0.0, 0.0))
        .show(ui, |ui| {
            for i in 0..8 {
                for j in 0..8 {
                    if (i + j) % 2 == 0 {
                        square_ui(ui, selected, egui::Color32::from_rgb(232, 215, 167));
                    } else {
                        square_ui(ui, selected, egui::Color32::from_rgb(133, 87, 53));
                    }
                }
                ui.end_row();
            }
        });
    square_ui(ui, selected, egui::Color32::RED)
}

pub fn chessboard(selected: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| chessboard_ui(ui, selected)
}
