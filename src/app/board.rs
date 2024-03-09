use super::game::*;
use leptos::*;

const BOARD_SIZE: usize = 15;

struct Cell {
    coord: (usize, usize),
    cell_kind: CellKind,
    // has_tile: Option<RwSignal<Tile>>,
    toggle: RwSignal<bool>,
    label: RwSignal<String>,
    letter_score: RwSignal<(char, u8)>,
}

impl Cell {
    fn new(coord: (usize, usize)) -> Self {
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

#[component]
fn Square(
    cell: Cell,
    coord_signal: RwSignal<(usize, usize)>,
    rack_signal: RwSignal<Vec<Tile>>,
) -> impl IntoView {
    // let global_state = use_context::<GlobalState>().unwrap();
    // let rack = move || global_state.global_rack.read_only().get();
    // let rack_copy = rack.clone();

    // let tile_roll = move || (rack_signal()[0].0, rack_signal()[0].1);

    //     (rack[0].0, rack[0].1)
    //     // if let Some(t) = new_vec.pop() {
    //     //     cell.toggle.update(|b| *b = true);
    //     //     global_state.global_rack.update(|t| {
    //     //         t.pop();
    //     //     });
    //     //     (t.0, t.1)
    //     // } else {
    //     //     cell.toggle.update(|b| *b = false);
    //     //     // global_state.global_rack.update();
    //     //     (' ', 9)
    //     // }
    // };

    view! {
        <div class="tile-inner" class=("tile-letter", move || cell.toggle.get()) on:click=move |_| {
                // cell.letter_score.set(tile_roll());
                if rack_signal().is_empty(){} else {rack_signal.update(|vec| {vec.remove(0);})};
                coord_signal.set(cell.coord);
            }>
                <div class=("hidden", move || cell.letter_score.with(|t| t.1) != 9)>{cell.label}</div>
                <div class=("hidden", move || cell.letter_score.with(|t| t.1) == 9)>
                    {move || cell.letter_score.with(|t| t.0)}
                    <sub class=("hidden", move || cell.letter_score.with(|t| t.1) == 0)>{move || cell.letter_score.with(|t| t.1)}</sub>
                </div>
        </div>
    }
}

#[component]
pub fn Board(
    coord_signal: RwSignal<(usize, usize)>,
    rack_signal: RwSignal<Vec<Tile>>,
) -> impl IntoView {
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
                CellKind::Header(Header::Zero) => {
                    view! {<div class="tile-header label-xs">"krabs"</div>}
                }
                CellKind::Header(Header::Top) => view! {<div class="tile-header">{c.coord.1}</div>},
                CellKind::Header(Header::Left) => {
                    view! {<div class="tile-header">{Cell::num_to_char(&c.coord.0)}</div>}
                }
                CellKind::DoubleLetter => {
                    view! {<div class="tile bg-cyan-200"><Square cell=c coord_signal rack_signal/></div>}
                }
                CellKind::TripleLetter => {
                    view! {<div class="tile bg-blue-400"><Square cell=c coord_signal rack_signal/></div>}
                }
                CellKind::DoubleWord => {
                    view! {<div class="tile bg-rose-200"><Square cell=c coord_signal rack_signal/></div>}
                }
                CellKind::TripleWord => {
                    view! {<div class="tile bg-orange-600"><Square cell=c coord_signal rack_signal/></div>}
                }
                _ => view! {<div class="tile"><Square cell=c coord_signal rack_signal/></div>},
            })
            .collect_view()
    };

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
