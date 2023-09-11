use opencv::core::{no_array, Size_};
use opencv::prelude::*;
use opencv::quality::QualitySSIM;
use opencv::videoio;
use opencv::videoio::CAP_ANY;

#[tokio::main]
async fn main() {
    let mut cam = videoio::VideoCapture::from_file(
        "C:\\Users\\Administrator\\Desktop\\Doraemon.the.Movie.Nobita&#39;s.Sky.Utopia.2023.1080p.WEB-DL.AAC.2.0.x264.mkv",
        CAP_ANY,
    )
        .unwrap();

    let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut last_frame: Option<Mat> = None;
    let mut frame_list = Vec::with_capacity(100000);
    let mut count = 1;

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame).unwrap();

        if frame.size().unwrap().width > 0 {
            // 第一次保存帧
            if last_frame.is_none() {
                frame_list.push(frame.clone());
                last_frame = Some(frame);
            }
            // 之后都走这个分支
            else {
                let result =
                    QualitySSIM::compute(last_frame.as_ref().unwrap(), &frame, &mut no_array())
                        .unwrap();

                // 相似
                if result.0[0] > 0.50 && result.0[1] > 0.50 && result.0[2] > 0.50 {
                    frame_list.push(frame.clone());

                    last_frame = Some(frame);
                }
                // 不相似
                else {
                    let fourcc = opencv::videoio::VideoWriter::fourcc('X', 'V', 'I', 'D').unwrap();

                    // 将之间的帧保存为视频
                    let mut vw = opencv::videoio::VideoWriter::new(
                        format!("C:\\Users\\Administrator\\Desktop\\slice\\{}.avi", count).as_str(),
                        fourcc,
                        25_f64,
                        Size_ {
                            width: 1920,
                            height: 1080,
                        },
                        true,
                    )
                    .unwrap();
                    for frame in &frame_list {
                        vw.write(frame).unwrap();
                    }
                    vw.release().unwrap();

                    count += 1;
                    frame_list.clear();
                    // 保存最新的帧
                    frame_list.push(frame.clone());
                    last_frame = Some(frame)
                }
            }
        }

        // let key = highgui::wait_key(1).unwrap();
        // // 按Q退出程序
        // if key == 113 {
        //     break;
        // }
    }
}
