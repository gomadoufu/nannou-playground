use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    let slowersine = (app.time / 2.0).sin();
    let sine = slowersine;

    let boundary = app.window_rect();

    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(sine, -1.0, 1.0, boundary.bottom(), boundary.top());

    draw.background().color(PLUM);

    draw.ellipse().color(STEELBLUE).x_y(x, y);
    draw.to_frame(app, &frame).unwrap();
}
