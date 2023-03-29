use std::io::stdin;

use super::lexer::{Token, Tokens};
use super::DEFAULT_CELL_COUNT;

pub type InterpreterResult = Result<(), (&'static str, usize)>;

#[derive(Debug, Clone)]
pub struct Interpreter {
    tokens: Tokens,
    loops: Vec<(usize, usize)>,

    /// Memory
    cells: Vec<u8>,

    /// Cell pointer
    ptr: usize,
}

impl Interpreter {
    /// Creates a new instance of Interpreter, cell count is: DEFAULT_CELL_COUNT
    pub fn new(tokens: Tokens) -> Self {
        let mut v = Vec::with_capacity(DEFAULT_CELL_COUNT);
        v.resize(v.capacity(), 0);

        Self {
            tokens,
            cells: v,
            ptr: 0,
            loops: Vec::new()
        }
    }

    /// cells with custom capacity
    pub fn with_cells(tokens: Tokens, cell_count: usize) -> Self {
        let mut v = Vec::with_capacity(cell_count);
        v.resize(cell_count, 0);

        Self {
            tokens,
            cells: v,
            ptr: 0,
            loops: Vec::new()
        }
    }

    /// execute the code
    pub fn interpret(&mut self) -> InterpreterResult {
        let mut idx = 0usize;
        while idx < self.tokens.0.len() {
            let res = match self.tokens.0[idx] {
                Token::Increment  => self.increment(),
                Token::Decrement  => self.decrement(),
                Token::Input      => self.input(),
                Token::Output     => self.output(),
                Token::ShiftRight => self.shift_right(),
                Token::ShiftLeft  => self.shift_left(),

                Token::LoopStart  => self.loop_start(&mut idx),
                Token::LoopEnd    => self.loop_end(&mut idx),

                _ => unreachable!()
            };

            if res.is_err() { return Err(res.unwrap_err()); }

            idx += 1;
        }

        Ok(())
    }

    /// <
    fn shift_left(&mut self) -> InterpreterResult {
        if self.ptr == 0 { return Err(("Cannot go past the first cell", self.ptr)); }

        self.ptr -= 1;
        Ok(())
    }

    /// >
    fn shift_right(&mut self) -> InterpreterResult {
        if self.ptr == self.cells.len() { return Err(("out of memory", self.ptr)); }

        self.ptr += 1;
        Ok(())
    }

    /// +
    fn increment(&mut self) -> InterpreterResult {
        let current_cell = &mut self.cells[self.ptr];

        if *current_cell == 255 { return Err(("Cannot increment more than 255", self.ptr)); }

        *current_cell += 1;
        Ok(())
    }

    /// -
    fn decrement(&mut self) -> InterpreterResult {
        let current_cell = &mut self.cells[self.ptr];

        if *current_cell == 0 { return Err(("Cannot decrement more than zero", self.ptr)); }

        *current_cell -= 1;
        Ok(())
    }

    /// .
    fn output(&self) -> InterpreterResult {
        let current_cell = self.cells[self.ptr];

        print!("{}", current_cell as char);
        Ok(())
    }

    /// ,
    fn input(&mut self) -> InterpreterResult {
        let current_cell = self.cells[self.ptr];

        let mut buffer = String::new();

        if let Err(_) = stdin().read_line(&mut buffer) {
            return Err(("Failed to get user input", self.ptr));
        }

        let Ok(current_cell): Result<u8, _> = buffer.trim().parse() else {
            return Err(("Cannot parse user input", self.ptr));
        };

        Ok(())
    }

    /// [
    fn loop_start(&mut self, idx: &mut usize) -> InterpreterResult {
        self.loops.push((*idx, self.ptr));
        Ok(())
    }

    /// ]
    fn loop_end(&mut self, idx: &mut usize) -> InterpreterResult {
        if !self.loops.is_empty() {
            let (loop_start, loop_ptr) = *self.loops.last().unwrap();

            if self.cells[loop_ptr] != 0 {
                *idx = loop_start;
            } else {
                self.loops.pop();
            }

            return Ok(());
        }

        Err(("Trailing loop end found", *idx))
    }
}
