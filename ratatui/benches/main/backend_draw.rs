use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group};
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Cell;
use ratatui::prelude::Backend;
use ratatui::style::{Color, Modifier, Style};

criterion_group!(benches, backend_draw);

fn backend_draw(c: &mut Criterion) {
    sequential_cells(c);
    scattered_cells(c);
    styled_cells(c);
}

/// Sequential cell positions: backend can skip MoveTo commands.
fn sequential_cells(c: &mut Criterion) {
    let mut group = c.benchmark_group("backend_draw/sequential_cells");

    for count in [1000u16, 10000] {
        let cell = Cell::new("x");
        let cells: Vec<(u16, u16, Cell)> = (0..count)
            .map(|i| {
                let x = i % 200;
                let y = i / 200;
                (x, y, cell.clone())
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            &cells,
            |b, cells| {
                b.iter_batched(
                    || CrosstermBackend::new(Vec::with_capacity(64 * 1024)),
                    |mut backend| {
                        backend
                            .draw(cells.iter().map(|(x, y, c)| (*x, *y, c)))
                            .unwrap();
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Scattered cell positions: every cell requires a MoveTo command.
fn scattered_cells(c: &mut Criterion) {
    let mut group = c.benchmark_group("backend_draw/scattered_cells");

    for count in [1000u16, 10000] {
        let cell = Cell::new("x");
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let cells: Vec<(u16, u16, Cell)> = (0..count)
            .map(|_| {
                let x = rng.random_range(0..200u16);
                let y = rng.random_range(0..50u16);
                (x, y, cell.clone())
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            &cells,
            |b, cells| {
                b.iter_batched(
                    || CrosstermBackend::new(Vec::with_capacity(64 * 1024)),
                    |mut backend| {
                        backend
                            .draw(cells.iter().map(|(x, y, c)| (*x, *y, c)))
                            .unwrap();
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Cells with varying styles: tests color/modifier change overhead.
fn styled_cells(c: &mut Criterion) {
    let mut group = c.benchmark_group("backend_draw/styled_cells");

    let colors = [
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Yellow,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::Rgb(128, 64, 32),
    ];
    let modifiers = [
        Modifier::BOLD,
        Modifier::ITALIC,
        Modifier::UNDERLINED,
        Modifier::empty(),
    ];

    let count = 10000u16;
    let cells: Vec<(u16, u16, Cell)> = (0..count)
        .map(|i| {
            let x = i % 200;
            let y = i / 200;
            let mut cell = Cell::new("x");
            cell.set_style(
                Style::default()
                    .fg(colors[i as usize % colors.len()])
                    .bg(colors[(i as usize + 3) % colors.len()])
                    .add_modifier(modifiers[i as usize % modifiers.len()]),
            );
            (x, y, cell)
        })
        .collect();

    group.bench_with_input(
        BenchmarkId::from_parameter(count),
        &cells,
        |b, cells| {
            b.iter_batched(
                || CrosstermBackend::new(Vec::with_capacity(64 * 1024)),
                |mut backend| {
                    backend
                        .draw(cells.iter().map(|(x, y, c)| (*x, *y, c)))
                        .unwrap();
                },
                BatchSize::SmallInput,
            );
        },
    );

    group.finish();
}
