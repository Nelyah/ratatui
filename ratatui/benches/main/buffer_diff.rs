use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group};
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

criterion_group!(benches, buffer_diff);

const SIZES: [(u16, u16); 3] = [(80, 24), (200, 50), (256, 256)];

fn buffer_diff(c: &mut Criterion) {
    identical(c);
    fully_different(c);
    sparse_changes(c);
    wide_chars(c);
    vs16_emoji(c);
}

/// Best case: identical buffers produce zero diffs.
fn identical(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff/identical");
    for (w, h) in SIZES {
        let buf = Buffer::empty(Rect::new(0, 0, w, h));
        group.bench_with_input(BenchmarkId::from_parameter(format!("{w}x{h}")), &buf, |b, buf| {
            b.iter(|| black_box(buf.diff_iter(buf).count()));
        });
    }
    group.finish();
}

/// Worst case: every cell differs.
fn fully_different(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff/fully_different");
    for (w, h) in SIZES {
        let area = Rect::new(0, 0, w, h);
        let prev = Buffer::empty(area);
        let next = Buffer::filled(area, Cell::new("x"));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{w}x{h}")),
            &(prev, next),
            |b, (prev, next)| {
                b.iter(|| black_box(prev.diff_iter(next).count()));
            },
        );
    }
    group.finish();
}

/// Realistic case: ~10% of cells changed.
fn sparse_changes(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff/sparse_changes");
    for (w, h) in SIZES {
        let area = Rect::new(0, 0, w, h);
        let prev = Buffer::filled(area, Cell::new("a"));
        let mut next = prev.clone();
        let total = (w as usize) * (h as usize);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        for _ in 0..total / 10 {
            let idx = rng.random_range(0..total);
            next.content[idx].set_symbol("b");
        }
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{w}x{h}")),
            &(prev, next),
            |b, (prev, next)| {
                b.iter(|| black_box(prev.diff_iter(next).count()));
            },
        );
    }
    group.finish();
}

/// CJK double-width characters exercise multi-width skip logic.
fn wide_chars(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff/wide_chars");
    let area = Rect::new(0, 0, 200, 50);
    let mut prev = Buffer::empty(area);
    let mut next = Buffer::empty(area);
    // Fill with CJK characters (each takes 2 cells)
    for y in 0..50u16 {
        for x in (0..200u16).step_by(2) {
            prev.set_string(x, y, "漢", Style::default());
            next.set_string(x, y, "漢", Style::default());
        }
    }
    // Change ~10% of characters
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    for y in 0..50u16 {
        for x in (0..200u16).step_by(2) {
            if rng.random_range(0..10u32) == 0 {
                next.set_string(x, y, "字", Style::default());
            }
        }
    }
    group.bench_with_input(BenchmarkId::from_parameter("200x50"), &(prev, next), |b, (prev, next)| {
        b.iter(|| black_box(prev.diff_iter(next).count()));
    });
    group.finish();
}

/// VS16 emoji exercise the TrailingState branch in BufferDiff.
fn vs16_emoji(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff/vs16_emoji");
    let area = Rect::new(0, 0, 200, 50);
    let mut prev = Buffer::empty(area);
    let mut next = Buffer::empty(area);
    // Fill with VS16 emoji (⌨️ takes 2 cells)
    for y in 0..50u16 {
        for x in (0..200u16).step_by(2) {
            prev.set_string(x, y, "⌨️", Style::default());
            next.set_string(x, y, "⌨️", Style::default());
        }
    }
    // Change ~10% with different emoji
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    for y in 0..50u16 {
        for x in (0..200u16).step_by(2) {
            if rng.random_range(0..10u32) == 0 {
                next.set_string(x, y, "☎️", Style::new().fg(Color::Red));
            }
        }
    }
    group.bench_with_input(BenchmarkId::from_parameter("200x50"), &(prev, next), |b, (prev, next)| {
        b.iter(|| black_box(prev.diff_iter(next).count()));
    });
    group.finish();
}
