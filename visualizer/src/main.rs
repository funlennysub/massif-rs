use eframe::egui::{CentralPanel, Context};
use eframe::{Frame, NativeOptions};
use egui_plot::{HLine, Line, LineStyle, Plot, PlotPoints, VLine};
use parser::Output;
use size::Size;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let path = env::args().collect::<Vec<_>>();
    let path = path.clone()[1].to_owned();
    let output = parser::read_file(&path);

    eframe::run_native(
        "massif visualizer",
        NativeOptions::default(),
        Box::new(|_| Box::new(App::new(output))),
    )
    .unwrap()
}

struct App {
    output: Output,
}

impl App {
    fn new(file: Output) -> Self {
        Self { output: file }
    }
}

fn micros_to_human(micros: f64) -> String {
    let duration = Duration::from_micros(micros as u64);
    let secs = duration.as_secs();
    let millis = duration.as_millis();

    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    match (hours, minutes, seconds, millis, micros) {
        (h, _, _, _, _) if h > 0 => format!("{}h", h),
        (0, m, _, _, _) if m > 0 => format!("{}m", m),
        (0, 0, s, _, _) if s > 0 => format!("{}s", s),
        (0, 0, 0, ms, _) if ms > 0 => format!("{}ms", ms),
        (0, 0, 0, 0, micros) if micros >= 0.0 => format!("{}μs", micros),
        _ => "NaN".to_string(),
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let plot_points: PlotPoints = self
                .output
                .snapshots
                .iter()
                .map(|s| [s.time.as_micros() as f64, s.mem_heap.bytes() as f64])
                .collect();
            let line = Line::new(plot_points).fill(0.0).width(3.0);
            let time_lines: Vec<_> = self
                .output
                .snapshots
                .iter()
                .map(|s| {
                    Line::new(PlotPoints::new(vec![
                        [s.time.as_micros() as f64, 0f64],
                        [s.time.as_micros() as f64, s.mem_heap.bytes() as f64],
                    ]))
                    .width(1.0)
                    .color(eframe::egui::Color32::LIGHT_RED)
                })
                .collect();
            Plot::new("my_plot")
                .view_aspect(2.0)
                .allow_scroll(false)
                .x_axis_formatter(|mark, _digits, _range| micros_to_human(mark.value))
                .y_axis_formatter(|mark, _digits, _range| Size::from_bytes(mark.value).to_string())
                .allow_drag(false)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                    for tl in time_lines {
                        plot_ui.line(tl);
                    }
                });
        });
    }
}
