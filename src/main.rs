use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::sync::mpsc;

mod cli;
mod sort;
mod sound;

use sort::{unsort, SortResult};
use sound::Scale;

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 800;
const BG_COLOR: Rgb<u8> = BLACK;
const FG_COLOR: Rgb<u8> = PLUM;
const FFG_COLOR: Rgb<u8> = RED;
const MAX_HZ: f64 = 440.0;

struct Audio {
    phase: f64,
    hz: f64,
}

struct Model {
    playing: bool,
    v: Vec<usize>,
    length: usize,
    used_indices: Vec<usize>,
    rx: mpsc::Receiver<Option<SortResult>>,
    stream: audio::Stream<Audio>,
    scale: Option<Scale>,
}

fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        let sine_amp = (2.0 * PI as f64 * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = sine_amp * volume;
        }
    }
}

fn model(app: &App) -> Model {
    let args = cli::parse();

    app.new_window()
        .size_pixels(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();
    // app.set_loop_mode(LoopMode::rate_fps(2.0));

    let audio_host = audio::Host::new();
    let model = Audio {
        phase: 0.0,
        hz: 0.0,
    };
    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();

    let scale: Option<Scale> = args.scale;

    let playing = false;
    let mut values: Vec<usize> = (1..=args.length).collect();
    if let Some(s) = &scale {
        values = (1..=s.frequencies().len()).collect();
    }
    let mut v = unsort(&values);

    let (tx, rx): (
        mpsc::Sender<Option<SortResult>>,
        mpsc::Receiver<Option<SortResult>>,
    ) = mpsc::channel();

    args.sorter.sort(&mut v, tx);

    match rx.recv().unwrap() {
        Some(result) => v = result.values,
        None => {}
    }

    let mut length = args.length;
    if let Some(s) = &scale {
        length = s.frequencies().len();
    }

    Model {
        playing,
        v,
        length,
        used_indices: vec![],
        rx,
        stream,
        scale,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.playing {
        if !model.stream.is_playing() {
            model.stream.play().unwrap();
        }
        if let Ok(Some(result)) = model.rx.recv() {
            model.v = result.values;
            model.used_indices = result.used_indices;
        } else {
            model.playing = false;
            model.stream.pause().unwrap();
        }
    } else if model.stream.is_playing() {
        model.stream.pause().unwrap();
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: Some(event),
        } => match event {
            KeyPressed(Key::Space) => {
                model.playing = !model.playing;
            }
            _ => {}
        },
        // Event::DeviceEvent(_, _) => todo!(),
        // Event::Update(_) => todo!(),
        // Event::Suspended => todo!(),
        // Event::Resumed => todo!(),
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BG_COLOR);

    let win = app.window_rect();
    let bar_width = win.w() / model.length as f32;

    let v = &model.v;
    for i in 0..model.length {
        let bar_height = v[i] as f32 * win.w() / model.length as f32;
        let x = -win.w() / 2.0 + bar_width / 2.0 + i as f32 * bar_width;
        let y = -win.h() / 2.0 + bar_height / 2.0;
        let draw = draw.rect().x_y(x, y).w_h(bar_width, bar_height);

        if model.used_indices.contains(&i) {
            draw.color(FFG_COLOR);
            let mut hz = v[i] as f64 * MAX_HZ / model.length as f64;
            if let Some(scale) = &model.scale {
                hz = scale.frequency(i);
            }
            model
                .stream
                .send(move |audio| {
                    audio.hz = hz;
                })
                .unwrap();
        } else {
            draw.color(FG_COLOR);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).event(event).run();
    // let floor = 261.63;
    // let ceil = 523.25;
    // let minor = MinorScale::new(floor, ceil);
    // println!("{:?}", minor);
}
