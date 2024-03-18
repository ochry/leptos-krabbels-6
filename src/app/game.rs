use super::board::*;
use super::rack::*;
use leptos::*;
use rand::prelude::*;
use rand::seq::SliceRandom;

#[derive(Clone, PartialEq)]
pub struct Tile(pub char, pub usize);

pub struct Bag(pub Vec<Tile>);

impl Bag {
    pub fn new() -> Self {
        let mut bag = Vec::new();

        bag.extend((0..9).map(|_| Tile('A', 1)));
        bag.extend((0..2).map(|_| Tile('B', 3)));
        bag.extend((0..2).map(|_| Tile('C', 3)));
        bag.extend((0..3).map(|_| Tile('D', 2)));
        bag.extend((0..15).map(|_| Tile('E', 1)));
        bag.extend((0..2).map(|_| Tile('F', 4)));
        bag.extend((0..2).map(|_| Tile('G', 2)));
        bag.extend((0..2).map(|_| Tile('H', 4)));
        bag.extend((0..8).map(|_| Tile('I', 1)));
        bag.extend((0..1).map(|_| Tile('J', 8)));
        bag.extend((0..1).map(|_| Tile('K', 10)));
        bag.extend((0..5).map(|_| Tile('L', 1)));
        bag.extend((0..3).map(|_| Tile('M', 2)));
        bag.extend((0..6).map(|_| Tile('N', 1)));
        bag.extend((0..6).map(|_| Tile('O', 1)));
        bag.extend((0..2).map(|_| Tile('P', 3)));
        bag.extend((0..1).map(|_| Tile('Q', 8)));
        bag.extend((0..6).map(|_| Tile('R', 1)));
        bag.extend((0..6).map(|_| Tile('S', 1)));
        bag.extend((0..6).map(|_| Tile('T', 1)));
        bag.extend((0..6).map(|_| Tile('U', 1)));
        bag.extend((0..2).map(|_| Tile('V', 4)));
        bag.extend((0..1).map(|_| Tile('W', 10)));
        bag.extend((0..1).map(|_| Tile('X', 10)));
        bag.extend((0..1).map(|_| Tile('Y', 10)));
        bag.extend((0..1).map(|_| Tile('Z', 10)));
        bag.extend((0..2).map(|_| Tile(' ', 0)));

        Self(bag)
    }

    pub fn draw_tiles(amount: u8, bag: &mut Vec<Tile>) -> Vec<Tile> {
        let mut rack = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..amount {
            if let Some(tile) = bag.choose(&mut rng) {
                let tile_index = bag.iter().position(|t| t.0 == tile.0).unwrap();
                let tile_out = bag.remove(tile_index);
                rack.push(tile_out);
            } else {
                break;
            }
        }
        rack
    }
}

