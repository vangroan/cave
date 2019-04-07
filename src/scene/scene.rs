use specs::{Dispatcher, World};

pub struct Scene<'a, 'b> {
    pub world: World,
    pub update_dispatcher: Dispatcher<'a, 'b>,
    pub render_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Scene<'a, 'b> {
    pub fn new(world: World, update: Dispatcher<'a, 'b>, render: Dispatcher<'a, 'b>) -> Self {
        Scene {
            world,
            update_dispatcher: update,
            render_dispatcher: render,
        }
    }
}
