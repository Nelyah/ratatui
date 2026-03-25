use std::hint::black_box;

use criterion::{Criterion, criterion_group};
use ratatui::buffer::{Cell, CellWidth};
use ratatui::style::{Color, Modifier, Style};

criterion_group!(benches, cell);

fn cell(c: &mut Criterion) {
    set_symbol(c);
    set_style(c);
    cell_width(c);
    reset(c);
    eq(c);
}

fn set_symbol(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/set_symbol");

    group.bench_function("ascii", |b| {
        let mut cell = Cell::default();
        b.iter(|| {
            cell.set_symbol(black_box("a"));
        });
    });

    group.bench_function("cjk", |b| {
        let mut cell = Cell::default();
        b.iter(|| {
            cell.set_symbol(black_box("漢"));
        });
    });

    // ZWJ sequence forces heap allocation in CompactString
    group.bench_function("emoji_zwj", |b| {
        let mut cell = Cell::default();
        b.iter(|| {
            cell.set_symbol(black_box("👨\u{200D}👩\u{200D}👧\u{200D}👦"));
        });
    });

    group.finish();
}

fn set_style(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/set_style");
    let style = Style::default()
        .fg(Color::Red)
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD | Modifier::ITALIC);

    group.bench_function("full", |b| {
        let mut cell = Cell::default();
        b.iter(|| {
            cell.set_style(black_box(style));
        });
    });

    group.finish();
}

fn cell_width(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/cell_width");

    group.bench_function("ascii", |b| {
        let cell = Cell::new("a");
        b.iter(|| black_box(black_box(&cell).cell_width()));
    });

    group.bench_function("cjk", |b| {
        let cell = Cell::new("漢");
        b.iter(|| black_box(black_box(&cell).cell_width()));
    });

    group.finish();
}

fn reset(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/reset");

    group.bench_function("default", |b| {
        let mut cell = Cell::new("x");
        cell.set_style(
            Style::default()
                .fg(Color::Red)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );
        b.iter(|| {
            black_box(&mut cell).reset();
        });
    });

    group.finish();
}

fn eq(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/eq");

    group.bench_function("identical", |b| {
        let a = Cell::new("x");
        let a2 = a.clone();
        b.iter(|| black_box(black_box(&a) == black_box(&a2)));
    });

    group.bench_function("different", |b| {
        let a = Cell::new("x");
        let mut d = Cell::new("y");
        d.set_style(Style::default().fg(Color::Red));
        b.iter(|| black_box(black_box(&a) == black_box(&d)));
    });

    group.finish();
}
