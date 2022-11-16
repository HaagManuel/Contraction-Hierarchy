
/// report progress in percentage
pub fn report_progress(current_round: usize, num_rounds: usize, message: &str, intervals: usize) {
    let progress: usize = ((current_round as f64 / num_rounds as f64) * 100f64).round() as usize;
    if current_round % (num_rounds / intervals) == 0 {
        eprintln!("{}% {}", progress, message);
    }
}