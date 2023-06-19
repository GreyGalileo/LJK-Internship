use plotters::prelude::*;
use std::f64::consts::PI;


pub fn approximate (mut x:f64, k:f64) -> f64 {
  //approximates the error function evaluated at x to a degree of precision equal to 10^(-k)
  let mut sign_x:f64 = 1.0;
  if  x < 0f64 {
    sign_x = -1.0;
    x = -x;
  }
  let xsquared:f64 = x*x;
  let a0:f64 = 2f64 * x / (PI).sqrt() * (-xsquared).exp() ;
  let precision:f64 = 10f64.powf(-k);
  let mut result:f64 = a0;
  let mut n:f64 = 1f64;
  let mut an:f64 = a0;
  while n < xsquared || (an * xsquared / (n - xsquared) > precision) {
    an = an * (2f64 * xsquared) / (2f64 * n + 1f64);
    result += an;
    n += 1.0;
  }

  return sign_x * result;
}


pub fn plot_from (a:i32, b:i32) {
  //Produces a graph of the error function plotted on [a,b] as a png named error_function.png
  let root_drawing_area = BitMapBackend::new("images/error_function.png", (1024, 768))
    .into_drawing_area();
  
  root_drawing_area.fill(&WHITE).unwrap();
  
  let mut chart = ChartBuilder::on(&root_drawing_area)
    // enables Y axis, the size is 40 px
    .set_label_area_size(LabelAreaPosition::Left, 40)
    // enable X axis, the size is 40 px
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .build_cartesian_2d(a as f64..b as f64, -1.0..1.0)
    .unwrap();
  
  chart.configure_mesh().draw().unwrap();
  
  chart.draw_series(LineSeries::new(
    (100*a..100*b).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, approximate(x,14f64))),
    &RED
)).unwrap();
}