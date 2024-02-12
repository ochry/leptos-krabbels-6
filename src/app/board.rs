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
    toggle: RwSignal<i32>,
    text: RwSignal<String>,
    num: i32,
}

impl Cell {
    fn new(
        coord: (usize, usize),
        cell_kind: CellKind,
        toggle: RwSignal<i32>,
        text: RwSignal<String>,
        num: i32,
    ) -> Self {
        Self {
            coord,
            cell_kind,
            toggle,
            text,
            num,
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
pub fn Board() -> impl IntoView {
    let mut empty_cells: Vec<Cell> = Vec::new();
    for row in 0..BOARD_SIZE + 1 {
        for col in 0..BOARD_SIZE + 1 {
            let new_cell = Cell::new(
                (row, col),
                CellKind::Normal,
                RwSignal::new(0),
                RwSignal::new("ok".to_string()),
                0,
            );
            empty_cells.push(new_cell);
        }
    }

    for c in &mut empty_cells {
        match c.coord {
            (0, 0) => c.set_cell_kind(CellKind::Header(Header::Zero)),
            (0, _) => c.set_cell_kind(CellKind::Header(Header::Top)),
            (_, 0) => c.set_cell_kind(CellKind::Header(Header::Left)),
            (1, 4) | (1, 12) | (3, 7) | (3, 9) => c.set_cell_kind(CellKind::DoubleLetter),
            (2, 5) | (2, 9) => c.set_cell_kind(CellKind::TripleLetter),
            (2, 2) | (2, 13) => c.set_cell_kind(CellKind::DoubleWord),
            (1, 1) | (1, 8) | (1, 15) | (8, 1) | (8, 15) | (15, 1) | (15, 8) | (15, 15) => {
                c.set_cell_kind(CellKind::TripleWord)
            }
            _ => c.set_cell_kind(CellKind::Normal),
        }
    }

    let draw_cells = empty_cells
        .into_iter()
        .map(|c| {
            view! {
                {move || match c.cell_kind {
                        CellKind::Header(Header::Zero) => view! {<div class="tile-header">Krab</div>},
                        CellKind::Header(Header::Top) => view! {<div class="tile-header">{c.coord.1}</div>},
                        CellKind::Header(Header::Left) => view! {<div class="tile-header">{Cell::num_to_char(&c.coord.0)}</div>},
                        CellKind::DoubleLetter => view! {<div class="tile bg-cyan-200">LD</div>},
                        CellKind::TripleLetter => view! {<div class="tile bg-blue-400">LT</div>},
                        CellKind::DoubleWord => view! {<div class="tile bg-rose-200">MD</div>},
                        CellKind::TripleWord => view! {<div class="tile bg-orange-600">MT</div>},
                        _ => view! {
                                <div class="tile" class=("tile-letter", move || c.toggle.with(|n| n % 2 == 1)) on:click=move |_| {c.toggle.update(|n| *n += 1);}>
                                    <p>{c.coord.0}":"{c.coord.1}</p>
                                </div>
                            }
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="grid gap-0 board lg:board-lg border-0">{draw_cells}</div>
    }
}

// if Cell::header_zero(&c) {
//     view! {
//         <div class="tile-header">Krab</div>
//     }
// } else if Cell::header_top(&c) {
//     view! {
//         <div class="tile-header">{c.coord.1}</div>
//     }
// } else if Cell::header_left(&c) {
//     view!{
//         <div class="tile-header">{Cell::num_to_char(&c.coord.0)}</div>
//         }
// } else {
// view! {
//     <div class="tile" class=("tile-letter", move || c.toggle.with(|n| n % 2 == 1))
//         on:click=move |_| {
//         c.toggle.update(|n| *n += 1);
//     }>
//         <p>{c.coord.0}":"{c.coord.1}</p>
//     </div>
// }
// }
