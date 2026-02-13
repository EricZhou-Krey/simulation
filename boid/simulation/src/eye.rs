use crate::*;
use std::f32::consts::*;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);
        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let to = food.position - position;
            let angle =
                na::Rotation2::rotation_between(&na::Vector2::y(), &to).angle() - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if to.norm() >= self.fov_range
                || angle < -self.fov_angle / 2.0
                || angle > self.fov_angle / 2.0
            {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;
        }

        cells
    }
}

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI * FRAC_PI_4;
const CELLS: usize = 9;

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}
