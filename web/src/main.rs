use faye::prelude::*;
use maud::html;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, KeyboardEvent};

use eval::eval;
use highlight::highlight;

mod element;
mod eval;
mod highlight;
mod renderer;

fn main() {
    let mut ctx = Context::default();
    let root = element::Element::root();

    let mut input_history = Vec::<String>::new();
    let mut i = 0;

    let terminal = root.push(html! {
        #terminal {
            .header {
                span.faye { "faye" } " playground ♡ "
                a href="https://codeberg.org/fawn/faye" { "faye" } ", "
                a href="https://codeberg.org/faye/web" { "website" }
                br;
                "press " span.key { "ctrl + c" } " to clear input"
            }
            br;
        }
    });

    let history = terminal.push(html! { .history {} });
    let command = terminal.push(html! { #command { .prompt { "λ" } } });
    // TODO: highlight command input
    let cmd = command.push(html! {
        input
            #command_input
            name="command"
            autofocus
            autocorrect="off"
            autocapitalize="off"
            autocomplete="off"
            spellcheck="false";
    });
    let help_hint = terminal.push(html! {
        .help-hint { "type help for playground commands" }
    });

    let cmd_input = cmd.inner.clone().unchecked_into::<HtmlInputElement>();

    cmd.clone().listen("keydown", move |e: KeyboardEvent| {
        cmd.scroll_into_view();

        match e.key().as_str() {
            "c" if e.ctrl_key() => cmd_input.set_value(""),
            "Enter" => {
                help_hint.inner.set_hidden(true);
                let input = cmd_input.value();
                cmd_input.set_value("");

                input_history.push(input.clone());
                i = input_history.len();

                // builtin playground commands
                let output = match input.as_str() {
                    "clear" => {
                        history.inner.set_inner_html("");
                        return;
                    }
                    "help" => html! {
                        "faye playground" br;
                        br;
                        span style="color: var(--thorns-yellow)" { "commands:" } br;
                        span style="color: var(--thorns-green)" {
                            "  help" br;
                            "  clear" br;
                            "  symbols" br;
                        }
                    },
                    "symbols" => html! {
                        "symbols available in scope" br;
                        br;
                        span style="color: var(--thorns-purple)" {
                            @for (i, sym) in ctx.list_globals().into_iter().enumerate() {
                                @let s = sym.0;
                                @match (12_usize).checked_sub(s.chars().count()) {
                                    Some(w) if i % 4 != 3 => (s) (" ".repeat(w + 4)),
                                    _ => (s) br;
                                }
                            }
                        }
                    },
                    s => eval(&mut ctx, s),
                };

                history.push(html! {
                    .history-entry {
                        .history-command { .prompt { "λ" } .history-input { (highlight(&input)) } }
                        .history-output { (output) }
                    }
                });

                cmd.scroll_into_view();
            }
            "ArrowUp" => {
                if i > 0 {
                    i -= 1;
                    cmd_input.set_value(&input_history[i]);
                }
            }
            "ArrowDown" => {
                if i < input_history.len() - 1 {
                    i += 1;
                    cmd_input.set_value(&input_history[i]);
                }
            }
            _ => {}
        }
    });
}
