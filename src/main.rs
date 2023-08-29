// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

mod eval;
mod lexer;
mod parser;
mod repl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    repl::start()
}
