use super::keys::{self, Cell, Command, Shape};
use super::window::{self, OVERSCAN};
use leptos::ev::{Event, FocusEvent, KeyboardEvent, MouseEvent};
use leptos::html::Div;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::Element;
use rustify_ui::Selection;
use std::sync::Arc;

const GRID: &str = "rui:relative rui:flex rui:flex-col rui:min-h-0 rui:border rui:border-border rui:rounded-md rui:bg-background rui:text-sm rui:outline-none rui:focus-visible:ring-ring/50 rui:focus-visible:ring-[3px]";
const HEADER: &str = "rui:flex rui:shrink-0 rui:border-b rui:border-border rui:bg-muted rui:font-medium rui:text-muted-foreground";
const HEADER_CELL: &str =
    "rui:shrink-0 rui:truncate rui:px-2 rui:py-1 rui:text-left rui:cursor-default rui:select-none";
const SCROLLER: &str = "rui:relative rui:flex-1 rui:min-h-0 rui:overflow-auto";
const ROW: &str = "rui:absolute rui:left-0 rui:flex rui:w-full rui:items-center rui:border-b rui:border-border/50 rui:aria-selected:bg-accent";
const CELL: &str = "rui:shrink-0 rui:truncate rui:px-2 rui:outline-none rui:focus-visible:ring-ring/50 rui:focus-visible:ring-2 rui:focus-visible:ring-inset";

/// One column of the table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Column {
    pub key: String,
    pub label: String,
}

impl Column {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
        }
    }
}

/// Reads one cell. The table never holds a value; it asks for the ones it is
/// about to draw and forgets them again.
pub type CellText = Arc<dyn Fn(usize, usize) -> String + Send + Sync>;
/// The business identity of a row. What a selection is made of.
pub type RowId = Arc<dyn Fn(usize) -> u32 + Send + Sync>;

