use glow::HasContext;
use pyxel_platform::keys::{Key, KEY_0, KEY_1, KEY_2, KEY_3, KEY_ALT, KEY_RETURN};
use pyxel_platform::Event;

use crate::audio::Audio;
use crate::graphics::Graphics;
use crate::image::{Image, SharedImage};
use crate::input::Input;
use crate::math::Math;
use crate::profiler::Profiler;
use crate::resource::Resource;
use crate::settings::{
    BACKGROUND_COLOR, DEFAULT_FPS, DEFAULT_QUIT_KEY, DEFAULT_TITLE, ICON_DATA, ICON_SCALE,
    MAX_ELAPSED_MS, NUM_COLORS, NUM_MEASURE_FRAMES,
};
use crate::utils::simplify_string;
use crate::DISPLAY_RATIO;

pub trait PyxelCallback {
    fn update(&mut self);
    fn draw(&mut self);
}

pub struct System {
    width: u32,
    height: u32,
    fps: u32,
    one_frame_ms: f64,
    next_update_ms: f64,
    frame_count: u32,
    quit_key: Key,
    paused: bool,
    fps_profiler: Profiler,
    update_profiler: Profiler,
    draw_profiler: Profiler,
    perf_monitor_enabled: bool,
}

unsafe_singleton!(System);

impl System {
    pub fn init(
        width: u32,
        height: u32,
        title: Option<&str>,
        fps: Option<u32>,
        quit_key: Option<Key>,
        display_scale: Option<u32>,
    ) {
        let title = title.unwrap_or(DEFAULT_TITLE);
        let fps = fps.unwrap_or(DEFAULT_FPS);
        let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);

        pyxel_platform::init(|| {
            let display_scale = (if let Some(display_scale) = display_scale {
                display_scale
            } else {
                let (display_width, display_height) = pyxel_platform::display_size();
                ((display_width as f64 / width as f64).min(display_height as f64 / height as f64)
                    * DISPLAY_RATIO) as u32
            })
            .max(1);
            (title, width * display_scale, height * display_scale)
        });

        Self::set_instance(Self {
            width,
            height,
            fps,
            one_frame_ms: 1000.0 / fps as f64,
            next_update_ms: 0.0,
            frame_count: 0,
            quit_key,
            paused: false,
            fps_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            update_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            draw_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            perf_monitor_enabled: false,
        });

