#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
)]

mod shared_functions;
use shared_functions::{ get_coords_from_id, slots_to_cards };

mod types;
use types::*;

use rand::{prelude::*, thread_rng};
use std::{cmp::Ordering, ops::RangeInclusive, panic};
use wasm_bindgen::prelude::*;

/// Ace (1) to king (13)
const CARD_RANGE: RangeInclusive<u32> = 1..=13;
const COLUMNS: u32 = 7;
const ROWS: u32 = 13;
const STACK_SIZE: u32 = 9;
const SUIT_COUNT: u32 = 4;
const SUITS: &[Suit] = &[Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

// For debugging
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str); // Change this type if needed
}

/// Stolen from the rust wasm-bindgen guide
/// Marked as allow unused because it's useful for debug purposes but that's
/// about it.
#[allow(unused_macros)]
#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn start() -> Result<JsValue, serde_wasm_bindgen::Error> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut available_cards = generate_card_deck();
    let stack = Stack::new();
    let selection = generate_selection_slots();
    let playing_area = generate_starting_board(&mut available_cards);
    let aces = generate_aces();

    let board_state = Board {
        available_cards,
        deck: None,
        stack,
        selection,
        playing_area,
        aces,
    };

    serde_wasm_bindgen::to_value(&board_state)
}

fn generate_card_deck() -> Vec<Card> {
    let mut rng = thread_rng();
    let mut deck = vec![];

    for suit in SUITS {
        for number in CARD_RANGE {
            deck.push(Card::new(number, *suit));
        }
    }
    deck.shuffle(&mut rng);

    deck
}

fn generate_starting_board(deck: &mut Vec<Card>) -> Vec<Vec<Slot>> {
    let mut columns = vec![];
    for column_index in 1..=COLUMNS {
        let mut column = vec![];
        for row_index in 0..ROWS {
            let id = unsafe {
                char::from_u32_unchecked(96 + column_index)
            }.to_string() + &format!("{row_index:x}");

            match row_index.cmp(&column_index) {
                Ordering::Less => column.push(Slot::new(id, SlotState::from(deck.pop()))),
                Ordering::Equal => column.push(Slot::new(id, SlotState::Empty)),
                Ordering::Greater => column.push(Slot::new(id, SlotState::Blank)),
            }
        }
        columns.push(column);
    }

    columns
}

fn generate_selection_slots() -> Selection {
    let mut slots = vec![];
    for i in 0..COLUMNS {
        slots.push(Slot::new("selected".to_string() + &i.to_string(), SlotState::Blank));
    }

    Selection::new(Origin::None, slots)
}

fn generate_aces() -> Vec<Slot> {
    let mut aces = vec![];

    for index in 0..SUIT_COUNT {
        aces.push(Slot::new("ace".to_string() + &index.to_string(), SlotState::Empty));
    }

    aces
}

#[wasm_bindgen]
pub fn click_element(id: JsValue, board_state: JsValue) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let split_id = {
        let mut tmp = serde_wasm_bindgen::from_value::<String>(id.clone()).unwrap();
        (tmp.pop(), tmp)
    };
    let old_state = serde_wasm_bindgen::from_value(board_state).unwrap();

    // if the id prefix length is one then it is the column specifier of the
    // playing board
    serde_wasm_bindgen::to_value(&if split_id.1.len() == 1 {
        playing_area_clicked(old_state, &serde_wasm_bindgen::from_value::<String>(id).unwrap())
    } else {
        match split_id.1.as_str() {
            "deck"      => deck_click(old_state),
            "return"    => return_click(old_state),
            "selection" => stack_click(old_state),
            "ace"       => ace_click(old_state, split_id.0.unwrap()),
            bad_id => panic!("Invalid ID: {bad_id}"),
        }
    })
}

