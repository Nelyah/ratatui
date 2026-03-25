pub mod main {
    pub mod backend_draw;
    pub mod barchart;
    pub mod block;
    pub mod buffer;
    pub mod buffer_diff;
    pub mod buffer_ops;
    pub mod cell;
    pub mod chart;
    pub mod constraints;
    pub mod gauge;
    pub mod line;
    pub mod list;
    pub mod paragraph;
    pub mod pipeline;
    pub mod rect;
    pub mod scrollbar;
    pub mod sparkline;
    pub mod style;
    pub mod table;
    pub mod text;
}
pub use main::*;

criterion::criterion_main!(
    backend_draw::benches,
    barchart::benches,
    block::benches,
    buffer::benches,
    buffer_diff::benches,
    buffer_ops::benches,
    cell::benches,
    chart::benches,
    constraints::benches,
    gauge::benches,
    line::benches,
    list::benches,
    paragraph::benches,
    pipeline::benches,
    rect::benches,
    scrollbar::benches,
    sparkline::benches,
    style::benches,
    table::benches,
    text::benches,
);
