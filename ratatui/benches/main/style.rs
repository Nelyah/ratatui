use std::hint::black_box;

use criterion::{Criterion, criterion_group};
use ratatui::style::{Color, Modifier, Style};

criterion_group!(benches, style);

fn style(c: &mut Criterion) {
    patch(c);
}

fn patch(c: &mut Criterion) {
    let mut group = c.benchmark_group("style/patch");

    // Both styles have all fields set
    group.bench_function("full", |b| {
        let base = Style::default()
            .fg(Color::Red)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD);
        let overlay = Style::default()
            .fg(Color::Green)
            .bg(Color::Yellow)
            .add_modifier(Modifier::ITALIC);
        b.iter(|| black_box(black_box(base).patch(black_box(overlay))));
    });

    // Overlay only sets fg (common case: highlight color)
    group.bench_function("partial", |b| {
        let base = Style::default()
            .fg(Color::Red)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD);
        let overlay = Style::default().fg(Color::Green);
        b.iter(|| black_box(black_box(base).patch(black_box(overlay))));
    });

    // Chain of 5 patches (simulates layered styling: widget → block → paragraph → span → highlight)
    group.bench_function("chain", |b| {
        let s1 = Style::default().fg(Color::Red);
        let s2 = Style::default().bg(Color::Blue);
        let s3 = Style::default().add_modifier(Modifier::BOLD);
        let s4 = Style::default().fg(Color::Green);
        let s5 = Style::default().add_modifier(Modifier::ITALIC);
        b.iter(|| {
            black_box(
                black_box(s1)
                    .patch(black_box(s2))
                    .patch(black_box(s3))
                    .patch(black_box(s4))
                    .patch(black_box(s5)),
            )
        });
    });

    group.finish();
}
