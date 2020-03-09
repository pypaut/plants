use quaternion;
use crate::vector3;


#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Turtle {
    pos: vector3::Vector3,
    heading: vector3::Vector3,
    left: vector3::Vector3,
    up: vector3::Vector3
}

impl Turtle {
    // Create a new default turtle pointing upward
    pub fn new() -> Turtle {
        Turtle{pos: vector3::Vector3::new(0f64, 0f64, 0f64),
        heading: vector3::Vector3::new(0f64, 0f64, 1f64),
        left: vector3::Vector3::new(0f64, -1f64, 0f64),
        up: vector3::Vector3::new(1f64, 0f64, 0f64)}
    }

    pub fn pos(&self) -> vector3::Vector3 {
        self.pos
    }

    pub fn heading(&self) -> vector3::Vector3 {
        self.heading
    }

    pub fn up(&self) -> vector3::Vector3 {
        self.up
    }

    pub fn forward(&mut self, dist: f64) {
        self.pos = self.pos + self.heading * dist;
    }

    pub fn rot_pitch(&mut self, a: f64) {  // y
        let quat = quaternion::axis_angle(self.left.to_arr(), a);

        self.heading = vector3::Vector3::from(quaternion::rotate_vector(quat,
                                                               self.heading.to_arr()));
        self.left = vector3::Vector3::from(quaternion::rotate_vector(quat,
                                                            self.left.to_arr()));
        self.up = vector3::Vector3::from(quaternion::rotate_vector(quat,
                                                            self.up.to_arr()));
    }

    pub fn rot_roll(&mut self, a: f64) {  // x
        let quat = quaternion::axis_angle(self.heading.to_arr(), a);

        self.heading = vector3::Vector3::from(quaternion::rotate_vector(quat,
                                                               self.heading.to_arr()));
        self.left = vector3::Vector3::from(quaternion::rotate_vector(quat,
                                                            self.left.to_arr()));
        self.up = vector3::Vector3::from(quaternion::rotate_vector(quat,
                                                          self.up.to_arr()));
    }

    pub fn rot_yaw(&mut self, a: f64) {  // z
        let quat = quaternion::axis_angle(self.up.to_arr(), a);

        self.heading = vector3::Vector3::from(quaternion::rotate_vector(quat,
                                                               self.heading.to_arr()));
        self.left = vector3::Vector3::from(quaternion::rotate_vector(quat,
                                                            self.left.to_arr()));
        self.up = vector3::Vector3::from(quaternion::rotate_vector(quat,
                                                          self.up.to_arr()));
    }
}