// Handles putting cards in the ace pile
fn ace_click(old_state: Board, id: char) -> Board {
    let mut new_state = old_state;
    let index = id.to_digit(10).unwrap() as usize;

    if new_state.selection.get_cards().len() != 1 {
        return new_state;
    }

    if new_state.aces[index].get_state() == &SlotState::Empty
    && new_state.selection.get_cards()[0].get_number() == 1
    ||(new_state.selection.get_cards()[0].get_suit()
    == new_state.aces[index].get_state().get_card().get_suit()
    && new_state.selection.get_cards()[0].get_number() - 1
    == new_state.aces[index].get_state().get_card().get_number()
    ) {
        new_state.aces[index].set_state(SlotState::Occupied(new_state.selection.get_cards()[0]));
        new_state.selection.clear();
    }

    new_state
}

// Handles moving cards out of the stack
fn stack_click(old_state: Board) -> Board {
    let mut new_state = old_state;

    if !new_state.selection.get_origin().is_none() {
        return new_state;
    }

    new_state.selection.set_contents(Origin::Stack, vec![new_state.stack.pop()]);

    new_state
}

// Handles return the selection to the origin point
fn return_click(old_state: Board) -> Board {
    let mut new_state = old_state;

    if new_state.selection.get_origin().is_none() {
        return new_state;
    }

    if new_state.selection.get_origin().has_id() {
        new_state.playing_area.add_at_id(new_state.selection.get_origin().clone().unwrap(), new_state.selection.get_cards());
        new_state.playing_area.update_empty_slots();
    } else {
        new_state.stack.push(new_state.selection.get_cards()[0]);
    }

    new_state.selection.clear();
    new_state
}

// Handlers for the click behaviors
fn deck_click(old_state: Board) -> Board {
    let mut new_state = old_state;

    if new_state.deck.is_none() {
        for _ in 0..3 {
            new_state.stack.push(new_state.available_cards.pop().expect("WRONG NUMBER OF CARDS IN THE DECK"));
        }
    } else {
        // TODO: implement holding ability!
        todo!()
    }

    if new_state.available_cards.is_empty() {
        new_state.deck = Some(Slot::new("deck0".to_string(), SlotState::Empty));
    }

    new_state
}

fn playing_area_clicked(old_state: Board, id: &str) -> Board {
    let mut new_state = old_state;
    let (column, row) = get_coords_from_id(id);

    if new_state.playing_area.get_from_id(id).get_state().is_occupied() {
        // Make sure that selection isn't already occupied
        if !new_state.selection.get_cards().is_empty() {
            return new_state
        }
        // Check if get list of the cards in that column
        let selection = slots_to_cards(&new_state.playing_area[column][row..]);

        if !alternating(&selection) || !decreasing(&selection) {
            return new_state;
        }

        new_state.selection.set_contents(Origin::PlayingAreaId(new_state.playing_area.get_from_id(id).get_id().to_string()), selection);

        new_state.playing_area.clear_slots(new_state.playing_area
            .get(column)
            .unwrap()[row..]
            .iter()
            .map(|slot| slot.get_id().to_string())
            .collect()
        );
    } else if new_state.playing_area.get_from_id(id).get_state() == &SlotState::Empty {
        if new_state.selection.get_origin().is_none() {
            return new_state;
        }

        // Try to place cards onto the blank spot
        let coords = get_coords_from_id(id);
        if coords.1 == 0 || (
               new_state.playing_area[coords.0][coords.1 - 1].get_state().get_card().get_color()
            != new_state.selection.get_cards()[0].get_color()
            && new_state.playing_area[coords.0][coords.1 - 1].get_state().get_card().get_number() - 1
            == new_state.selection.get_cards()[0].get_number()
        ) {
            new_state.playing_area.add_at_id(id.to_string(), new_state.selection.get_cards());
            new_state.selection.clear();
        }
    }

    new_state.playing_area.update_empty_slots();
    new_state
}

fn alternating(list: &[Card]) -> bool {
    if list.len() == 1 {
        return true;
    }

    list[0].get_color() != list[1].get_color() && alternating(&list[1..])
}

fn decreasing(list: &[Card]) -> bool {
    if list.len() == 1 {
        return true;
    }

    list[0].get_number() - 1 == list[1].get_number() && decreasing(&list[1..])
}

#[wasm_bindgen]
pub fn debug_delete_selection(board_state: JsValue) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let mut board = serde_wasm_bindgen::from_value::<Board>(board_state).unwrap();
    board.selection.clear();

    serde_wasm_bindgen::to_value(&board)
}
