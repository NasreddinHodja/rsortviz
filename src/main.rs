use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;

mod sort;
use sort::*;

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 800;
const BG_COLOR: Rgb<u8> = BLACK;
const FG_COLOR: Rgb<u8> = PLUM;
const FFG_COLOR: Rgb<u8> = RED;
const MAX_HZ: f64 = 200.0;
const LEN: usize = 100;

struct Audio {
    phase: f64,
    hz: f64,
}

struct Model<T: Sorter> {
    playing: bool,
    sorter: T,
    stream: audio::Stream<Audio>,
}

type CurrentSorter = InsertionSorter;

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

fn model(app: &App) -> Model<CurrentSorter> {
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
    stream.play().unwrap();

    let playing = false;
    let v: Vec<usize> = (1..=LEN).collect();
    let v = unsort(&v);
    let sorter = CurrentSorter::new(&v);

    Model {
        playing,
        sorter,
        stream,
    }
}

fn update(_app: &App, model: &mut Model<CurrentSorter>, _update: Update) {
    if model.playing {
        step(model);
        if model.stream.is_paused() {
            model.stream.play().unwrap();
        }
    } else if model.stream.is_playing() {
        let _ = model.stream.pause();
    }
}

fn step(model: &mut Model<CurrentSorter>) {
    if model.sorter.next_step() == None {
        model.playing = false;
    }
}

fn event(_app: &App, model: &mut Model<CurrentSorter>, event: Event) {
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

fn view(app: &App, model: &Model<CurrentSorter>, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BG_COLOR);

    let win = app.window_rect();
    let bar_width = win.w() / LEN as f32;

    let v = model.sorter.current_state();
    for i in 0..LEN {
        let bar_height = v[i] as f32 * win.w() / LEN as f32;
        let x = -win.w() / 2.0 + bar_width / 2.0 + i as f32 * bar_width;
        let y = -win.h() / 2.0 + bar_height / 2.0;
        let draw = draw.rect().x_y(x, y).w_h(bar_width, bar_height);
        if model.sorter.used_indices().contains(&i) {
            draw.color(FFG_COLOR);
            let hz = model.sorter.current_state()[i] as f64 * MAX_HZ / LEN as f64;
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

    // let v: Vec<usize> = (1..=LEN).collect();

    // let mut v = unsort(&v);
    // println!("v = {v:?}\n");
    // // sort::insertion_sort(&mut v);
    // // println!("{:?}", v);
    // let mut sorter = CurrentSorter::new(&v);

    // let mut i = 0;
    // loop {
    //     sorter.next_step();
    //     println!("{i} -> {:?}", sorter.current_state());
    //     if let Some(step) = sorter.next_step() {
    //         i += 1;
    //     } else {
    //         break;
    //     }
    // }
    // println!("result = {:?}", &sorter.current_state());
}
