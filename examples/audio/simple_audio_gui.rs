use core::time::Duration;
use nannou::prelude::*;
use nannou::ui::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;
use nannou::Ui;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

// GOALS
// -> 1 knob/slider controls global tempo
// -> 1 panel for 1 instr
//   -> knobs/slider for:
//      -> vol
//      -> hz
//      -> attack
//      -> decay
//      -> time division

// NEXT
// add a slider to the GUI
// -> print out values as slider adjusts
// -> adjust note rate with slider
// play notes at a rate, rather than a continuous tone
// tempo LED flashes
// -> time division LED flashes
// arps
// -> perhaps slider chooses maj7 / m7 / 7 / m7b5
//      -> convert freeform hz to snap to notes

struct Model {
    stream: audio::Stream<Audio>,
    ui: Ui,
    ids: Ids,
    resolution: usize,
    scale: f32,
    rotation: f32,
    color: Rgb,
    position: Point2
}

struct Audio {
    phase: f64,
    hz: f64,
    vol: f32
}

struct Ids {
    resolution: widget::Id,
    scale: widget::Id,
    rotation: widget::Id,
    random_color: widget::Id,
    position: widget::Id,
}

fn model(app: &App) -> Model {
    // println!("model() called...");

    // Initialise the audio API so we can spawn an audio stream.
    let audio_host = audio::Host::new();
    // Initialise the state that we want to live on the audio thread.
    let audio_model = Audio {
        phase: 0.0,
        hz: 440.0,
        vol: 0.5
    };
    let stream = audio_host
        .new_output_stream(audio_model)
        .render(audio)
        .build()
        .unwrap();

    // Create the UI
    let mut ui = app.new_ui().build().unwrap();

    // TODO: is this appropriate/necessary?
    // app.set_loop_mode(LoopMode::Rate {
    //     update_interval: Duration::from_millis(1),
    // });

    // Generate some ids for our widgets.
    let ids = Ids {
        resolution: ui.generate_widget_id(),
        scale: ui.generate_widget_id(),
        rotation: ui.generate_widget_id(),
        random_color: ui.generate_widget_id(),
        position: ui.generate_widget_id(),
    };

    // Init our variables
    let resolution = 6;
    let scale = 200.0;
    let rotation = 0.0;
    let position = pt2(0.0, 0.0);
    let color = rgb(1.0, 0.0, 1.0);

    Model {
        stream,
        ui,
        ids,
        resolution,
        scale,
        rotation,
        position,
        color
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // println!("update() called...");

    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.resolution as f32, 3.0, 15.0)
        .top_left_with_margin(20.0)
        .label("Resolution")
        .set(model.ids.resolution, ui)
    {
        model.resolution = value as usize;
        println!("Resolution = {}", value);
    }

    for value in slider(model.scale, 0.0, 1.0)
        .down(10.0)
        .label("Scale")
        .set(model.ids.scale, ui)
    {
        // existing code
        model.scale = value;
        println!("Scale = {}", value);

        // TODO: trying to set volume with slider...
        // model
        //     .stream
        //     .send(|audio| {
        //         audio.hz = model.scale as f64;
        //         println!("Audio hz = {}", audio.hz);
        //     })
        //     .unwrap();

    }

    for value in slider(model.rotation, -PI as f32, PI as f32)
        .down(10.0)
        .label("Rotation")
        .set(model.ids.rotation, ui)
    {
        model.rotation = value;
        println!("Rotation = {}", value);
    }

    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 60.0)
        .label("Random Color")
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.random_color, ui)
    {
        model.color = rgb(random(), random(), random());
        println!("Random color set");
    }

    for (x, y) in widget::XYPad::new(
        model.position.x,
        -200.0,
        200.0,
        model.position.y,
        -200.0,
        200.0,
    )
        .down(10.0)
        .w_h(200.0, 200.0)
        .label("Position")
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.position, ui)
    {
        model.position = Point2::new(x, y);
        println!("Position set");
    }
}

// // Draw the state of your `Model` into the given `Frame` here.
// fn view(app: &App, model: &Model, frame: Frame) {
//     // Begin drawing
//     let draw = app.draw();
//
//     draw.background().rgb(0.02, 0.02, 0.02);
//
//     draw.ellipse()
//         .xy(model.position)
//         .radius(model.scale)
//         .resolution(model.resolution)
//         .rotate(model.rotation)
//         .color(model.color);
//
//     // Write the result of our drawing to the window's frame.
//     draw.to_frame(app, &frame).unwrap();
//
//     // Draw the state of the `Ui` to the frame.
//     model.ui.draw_to_frame(app, &frame).unwrap();
// }

fn view(app: &App, model: &Model, frame: Frame) {
    // println!("view() called...");

    // frame.clear(DIMGRAY);
    // Begin drawing
    let draw = app.draw();

    draw.background().rgb(0.02, 0.02, 0.02);

    draw.ellipse()
        .xy(model.position)
        .radius(model.scale)
        .resolution(model.resolution)
        .rotate(model.rotation)
        .color(model.color);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play a simple sine wave at the audio's current frequency in `hz`.

// fn audio(audio: &mut Audio, buffer: &mut Buffer, model: &mut Model) { // TODO: model doesn't need to be &ref
fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    println!("audio() called...");

    let sample_rate = buffer.sample_rate() as f64;

    // Set volume
    let volume = audio.vol;
    // let volume = model.scale;
    // println!("vol = {}", model.scale);

    for frame in buffer.frames_mut() {
        let sine_amp = (2.0 * PI * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = sine_amp * volume;
        }
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    // println!("key_pressed() called...");

    match key {
        // Pause or unpause the audio when Space is pressed.
        Key::Space => {
            if model.stream.is_playing() {
                model.stream.pause().unwrap();
            } else {
                model.stream.play().unwrap();
            }
        }
        // Raise the frequency when the up key is pressed.
        Key::Up => {
            model
                .stream
                .send(|audio| {
                    audio.hz += 10.0;
                    println!("Audio hz = {}", audio.hz);
                })
                .unwrap();
        }
        // Lower the frequency when the down key is pressed.
        Key::Down => {
            model
                .stream
                .send(|audio| {
                    audio.hz -= 10.0;
                    println!("Audio hz = {}", audio.hz);
                })
                .unwrap();
        }

        // Raise the volume when the right key is pressed.
        Key::Right => {
            model
                .stream
                .send(|audio| {
                    if audio.vol + 0.1 > 1.0 {
                        audio.vol = 0.9;
                    } else {
                        audio.vol += 0.1;
                    }
                    println!("Audio vol = {}", audio.vol);
                })
                .unwrap();
        }

        // Lower the volume when the right key is pressed.
        Key::Left => {
            model
                .stream
                .send(|audio| {
                    if audio.vol - 0.1 < 0.0 {
                        audio.vol = 0.1;
                    } else {
                        audio.vol -= 0.1;
                    }
                    println!("Audio vol = {}", audio.vol);
                })
                .unwrap();
        }
        _ => {}
    }
}

