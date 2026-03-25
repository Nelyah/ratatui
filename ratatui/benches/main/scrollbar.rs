use criterion::{BatchSize, Bencher, BenchmarkId, Criterion, criterion_group};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget};

criterion_group!(benches, scrollbar);

fn scrollbar(c: &mut Criterion) {
    let mut group = c.benchmark_group("scrollbar");

    for content_length in [100, 1000, 10000] {
        group.bench_with_input(
            BenchmarkId::new("render_vertical", content_length),
            &content_length,
            |b, &content_length| {
                render(
                    b,
                    &Scrollbar::new(ScrollbarOrientation::VerticalRight),
                    ScrollbarState::new(content_length).position(content_length / 2),
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("render_horizontal", content_length),
            &content_length,
            |b, &content_length| {
                render(
                    b,
                    &Scrollbar::new(ScrollbarOrientation::HorizontalBottom),
                    ScrollbarState::new(content_length).position(content_length / 2),
                );
            },
        );
    }

    group.finish();
}

fn render(bencher: &mut Bencher, scrollbar: &Scrollbar, mut state: ScrollbarState) {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 50));
    bencher.iter_batched(
        || scrollbar.clone(),
        |bench_scrollbar| {
            StatefulWidget::render(bench_scrollbar, buffer.area, &mut buffer, &mut state);
        },
        BatchSize::SmallInput,
    );
}
