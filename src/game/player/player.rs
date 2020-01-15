pub struct Player {
    name: &'static str,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: "something",
        }
    }
}
