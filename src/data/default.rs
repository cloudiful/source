pub(crate) struct Velocity {
    pub(crate) story_page: f32,
}

impl Velocity {
    pub(crate) fn new() -> Velocity {
        Velocity { story_page: 5.0 }
    }
}
