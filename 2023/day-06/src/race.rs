pub struct Race {
    pub time: u64,
    pub distance: u64,
}

impl Race {
    // h = time held
    // r = time running
    // d = distance
    // w = winning distance
    // t = time
    // t = h + r => r = t - h
    // d = h * r
    // d = h * (t - h) = h*t - h^2
    //
    // Win if d - w > 0
    // h * t - h^2 - w > 0
    //
    // Thresholds at:
    //   h*t - h^2 - w = 0
    //   -1 * h^2 + t * h - w = 0 => a= -1, b = t, c = -w
    //
    //  h = (-t +/- sqrt(t^2 - 4*-1*-w)) / (2 * -1)
    //  h = (-t +/- sqrt(t^2 - 4*w)) / (-2)
    pub fn ways_to_win(&self) -> u64 {
        let time = self.time as f64;
        let distance = self.distance as f64;
        let sqrt = f64::sqrt(time * time - 4.0 * distance);

        let first = (-time + sqrt) / -2.0;
        let first = match () {
            _ if first.fract() == 0.0 => f64::ceil(first) as u64 + 1,
            _ => f64::ceil(first) as u64,
        };

        let last = (-time - sqrt) / -2.0;
        let last = match () {
            _ if last.fract() == 0.0 => f64::floor(last) as u64 - 1,
            _ => f64::floor(last) as u64,
        };

        last - first + 1
    }
}
