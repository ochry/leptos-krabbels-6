use leptos::*;

const BOARD_SIZE: usize = 15;

struct Game {
    bag: Vec<Tile>,
    rack: Vec<Tile>,
}

#[derive(Copy, Clone)]
struct Tile(char, u8);

struct Cell {
    coord: (usize, usize),
    cell_kind: CellKind,
    has_tile: Option<RwSignal<Tile>>,
    toggle: RwSignal<bool>,
    label: RwSignal<String>,
    score: RwSignal<(char, u8)>,
}

impl Cell {
    fn new(coord: (usize, usize)) -> Self {
        Self {
            coord,
            cell_kind: CellKind::Normal,
            has_tile: None,
            toggle: RwSignal::new(false),
            label: RwSignal::new(String::new()),
            score: RwSignal::new((' ', 9)),
        }
    }

    fn set_cell_kind(&mut self, new_kind: CellKind) {
        self.cell_kind = new_kind;
    }

    fn num_to_char(num: &usize) -> char {
        ((num - 1) as u8 + b'A') as char
    }
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

#[component]
fn Square(cell: Cell, setter: RwSignal<(usize, usize)>) -> impl IntoView {
    let tile_a = Tile('A', 1);
    let tile_b = Tile('B', 3);
    let tile_c = Tile('C', 3);

    let mut tile_pool = Vec::from([tile_a, tile_b, tile_c]);
    // let mut chars_vec = chars_pool.chars().rev().collect::<Vec<char>>();
    let mut tile_pop = move || {
        if let Some(t) = tile_pool.pop() {
            cell.toggle.update(|b| *b = true);
            (t.0, t.1)
        } else {
            cell.toggle.update(|b| *b = false);
            tile_pool = vec![tile_a, tile_b, tile_c];
            (' ', 9)
        }
    };

    view! {
        <div class="tile-inner" class=("tile-letter", move || cell.toggle.get()) on:click=move |_| {
                setter.set(cell.coord);
                cell.score.set(tile_pop());
            }>
                <div class=("hidden", move || cell.score.with(|t| t.1) != 9)>{cell.label}</div>
                <div class=("hidden", move || cell.score.with(|t| t.1) == 9)>{move || cell.score.with(|t| t.0)}<sub>{move || cell.score.with(|t| t.1)}</sub></div>
        </div>
    }
}

#[component]
pub fn Board(setter: RwSignal<(usize, usize)>) -> impl IntoView {
    let mut empty_cells: Vec<Cell> = Vec::new();
    for row in 0..BOARD_SIZE + 1 {
        for col in 0..BOARD_SIZE + 1 {
            let new_cell = Cell::new((row, col));
            empty_cells.push(new_cell);
        }
    }

    for c in &mut empty_cells {
        match c.coord {
            (0, 0) => c.set_cell_kind(CellKind::Header(Header::Zero)),
            (0, _) => c.set_cell_kind(CellKind::Header(Header::Top)),
            (_, 0) => c.set_cell_kind(CellKind::Header(Header::Left)),
            (8, 8) => {
                c.set_cell_kind(CellKind::DoubleWord);
                c.label.set("★".to_string())
            }
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
            | (15, 12) => {
                c.set_cell_kind(CellKind::DoubleLetter);
                c.label.set("MD".to_string())
            }
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
            | (14, 10) => {
                c.set_cell_kind(CellKind::TripleLetter);
                c.label.set("LT".to_string())
            }
            (2, 2)
            | (2, 14)
            | (3, 3)
            | (3, 13)
            | (4, 4)
            | (4, 12)
            | (5, 5)
            | (5, 11)
            | (11, 5)
            | (11, 11)
            | (12, 4)
            | (12, 12)
            | (13, 3)
            | (13, 13)
            | (14, 2)
            | (14, 14) => {
                c.set_cell_kind(CellKind::DoubleWord);
                c.label.set("MD".to_string())
            }
            (1, 1) | (1, 8) | (1, 15) | (8, 1) | (8, 15) | (15, 1) | (15, 8) | (15, 15) => {
                c.set_cell_kind(CellKind::TripleWord);
                c.label.set("MT".to_string())
            }
            _ => c.set_cell_kind(CellKind::Normal),
        }
    }

    let draw_cells = move || {
        empty_cells
            .into_iter()
            .map(|c| match c.cell_kind {
                CellKind::Header(Header::Zero) => view! {<div class="tile-header label-xs"
                on:click=move |_| setter.set(c.coord)>"krabs"</div>},
                CellKind::Header(Header::Top) => view! {<div class="tile-header">{c.coord.1}</div>},
                CellKind::Header(Header::Left) => {
                    view! {<div class="tile-header">{Cell::num_to_char(&c.coord.0)}</div>}
                }
                CellKind::DoubleLetter => {
                    view! {<div class="tile bg-cyan-200"><Square cell=c setter=setter/></div>}
                }
                CellKind::TripleLetter => {
                    view! {<div class="tile bg-blue-400"><Square cell=c setter=setter/></div>}
                }
                CellKind::DoubleWord => {
                    view! {<div class="tile bg-rose-200"><Square cell=c setter=setter/></div>}
                }
                CellKind::TripleWord => {
                    view! {<div class="tile bg-orange-600"><Square cell=c setter=setter/></div>}
                }
                _ => view! {<div class="tile"><Square cell=c setter=setter/></div>},
            })
            .collect_view()
    };

    view! {

        <div class="grid gap-0 board lg:board-lg border-0">{draw_cells()}</div>
    }
}
