use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::sleep_ms;
use std::thread;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

#[derive(Clone, Copy)]
struct Note {
    pub line_index: i32,
    pub row_index: i32,
    pub col: i32
} 

const rows: i32 = 40;
type Score = Arc<Mutex<i32>>;
type Notes = Arc<Mutex<Vec<Note>>>;


fn main() {
    clear_console();
    set_cursor(1, 1);

    let score: Score = Arc::new(Mutex::new(0));

    let green_col: [i32; 3] = [0, 255, 0];
    let red_col: [i32; 3] = [255, 0, 0];
    let yellow_col: [i32; 3] = [255, 255, 0];
    let blue_col: [i32; 3] = [0, 0, 255];
    let orange_col: [i32; 3] = [255, 165, 0];

    let notes: Notes = Arc::new(Mutex::new(Vec::new()));
    {
        let mut note = notes.lock().unwrap();
        note.push(Note { line_index: 0, row_index: 1, col: 9 });
        note.push(Note { line_index: 0, row_index: 10, col: 9 });
        note.push(Note { line_index: 1, row_index: 1, col: 9*3 - 2 });
    }


    hide_cursor();
    handle_input(Arc::clone(&notes), Arc::clone(&score));
    loop {
        set_cursor(1, 1);
        for row in 1..rows {
            set_cursor(1, row);
            print_color(0, 0, 0, "\t");
            let note_gard = notes.lock().unwrap();
            for x in 0..5 {

                if should_print_line(&note_gard, x, row) == false { 
                    print_color(0, 0, 0, "\t\t");
                    continue;
                }

                let mut char_to_print = "│";

                if row == rows - 1 {
                    char_to_print = "◼"
                }
                

                match x {
                    0 => { print_color(green_col[0], green_col[1], green_col[2], char_to_print);}
                    1 => { print_color(red_col[0], red_col[1], red_col[2], char_to_print);}
                    2 => { print_color(yellow_col[0], yellow_col[1], yellow_col[2], char_to_print);}
                    3 => { print_color(blue_col[0], blue_col[1], blue_col[2], char_to_print);}
                    4 => { print_color(orange_col[0], orange_col[1], orange_col[2], char_to_print)}
                    _ => {}

                }
                print_color(0, 0, 0, "\t\t");
            }
        }

        set_cursor(80, rows);
        {
            let arc_score = *score.lock().unwrap();
            print_color(255, 255, 255, format!("Score : {arc_score}").as_str());
        }


        print_notes(&notes, rows);
        sleep_ms(50);
    }
}


fn inbounds(line_index: i32, notes: &Notes, score: &Score) {

    let arc_notes = notes.lock().unwrap();

    for note in arc_notes.iter() {
        if note.line_index != line_index { continue; } 
        if note.row_index + 2 >= rows - 2 {
            let mut arc_score = score.lock().unwrap();
            *arc_score += 1;
            break;
       }
    }

}


fn print_notes(notes: &Notes, on_row: i32) {
    let mut to_remove: Vec<usize> = Vec::new();
    let mut index: usize = 0;
    let mut arc_note = notes.lock().unwrap();
    for note in arc_note.iter_mut() {
        set_cursor(note.col, note.row_index);
        print_color(255, 255, 255, "◈");

        note.row_index += 1;


        if note.row_index == on_row {
            note.row_index = 1;
            to_remove.push(index);
        }
        else{
            index += 1;
        }
    }


    for notes_to_rm in to_remove {
        arc_note.remove(notes_to_rm);
    }
}

fn handle_input(notes: Notes, score: Score) {
    thread::spawn(move || {
        loop {
            let event = event::read();

            match &event {
                Err(e) => { continue; }
                Ok(a) => { }
                _ => {}
            }


            if let Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Release,
                ..

            }) = event.unwrap()
            {
                match c {
                    'q' => {
                        show_cursor();
                        exit(1);
                    }
                    '1' => {
                        let mut note = notes.lock().unwrap();
                        note.push(Note { line_index: 0, row_index: 1, col: 9 });
                    }
                    '2' => {
                        let mut note = notes.lock().unwrap();
                        note.push(Note { line_index: 1, row_index: 1, col: 9*3 - 2 });
                    }
                    '3' => {
                        let mut note = notes.lock().unwrap();
                        note.push(Note { line_index: 2, row_index: 1, col: 9*4 + 5 });
                    }
                    '4' => {
                        let mut note = notes.lock().unwrap();
                        note.push(Note { line_index: 3, row_index: 1, col: 9*6 + 3 });
                    }
                    '5' => {
                        let mut note = notes.lock().unwrap();
                        note.push(Note { line_index: 4, row_index: 1, col: 9*8 + 1 });
                    }
                    'a' => {
                        let line_index = 0;
                        inbounds(line_index, &notes, &score);
                    }
                    's' => {
                        let line_index = 1;
                        inbounds(line_index, &notes, &score);
                    }
                    'f' => {
                        let line_index = 2;
                        inbounds(line_index, &notes, &score);
                    }
                    'g' => {
                        let line_index = 3;
                        inbounds(line_index, &notes, &score);
                    }
                    'h' => {
                        let line_index = 4;
                        inbounds(line_index, &notes, &score);
                    }
                    _ => { continue; }
                }
            }
        }

    });
}

fn should_print_line(notes: &[Note] ,col: i32, row: i32) -> bool {
    for note in notes.iter() {
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
