use std::{collections::HashMap, env, fs, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = PathBuf::from(&args[1]);

    let code = fs::read_to_string(path).expect("");

    brainfricc(&code);
}

fn brainfricc(code: &str) {
    // Current cell position pointer
    let mut cell_ptr = 0;
    // A hashmap of the bracket pairs
    let brackets = map_brackets(code);
    // A vector of all cell values
    let mut cells: Vec<usize> = Vec::new();
    // Our current position in the code
    let mut code_ptr = 0;

    while code_ptr < code.len() {
        let c = code.chars().nth(code_ptr).unwrap();

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
                cell_ptr += 1;
            }
            '<' => {
                cell_ptr -= 1;
            }
            '+' => {
                cells.push(cell_ptr);
                cells[cell_ptr] = (cells[cell_ptr] + 1) % 256
            }
            '-' => {
                cells.push(cell_ptr);
                cells[cell_ptr] = (cells[cell_ptr] - 1) % 256
            }
            '[' => {
                if cells[cell_ptr] == 0 {
                    code_ptr = brackets[&code_ptr];
                }
            }
            ']' => {
                if cells[cell_ptr] != 0 {
                    code_ptr = brackets[&code_ptr]
                }
            }
            ',' => cells[cell_ptr] = c as usize,
            '.' => {
                print!("{}", (cells[cell_ptr] as u8) as char)
            }
            _ => {}
        }

        code_ptr += 1;
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
