use rustyline::DefaultEditor;

use dirs::home_dir;

pub fn load(editor: &mut DefaultEditor) {

    let mut path = home_dir().unwrap();

    path.push(".astra_history");

    if path.exists() {
        let _ = editor.load_history(&path);
    }
}


pub fn save(editor: &mut DefaultEditor) {

    let mut path = home_dir().unwrap();

    path.push(".astra_history");

    let _ = editor.save_history(&path);
}
