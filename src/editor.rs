use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use include_dir::include_dir;
use nih_plug::midi::{MidiResult, NoteEvent};
use nih_plug_webview::{
    http::Response, DropData, DropEffect, EventStatus, HTMLSource, Key, MouseEvent, WebViewEditor,
};
use rtrb::Consumer;
use serde_json::json;

pub fn create_editor(buffer: Consumer<NoteEvent<()>>) -> WebViewEditor {
    let output = Arc::new(Mutex::new(buffer));

    let size = (400, 400);

    let src = HTMLSource::URL("http://localhost:3000".to_owned());
    let mut editor = WebViewEditor::new(src, size);

    #[cfg(not(debug_assertions))]
    {
        editor = {
            let protocol_name = "assets";

            #[cfg(target_os = "windows")]
            let url_scheme = format!("http://{}.localhost", protocol_name);

            #[cfg(not(target_os = "windows"))]
            let url_scheme = format!("{}://localhost", protocol_name);

            let src = HTMLSource::URL(url_scheme);
            let mut editor = WebViewEditor::new(src, size);

            editor = editor.with_custom_protocol(protocol_name.to_string(), move |req| {
                let path = req.uri().path();
                let file = if path == "/" {
                    "index.html"
                } else {
                    &path[1..]
                };

                let dir = include_dir!("$CARGO_MANIFEST_DIR/target/bundled/dist");

                // mime guess is awesome!
                let mime_type =
                    mime_guess::from_ext(Path::new(file).extension().unwrap().to_str().unwrap())
                        .first_or_text_plain() // TODO: fix _or_...
                        .to_string();
                if let Some(result_file) = dir.get_file(file) {
                    return Response::builder()
                        .header("content-type", mime_type)
                        .header("Access-Control-Allow-Origin", "*")
                        .body(result_file.contents().into())
                        .map_err(Into::into);
                }
                panic!("Web asset not found.")
            });
            editor
        };
    }

    editor = editor
        .with_developer_mode(true)
        .with_keyboard_handler(move |event| {
            println!("keyboard event: {event:#?}");
            event.key == Key::Escape
        })
        .with_mouse_handler(|event| match event {
            MouseEvent::DragEntered { .. } => {
                println!("drag entered");
                EventStatus::AcceptDrop(DropEffect::Copy)
            }
            MouseEvent::DragMoved { .. } => {
                println!("drag moved");
                EventStatus::AcceptDrop(DropEffect::Copy)
            }
            MouseEvent::DragLeft => {
                println!("drag left");
                EventStatus::Ignored
            }
            MouseEvent::DragDropped { data, .. } => {
                if let DropData::Files(files) = data {
                    println!("drag dropped: {:?}", files);
                }
                EventStatus::AcceptDrop(DropEffect::Copy)
            }
            _ => EventStatus::Ignored,
        })
        .with_event_loop(move |ctx, _setter, _window| {
            while let Ok(_value) = ctx.next_event() {}

            // TODO:
            // does unwrap need to be fixed?
            let mut output_buffer_lock = output.lock().unwrap();

            if let Ok(raw_midi_data) = output_buffer_lock.pop() {
                if let Some(MidiResult::Basic(midi_data_array)) = raw_midi_data.as_midi() {
                    ctx.send_json(json!(midi_data_array)).unwrap();
                }
            }
        });
    editor
}
