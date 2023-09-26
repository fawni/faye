use faye::prelude::*;
use maud::html;
use web_sys::{Event, KeyboardEvent};

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
    let mut saved_entry = String::new();
    let mut i = 0;

    let terminal = root.push(html! {
        #terminal {
            header {
                span.faye { "faye" } " playground ♡ "
                a href="https://codeberg.org/fawn/faye" { "faye" } ", "
                a href="https://codeberg.org/fawn/faye/src/branch/master/web" { "website" }
                br;
                "press " span.key { "ctrl + c" } " to clear input"
            }
            br;
        }
    });

    let history = terminal.push(html! { .history {} });
    let command = terminal.push(html! { #command { span.prompt { "λ " } } });
    let cmd_display = command.push(html! {
        span #command_display {}
    });
    let cmd_input = command.push(html! {
        span
            #command_input
            contenteditable
            autocorrect="off"
            autocapitalize="off"
            autocomplete="off"
            spellcheck="false" {}
    });

    let help_hint = terminal.push(html! {
        .help-hint { "type help for playground commands" }
    });

    {
        let cmd_input = cmd_input.clone();
        terminal.listen("click", move |_: Event| {
            cmd_input.inner.focus().unwrap_or(())
        });
    }

    {
        let cmd_input = cmd_input.clone();
        let cmd_display = cmd_display.clone();
        cmd_input.clone().listen("input", move |_: Event| {
            cmd_display.update(highlight(&cmd_input.inner.inner_text()));
        });
    }

    cmd_input.inner.focus().ok();

    cmd_input
        .clone()
        .listen("keydown", move |e: KeyboardEvent| {
            let input = cmd_input.text();

            cmd_input.scroll_into_view();

            match e.key().as_str() {
                "c" if e.ctrl_key() => {
                    cmd_input.update("");
                    cmd_display.update("");
                }
                "Enter" => {
                    e.prevent_default();
                    cmd_input.update("");
                    cmd_display.update("");
                    help_hint.inner.set_hidden(true);

                    // builtin playground commands
                    let output = match input.as_str() {
                        "clear" => {
                            history.update("");
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
                            br;
                        },
                        s => eval(&mut ctx, s),
                    };

                    history.push(html! {
                        span.prompt { "λ " } (highlight(&input)) br;
                        (output)
                    });

                    input_history.push(input);
                    i = input_history.len();

                    cmd_input.scroll_into_view();
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if i == input_history.len() {
                        saved_entry = input;
                    }
                    i = i.saturating_sub(1);
                    let input = input_history.get(i).unwrap_or(&saved_entry);
                    cmd_display.update(highlight(input));
                    cmd_input.update(input);
                    cmd_input.set_cursor(1);
                }
                "ArrowDown" => {
                    e.prevent_default();
                    if i < input_history.len() {
                        i += 1;
                        let input = input_history.get(i).unwrap_or(&saved_entry);
                        cmd_display.update(highlight(input));
                        cmd_input.update(input);
                        cmd_input.set_cursor(1);
                    }
                }
                _ => {}
            }
        });
}
