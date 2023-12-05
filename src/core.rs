use opencv::videoio::VideoCapture;
use opencv::{
    prelude::*,
    videoio::{self, CAP_ANY},
};

pub struct Video<'a> {
    source_file_path: &'a str,
    dst_path: &'a str,
    cam: VideoCapture,
}

impl<'a> Video<'a> {
    pub fn new(source_file_path: &'a str, dst_path: &'a str) -> Self {
        Video {
            source_file_path,
            dst_path,
            cam: VideoCapture::from_file(source_file_path, CAP_ANY).unwrap(),
        }
    }

    pub fn split(&mut self) {
        // 获取视频的总帧数
        let frame_count = self.cam.get(videoio::CAP_PROP_FRAME_COUNT).unwrap() as i32;

        // 获取电脑的逻辑线程数
        let cpu_available_count = num_cpus::get() as i32;

        let avg = (frame_count / cpu_available_count) as f64;
        let is_int = frame_count % cpu_available_count;

        // 线程工作列表
        let mut tasks: Vec<Vec<Mat>> = Vec::with_capacity(cpu_available_count as usize);

        // 刚好能被平均分配
        if is_int == 0 {
            todo!()
        }
        // 不能平均分配, 需要最后一个线程去承担更多工作量
        else {
            // 正常线程要工作的数量
            let manual_worker_num = avg.trunc() as i32;

            let mut manual_task_list = Vec::with_capacity(avg as usize);
            let mut last_task_list = Vec::with_capacity(
                (frame_count - (manual_worker_num * (cpu_available_count - 1))) as usize,
            );

            // 记录当前的帧走到了哪里
            let mut global_frame = 0;

            for index in 0..cpu_available_count {
                // 最后一个线程
                if index == (cpu_available_count - 1) {
                    loop {
                        let mut frame = Mat::default();
                        let result = self.cam.read(&mut frame).unwrap();
                        if !result {
                            break;
                        }

                        last_task_list.push(frame);
                    }

                    println!("{:?}", last_task_list);

                    continue;
                }

                loop {
                    if global_frame % manual_worker_num == 0 && global_frame != 0 {
                        break;
                    }
                    let mut frame = Mat::default();

                    self.cam.read(&mut frame).unwrap();

                    manual_task_list.push(frame);

                    global_frame += 1;
                }
            }

            // 收集到线程工作列表中, 这批任务会被某个线程所处理
            tasks.push(manual_task_list);
        }
    }
}