#[component]
pub fn Game() -> impl IntoView {
    let empty_board = Vec::from([Cell::new((0, 0))]);
    let mut bag = Bag::new().0;

    let board_signal = RwSignal::new(empty_board);
    let valid_signal = RwSignal::new(false);
    let played_signal = RwSignal::new(false);
    let score_signal = RwSignal::new(0);
    let word_signal = RwSignal::new(String::from(""));
    let in_rack_signal = RwSignal::new(false);
    let adjacent_signal = RwSignal::new(false);
    let scrabble_signal = RwSignal::new(false);
    let coord_signal = RwSignal::new((0, 0));
    let coord_x_signal = move || coord_signal.with(|coord| coord.0);
    let coord_y_signal = move || coord_signal.with(|coord| coord.1);
    let bag_signal = RwSignal::new(bag.to_vec());
    let rack_signal = RwSignal::new(Vec::new());

    let mut pick_tiles = move || {
        // let vec_size: u8 = rack_signal.with(|vec| vec.len() as u8);
        // amount.set(7 - vec_size);
        rack_signal.set(Bag::draw_tiles(7, &mut bag));
        bag_signal.set(bag.to_vec())
    };

    fn are_adjacent(coords: &[(usize, usize)]) -> bool {
        let mut sorted_coords = coords.to_vec();
        sorted_coords.sort();

        for i in 0..sorted_coords.len() - 1 {
            let (row1, col1) = sorted_coords[i];
            let (row2, col2) = sorted_coords[i + 1];

            if row1 == row2 && (col1 + 1 == col2 || col1 == col2 + 1) {
                continue; // Adjacent horizontally
            } else if col1 == col2 && (row1 + 1 == row2 || row1 == row2 + 1) {
                continue; // Adjacent vertically
            } else {
                return false;
            }
        }
        true
    }

    let validate = move || {
        let mut word = String::from("");
        let mut cells_coords = Vec::new();
        let mut in_rack = true;
        let mut scrabble = false;
        let mut rack_letters = rack_signal()
            .into_iter()
            .map(|tile| tile.0)
            .collect::<Vec<_>>();
        for cell in board_signal() {
            if (cell.letter_score)().1 != 9 {
                word.push((cell.letter_score)().0);
                cells_coords.push(cell.coord);
                if let Some(idx) = rack_letters
                    .iter()
                    .position(|l| *l == (cell.letter_score)().0)
                {
                    rack_letters.remove(idx);
                } else {
                    in_rack = false;
                }
            }
        }
        if rack_letters.is_empty() {
            scrabble = true
        };
        word_signal.set(word);
        in_rack_signal.set(in_rack);

        let adjacent = are_adjacent(&cells_coords);
        if adjacent {
            adjacent_signal.set(true);
        } else {
            adjacent_signal.set(false);
        }

        let mut score = 0;
        for cell in board_signal() {
            if (cell.letter_score)().1 != 9 {
                match cell.cell_kind {
                    CellKind::DoubleLetter => score += (cell.letter_score)().1 * 2,
                    CellKind::TripleLetter => score += (cell.letter_score)().1 * 3,
                    _ => score += (cell.letter_score)().1,
                }
            }
        }
        for cell in board_signal() {
            if (cell.letter_score)().1 != 9 {
                match cell.cell_kind {
                    CellKind::DoubleWord => score *= 2,
                    CellKind::TripleWord => score *= 3,
                    _ => (),
                }
            }
        }
        if scrabble {
            score += 50
        }
        score_signal.set(score);
        scrabble_signal.set(scrabble);

        if in_rack && adjacent {
            valid_signal.set(true);
        } else {
            valid_signal.set(false);
        }

        played_signal.set(true);
    };

    view! {

        <main class="container mx-auto pt-2 lg:p-5 grid grid-rows-2 lg:grid-rows-1 lg:grid-cols-2">
            <div class="flex justify-center">
                <Board board_signal coord_signal rack_signal/>
            </div>

            <div class="pl-5">
                <h1 class="hidden lg:block p-5 text-4xl font-bold dark:text-yellow-100 text-center">"KRABBELS"</h1>
                <h2 class="text-xs text-center p-2 mb-5 border-b-2 border-black dark:border-white dark:text-white">"A study project to learn further RUST, LEPTOS framework and TAILDWIND css."</h2>

                <div class=("hidden", move || !played_signal())>
                    <p class=("hidden", move || !valid_signal())>"✅ Le mot "<strong>{word_signal}</strong>" est correct !"</p>
                    <p class=("hidden", move || valid_signal())>"❌ Le mot "<strong>{word_signal}</strong>" est incorrect."</p>
                    <p class=("hidden", move || !valid_signal())>"Votre score est de "<strong>{score_signal}</strong>" points."</p>
                    <p>"Mot dans le chevalet? " {in_rack_signal}</p>
                    <p>"Lettres adjacentes? " {adjacent_signal}</p>
                    <p>"Scrabble? " {scrabble_signal} <span class=("hidden", move || !scrabble_signal())>"🥳"</span></p>
                </div>


                <Rack rack_signal bag_signal/>
                <p>"Case sélectionnée: ("{coord_x_signal}":"{coord_y_signal}")"</p>

                <button class="p-3 m-3 border-2 border-purple-400 bg-purple-300 rounded-md hover:border-purple-600 hover:shadow-lg"
                on:click=move |_| {pick_tiles()}>Piocher des lettres</button>

                <button class="p-3 m-3 border-2 border-purple-400 bg-purple-300 rounded-md hover:border-purple-600 hover:shadow-lg"
                on:click=move |_| {validate()}>Valider le coup</button>

            </div>

        </main>
    }
}
