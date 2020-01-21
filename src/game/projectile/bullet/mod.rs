pub mod standard;

pub trait Bullet  {
    fn damage(&self) -> ();
    fn should_drop(&self) -> bool;
}
