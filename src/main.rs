use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    prelude::*,
    types,
    videoio,
};

fn detect_objects() -> opencv::Result<()> {
    // Load pre-trained Haar cascade for face detection
    let mut face_cascade = objdetect::CascadeClassifier::new("models/haarcascade_frontalface_default.xml")?;
    
    // Open video capture (0 for default camera)
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    
    let mut frame = Mat::default();
    
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

fn main() {
    match detect_objects() {
        Ok(_) => println!("Detection completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