        crate::icon(&ICON_DATA, ICON_SCALE);
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }

    fn process_frame(&mut self, callback: &mut dyn PyxelCallback) {
        let tick_count = pyxel_platform::elapsed_time();
        let elapsed_ms = tick_count as f64 - self.next_update_ms;
        if elapsed_ms < 0.0 {
            return;
        }
        if self.frame_count == 0 {
            self.next_update_ms = tick_count as f64 + self.one_frame_ms;
        } else {
            self.fps_profiler.end(tick_count);
            self.fps_profiler.start(tick_count);
            let update_count: u32;
            if elapsed_ms > MAX_ELAPSED_MS as f64 {
                update_count = 1;
                self.next_update_ms = pyxel_platform::elapsed_time() as f64 + self.one_frame_ms;
            } else {
                update_count = (elapsed_ms / self.one_frame_ms) as u32 + 1;
                self.next_update_ms += self.one_frame_ms * update_count as f64;
            }
            for _ in 1..update_count {
                self.update_frame(Some(callback));
                self.frame_count += 1;
            }
        }
        self.update_frame(Some(callback));
        self.draw_frame(Some(callback));
        self.frame_count += 1;
    }

    #[cfg(not(target_os = "emscripten"))]
    fn process_frame_for_flip(&mut self) {
        self.update_profiler.end(pyxel_platform::elapsed_time());
        self.draw_frame(None);
        self.frame_count += 1;
        let mut tick_count;
        let mut elapsed_ms;
        loop {
            tick_count = pyxel_platform::elapsed_time();
            elapsed_ms = tick_count as f64 - self.next_update_ms;
            let wait_ms = self.next_update_ms - pyxel_platform::elapsed_time() as f64;
            if wait_ms > 0.0 {
                pyxel_platform::sleep((wait_ms / 2.0) as u32);
            } else {
                break;
            }
        }
        self.fps_profiler.end(tick_count);
        self.fps_profiler.start(tick_count);
        if elapsed_ms > MAX_ELAPSED_MS as f64 {
            self.next_update_ms = pyxel_platform::elapsed_time() as f64 + self.one_frame_ms;
        } else {
            self.next_update_ms += self.one_frame_ms;
        }
        self.update_frame(None);
    }

    fn update_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        self.update_profiler.start(pyxel_platform::elapsed_time());
        self.process_events();
        if self.paused {
            return;
        }
        self.check_special_input();
        if let Some(callback) = callback {
            callback.update();
            self.update_profiler.end(pyxel_platform::elapsed_time());
        }
    }

    fn process_events(&mut self) {
        Input::instance().reset_input_states();
        let events = pyxel_platform::poll_events();
        for event in events {
            match event {
                Event::WindowShown => {
                    self.paused = false;
                    pyxel_platform::set_audio_enabled(true);
                }
                Event::WindowHidden => {
                    self.paused = true;
                    pyxel_platform::set_audio_enabled(false);
                }
                Event::KeyPressed { key } => {
                    Input::instance().press_key(key);
                }
                Event::KeyReleased { key } => {
                    Input::instance().release_key(key);
                }
                Event::KeyValueChanged { key, value } => {
                    Input::instance().change_key_value(key, value);
                }
                Event::TextInput { text } => {
                    Input::instance().add_input_text(&text);
                }
                Event::FileDropped { filename } => {
                    Input::instance().add_dropped_file(&filename);
                }
                Event::Quit => {
                    pyxel_platform::quit();
                }
            }
        }
    }

    fn check_special_input(&mut self) {
        if crate::btn(KEY_ALT) {
            if crate::btnp(KEY_RETURN, None, None) {
                crate::fullscreen(!crate::is_fullscreen());
            }
            if crate::btnp(KEY_0, None, None) {
                self.perf_monitor_enabled = !self.perf_monitor_enabled;
            }
            if crate::btnp(KEY_1, None, None) {
                crate::screenshot(None);
            }
            if crate::btnp(KEY_2, None, None) {
                crate::reset_capture();
            }
            if crate::btnp(KEY_3, None, None) {
                crate::screencast(None);
            }
        }
        if crate::btnp(self.quit_key, None, None) {
            crate::quit();
        }
    }

    fn draw_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        if self.paused {
            return;
        }
        self.draw_profiler.start(pyxel_platform::elapsed_time());
        if let Some(callback) = callback {
            callback.draw();
        }
        self.draw_perf_monitor();
        self.draw_cursor();
        let screen = crate::screen();
        {
            let screen = screen.lock();
            let gl = pyxel_platform::gl();
            unsafe {
                gl.clear_color(1.0, 0.0, 0.0, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
            }
            pyxel_platform::swap_window();

            /*pyxel_Platform::instance().render_screen(
                screen.canvas.width(),
                screen.canvas.height(),
                &screen.canvas.data,
                &*crate::colors().lock(),
                BACKGROUND_COLOR,
            );*/
            Resource::instance().capture_screen(
                screen.width(),
                screen.height(),
                &screen.canvas.data,
                &crate::colors().lock(),
                self.frame_count,
            );
        }
        self.draw_profiler.end(pyxel_platform::elapsed_time());
    }

    fn draw_perf_monitor(&mut self) {
        if !self.perf_monitor_enabled {
            return;
        }
        let screen = crate::screen();
        let mut screen = screen.lock();
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette1 = screen.palette[1];
        let palette2 = screen.palette[2];
        screen.clip0();
        screen.camera0();
        screen.pal(1, NUM_COLORS as u8 + 1);
        screen.pal(2, NUM_COLORS as u8 + 9);

        let fps = format!("{:.*}", 2, self.fps_profiler.average_fps());
        screen.text(1.0, 0.0, &fps, 1);
        screen.text(0.0, 0.0, &fps, 2);

        let update_time = format!("{:.*}", 2, self.update_profiler.average_time());
        screen.text(1.0, 6.0, &update_time, 1);
        screen.text(0.0, 6.0, &update_time, 2);

        let draw_time = format!("{:.*}", 2, self.draw_profiler.average_time());
        screen.text(1.0, 12.0, &draw_time, 1);
        screen.text(0.0, 12.0, &draw_time, 2);

        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.pal(1, palette1);
        screen.pal(2, palette2);
    }

    fn draw_cursor(&mut self) {
        let x = crate::mouse_x();
        let y = crate::mouse_y();
        pyxel_platform::set_mouse_visible(
            x < 0 || x >= crate::width() as i32 || y < 0 || y >= crate::height() as i32,
        );
        if !Input::instance().is_mouse_visible() {
            return;
        }
        let width = crate::cursor().lock().width() as i32;
        let height = crate::cursor().lock().height() as i32;
        if x <= -width || x >= crate::width() as i32 || y <= -height || y >= crate::height() as i32
        {
            return;
        }
        let screen = crate::screen();
        let mut screen = screen.lock();
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette = screen.palette;
        screen.clip0();
        screen.camera0();
        for i in 0..NUM_COLORS {
            screen.pal(i as u8, (NUM_COLORS + i) as u8);
        }
        screen.blt(
            x as f64,
            y as f64,
            crate::cursor(),
            0.0,
            0.0,
            width as f64,
            height as f64,
            Some(0),
        );
        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.palette = palette;
    }
}

