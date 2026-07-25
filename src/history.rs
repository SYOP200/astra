use rustyline::Editor;
use rustyline::history::DefaultHistory;

use dirs::home_dir;

use crate::completion::AstraCompleter;


pub fn load(
    editor: &mut Editor<AstraCompleter, DefaultHistory>
) {

    let mut path = home_dir().unwrap();

    path.push(".astra_history");


    if path.exists() {
        let _ = editor.load_history(&path);
    }
}


pub fn save(
    editor: &mut Editor<AstraCompleter, DefaultHistory>
) {

    let mut path = home_dir().unwrap();

    path.push(".astra_history");


    let _ = editor.save_history(&path);

}
