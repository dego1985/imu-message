use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IMUMessage {
    pub id: u32,
    pub gyro: Vec<f32>,
    pub accl: Vec<f32>,
    pub mag: Vec<f32>,
}
