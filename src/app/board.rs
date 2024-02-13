use leptos::*;

const BOARD_SIZE: usize = 15;

struct Game {
    bag: Vec<Tile>,
    rack: Vec<Tile>,
}

#[derive(Copy, Clone)]
struct Tile {
    letter: char,
    value: u8,
}

struct Board {
    squares: [[Option<Tile>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Self {
            squares: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

struct TilePlacement {
    tile: Tile,
    board_coord: Board,
    cell_kind: CellKind,
}

enum CellKind {
    Header(Header),
    Normal,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

enum Header {
    Zero,
    Top,
    Left,
}

enum Orientation {
    Horizontal,
    Vertical,
}

struct Cell {
    coord: (usize, usize),
    cell_kind: CellKind,
    toggle: RwSignal<bool>,
    text: RwSignal<String>,
}

impl Cell {
    fn new(
        coord: (usize, usize),
        cell_kind: CellKind,
        toggle: RwSignal<bool>,
        text: RwSignal<String>,
    ) -> Self {
        Self {
            coord,
            cell_kind,
            toggle,
            text,
        }
    }

    fn set_cell_kind(&mut self, new_kind: CellKind) {
        self.cell_kind = new_kind;
    }

    fn num_to_char(num: &usize) -> char {
        ((num - 1) as u8 + b'A') as char
    }
}

#[component]
fn Square(cell: Cell, setter: RwSignal<(usize, usize)>, text: String) -> impl IntoView {
    view! {
        <div class="tile-inner" class=("tile-letter", move || cell.toggle.get()) on:click=move |_| {
                setter.set(cell.coord);
                cell.toggle.update(|b| *b = !*b);}>{text}
        </div>
    }
}

#[component]
pub fn Board(setter: RwSignal<(usize, usize)>) -> impl IntoView {
    let mut empty_cells: Vec<Cell> = Vec::new();
    for row in 0..BOARD_SIZE + 1 {
        for col in 0..BOARD_SIZE + 1 {
            let new_cell = Cell::new(
                (row, col),
                CellKind::Normal,
                RwSignal::new(false),
                RwSignal::new("ok".to_string()),
            );
            empty_cells.push(new_cell);
        }
    }

    for c in &mut empty_cells {
        match c.coord {
            (0, 0) => c.set_cell_kind(CellKind::Header(Header::Zero)),
            (0, _) => c.set_cell_kind(CellKind::Header(Header::Top)),
            (_, 0) => c.set_cell_kind(CellKind::Header(Header::Left)),
            (1, 4)
            | (1, 12)
            | (3, 7)
            | (3, 9)
            | (4, 1)
            | (4, 8)
            | (4, 15)
            | (7, 3)
            | (7, 7)
            | (7, 9)
            | (7, 13)
            | (8, 4)
            | (8, 12)
            | (9, 3)
            | (9, 7)
            | (9, 9)
            | (9, 13)
            | (12, 1)
            | (12, 8)
            | (12, 15)
            | (13, 7)
            | (13, 9)
            | (15, 4)
            | (15, 12) => c.set_cell_kind(CellKind::DoubleLetter),
            (2, 6)
            | (2, 10)
            | (6, 2)
            | (6, 6)
            | (6, 10)
            | (6, 14)
            | (10, 2)
            | (10, 6)
            | (10, 10)
            | (10, 14)
            | (14, 6)
            | (14, 10) => c.set_cell_kind(CellKind::TripleLetter),
            (2, 2)
            | (2, 14)
            | (3, 3)
            | (3, 13)
            | (4, 4)
            | (4, 12)
            | (5, 5)
            | (5, 11)
            | (8, 8)
            | (11, 5)
            | (11, 11)
            | (12, 4)
            | (12, 12)
            | (13, 3)
            | (13, 13)
            | (14, 2)
            | (14, 14) => c.set_cell_kind(CellKind::DoubleWord),
            (1, 1) | (1, 8) | (1, 15) | (8, 1) | (8, 15) | (15, 1) | (15, 8) | (15, 15) => {
                c.set_cell_kind(CellKind::TripleWord)
            }
            _ => c.set_cell_kind(CellKind::Normal),
        }
    }

    let draw_cells = move || {
        empty_cells
        .into_iter()
        .map(|c| {
            match c.cell_kind {
                CellKind::Header(Header::Zero) => view! {<div class="tile-header text-xs"
                    on:click=move |_| setter.set(c.coord)>"krabs"</div>},
                CellKind::Header(Header::Top) => view! {<div class="tile-header">{c.coord.1}</div>},
                CellKind::Header(Header::Left) => view! {<div class="tile-header">{Cell::num_to_char(&c.coord.0)}</div>},
                CellKind::DoubleLetter => view! {<div class="tile bg-cyan-200"><Square cell=c setter=setter text="LD".to_string()/></div>},
                CellKind::TripleLetter => view! {<div class="tile bg-blue-400"><Square cell=c setter=setter text="LT".to_string()/></div>},
                CellKind::DoubleWord => view! {<div class="tile bg-rose-200"><Square cell=c setter=setter text="MD".to_string()/></div>},
                CellKind::TripleWord => view! {<div class="tile bg-orange-600"><Square cell=c setter=setter text="MT".to_string()/></div>},
                _ => view! {<div class="tile"><Square cell=c setter=setter text="".to_string()/></div>},
                }
        })
        .collect_view()
    };

    view! {

        <div class="grid gap-0 board lg:board-lg border-0">{draw_cells()}</div>
    }
}
