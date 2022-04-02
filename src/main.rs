use draw::Stage;
use miniquad::{conf::Conf, UserData};

pub mod draw;

fn main() {
    miniquad::start(Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}
