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
    prev_t: f32,
    p: f32,
    freq: f32,
    width: f32,
    height: f32,

    switch_point: f32,

    rotate: f32,
    shift: (f32, f32),
    scale: (f32, f32)
}

impl Rect {
    pub const fn new(width: f32, height: f32, freq: f32) -> Self {
        return Rect {
            freq,
            width,
            height,

            switch_point: 4.0,

            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0),

            prev_t: 0.0,
            p: 0.0
        };
    }
}

impl Primitive for Rect {
    fn draw(&mut self, switch: bool, t: f32) -> (f32, f32, bool) {
        
        if switch {
            self.p = 0.0;
            self.prev_t = t;
        }
        

        self.p += (t - self.prev_t) * self.freq;

        self.prev_t = t;

        let p = self.p % 4.0;

        let x = -self.width / 2.0;
        let y = -self.height / 2.0;

        let (point_x, point_y) = (p, 0.0);

        
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


        return (point_x, point_y, self.p >= 4.0);
    }
}

struct Ctx {
    current_primitive: usize,
    switch_primitive: bool,
    rect_0: Rect,
    rect_1: Rect
}

fn process_sample(ctx: &mut Ctx, t: f32) -> (f32, f32) {

    ctx.rect_0.shift = (0.2, 0.0);
    ctx.rect_0.scale = (1.0, 0.5 * (t * 2.15).sin());
    ctx.rect_0.freq = 1000.0 + 100.0 * (t * 6.0).cos();

    // ctx.rect_0.switch_point += 0.01;

    ctx.rect_1.rotate = 1.0 * t;
    ctx.rect_0.rotate = 0.5 * t;
    ctx.rect_1.scale = (1.0 * (t * 1.0).cos(), 1.0);
    // ctx.rect_1.freq = 1000.0 + 100.0 * (t * 50.0).cos();

    let primitives: [&mut Primitive; 2] = [&mut ctx.rect_0, &mut ctx.rect_1];
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
    rect_0: Rect::new(0.95, 0.95, 500.0),
    rect_1: Rect::new(1.0, 1.0, 750.0)
};

#[no_mangle]
pub fn request_frame(init_t: f32, fs: f32) -> f32 {
    let mut t = init_t;

    let buffer_len = get_buffer_len();
    for i in 0..buffer_len/2 {
        unsafe {
            let (x, y) = process_sample(&mut CTX, t);

            BUFFER[i] = x;
            BUFFER[i + buffer_len/2] = y;
        }

        t += 1.0/fs;
    }

    return t;
}

