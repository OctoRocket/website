use crate::types::{ Card, Slot, SlotState };

pub fn get_coords_from_id(id: &str) -> (usize, usize) {
  let mut id_chars = id.chars();
  let column = usize::from_str_radix(
      &id_chars
          .next()
          .unwrap_or_else(|| panic!("Column invalid! Malformed id: {id}"))
          .to_string(),36
  ).expect("Invalid ascii character for column!") - 10;
  let row = usize::from_str_radix(&id_chars
      .next()
      .unwrap_or_else(|| panic!("Row invalid! Malformed id: {id}"))
      .to_string(), 16
  ).expect("Invalid hex character for row!");

  (column, row)
}

pub fn slots_to_cards(slots: &[Slot]) -> Vec<Card> {
  slots.iter()
    .filter(|slot| slot.state.is_occupied())
    .map(|slot| {
        let SlotState::Occupied(card) = slot.state else {
            panic!("This should never happen");
        };
        card
    })
    .collect()
}
