use lambert_w::{LambertW, NEG_INV_E};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("lambert_w_plot.png", (1920, 1080)).into_drawing_area();

    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption("Lambert W function", ("sans-serif", 40).into_font())
        .set_all_label_area_size(20)
        .top_x_label_area_size(0)
        .right_y_label_area_size(0)
        .build_cartesian_2d(NEG_INV_E..10.0, -3.0..3.0)?;

    chart.configure_mesh().x_labels(20).y_labels(20).draw()?;

    let steps: u32 = 10000;
    chart
        .draw_series(LineSeries::new(
            (0..steps)
                .map(|x| f64::from(x) / f64::from(steps))
                .map(|t| t * (10.0 - NEG_INV_E) + NEG_INV_E)
                .map(|x| (x, x.lambert_w0())),
            &BLACK,
        ))?
        .label("W_0(x)")
        .legend(|(x, y)| Rectangle::new([(x - 5, y), (x + 10, y)], BLACK));

    chart
        .draw_series(LineSeries::new(
            (0..steps)
                .map(|x| f64::from(x) / f64::from(steps))
                .map(|t| NEG_INV_E - t * (NEG_INV_E + 0.15))
                .map(|x| (x, x.lambert_wm1())),
            &RED,
        ))?
        .label("W_-1(x)")
        .legend(|(x, y)| Rectangle::new([(x - 5, y), (x + 10, y)], RED));

    chart
        .configure_series_labels()
        .background_style(&BLACK.mix(0.1))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
