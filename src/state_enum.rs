pub trait StateTrait {
    fn update(&self);
    fn draw(&self);
}

pub enum StateEnum {
    Game,
    MainMenu,
}
