use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use nih_plug::midi::{MidiResult, NoteEvent};
use nih_plug_webview::{
    editors::add_asset_dir_protocol, DropData, DropEffect, EventStatus, HTMLSource, Key,
    MouseEvent, WebViewEditor,
};
use rtrb::Consumer;
use serde_json::json;

pub fn create_editor(buffer: Consumer<NoteEvent<()>>) -> WebViewEditor {
    let output = Arc::new(Mutex::new(buffer));

    let size = (400, 400);

    #[cfg(debug_assertions)]
    let src = HTMLSource::URL("http://localhost:3000".to_owned());

    let mut editor = WebViewEditor::new(src, size);

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
