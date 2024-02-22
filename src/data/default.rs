pub(crate) struct Velocity {
    pub(crate) story_point: f32,
}

impl Velocity {
    pub(crate) fn new() -> Velocity {
       Velocity {
           story_point: 5.0,
       }
    }
}