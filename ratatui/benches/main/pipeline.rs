use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group};
use ratatui::backend::TestBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, List, ListItem, Paragraph, Widget};
use ratatui::Terminal;

criterion_group!(benches, pipeline);

fn pipeline(c: &mut Criterion) {
    draw_paragraph(c);
    draw_complex_layout(c);
    draw_no_change(c);
}

/// Full Terminal::draw cycle with a Paragraph widget.
fn draw_paragraph(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/draw_paragraph");

    for (w, h) in [(80u16, 24u16), (200, 50)] {
        let text = make_text(h as usize);

        // First draw: all cells differ from empty buffer
        group.bench_with_input(
            BenchmarkId::new("first_draw", format!("{w}x{h}")),
            &text,
            |b, text| {
                b.iter_batched(
                    || Terminal::new(TestBackend::new(w, h)).unwrap(),
                    |mut terminal| {
                        terminal
                            .draw(|frame| {
                                Paragraph::new(text.as_str()).render(frame.area(), frame.buffer_mut());
                            })
                            .unwrap();
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        // Second draw: same content, tests diff efficiency
        group.bench_with_input(
            BenchmarkId::new("redraw_same", format!("{w}x{h}")),
            &text,
            |b, text| {
                b.iter_batched(
                    || {
                        let mut terminal = Terminal::new(TestBackend::new(w, h)).unwrap();
                        terminal
                            .draw(|frame| {
                                Paragraph::new(text.as_str()).render(frame.area(), frame.buffer_mut());
                            })
                            .unwrap();
                        terminal
                    },
                    |mut terminal| {
                        terminal
                            .draw(|frame| {
                                Paragraph::new(text.as_str()).render(frame.area(), frame.buffer_mut());
                            })
                            .unwrap();
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Realistic layout: Block with borders + Paragraph + List side by side.
fn draw_complex_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/draw_complex_layout");
    let (w, h) = (200u16, 50u16);
    let text = make_text(50);
    let items: Vec<ListItem> = (0..100)
        .map(|i| ListItem::new(format!("Item {i}: {}", fakeit::words::sentence(5))))
        .collect();

    group.bench_function(format!("{w}x{h}"), |b| {
        b.iter_batched(
            || Terminal::new(TestBackend::new(w, h)).unwrap(),
            |mut terminal| {
                terminal
                    .draw(|frame| {
                        let outer = Block::bordered().title("App");
                        let inner = outer.inner(frame.area());
                        outer.render(frame.area(), frame.buffer_mut());

                        let chunks = Layout::horizontal([
                            Constraint::Percentage(50),
                            Constraint::Percentage(50),
                        ])
                        .split(inner);

                        Paragraph::new(text.as_str())
                            .block(Block::bordered().title("Text"))
                            .render(chunks[0], frame.buffer_mut());

                        List::new(items.clone())
                            .block(Block::bordered().title("List"))
                            .render(chunks[1], frame.buffer_mut());
                    })
                    .unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Two identical draws: the second measures diff-with-no-changes overhead.
fn draw_no_change(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/draw_no_change");
    let (w, h) = (200u16, 50u16);

    group.bench_function(format!("{w}x{h}"), |b| {
        b.iter_batched(
            || {
                let mut terminal = Terminal::new(TestBackend::new(w, h)).unwrap();
                // First draw populates both buffers after swap
                terminal
                    .draw(|frame| {
                        frame.buffer_mut().set_string(0, 0, "static content", ratatui::style::Style::default());
                    })
                    .unwrap();
                // Second draw so previous buffer also has the content
                terminal
                    .draw(|frame| {
                        frame.buffer_mut().set_string(0, 0, "static content", ratatui::style::Style::default());
                    })
                    .unwrap();
                terminal
            },
            |mut terminal| {
                terminal
                    .draw(|frame| {
                        frame.buffer_mut().set_string(0, 0, "static content", ratatui::style::Style::default());
                    })
                    .unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn make_text(line_count: usize) -> String {
    (0..line_count)
        .map(|_| fakeit::words::sentence(11))
        .collect::<Vec<_>>()
        .join("\n")
}
