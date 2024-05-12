use std::thread::sleep_ms;

#[derive(Clone, Copy)]
struct Note {
    pub line_index: i32,
    pub row_index: i32,
    pub row: i32
} 

fn main() {
    clear_console();
    set_cursor(1, 1);


    let rows = 40;

    let green_col: [i32; 3] = [0, 255, 0];
    let red_col: [i32; 3] = [255, 0, 0];
    let yellow_col: [i32; 3] = [255, 255, 0];
    let blue_col: [i32; 3] = [0, 0, 255];
    let orange_col: [i32; 3] = [255, 165, 0];

    let mut notes: Vec<Note> = Vec::new();
    notes.push(Note { line_index: 0, row_index: 1, row: 9 });
    notes.push(Note { line_index: 0, row_index: 10, row: 9 });
    notes.push(Note { line_index: 1, row_index: 1, row: 9*3 - 2 });

    hide_cursor();
    while true {
        set_cursor(1, 1);
        for row in 1..rows {
            set_cursor(1, row);
            print_color(0, 0, 0, "\t");
            for x in 0..5 {

                if should_print_line(&mut notes, x, row) == false { 
                    print_color(0, 0, 0, "\t\t");
                    continue;
                }

                let mut charToPrint = "│";

                if row == rows - 1 {
                    charToPrint = "◼"
                }

                match x {
                    0 => { print_color(green_col[0], green_col[1], green_col[2], charToPrint);}
                    1 => { print_color(red_col[0], red_col[1], red_col[2], charToPrint);}
                    2 => { print_color(yellow_col[0], yellow_col[1], yellow_col[2], charToPrint);}
                    3 => { print_color(blue_col[0], blue_col[1], blue_col[2], charToPrint);}
                    4 => { print_color(orange_col[0], orange_col[1], orange_col[2], charToPrint)}
                    _ => {}

                }
                print_color(0, 0, 0, "\t\t");
            }
        }


        print_notes(&mut notes, rows);


        sleep_ms(100);

    }
}



fn print_notes(notes: &mut Vec<Note>, on_row: i32) {
    for note in notes {
        set_cursor(note.row, note.row_index);
        print_color(255, 255, 255, "◈");

        note.row_index += 1;


        if note.row_index == on_row - 1{
            note.row_index = 1;
        }
    }
}

fn should_print_line(notes: &mut Vec<Note> ,col: i32, row: i32) -> bool {
    for note in notes {
        if note.line_index == col {
            if note.row_index == row {
                return  false;
            }
            if note.row_index == row+1 {
                return  false;
            }
        }
    }

    return true;
}

fn show_cursor(){
    print!("\x1B[?25h")
}

fn hide_cursor() { 
    print!("\x1B[?25l")
}

fn clear_console() {
    print!("\x1B[2J")
}


fn set_cursor(x: i32, y: i32){
    print!("\x1B[{};{}H", y, x)
}

fn println_color(r: i32, g: i32, b:i32, text: &str){
    println!("{}", format_color(r, g, b, text))
}

fn print_color(r: i32, g: i32, b: i32, text: &str){
    print!("{}", format_color(r, g, b, text))
}


fn format_color(r: i32, g: i32, b:i32, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}
