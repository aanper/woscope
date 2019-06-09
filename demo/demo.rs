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

fn process_sample(t: f32) -> (f32, f32) {
    let left = 0.3 * (2.0 * f32::consts::PI * 120.1 * t).sin();
    let right = 0.3 * (2.0 * f32::consts::PI * 180.0 * t).sin();

    return (left, right);
}


#[no_mangle]
pub fn request_frame(init_t: f32, fs: f32) -> f32 {
    let mut t = init_t;

    let buffer_len = get_buffer_len();
    for i in 0..buffer_len/2 {
        unsafe {
            let (left, right) = process_sample(t);

            BUFFER[i] = left;
            BUFFER[i + buffer_len/2] = right;
        }

        t += 1.0/fs;
    }

    return t;
}