pub fn init(
    width: u32,
    height: u32,
    title: Option<&str>,
    fps: Option<u32>,
    quit_key: Option<Key>,
    display_scale: Option<u32>,
    capture_scale: Option<u32>,
    capture_sec: Option<u32>,
) {
    System::init(width, height, title, fps, quit_key, display_scale);
    Resource::init(capture_scale, capture_sec);
    Input::init();
    Graphics::init();
    Audio::init();
    Math::init();
}

pub fn width() -> u32 {
    System::instance().width
}

pub fn height() -> u32 {
    System::instance().height
}

pub fn frame_count() -> u32 {
    System::instance().frame_count
}

pub fn title(title: &str) {
    pyxel_platform::set_window_title(title);
}

pub fn icon(data_str: &[&str], scale: u32) {
    let width = simplify_string(data_str[0]).len() as u32;
    let height = data_str.len() as u32;
    let image = Image::new(width, height);
    let mut image = image.lock();
    image.set(0, 0, data_str);
    /*pyxel_platform::set_icon(
        width,
        height,
        &image.canvas.data,
        &*crate::colors().lock(),
        scale,
    );*/
}

pub fn is_fullscreen() -> bool {
    pyxel_platform::is_fullscreen()
}

pub fn fullscreen(is_fullscreen: bool) {
    pyxel_platform::set_fullscreen(is_fullscreen)
}

pub fn run<T: PyxelCallback>(mut callback: T) {
    pyxel_platform::run(move || {
        System::instance().process_frame(&mut callback);
    });
}

pub fn show() {
    struct App {
        image: SharedImage,
    }
    impl PyxelCallback for App {
        fn update(&mut self) {}
        fn draw(&mut self) {
            crate::screen().lock().blt(
                0.0,
                0.0,
                self.image.clone(),
                0.0,
                0.0,
                crate::width() as f64,
                crate::height() as f64,
                None,
            );
        }
    }
    let image = Image::new(crate::width(), crate::height());
    image.lock().blt(
        0.0,
        0.0,
        crate::screen(),
        0.0,
        0.0,
        crate::width() as f64,
        crate::height() as f64,
        None,
    );
    crate::run(App { image });
}

pub fn flip() {
    #[cfg(target_os = "emscripten")]
    panic!("flip is not supported on Web");

    #[cfg(not(target_os = "emscripten"))]
    System::instance().process_frame_for_flip();
}

pub fn quit() {
    pyxel_platform::quit();
}
