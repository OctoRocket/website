import init, { start, click_element, debug_delete_selection } from "./solitaire-wasm/pkg/solitaire_wasm.js";

await init();

let board_state = start();

update_board_state();

function construct_name(number, suit) {
  let card_name = number.toString();

  switch (number) {
    case 1:
      card_name = "A";
      break;
    case 10:
      card_name = "0";
      break;
    case 11:
      card_name = "J";
      break;
    case 12:
      card_name = "Q";
      break;
    case 13:
      card_name = "K";
      break;
    }

    switch (suit) {
      case "Spade":
        card_name += "♠";
        break;
      case "Heart":
        card_name += "♥";
        break;
      case "Diamond":
        card_name += "♦";
        break;
      case "Club":
        card_name += "♣";
        break;
    }

  return card_name;
}

function set_slot(slot) {
  let slot_data = document.getElementById(slot.id);
  if (slot.state == "Blank") {
    slot_data.textContent = "  ";
  } else if (slot.state == "Empty") {
    slot_data.textContent = "□□";
    slot_data.setAttribute("style", "color: black;");
  } else {
    slot_data.textContent = construct_name(slot.state.Occupied.number, slot.state.Occupied.suit);
    slot_data.setAttribute("style", "color: " + slot.state.Occupied.color + ";");
  }
}

function update_board() {
  board_state.playing_area.flat().forEach(slot => {
    set_slot(slot);
  });
}

function update_selection() {
  board_state.selection.contents.forEach(slot => {
    set_slot(slot);
  })
}

function update_stack() {
  board_state.stack.available_slots.forEach(function a(value, index) {
    let slot = value;

    let contents = board_state.stack.contents[index];
    if (contents == undefined) {
      slot.state = "Empty";
    } else {
      slot.state = { Occupied: { number: contents.number, suit: contents.suit, color: contents.color } };
    }

    set_slot(slot);
  })
}

function update_deck() {
  if (board_state.deck != undefined) {
    set_slot(board_state.deck);
    document.getElementById(board_state.deck.id).setAttribute("style", "padding: 0.3em;");
  }
}

function update_aces() {
  board_state.aces.forEach(slot => {
    set_slot(slot);
  })
}

function update_board_state() {
  update_board();
  update_selection();
  update_stack();
  update_deck();
  update_aces();
}

export function clicked(id) {
  board_state = click_element(id, board_state);
  update_board_state();
}

export function cheat_delete_selection() {
  board_state = debug_delete_selection(board_state);
  update_board_state();
}

// Add function to main window
window.clicked = clicked;
window.cheat_delete_selection= cheat_delete_selection;
