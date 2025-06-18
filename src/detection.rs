use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    prelude::*,
    types,
    videoio,
};

pub fn process_webcam() -> opencv::Result<()> {
    let mut face_cascade = objdetect::CascadeClassifier::new("models/haarcascade_frontalface_default.xml")?;
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    highgui::named_window("Object Detection", highgui::WINDOW_NORMAL)?;

    loop {
        cap.read(&mut frame)?;
        if frame.empty()? {
            break;
        }

        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        let mut faces = types::VectorOfRect::new();
        face_cascade.detect_multi_scale(
            &gray,
            &mut faces,
            1.1,
            3,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size::new(30, 30),
            core::Size::new(0, 0),
        )?;

        for face in faces {
            imgproc::rectangle(
                &mut frame,
                face,
                core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_8,
                0,
            )?;
        }

        highgui::imshow("Object Detection", &frame)?;

        if highgui::wait_key(10)? > 0 {
            break;
        }
    }

    Ok(())
}

pub fn process_video(path: &std::path::Path) -> opencv::Result<()> {
    let mut face_cascade = objdetect::CascadeClassifier::new("models/haarcascade_frontalface_default.xml")?;
    let mut cap = videoio::VideoCapture::from_file(path.to_str().unwrap(), videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    let total_frames = cap.get(videoio::CAP_PROP_FRAME_COUNT)? as usize;
    let mut frame_count = 0;

    highgui::named_window("Video Processing", highgui::WINDOW_NORMAL)?;

    while cap.read(&mut frame)? {
        if frame.empty()? {
            break;
        }

        frame_count += 1;
        println!("Processing frame {}/{}", frame_count, total_frames);

        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        let mut faces = types::VectorOfRect::new();
        face_cascade.detect_multi_scale(
            &gray,
            &mut faces,
            1.1,
            3,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size::new(30, 30),
            core::Size::new(0, 0),
        )?;

        for face in faces {
            imgproc::rectangle(
                &mut frame,
                face,
                core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_8,
                0,
            )?;
        }

        highgui::imshow("Video Processing", &frame)?;

        if highgui::wait_key(1)? > 0 {
            break;
        }
    }

    Ok(())
}
