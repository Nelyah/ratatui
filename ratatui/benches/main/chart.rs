use criterion::{BatchSize, Bencher, BenchmarkId, Criterion, criterion_group};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Axis, Block, Chart, Dataset, GraphType, Widget};

criterion_group!(benches, chart);

fn chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("chart");

    for data_count in [64, 256, 2048] {
        let data: Vec<(f64, f64)> = (0..data_count)
            .map(|i| {
                let x = i as f64;
                let y = (x * 0.1).sin() * 50.0 + 50.0;
                (x, y)
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("render", data_count),
            &data,
            |b, data| {
                let datasets = vec![Dataset::default()
                    .name("data")
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Color::Cyan))
                    .data(data)];
                let chart = Chart::new(datasets)
                    .block(Block::bordered().title("Chart"))
                    .x_axis(Axis::default().bounds([0.0, data_count as f64]))
                    .y_axis(Axis::default().bounds([0.0, 100.0]));
                render(b, &chart);
            },
        );
    }

    // Multi-dataset benchmark
    let data1: Vec<(f64, f64)> = (0..256)
        .map(|i| (i as f64, (i as f64 * 0.1).sin() * 50.0 + 50.0))
        .collect();
    let data2: Vec<(f64, f64)> = (0..256)
        .map(|i| (i as f64, (i as f64 * 0.15).cos() * 40.0 + 50.0))
        .collect();
    let data3: Vec<(f64, f64)> = (0..256)
        .map(|i| (i as f64, (i as f64 * 0.05).sin() * 30.0 + 50.0))
        .collect();

    group.bench_function("render_multi_dataset/3x256", |b| {
        let datasets = vec![
            Dataset::default()
                .name("sin")
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Cyan))
                .data(&data1),
            Dataset::default()
                .name("cos")
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Red))
                .data(&data2),
            Dataset::default()
                .name("mixed")
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Green))
                .data(&data3),
        ];
        let chart = Chart::new(datasets)
            .block(Block::bordered().title("Multi"))
            .x_axis(Axis::default().bounds([0.0, 256.0]))
            .y_axis(Axis::default().bounds([0.0, 100.0]));
        render(b, &chart);
    });

    group.finish();
}

fn render(bencher: &mut Bencher, chart: &Chart) {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 50));
    bencher.iter_batched(
        || chart.clone(),
        |bench_chart| {
            bench_chart.render(buffer.area, &mut buffer);
        },
        BatchSize::LargeInput,
    );
}
