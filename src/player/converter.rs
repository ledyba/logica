pub trait Converter<T>
  where
    T: cpal::Sample + Sync + Send + 'static,
{
  fn convert(input: &[f32], output: &mut [T]);
}

pub struct ConverterImpl<T> {
  _phantom: std::marker::PhantomData<T>,
}

impl Converter<i16> for ConverterImpl<i16> {
  fn convert(input: &[f32], output: &mut [i16]) {
    input.iter().zip(output.iter_mut()).for_each(|(i, o)| {
      *o = (i * 32767.0) as i16;
    });
  }
}

impl Converter<u16> for ConverterImpl<u16> {
  fn convert(input: &[f32], output: &mut [u16]) {
    input.iter().zip(output.iter_mut()).for_each(|(i, o)| {
      *o = (((i+1.0)/2.0) * 65525.0) as u16;
    });
  }
}
