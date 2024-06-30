window.onload = init;

const COLUMN_HEIGHT = 13;
const ROW_COUNT = 7;
const MAX_STACK_SIZE = 9;

let STARTED = false;

const PLAY_AREA = [
  ["a0", "b0", "c0", "d0", "e0", "f0", "g0"],
  ["a1", "b1", "c1", "d1", "e1", "f1", "g1"],
  ["a2", "b2", "c2", "d2", "e2", "f2", "g2"],
  ["a3", "b3", "c3", "d3", "e3", "f3", "g3"],
  ["a4", "b4", "c4", "d4", "e4", "f4", "g4"],
  ["a5", "b5", "c5", "d5", "e5", "f5", "g5"],
  ["a6", "b6", "c6", "d6", "e6", "f6", "g6"],
  ["a7", "b7", "c7", "d7", "e7", "f7", "g7"],
  ["a8", "b8", "c8", "d8", "e8", "f8", "g8"],
  ["a9", "b9", "c9", "d9", "e9", "f9", "g9"],
  ["aa", "ba", "ca", "da", "ea", "fa", "ga"],
  ["ab", "bb", "cb", "db", "eb", "fb", "gb"],
  ["ac", "bc", "cc", "dc", "ec", "fc", "gc"],
];

const SELECTED_LIST = [
  "selected-0",
  "selected-1",
  "selected-2",
  "selected-3",
  "selected-4",
  "selected-5",
  "selected-6",
  "selected-7",
  "selected-8",
  "selected-9",
  "selected-a",
  "selected-b",
  "selected-c",
];

const STACK_IDS = [
  "s0",
  "s1",
  "s2",
  "s3",
  "s4",
  "s5",
  "s6",
  "s7",
  "s8",
];

let STACK = [];

let CURRENT_SELECTION = {occupied: false, origin: undefined, content: []};

let CARDS = new Map();

["♠", "♥", "♦", "♣"].forEach(suit => {
  [...Array(13).keys()].map(n => n + 1).forEach(number => {
    let name;
    if (number == 1) {
      name = "A";
    } else if (number == 10) {
      name = "0"
    } else if (number == 11) {
      name = "J";
    } else if (number == 12) {
      name = "Q";
    } else if (number == 13) {
      name = "K";
    } else {
      name = number;
    }

    let color;
    if (["♥", "♦"].includes(suit)) {
      color = "red";
    } else {
      color = "black";
    }

    CARDS.set(name + suit, {name: name, number: number, suit: suit, color: color});
  });
});

let deck = shuffle(Array.from(CARDS.keys()));

function init() {
  PLAY_AREA.flat().forEach(element => {
    document.getElementById(element).textContent = "  ";
  });

  PLAY_AREA[0].forEach(element => {
    document.getElementById(element).textContent = "□□";
  })
}

function start() {
  [...Array(ROW_COUNT).keys()].reverse().forEach(row => {
    PLAY_AREA[row].slice(row).forEach(id => {
      let card = CARDS.get(deck.pop());
      change_slot(id, card);
    })
  })

  STARTED = true;
  update_blank_sections();
}

function deck_click() {
  if (!STARTED) {
    start();
  } else {
    if (deck.length > 0) {
      if (CURRENT_SELECTION.origin == "s0") {
        return_selection();
      }
      for (let index = 0; index < 3; index++) {
        STACK.unshift(deck.pop());
      }
      update_stack();
      if (deck.length == 0) {
        document.getElementById("deck").textContent = "□□"
        document.getElementById("deck").setAttribute("style", "padding-left: 0.3em; padding-right: 0.3em;")
      }
    } else if (document.getElementById("deck").textContent != "□□" && CURRENT_SELECTION.content.length == 0) {
      copy_to_selection(["deck"]);
      document.getElementById("deck").textContent = "□□";
      document.getElementById("deck").setAttribute("style", "padding-left: 0.3em; padding-right: 0.3em;");
    } else if (CURRENT_SELECTION.content.length == 1) {
      CURRENT_SELECTION.origin = "deck";
      return_selection();
    }
  }
}

function update_stack() {
  for (let index = 0; index < MAX_STACK_SIZE; index++) {
    let card = CARDS.get(STACK[index]);
    if (card != undefined) {
      change_slot(STACK_IDS[index], card);
    } else {
      document.getElementById(STACK_IDS[index]).textContent = "□□";
      document.getElementById(STACK_IDS[index]).setAttribute("style", "color: black;");
    }
  }
}

function change_slot(id, card) {
  document.getElementById(id).textContent = card.name + card.suit;
  document.getElementById(id).setAttribute("style", "color: " + card.color + ";");
  if (id == "deck") {
    document.getElementById("deck").setAttribute("style", "color: " + card.color + "; padding-left: 0.3em; padding-right: 0.3em;")
  }
} 

function update_blank_sections() {
  transpose(PLAY_AREA).forEach(column => {
    let first_blank_found = false;
    let already_updated = false;

    column.forEach(slot => {
      let content = document.getElementById(slot);
      if (content.textContent == "□□") {
        already_updated = true;
      }
      if (content.textContent == "  " && first_blank_found == false && already_updated == false) {
        first_blank_found = true;
        content.textContent = "□□";
        content.setAttribute("style", "color: black;");
      }
    })
  })
}

// Stolen from stack overflow lol
function transpose(array) {
  return array[0].map((_, colIndex) => array.map(row => row[colIndex]));
}

