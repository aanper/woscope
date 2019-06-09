static mut BUFFER: [f32;8192] = [0.0;8192];

use std::f32;

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 5
}


#[no_mangle]
pub fn get_buffer() -> *const f32 {
    unsafe {
        BUFFER.as_ptr()
    }
}

#[no_mangle]
pub fn get_buffer_len() -> usize {
    unsafe {
        BUFFER.len()
    }
}

trait Primitive {
    fn draw(&mut self, switch: bool, t: f32) -> (f32, f32, bool);
}

fn scale(point: (f32, f32), factor: (f32, f32)) -> (f32, f32) {
    (point.0 * factor.0, point.1 * factor.1)
}

fn shift(point: (f32, f32), vector: (f32, f32)) -> (f32, f32) {
    (point.0 + vector.0, point.1 + vector.1)
}

fn rotate(point: (f32, f32), angle: f32) -> (f32, f32) {
    (
        point.0 * angle.cos() - point.1 * angle.sin(),
        point.0 * angle.sin() + point.1 * angle.cos()
    )
}

struct Rect {
    fs: f32,
    freq: f32,
    width: f32,
    height: f32,

    rotate: f32,
    shift: (f32, f32),
    scale: (f32, f32)
}

impl Rect {
    pub fn new(width: f32, height: f32, fs: f32, freq: f32) -> Self {
        return Rect {
            fs,
            freq,
            width,
            height,

            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0)
        };
    }
}

impl Primitive for Rect {
    fn draw(&mut self, switch: bool, t: f32) -> (f32, f32, bool) {
        let p = (t * self.freq) % 4.0;

        let x = -self.width / 2.0;
        let y = -self.height / 2.0;

        let (point_x, point_y) = match p {
            d if d >= 0.0 && d < 1.0 => (x + p * self.width, y),
            d if d >= 1.0 && d < 2.0 => (x + self.width, y + (p - 1.0) * self.height),
            d if d >= 2.0 && d < 3.0 => (x + (3.0 - p) * self.width, y + self.height),
            d if d >= 3.0 && d < 4.0 => (x,  (4.0 - p) * self.height + y),
            _ => (0.0, 0.0)
        };

        let (point_x, point_y) = scale((point_x, point_y), self.scale);
        let (point_x, point_y) = rotate((point_x, point_y), self.rotate);
        let (point_x, point_y) = shift((point_x, point_y), self.shift);


        return (point_x, point_y, (p < (1.0 * self.freq)/self.fs && !switch));
    }
}

struct Ctx {
    current_primitive: usize,
    switch_primitive: bool,
}

fn process_sample(ctx: &mut Ctx, t: f32, fs: f32) -> (f32, f32) {
    // let x = 0.3 * (2.0 * f32::consts::PI * 120.1 * t).sin();
    // let y = 0.3 * (2.0 * f32::consts::PI * 180.0 * t).sin();

    let mut rect_0: Rect = Rect::new(0.5, 0.5, fs, 500.0);
    rect_0.shift = (0.2 * t.cos(), 0.2 * t.sin());
    rect_0.scale = (0.5 * (t * 0.5).cos(), 0.5 * (t * 0.8).sin());

    let mut rect_1: Rect = Rect::new(1.0, 1.0, fs, 800.0);
    rect_1.rotate = 1.0 * t;
    rect_1.scale = (1.0 * (t * 2.0).cos(), 1.0 * (t * 2.5).sin());

    let primitives: [&mut Primitive; 2] = [&mut rect_0, &mut rect_1];
    let (x, y, is_complete) = primitives[ctx.current_primitive].draw(ctx.switch_primitive, t);

    ctx.switch_primitive = is_complete;

    if is_complete {
        // next primitive
        ctx.current_primitive += 1;
        if ctx.current_primitive >= primitives.len() {
            ctx.current_primitive = 0;
        }
    }

    return (x, y);
}

static mut CTX: Ctx = Ctx { 
    current_primitive: 0,
    switch_primitive: true,
};

#[no_mangle]
pub fn request_frame(init_t: f32, fs: f32) -> f32 {
    let mut t = init_t;

    let buffer_len = get_buffer_len();
    for i in 0..buffer_len/2 {
        unsafe {
            let (x, y) = process_sample(&mut CTX, t, fs);

            BUFFER[i] = x;
            BUFFER[i + buffer_len/2] = y;
        }

        t += 1.0/fs;
    }

    return t;
}

