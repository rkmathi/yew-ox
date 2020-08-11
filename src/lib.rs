#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    None,
    O,
    X,
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CellState::None => write!(f, "　"),
            CellState::O => write!(f, "O"),
            CellState::X => write!(f, "X"),
        }
    }
}

enum Msg {
    Put(usize),
    Reset,
}

struct Model {
    link: ComponentLink<Self>,
    cells: [CellState; 9],
    turn: CellState,
    won: CellState,
}

fn is_won(cells: &[CellState; 9]) -> bool {
    for cell in [CellState::O, CellState::X].iter() {
        if (cells[0] == *cell && cells[1] == *cell && cells[2] == *cell) ||
            (cells[3] == *cell && cells[4] == *cell && cells[5] == *cell) ||
            (cells[6] == *cell && cells[7] == *cell && cells[8] == *cell) ||
            (cells[0] == *cell && cells[3] == *cell && cells[6] == *cell) ||
            (cells[1] == *cell && cells[4] == *cell && cells[7] == *cell) ||
            (cells[2] == *cell && cells[5] == *cell && cells[8] == *cell) ||
            (cells[0] == *cell && cells[4] == *cell && cells[8] == *cell) ||
            (cells[2] == *cell && cells[4] == *cell && cells[7] == *cell) {
            return true;
        }
    }
    false
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::log("create");

        Self {
            link,
            cells: [CellState::None; 9],
            turn: CellState::O,
            won: CellState::None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::log("update");

        match msg {
            Msg::Put(position) => {
                ConsoleService::log(format!("Put({})", position).as_str());

                if self.won != CellState::None {
                    return true;
                }
                if self.cells[position] != CellState::None {
                    return true;
                }

                self.cells[position] = self.turn;

                if is_won(&self.cells) {
                    self.won = self.turn;
                    self.turn = CellState::None;
                }

                match self.turn {
                    CellState::None => return true,
                    CellState::O => self.turn = CellState::X,
                    CellState::X => self.turn = CellState::O,
                }
            }
            Msg::Reset => {
                ConsoleService::log("Reset");

                for cell in self.cells.iter_mut() {
                    *cell = CellState::None;
                }
                self.turn = CellState::O;
                self.won = CellState::None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        ConsoleService::log("change");

        false
    }

    fn view(&self) -> Html {
        ConsoleService::log("view");

        let draw_button = |pos, cell| html! {
            <button onclick=self.link.callback(move |_| Msg::Put(pos))>{ cell }</button>
        };

        let draw_board = |cells: [CellState; 9]| html! {
            <p>
                <table>
                    <tr>
                        <td>{ draw_button(0, cells[0]) }</td>
                        <td>{ draw_button(1, cells[1]) }</td>
                        <td>{ draw_button(2, cells[2]) }</td>
                    </tr>
                    <tr>
                        <td>{ draw_button(3, cells[3]) }</td>
                        <td>{ draw_button(4, cells[4]) }</td>
                        <td>{ draw_button(5, cells[5]) }</td>
                    </tr>
                    <tr>
                        <td>{ draw_button(6, cells[6]) }</td>
                        <td>{ draw_button(7, cells[7]) }</td>
                        <td>{ draw_button(8, cells[8]) }</td>
                    </tr>
                </table>
            </p>
        };

        let draw_message = |turn, won| {
            if self.won != CellState::None {
                let body = match won {
                    CellState::None => "",
                    CellState::O => "O won!",
                    CellState::X => "X won!",
                };
                return html! { <p>{ body }</p> };
            }

            let body = match turn {
                CellState::None => "",
                CellState::O => "O's turn",
                CellState::X => "X's turn"
            };
            html! { <p>{ body }</p> }
        };

        let draw_reset = || html! {
            <p>
                <button onclick=self.link.callback(|_| Msg::Reset)>{ "reset" }</button>
            </p>
        };


        html! {
            <div>
                <p>{ "OX game" }</p>
                { draw_board(self.cells) }
                { draw_message(self.turn, self.won) }
                { draw_reset() }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