function shuffle(cards) {
  let deck = [];
  while (cards.length > 0) {
    let index = Math.floor(Math.random() * cards.length);
    deck.push(cards[index]);
    cards.splice(index, 1);
  }

  return deck;
}

function return_selection() {
  if (CURRENT_SELECTION.origin == undefined) {
    return;
  }
  if (!CURRENT_SELECTION.occupied) {
    return;
  }

  if (CURRENT_SELECTION.origin == "s0") {
    STACK.unshift(CURRENT_SELECTION.content[0].name + CURRENT_SELECTION.content[0].suit);
    clear_selection();
    update_stack();
  } else if (CURRENT_SELECTION.origin == "deck") {
    change_slot("deck", CURRENT_SELECTION.content[0]);
    clear_selection();
  } else {
    place_list_of_card_names(CURRENT_SELECTION.content, CURRENT_SELECTION.origin);
    clear_selection();
    update_blank_sections();
  }
}

function place_list_of_card_names(list, id) {
  list.forEach(function lambda(value, index) {
    change_slot(id[0] + (parseInt(id[1], 16) + index).toString(16), value)
  });
}

function stack_click(id) {
  if (id != "s0") {
    return;
  }
  if (document.getElementById(id).textContent == "□□") {
    return;
  }
  if (CURRENT_SELECTION.content.length > 0) {
    return;
  }

  copy_to_selection([id]);
  STACK.shift();
  update_stack();
}

function ace_pile(id) {
  if (CURRENT_SELECTION.content.length != 1) {
    return;
  }
  let card = CURRENT_SELECTION.content[0];
  if (card.number == 1 && document.getElementById(id).textContent == "□□") {
    change_slot(id, card);
    clear_selection();
  } else if (card.number == CARDS.get(document.getElementById(id).textContent).number + 1 &&
             card.suit == CARDS.get(document.getElementById(id).textContent).suit) {
    change_slot(id, card);
    clear_selection();
  }
}

function clicked(id) {
  if (document.getElementById(id).textContent != "□□") {
    if (!check_if_sequential(id) || CURRENT_SELECTION.occupied) {
      return;
    }
  
    clear_selection();
    copy_to_selection(get_ids_in_column_after_id(id[0], id));
    clear_after_id(id);
    update_blank_sections();
  } else if (CURRENT_SELECTION.occupied) {
    if (id[1] == "0") {
      place_list_of_card_names(CURRENT_SELECTION.content, id);
      clear_selection();
      update_blank_sections();
    } else {
      let top_card = CARDS.get(document.getElementById(id[0] + (parseInt(id[1], 16) - 1).toString(16)).textContent);
      let bottom_card = CURRENT_SELECTION.content[0];

      if (top_card.color != bottom_card.color && top_card.number == bottom_card.number + 1) {
        place_list_of_card_names(CURRENT_SELECTION.content, id);
        clear_selection()
        update_blank_sections();
      }
    }
  }
}

function clear_after_id(id) {
  [...Array(COLUMN_HEIGHT).keys()].slice(parseInt(id[1])).forEach(number => {
    document.getElementById(id[0] + number.toString(16)).textContent = "  ";
  });
}

function clear_selection() {
  CURRENT_SELECTION.content = [];
  CURRENT_SELECTION.occupied = false;
  CURRENT_SELECTION.origin = undefined;
  SELECTED_LIST.forEach(id => document.getElementById(id).textContent = "  ");
}

function copy_to_selection(ids) {
  let sublist = SELECTED_LIST.slice(0, ids.length);
  CURRENT_SELECTION.occupied = true;
  CURRENT_SELECTION.origin = ids[0];
  CURRENT_SELECTION.content = [];

  for (let index = 0; index < sublist.length; index++) {
    let card = CARDS.get(document.getElementById(ids[index]).textContent);
    CURRENT_SELECTION.content.push(card)
    change_slot(sublist[index], card);
  }
}

function get_ids_in_column(column) {
  let ids = [];
  let index = parseInt(column, 36) - 10;

  let transposed_play_area = transpose(PLAY_AREA);

  transposed_play_area[index].forEach(slot => {
    let content = document.getElementById(slot);
    if (!["  ", "□□"].includes(content.textContent)) {
      ids.push(slot)
    }
  });

  return ids;
}

function check_if_sequential(id) {
  if (["  ", "□□"].includes(document.getElementById(id).textContent)) {
    return false;
  }

  let order = get_ids_in_column_after_id(id[0], id)

  if (!is_alternating(order
    .map(id => CARDS.get(document.getElementById(id).textContent).color)
    .map(color => {if (color == "red") { return 1 } else { return 0 } }))) {
    return false;
  }

  order = order.map(id => CARDS.get(document.getElementById(id).textContent).number);
  let additives = ([...Array(order.length).keys()]);
  let sum = order.map(function combine(number, index) {
    return number + additives[index] == order[0] - additives[0];
  });

  if (sum.reduce(function add(a, b) { return a + b; }, 0) != sum.length) {
    return false;
  }

  return true;
}

function is_alternating(array) {
  if (array.length == 1) {
    return true;
  }

  return array[0] != array[1] && is_alternating(array.slice(1))
}

function get_ids_in_column_after_id(column, id) {
  all_ids_in_column = get_ids_in_column(column);

  return all_ids_in_column.slice(all_ids_in_column.indexOf(id));
}
