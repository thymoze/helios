use image;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use videocore::{
    bcm_host, dispmanx,
    image::{ImageType, Rect},
};

pub fn capture() -> image::RgbImage {
    bcm_host::init();

    let display = dispmanx::display_open(0);
    if display == dispmanx::DISPMANX_NO_HANDLE {
        bcm_host::deinit();
        panic!("Unable to open display!");
    }

    let mut info = MaybeUninit::<dispmanx::Modeinfo>::uninit();

    let result = dispmanx::display_get_info(display, info.as_mut_ptr());
    if result {
        dispmanx::display_close(display);
        bcm_host::deinit();
        panic!("Unable to get display information");
    }

    let mut rect = MaybeUninit::<Rect>::uninit();
    let result = dispmanx::rect_set(rect.as_mut_ptr(), 0, 0, 256, 144);
    if result {
        dispmanx::display_close(display);
        bcm_host::deinit();
        panic!("Unable to create rectangle buffer");
    }
    let rect = unsafe { rect.assume_init() };

    let mut native_image_handle: u32 = 0;
    let resource = dispmanx::resource_create(
        ImageType::RGB888,
        rect.width as u32,
        rect.height as u32,
        &mut native_image_handle as *mut u32,
    );

    let result = dispmanx::snapshot(display, resource, dispmanx::Transform::NO_ROTATE);
    if result {
        dispmanx::resource_delete(resource);
        dispmanx::display_close(display);
        bcm_host::deinit();
        panic!("Snapshot failed!");
    }

    let pitch = 4 * ((rect.width + 15) & !15);

    let mut buf = vec![0; (pitch * rect.height) as usize];

    let result = dispmanx::resource_read_data(
        resource,
        &rect as *const Rect,
        buf.as_mut_ptr() as *mut c_void,
        pitch as u32,
    );

    dispmanx::resource_delete(resource);
    dispmanx::display_close(display);
    bcm_host::deinit();

    assert!(!result, "resource_read_data failed!");

    //let file = File::create("screen.png").unwrap();
    //let ref mut w = BufWriter::new(file);
    //
    //let mut encoder = png::Encoder::new(w, rect.width as u32, rect.height as u32);
    //encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    //
    //println!("saving....");
    //
    //let mut writer = encoder.write_header().unwrap();
    //writer.write_image_data(&buf).unwrap();
    //
    //println!("saved!");

    image::ImageBuffer::from_raw(rect.width as u32, rect.height as u32, buf)
        .expect("Couldnt read buffer into image.")
}
