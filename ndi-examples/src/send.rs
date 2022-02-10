extern crate ndi;
use std::thread;
use ndi::VideoData;

fn get_frame(source: &ndi::Source) -> ndi::VideoData {
    let mut recv = ndi::RecvBuilder::new().build().unwrap();
    recv.connect(source);

    let mut video_data = None;
    loop {
        let response = recv.capture_video(&mut video_data, 1000);
        if response == ndi::FrameType::Video {
            break;
        }
    }

    video_data.expect("no video data")
}

fn main() {


    ndi::initialize().unwrap();

    //test receive 
    let find = ndi::FindBuilder::new().build().unwrap();
    let sources = find.current_sources(1000).unwrap();
    println!("{:?} sources",sources);
    let frame = get_frame(&sources[0]);
    println!("{:?} frame", frame);


    let send = ndi::SendBuilder::new().ndi_name("MyVideo".to_string()).clock_video(true).build().expect("Failed to create send");
    println!("{}", send.get_source().get_name());



    let mut i = 0;
    let framerate= 30;
    const w : usize = 512;
    const h : usize = 512;
    let mut buffer = [128u8 ; w*h*4];
    
    while send.get_no_connections(1000) == 0 {
        println!("No Connections", );

    }

    while true {
        i = i + 1; 

        buffer[(i*2) % (w*h*4)] = buffer[(i*2) % (w*h*4)] + 63;
        println!("changed at (i*2) % (w*h*4) {}  to {}", (i*2) % (w*h*4), buffer[(i*2) % (w*h*4)]);


        let time = i as i64 * framerate as i64 * 10i64;

        let frame = VideoData::from_buffer(w as i32, h as i32, ndi::FourCCVideoType::RGBA,
                                         framerate, ndi::FrameFormatType::Progressive, time, w as i32 *4, &mut buffer);
        thread::sleep(std::time::Duration::from_millis(1000/(framerate as u64)));

        send.send_video(&frame);

        println!("Frame send: {}x{}", frame.width(), frame.height());
    }
}
