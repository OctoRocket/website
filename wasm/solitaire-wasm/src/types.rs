use crate::{shared_functions::slots_to_cards, ROWS, STACK_SIZE};
use crate::shared_functions::get_coords_from_id;

use serde::{ Serialize, Deserialize };
use std::mem::discriminant;

/// Representation of the entire game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    /// The cards that are still unused and can be pulled from the deck.
    pub available_cards: Vec<Card>,

    // The current state of the visible board
    /// By default this is None and clicking on the deck draws cards from
    /// `available_cards`, but all the available cards runs out it becomes a slot
    pub deck: Option<Slot>,
    /// The stack (it's not a queue ;3) is where the cards you draw from the
    /// deck go. The deck draws three at a time, cards are inserted at the
    /// beginning are are moved in a FILO manner.
    pub stack: Stack,
    /// This is where currently selected cards are stored.
    pub selection: Selection,
    /// This is the representation of the current main playing area.
    /// Columns -> Rows -> Slots
    pub playing_area: Vec<Vec<Slot>>,
    /// Aces are stored simply as a vector of slots.
    pub aces: Vec<Slot>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Card {
    number: u32,
    suit: Suit,
    color: Color,
}

impl Card {
    pub const fn new(number: u32, suit: Suit) -> Self {
        let color = suit.color();
        Self {
            number,
            suit,
            color,
        }
    }

    pub const fn get_color(self) -> Color {
        self.color
    }

    pub const fn get_number(self) -> u32 {
        self.number
    }

    pub const fn get_suit(self) -> Suit {
        self.suit
    }
}

// Default is for when you need a generic card but the value of the card doesn't
// matter.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Suit {
    #[default]
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    const fn color (self) -> Color {
        match self {
            Self::Spade | Self::Club    => Color::Black,
            Self::Heart | Self::Diamond => Color::Red,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Color {
    #[default]
    Black,
    Red,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SlotState {
    Occupied(Card),
    Empty,
    Blank,
}

impl SlotState {
    pub fn is_occupied(&self) -> bool {
        discriminant(self) == discriminant(&Self::Occupied(Card::default()))
    }

    pub const fn get_card(&self) -> &Card {
        if let Self::Occupied(card) = &self {
            card
        } else {
            panic!("Expected an occupied card slot but instead got empty or blank slot!");
        }
    }
}

impl From<Option<Card>> for SlotState {
    fn from(value: Option<Card>) -> Self {
        value.map_or(Self::Empty, Self::Occupied)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    id: String,
    pub state: SlotState,
}

impl Slot {
    pub const fn new(id: String, state: SlotState) -> Self {
        Self {
            id,
            state,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    available_slots: Vec<Slot>,
    contents: Vec<Card>,
}

impl Stack {
    pub fn new() -> Self {
        let available_slots = vec!["s".to_string(); STACK_SIZE as usize]
            .into_iter()
            .enumerate()
            .map(|id| Slot::new(id.1 + &id.0.to_string(), SlotState::Empty))
            .collect();

        Self {
            available_slots,
            contents: vec![],
        }
    }

    fn update_stack_slots(&mut self) {
        for index in 0..self.available_slots.len() {
            match self.contents.get(index) {
            Some(card) => self.available_slots[index].state = SlotState::Occupied(*card),
            None => self.available_slots[index].state = SlotState::Empty,
            }
        }
    }

    pub fn push(&mut self, card: Card) {
        self.contents.insert(0, card);

        self.update_stack_slots();
    }

    pub fn pop(&mut self) -> Card {
        let card = self.contents.remove(0);

        self.update_stack_slots();

        card
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selection {
    /// `None` when empty
    pub origin: Origin,
    contents: Vec<Slot>,
}

impl Selection {
    pub fn new(origin: Origin, contents: Vec<Slot>) -> Self {
        Self {
            origin,
            contents,
        }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        slots_to_cards(&self.contents)
    }

    pub fn set_contents(&mut self, origin: Origin, cards: Vec<Card>) {
        if !self.origin.is_none() {
            return;
        }

        self.origin = origin;

        for slot in cards.into_iter().zip(&mut self.contents) {
            slot.1.state = SlotState::Occupied(slot.0);
        }
    }

    pub fn clear(&mut self) {
        self.origin = Origin::None;
        for slot in &mut self.contents {
            slot.state = SlotState::Blank;
        }
    }

    pub fn len(&self) -> usize {
        self.get_cards().len()
    }

    pub fn is_empty(&self) -> bool {
        self.get_cards().is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Origin {
    None,
    Deck,
    Stack,
    PlayingAreaId(String),
}

impl Origin {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }

    pub fn has_id(&self) -> bool {
        discriminant(self) == discriminant(&Self::PlayingAreaId(String::default()))
    }

    pub fn unwrap(self) -> String {
        if let Self::PlayingAreaId(id) = self {
            id
        } else {
            panic!("Failed to unwrap origin! Instead of an origin with an id, got origin of type: {self:?}");
        }
    }
}

pub trait PlayingArea {
    fn get_from_id(&self, id: &str) -> &Slot;

    fn get_from_id_mut(&mut self, id: &str) -> &mut Slot;

    fn clear_slots(&mut self, ids: Vec<String>);

    fn update_empty_slots(&mut self);

    fn set_slot_state_at_coord(&mut self, column: usize, row: usize, state: SlotState);

    fn add_at_id(&mut self, id: String, to_add: Vec<Card>);
}

impl PlayingArea for Vec<Vec<Slot>> {
    fn get_from_id(&self, id: &str) -> &Slot {
        let (column, row) = get_coords_from_id(id);

        self
            .get(column)
            .unwrap_or_else(|| panic!("Can't get column number: {column}!"))
            .get(row)
            .unwrap_or_else(|| panic!("Can't get row number: {row}!"))
    }

    fn get_from_id_mut(&mut self, id: &str) -> &mut Slot {
        let (column, row) = get_coords_from_id(id);

        self
            .get_mut(column)
            .unwrap_or_else(|| panic!("Can't get column number: {column}!"))
            .get_mut(row)
            .unwrap_or_else(|| panic!("Can't get row number: {row}!"))
    }

    fn clear_slots(&mut self, ids: Vec<String>) {
        for id in ids {
            self.get_from_id_mut(&id).state = SlotState::Blank;
        }
    }

    fn update_empty_slots(&mut self) {
        for column in self {
            if column[0].state == SlotState::Blank {
                column[0].state = SlotState::Empty;
            }
            for row in 1..column.len() {
                if column[row - 1].state.is_occupied()
                && column[row].state == SlotState::Blank {
                    column[row].state = SlotState::Empty;
                }
            }
        }
    }

    fn set_slot_state_at_coord(&mut self, column: usize, row: usize, state: SlotState) {
        self
            .get_mut(column).unwrap_or_else(|| panic!("Column index: {column} is too large!"))
            .get_mut(row).unwrap_or_else(|| panic!("Row index: {row} is too large!")).state = state;
    }

    fn add_at_id(&mut self, id: String, mut to_add: Vec<Card>) {
        to_add.reverse();

        let (origin_column, origin_row) = get_coords_from_id(&id);
        if origin_row + to_add.len() > ROWS as usize {
            return;
        }

        for row in 0..to_add.len() {
            self.set_slot_state_at_coord(origin_column, row + origin_row, SlotState::Occupied(to_add.pop().unwrap()));
        }
    }
}
