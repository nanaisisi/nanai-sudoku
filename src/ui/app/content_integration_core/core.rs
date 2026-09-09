use super::super::content_core::{metric_bytes, metric_value};
use crate::category::classify_process;
use crate::memory::ProcessMemoryEntry;
use crate::treemap::{Rect, TreemapItem, layout_treemap};
use crate::ui::app_core::RammapApp;
use crate::ui::theme::{color_for_category, wrap_canvas};
use crate::ui::types::RammapMessage;
use windows_reactor::*;

const CANVAS_WIDTH: f64 = 1000.0;
const CANVAS_HEIGHT: f64 = 540.0;

fn bounds() -> Rect {
    Rect {
        x: 0.0,
        y: 0.0,
        width: CANVAS_WIDTH,
        height: CANVAS_HEIGHT,
    }
}

pub(super) fn individual_treemap<S: Fn(RammapMessage) + Clone + 'static>(
    app: &RammapApp,
    sender: S,
    processes: Vec<ProcessMemoryEntry>,
) -> View {
    let items = processes
        .into_iter()
        .map(|p| TreemapItem {
            value: metric_value(&p, app.metric),
            data: p,
        })
        .collect::<Vec<_>>();
    let mut elements = Vec::new();

    for node in layout_treemap(&items, bounds()) {
        let p = node.data;
        let pid = p.pid;
        let label_value = format_bytes(metric_bytes(&p, app.metric));
        let category = classify_process(&p.name);
        let selected = app.selected_process.as_ref().is_some_and(|s| s.pid == pid);
        let background = if selected {
            Color::argb(255, 240, 160, 40)
        } else {
            color_for_category(category, pid)
        };
        let border = if selected {
            Color::argb(255, 255, 255, 255)
        } else {
            Color::argb(180, 20, 20, 20)
        };
        let label = if node.rect.width > 55.0 && node.rect.height > 26.0 {
            if node.rect.height > 42.0 && node.rect.width > 70.0 {
                format!("{}\n{}", p.name, label_value)
            } else {
                format!("{} ({})", p.name, label_value)
            }
        } else if node.rect.width > 30.0 && node.rect.height > 18.0 {
            p.name.clone()
        } else {
            String::new()
        };
        let selected_process = p.clone();
        let click_sender = sender.clone();
        let block = Border::new()
            .width(node.rect.width.max(2.0))
            .height(node.rect.height.max(2.0))
            .background(Brush::Solid(background))
            .border_brush(Brush::Solid(border))
            .border_thickness(Thickness::uniform(1.0))
            .corner_radius(CornerRadius::uniform(2.0))
            .canvas_left(node.rect.x)
            .canvas_top(node.rect.y)
            .on_pointer_pressed(move |_| {
                click_sender(RammapMessage::SelectProcess(
                    Some(selected_process.clone()),
                    None,
                ));
            })
            .content(
                TextBlock::new()
                    .text(label)
                    .font_size(if node.rect.height > 35.0 { 11.0 } else { 9.5 })
                    .margin(Thickness::xy(3.0, 2.0)),
            );
        elements.push(KeyedView::new(pid, block));
    }
    wrap_canvas(
        Canvas::new()
            .width(CANVAS_WIDTH)
            .height(CANVAS_HEIGHT)
            .keyed_children(elements),
    )
}
