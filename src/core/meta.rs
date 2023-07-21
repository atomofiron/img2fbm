use image::Delay;
use crate::ext::iter_ext::Sum;

pub struct FrameData {
    pub index: usize,
    pub duration: f32,
}

impl FrameData {
    pub fn from(index: usize, delay: &Delay) -> FrameData {
        let (num, del) = delay.clone().numer_denom_ms();
        FrameData { index, duration: num as f32 / del as f32 }
    }
}

pub fn get_meta(height: u8, frames: &Vec<FrameData>) -> String {
    let duration = frames.iter().sum_of(0f32, |it| it.duration) as usize;
    let min_dur = frames.iter()
        .min_by(|&f,&s| f.duration.partial_cmp(&s.duration).unwrap())
        .unwrap()
        .duration;
    let mut order = Vec::<usize>::new();
    for it in frames {
        let times = (it.duration / min_dur) as u32;
        for _ in 0..times {
            order.push(it.index);
        }
    }
    let frame_rate = (1000.0 / min_dur) as u32;
    let a_frames = frames.iter()
        .max_of(0, |it| it.index) + 1;
    let p_frames = order.len() - a_frames;
    let order = order.iter()
        .map(|it| it.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    return format!("Filetype: Flipper Animation
Version: 1

Width: 128
Height: {height}
Passive frames: {p_frames}
Active frames: {a_frames}
Frames order: {order}
Active cycles: 1
Frame rate: {frame_rate}
Duration: {duration}
Active cooldown: 0

Bubble slots: 0
")
}

pub fn get_manifest(with_header: bool, name: String) -> String {
    let header = if with_header { "Filetype: Flipper Animation Manifest\nVersion: 1" } else { "" };
    return format!("{header}

Name: {name}
Min butthurt: 0
Max butthurt: 13
Min level: 1
Max level: 3
Weight: 8
")
}
