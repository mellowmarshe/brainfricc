use std::{collections::HashMap, env, fs, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = PathBuf::from(&args[1]);

    // We will default to this, it prints Hello World!
    let default = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";

    let code = fs::read_to_string(path).unwrap_or_else(|_| default.to_string());

    brainfricc(&code);
}

fn brainfricc(code: &str) {
    // Current cell position pointer
    let mut cellptr = 0;
    // A hashmap of the bracket pairs
    let brackets = map_brackets(code);
    // A vector of all cell values
    let mut cells: Vec<usize> = Vec::new();
    // Our current position in the code
    let mut codeptr = 0;

    while codeptr < code.len() {
        let c = code.chars().nth(codeptr).unwrap();

        /*
        Valid characters and what they do:

        >	increment pointer
        <	decrement pointer
        +	increment value at pointer
        -	decrement value at pointer
        [	begin loop (continues while value at pointer is non-zero)
        ]	end loop
        ,	read one character from input into value at pointer
        .	print value at pointer to output as a character
        */
        match c {
            '>' => {
                cellptr += 1;
            }
            '<' => {
                cellptr -= 1;
            }
            '+' => {
                cells.push(cellptr);
                cells[cellptr] = (cells[cellptr] + 1) % 256
            }
            '-' => {
                cells.push(cellptr);
                cells[cellptr] = (cells[cellptr] - 1) % 256
            }
            '[' => {
                if cells[cellptr] == 0 {
                    codeptr = brackets[&codeptr];
                }
            }
            ']' => {
                if cells[cellptr] != 0 {
                    codeptr = brackets[&codeptr]
                }
            }
            ',' => cells[cellptr] = c as usize,
            '.' => {
                print!("{}", (cells[cellptr] as u8) as char)
            }
            _ => {}
        }

        codeptr += 1;
    }
}

fn map_brackets(code: &str) -> HashMap<usize, usize> {
    // This will temporarily hold the position of an opening bracket
    let mut temp: Vec<usize> = Vec::new();
    // The hashmap of bracket pairs
    let mut brackets: HashMap<usize, usize> = HashMap::new();

    for (i, c) in code.chars().into_iter().enumerate() {
        match c {
            '[' => temp.push(i),
            ']' => {
                let start = temp.pop().unwrap();
                brackets.insert(start, i);
                brackets.insert(i, start);
            }

            _ => {}
        }
    }

    brackets
}
