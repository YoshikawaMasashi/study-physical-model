use ndarray::prelude::*;

pub struct FDMRunner {
    size: (usize, usize),
    speed: f64,
    delta_x: f64,
    delta_t: f64,
    source_pos: (usize, usize),
}

impl FDMRunner {
    pub fn new(
        size: (usize, usize),
        speed: f64,
        delta_x: f64,
        delta_t: f64,
        source_pos: (usize, usize),
    ) -> Self {
        FDMRunner {
            size,
            speed,
            delta_x,
            delta_t,
            source_pos,
        }
    }

    pub fn step(&self, before: Array2<f64>, before_before: Array2<f64>, time_index: usize) -> Array2<f64> {
        let mut next = Array::zeros(self.size.f());

        for calc_x in 0..self.size.0 {
            for calc_y in 0..self.size.1 {
                if calc_x == 0
                    || calc_y == 0
                    || calc_x == self.size.0 - 1
                    || calc_y == self.size.1 - 1
                {
                    next[[calc_x, calc_y]] = 0.0;
                } else {
                    let d2pdx2 = (
                        before[[calc_x + 1, calc_y]]
                        - 2.0 * before[[calc_x, calc_y]]
                        + before[[calc_x - 1, calc_y]]
                    ) / self.delta_x;
                    let d2pdy2 = (
                        before[[calc_x, calc_y + 1]]
                        - 2.0 * before[[calc_x, calc_y]]
                        + before[[calc_x, calc_y - 1]]
                    ) / self.delta_x;

                    next[[calc_x, calc_y]] = self.speed * self.speed * self.delta_t * self.delta_t * (
                        d2pdx2 + d2pdy2
                    ) + 2.0 * before[[calc_x, calc_y]] - before_before[[calc_x, calc_y]];

                    if calc_x == self.source_pos.0 && calc_y == self.source_pos.1 {
                        next[[calc_x, calc_y]] += ((time_index as f64) * self.delta_t * 0.3).cos();
                    }
                }
            }
        }

        next
    }

    pub fn run(&self, steps: usize) -> Vec<Array2<f64>> {
        let mut ret = vec![];
        for time_index in 0..steps {
            if time_index == 0 {
                let next = self.step(Array::zeros(self.size.f()), Array::zeros(self.size.f()), time_index);
                ret.push(next);
            } else if time_index == 1 {
                let next = self.step(ret[ret.len() - 1].clone(), Array::zeros(self.size.f()), time_index);
                ret.push(next);
            } else {
                let next = self.step(ret[ret.len() - 1].clone(), ret[ret.len() - 2].clone(), time_index);
                ret.push(next);
            }
        }
        ret
    }
}
