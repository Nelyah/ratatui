use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

criterion_group!(benches, buffer_ops);

fn buffer_ops(c: &mut Criterion) {
    set_string_ascii(c);
    set_string_unicode(c);
    set_line_styled(c);
    set_style(c);
    merge(c);
}

/// ASCII text rendering via set_string.
fn set_string_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_ops/set_string_ascii");
    let text = "The quick brown fox jumps over the lazy dog. Pack my box with five dozen liquor jugs!!";
    let area = Rect::new(0, 0, 200, 50);
    let style = Style::default().fg(Color::White).bg(Color::Black);
    group.bench_function("200x50", |b| {
        let mut buffer = Buffer::empty(area);
        b.iter(|| {
            for y in 0..50u16 {
                buffer.set_string(black_box(0), black_box(y), black_box(text), style);
            }
        });
    });
    group.finish();
}

/// CJK text (double-width graphemes) exercises unicode width calculation.
fn set_string_unicode(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_ops/set_string_unicode");
    let text = "漢字テスト東京都渋谷区日本語全角文字混在テスト用文字列ゼロ幅スペース結合";
    let area = Rect::new(0, 0, 200, 50);
    let style = Style::default();
    group.bench_function("200x50", |b| {
        let mut buffer = Buffer::empty(area);
        b.iter(|| {
            for y in 0..50u16 {
                buffer.set_string(black_box(0), black_box(y), black_box(text), style);
            }
        });
    });
    group.finish();
}

/// set_line with multiple styled spans (common widget rendering path).
fn set_line_styled(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_ops/set_line_styled");
    let line = Line::from(vec![
        Span::styled("Hello ", Style::default().fg(Color::Red)),
        Span::styled("beautiful ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::styled("world ", Style::default().fg(Color::Blue).add_modifier(Modifier::ITALIC)),
        Span::styled("from ", Style::default().fg(Color::Yellow)),
        Span::styled("ratatui!", Style::default().fg(Color::Magenta).add_modifier(Modifier::UNDERLINED)),
    ]);
    let area = Rect::new(0, 0, 200, 50);
    group.bench_function("200x50", |b| {
        let mut buffer = Buffer::empty(area);
        b.iter(|| {
            for y in 0..50u16 {
                buffer.set_line(black_box(0), black_box(y), black_box(&line), 200);
            }
        });
    });
    group.finish();
}

/// set_style applies a style to an entire region.
fn set_style(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_ops/set_style");
    let style = Style::default()
        .fg(Color::Red)
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD | Modifier::ITALIC);
    for (w, h) in [(80, 24), (200, 50), (256, 256)] {
        let area = Rect::new(0, 0, w, h);
        group.bench_with_input(BenchmarkId::from_parameter(format!("{w}x{h}")), &area, |b, &area| {
            let mut buffer = Buffer::empty(area);
            b.iter(|| {
                buffer.set_style(black_box(area), style);
            });
        });
    }
    group.finish();
}

/// merge overlays one buffer onto another (used by Block widget).
fn merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_ops/merge");
    let base_area = Rect::new(0, 0, 200, 50);
    let overlay_area = Rect::new(10, 5, 100, 25);
    let mut overlay = Buffer::empty(overlay_area);
    // Fill overlay with content
    for y in overlay_area.top()..overlay_area.bottom() {
        overlay.set_string(
            overlay_area.x,
            y,
            "overlay content that fills up the buffer row",
            Style::default().fg(Color::Green),
        );
    }
    group.bench_function("200x50", |b| {
        let mut base = Buffer::empty(base_area);
        b.iter(|| {
            base.merge(black_box(&overlay));
        });
    });
    group.finish();
}
