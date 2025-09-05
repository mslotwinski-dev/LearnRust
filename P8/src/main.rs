use std::f64;

fn get_wave_amplitude(time: f64, frequency: f64, sample_rate: i64) -> Vec<f64> {
    let mut amplitudes = Vec::new();
    let mut t = 0.0;
    let dt = 1.0 / sample_rate as f64;

    while t < time {
        let amplitude = (2.0 * f64::consts::PI * frequency * t).sin();
        amplitudes.push(amplitude);
        t += dt;
    }

    amplitudes
}

fn main() {}
