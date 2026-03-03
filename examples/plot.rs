// Copyright 2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example is intended to be ran more than studied.
//! It generates a plot of the two branches of the function
//! and saves it as a png file.

use kuva::{
    backend::svg::SvgBackend,
    plot::LinePlot,
    render::{layout::Layout, plots::Plot, render::render_multiple},
};
use lambert_w::{lambert_w0, lambert_wm1, NEG_INV_E};

fn main() {
    const X_MAX: f64 = 5.0;
    const X_MIN: f64 = NEG_INV_E;
    const NUM_PLOT_POINTS: u32 = 1000;

    let x_coords: Vec<f64> = (0..NUM_PLOT_POINTS)
        // Map to the range [0, 1].
        .map(|i| f64::from(i) / f64::from(NUM_PLOT_POINTS - 1))
        // Map to the range [X_MIN, X_MAX].
        .map(|t| t * (X_MAX - X_MIN) + X_MIN)
        .collect();

    let plots = vec![
        Plot::Line(
            LinePlot::new()
                .with_data(x_coords.iter().map(|&x| (x, lambert_w0(x))))
                .with_color("black")
                .with_stroke_width(3.0)
                .with_legend("Principal branch, k=0"),
        ),
        Plot::Line(
            LinePlot::new()
                .with_data(x_coords.into_iter().map(|x| (x, lambert_wm1(x))))
                .with_color("red")
                .with_stroke_width(3.0)
                .with_legend("Secondary branch, k=-1"),
        ),
    ];

    let layout = Layout::auto_from_plots(&plots)
        .with_title("Lambert W function")
        .with_x_label("x")
        .with_y_label("y");

    let scene = render_multiple(plots, layout);
    let svg = SvgBackend.render_scene(&scene);

    std::fs::write("lambert_w_plot.svg", &svg).unwrap();
}