/// A table of any number of rows, of which it draws the ones on screen.
///
/// The row elements are a pool: there are as many as fit the viewport plus the
/// overscan, and scrolling moves them and rewrites their text rather than
/// making new ones. So `<For>` is keyed by the slot rather than by the row -
/// the row a slot is showing is in its `aria-rowindex` and its `data-row-id`,
/// which is where a person and a test both read it.
#[component]
pub fn DataTable(
    /// How many rows there are, in the order the application is showing them.
    #[prop(into)]
    rows: Signal<usize>,
    #[prop(into)] columns: Signal<Vec<Column>>,
    cell: CellText,
    row_id: RowId,
    #[prop(into)] selected: Signal<Selection>,
    /// The row a person asked to see. Cleared once the table has shown it, so
    /// asking for the same row twice works.
    goto: RwSignal<Option<usize>>,
    /// Raised every time the visible range is worked out again. A frame budget
    /// reads this: an animation frame in which it did not move is a frame in
    /// which the table presented nothing.
    #[prop(optional)]
    version: Option<RwSignal<u64>>,
    /// The cell the keyboard is on. Owned outside so that the application can
    /// put the keyboard on a row it has just found.
    focus: RwSignal<Cell>,
    /// A row was selected or deselected.
    on_select: impl Fn(usize) + Send + Sync + 'static,
    /// A row was opened - Enter, F2, or a double click.
    on_activate: impl Fn(usize) + Send + Sync + 'static,
    #[prop(optional)] row_height: Option<f64>,
    #[prop(optional)] column_width: Option<f64>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let row_height = row_height.unwrap_or(24.0);
    let column_width = column_width.unwrap_or(120.0);
    let grid = NodeRef::<Div>::new();
    let scroller = NodeRef::<Div>::new();
    let scroll_top = RwSignal::new(0.0f64);
    let scroll_left = RwSignal::new(0.0f64);
    // Measured rather than assumed: the pool is as big as the viewport, and
    // the viewport is whatever the application's layout gave it.
    let viewport = RwSignal::new((0.0f64, 0.0f64));
    let on_select = Arc::new(on_select);
    let on_activate = Arc::new(on_activate);

    let visible_rows = Memo::new(move |_| {
        let (_, height) = viewport.get();
        window::rows(scroll_top.get(), height, row_height, rows.get(), OVERSCAN)
    });
    let visible_columns = Memo::new(move |_| {
        let (width, _) = viewport.get();
        window::columns(scroll_left.get(), width, column_width, columns.get().len())
    });
    // The pool is the size of the widest window this viewport can need, so it
    // does not grow and shrink as the table is scrolled.
    let slots = Memo::new(move |_| visible_rows.get().len());

    Effect::new(move || {
        visible_rows.track();
        visible_columns.track();
        if let Some(version) = version {
            version.update(|count| *count += 1);
        }
    });

    // Through `Element`, read with `into` and written with `scroll_to`: the
    // scroll accessors exist on more than one type in the deref chain, and
    // web-sys gives `scrollTop` an `i32` normally and an `f64` under
    // `web_sys_unstable_apis` - which this workspace's own build turns on and
    // an ordinary `cargo build` does not. Both of those convert into `f64`,
    // and `scroll_to` takes `f64` either way.
    let measure = move || {
        let Some(element) = scroller.get_untracked() else {
            return;
        };
        let element: &Element = element.as_ref();
        let size = (
            element.client_width() as f64,
            element.client_height() as f64,
        );
        if viewport.get_untracked() != size {
            viewport.set(size);
        }
    };
    Effect::new(move || {
        scroller.track();
        measure();
    });

    // Scrolling to a row the application asked for. It is done against the
    // element rather than a signal, because the scroll position a browser
    // ends up at is the browser's answer, not ours.
    Effect::new(move || {
        let Some(row) = goto.get() else {
            return;
        };
        goto.set(None);
        let Some(element) = scroller.get_untracked() else {
            return;
        };
        let element: &Element = element.as_ref();
        let (_, height) = viewport.get_untracked();
        let here = scroll_top.get_untracked();
        if let Some(top) = window::scroll_to(row, here, height, row_height, rows.get_untracked()) {
            element.scroll_to_with_x_and_y(element.scroll_left().into(), top);
            scroll_top.set(top);
        }
    });

    let on_scroll = move |_: Event| {
        let Some(element) = scroller.get_untracked() else {
            return;
        };
        let element: &Element = element.as_ref();
        scroll_top.set(element.scroll_top().into());
        scroll_left.set(element.scroll_left().into());
        measure();
    };

    let shape = move || Shape {
        rows: rows.get_untracked(),
        columns: columns.get_untracked().len(),
        viewport: viewport.get_untracked().1,
        row_height,
    };
    let name = test_id_of(&test_id);
    let on_keydown = {
        let on_select = on_select.clone();
        let on_activate = on_activate.clone();
        let name = name.clone();
        move |ev: KeyboardEvent| {
            let at = focus.get_untracked();
            let Some(command) =
                keys::command(&ev.key(), ev.ctrl_key() || ev.meta_key(), at, shape())
            else {
                return;
            };
            ev.prevent_default();
            match command {
                Command::Move(next) => {
                    focus.set(next);
                    // Into view first, so the cell the focus is about to land
                    // on exists to receive it.
                    if !visible_rows.get_untracked().contains(next.row) {
                        goto.set(Some(next.row));
                    }
                    focus_cell(&grid, next);
                }
                Command::Toggle => on_select(at.row),
                Command::Activate => on_activate(at.row),
            }
        }
    };

    // One listener for the whole grid rather than one per cell. Six hundred
    // cells with two listeners each is twelve hundred JavaScript values wasm
    // has to hold, and the table it holds them in has a ceiling: crossing it
    // aborts the module rather than failing an allocation.
    let on_focusin = move |ev: FocusEvent| {
        let Some(target) = ev
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let index = |name: &str| {
            target
                .get_attribute(name)
                .and_then(|value| value.parse::<usize>().ok())
        };
        let Some(column) = index("aria-colindex") else {
            return;
        };
        let Some(row) = target
            .closest("[aria-rowindex]")
            .ok()
            .flatten()
            .and_then(|row| row.get_attribute("aria-rowindex"))
            .and_then(|value| value.parse::<usize>().ok())
        else {
            return;
        };
        let next = Cell {
            row: row.saturating_sub(1),
            column: column.saturating_sub(1),
        };
        if focus.get_untracked() != next {
            focus.set(next);
        }
    };
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    // One clone for each `<For>` body: both are called again whenever their
    // list changes, so neither can own the name.
    let headings = name.clone();
    let pool = move || (0..slots.get()).collect::<Vec<_>>();
    let header_columns = move || {
        let all = columns.get();
        visible_columns
            .get()
            .iter()
            .filter_map(|index| all.get(index).cloned().map(|column| (index, column)))
            .collect::<Vec<_>>()
    };

    view! {
        <div
            node_ref=grid
            class=crate::macros::merge(GRID, &class)
            data-name="DataTable"
            data-testid=test_id.clone()
            role="grid"
            aria-label=aria_label
            aria-rowcount=move || rows.get().to_string()
            aria-colcount=move || columns.get().len().to_string()
            on:keydown=on_keydown
            on:focusin=on_focusin
        >
            <div role="rowgroup" class=HEADER>
                <div
                    role="row"
                    aria-rowindex="0"
                    class="rui:flex"
                    style:padding-left=move || {
                        format!("{}px", visible_columns.get().start as f64 * column_width)
                    }
                >
                    <For each=header_columns key=|(index, _)| *index let:entry>
                        {
                            let (index, column) = entry;
                            let name = headings.clone();
                            view! {
                                <div
                                    role="columnheader"
                                    class=HEADER_CELL
                                    data-testid=format!("{name}-column-{index}")
                                    aria-colindex=(index + 1).to_string()
                                    style:width=format!("{column_width}px")
                                >
                                    {column.label}
                                </div>
                            }
                        }
                    </For>
                </div>
            </div>
            <div node_ref=scroller class=SCROLLER role="presentation" on:scroll=on_scroll>
                <div
                    role="presentation"
                    style:position="relative"
                    style:height=move || format!("{}px", rows.get() as f64 * row_height)
                    style:width=move || format!("{}px", columns.get().len() as f64 * column_width)
                >
                    <div role="rowgroup">
                        <For each=pool key=|slot| *slot let:slot>
                            {
                                let name = name.clone();
                                let cell = cell.clone();
                                let row_id = row_id.clone();
                                let select = on_select.clone();
                                let activate = on_activate.clone();
                                let row = Memo::new(move |_| visible_rows.get().start + slot);
                                let within = Memo::new(move |_| row.get() < rows.get());
                                let id = Memo::new(move |_| {
                                    if within.get() { row_id(row.get()) } else { 0 }
                                });
                                let chosen = Memo::new(move |_| {
                                    within.get() && selected.get().contains(id.get())
                                });
                                let cells = move || visible_columns.get().iter().collect::<Vec<_>>();
                                view! {
                                    <div
                                        role="row"
                                        class=ROW
                                        class=("rui:hidden", move || !within.get())
                                        data-testid=format!("{name}-row-{slot}")
                                        data-row-id=move || id.get().to_string()
                                        aria-rowindex=move || (row.get() + 1).to_string()
                                        aria-selected=move || chosen.get().to_string()
                                        style:height=format!("{row_height}px")
                                        style:transform=move || {
                                            format!("translateY({}px)", row.get() as f64 * row_height)
                                        }
                                        on:click=move |_: MouseEvent| {
                                            if within.get_untracked() {
                                                select(row.get_untracked());
                                            }
                                        }
                                        on:dblclick=move |_: MouseEvent| {
                                            if within.get_untracked() {
                                                activate(row.get_untracked());
                                            }
                                        }
                                    >
                                        <For each=cells key=|column| *column let:column>
                                            {
                                                let cell = cell.clone();
                                                let focused = Memo::new(move |_| {
                                                    let at = focus.get();
                                                    at.row == row.get() && at.column == column
                                                });
                                                view! {
                                                    <div
                                                        role="gridcell"
                                                        class=CELL
                                                        aria-colindex=(column + 1).to_string()
                                                        tabindex=move || {
                                                            if focused.get() { "0" } else { "-1" }
                                                        }
                                                        style:position="absolute"
                                                        style:width=format!("{column_width}px")
                                                        style:transform=format!(
                                                            "translateX({}px)",
                                                            column as f64 * column_width,
                                                        )
                                                    >
                                                        {move || {
                                                            if within.get() {
                                                                cell(row.get(), column)
                                                            } else {
                                                                String::new()
                                                            }
                                                        }}
                                                    </div>
                                                }
                                            }
                                        </For>
                                    </div>
                                }
                            }
                        </For>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Puts the keyboard on one cell of the grid, found by the indices a person
/// and a screen reader both read it by.
fn focus_cell(grid: &NodeRef<Div>, at: Cell) {
    let Some(element) = grid.get_untracked() else {
        return;
    };
    let element: &Element = element.as_ref();
    crate::dom::focus_within(
        element,
        &format!(
            "[aria-rowindex=\"{}\"] [aria-colindex=\"{}\"]",
            at.row + 1,
            at.column + 1
        ),
    );
}

fn test_id_of(test_id: &str) -> String {
    if test_id.is_empty() {
        "table".to_string()
    } else {
        test_id.to_string()
    }
}
