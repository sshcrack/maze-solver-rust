use crate::tools::options::MazeData;

pub fn count_to_percentage(data: &MazeData, size: usize, count: u64) -> Option<f64> {
    if count % data.speed_anim().max(1.0) as u64 != 0 {
        return None;
    }

    let total = (size - 3)*(size + 1)/4;

    Some(count as f64 / total as f64)
}