
pub trait Plugin
{
  fn new() -> Self where Self: Sized;
}
