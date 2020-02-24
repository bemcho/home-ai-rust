use std::{thread, time::Duration};

use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    prelude::*,
    types,
    videoio,
};

use tts::*;

fn run() -> opencv::Result<()> {
    let window = "Home AI";
    highgui::named_window(window, 1)?;
    #[cfg(feature = "opencv-32")]
        let (xml, mut cam) = {
        (
            "/usr/share/OpenCV/haarcascades/haarcascade_frontalface_alt.xml".to_owned(),
            videoio::VideoCapture::new(0)?,  // 0 is the default camera
        )
    };
    #[cfg(not(feature = "opencv-32"))]
        let (xml, mut cam) = {
        (
            core::find_file("haarcascades/haarcascade_frontalface_alt.xml", true, false)?,
            videoio::VideoCapture::new_with_backend(0, videoio::CAP_ANY)?,  // 0 is the default camera
        )
    };
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        txt_to_speech("Unable to open default camera!",30);
        panic!("Unable to open default camera!");
    }
    let mut face = objdetect::CascadeClassifier::new(&xml)?;
    loop {
        let mut frame = Mat::default()?;
        cam.read(&mut frame)?;
        if frame.size()?.width == 0 {
            thread::sleep(Duration::from_secs(50));
            continue;
        }
        let mut gray = Mat::default()?;
        imgproc::cvt_color(
            &frame,
            &mut gray,
            imgproc::COLOR_BGR2GRAY,
            0,
        )?;
        let mut reduced = Mat::default()?;
        imgproc::resize(
            &gray,
            &mut reduced,
            core::Size {
                width: 0,
                height: 0,
            },
            0.25f64,
            0.25f64,
            imgproc::INTER_LINEAR,
        )?;
        let mut faces = types::VectorOfRect::new();
        face.detect_multi_scale(
            &reduced,
            &mut faces,
            1.1,
            2,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size {
                width: 5,
                height: 5,
            },
            core::Size {
                width: 0,
                height: 0,
            },
        )?;
        println!("faces: {}", faces.len());
        for face in faces {
            println!("face {:?}", face);
            txt_to_speech("Face detected.",30);
            let scaled_face = core::Rect {
                x: face.x * 4,
                y: face.y * 4,
                width: face.width * 4,
                height: face.height * 4,
            };
            imgproc::rectangle(
                &mut frame,
                scaled_face,
                core::Scalar::new(0f64, -1f64, -1f64, -1f64),
                1,
                8,
                0,
            )?;
        }
        highgui::imshow(window, &frame)?;
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}

fn txt_to_speech(txt: &str, rate: u8) {
    let mut tts = TTS::default().expect("Error");
    let _ = tts.set_rate(rate);
    let _ = tts.set_volume(15);
    tts.speak(txt, false).expect("Error");
}

fn main() {
    txt_to_speech("Hello. This is a Home AI, written in Rust using Open CV.", 21);
    run().unwrap()
}