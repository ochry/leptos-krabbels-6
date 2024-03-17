use super::game::*;
use leptos::*;

const BOARD_SIZE: usize = 15;

#[derive(Clone)]
pub struct Cell {
    coord: (usize, usize),
    pub cell_kind: CellKind,
    // has_tile: Option<RwSignal<Tile>>,
    toggle: RwSignal<bool>,
    label: RwSignal<String>,
    pub letter_score: RwSignal<(char, usize)>,
}

impl Cell {
    pub fn new(coord: (usize, usize)) -> Self {
        Self {
            coord,
            cell_kind: CellKind::Normal,
            // has_tile: None,
            toggle: RwSignal::new(false),
            label: RwSignal::new(String::new()),
            letter_score: RwSignal::new((' ', 9)),
        }
    }

    fn set_cell_kind(&mut self, new_kind: CellKind) {
        self.cell_kind = new_kind;
    }

    fn num_to_char(num: &usize) -> char {
        ((num - 1) as u8 + b'A') as char
    }
}

#[derive(Clone)]
pub enum CellKind {
    Header(Header),
    Normal,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

#[derive(Clone)]
pub enum Header {
    Zero,
    Top,
    Left,
}

#[component]
fn Square(
    cell: Cell,
    coord_signal: RwSignal<(usize, usize)>,
    rack_signal: RwSignal<Vec<Tile>>,
) -> impl IntoView {
    // let global_state = use_context::<GlobalState>().unwrap();
    // let rack = move || global_state.global_rack.read_only().get();
    // let rack_copy = rack.clone();

    let untracked_rack = RwSignal::new(Vec::new());
    let new_untrack = move || untracked_rack.set(rack_signal.get_untracked());

    let tile_roll = move || {
        if untracked_rack().is_empty() {
            cell.toggle.update(|b| *b = false);
            (' ', 9)
        } else {
            cell.toggle.update(|b| *b = true);
            (untracked_rack()[0].0, untracked_rack()[0].1)
        }
    };

    view! {
        <div class="tile-inner" class=("tile-letter", move || cell.toggle.get()) on:click=move |_| {
                if !rack_signal().is_empty() && untracked_rack().is_empty() {
                    new_untrack()
                }
                else if !untracked_rack().is_empty() {
                    untracked_rack.update(|vec| {vec.remove(0);})
                };
                cell.letter_score.set(tile_roll());
                coord_signal.set(cell.coord);
            }>
                <div class=("hidden", move || cell.letter_score.with(|t| t.1) != 9)>{cell.label}/*{if rack_cell_signal().is_empty() {'*'} else {rack_cell_signal()[0].0}}*/</div>

                <div class=("hidden", move || cell.letter_score.with(|t| t.1) == 9)>
                    {move || cell.letter_score.with(|t| t.0)}
                    <sub class=("hidden", move || cell.letter_score.with(|t| t.1) == 0)>{move || cell.letter_score.with(|t| t.1)}</sub>
                </div>
        </div>
    }
}

#[component]
pub fn Board(
    board_signal: RwSignal<Vec<Cell>>,
    coord_signal: RwSignal<(usize, usize)>,
    rack_signal: RwSignal<Vec<Tile>>,
) -> impl IntoView {
    let mut board_cells: Vec<Cell> = Vec::new();
    for row in 0..BOARD_SIZE + 1 {
        for col in 0..BOARD_SIZE + 1 {
            let new_cell = Cell::new((row, col));
            board_cells.push(new_cell);
        }
    }

    for c in &mut board_cells {
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
                c.label.set("LD".to_string())
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
        board_signal()
            .into_iter()
            .map(|cell| match cell.cell_kind {
                CellKind::Header(Header::Zero) => {
                    view! {<div class="tile-header label-xs">"krabs"</div>}
                }
                CellKind::Header(Header::Top) => view! {<div class="tile-header">{cell.coord.1}</div>},
                CellKind::Header(Header::Left) => {
                    view! {<div class="tile-header">{Cell::num_to_char(&cell.coord.0)}</div>}
                }
                CellKind::DoubleLetter => {
                    view! {<div class="tile bg-cyan-200"><Square cell coord_signal rack_signal/></div>}
                }
                CellKind::TripleLetter => {
                    view! {<div class="tile bg-blue-400"><Square cell coord_signal rack_signal/></div>}
                }
                CellKind::DoubleWord => {
                    view! {<div class="tile bg-rose-200"><Square cell coord_signal rack_signal/></div>}
                }
                CellKind::TripleWord => {
                    view! {<div class="tile bg-orange-600"><Square cell coord_signal rack_signal/></div>}
                }
                _ => view! {<div class="tile"><Square cell coord_signal rack_signal/></div>},
            })
            .collect_view()
    };

    board_signal.set(board_cells);

    view! {

        <div class="grid gap-0 board lg:board-lg border-0">{draw_cells()}</div>

    }
}

// struct Board {
//     squares: [[Option<Tile>; BOARD_SIZE]; BOARD_SIZE],
// }

// impl Board {
//     fn new() -> Self {
//         Self {
//             squares: [[None; BOARD_SIZE]; BOARD_SIZE],
//         }
//     }
// }
