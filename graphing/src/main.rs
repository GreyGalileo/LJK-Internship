use plotters::prelude::*;
mod error_function;
mod tchebychev;
mod lagrange;

fn compare_uniform_tchebychev(f: impl Fn(f64) -> f64, a: f64, b: f64, n: u32) {
/*
Generates and plots the polynomials generated from 
the lagrange interpolation using uniformly distributed points (Green)
and the Tchebychev roots (Blue)
approximating the function f (Red) on the interval [a,b]
*/

  let s: f64 = (b-a)/((n - 1)  as f64);
  let xn: Vec<f64> = (0..n).map(|x| a + s * x as f64).collect::<Vec<f64>>();
  let fxn: Vec<f64> = xn.iter().map(|x| f(*x)).collect();

  let xtche: Vec<f64> = tchebychev::roots_interval(n, a, b);
  let fxtche: Vec<f64> = xtche.iter().map(|x| f(*x)).collect();

  let approx_f =  lagrange::build_function(&xn , &fxn);
  let tche_f = lagrange::build_function(&xtche , &fxtche);


  let root_drawing_area = BitMapBackend::new("images/comparison.png", (1024, 768))
  .into_drawing_area();
  
  root_drawing_area.fill(&WHITE).unwrap();
  
  let mut chart = ChartBuilder::on(&root_drawing_area)
  // enables Y axis, the size is 40 px
  .set_label_area_size(LabelAreaPosition::Left, 40)
  // enable X axis, the size is 40 px
  .set_label_area_size(LabelAreaPosition::Bottom, 40)
  .build_cartesian_2d::<std::ops::Range<f64>,std::ops::Range<f64>>(a..b, 0.0..0.2)
  .unwrap();
  
  chart.configure_mesh().draw().unwrap();
  
  chart.draw_series(LineSeries::new(
  (a as i32 * 100.. b as i32 * 100).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, f(x))),
  &RED
  )).unwrap();
  
  chart.draw_series(LineSeries::new(
  (a as i32 * 100.. b as i32 * 100).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, approx_f(x))),
  &GREEN
  )).unwrap();

  chart.draw_series(LineSeries::new(
    (a as i32 * 100.. b as i32 * 100).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, tche_f(x))),
    &BLUE
    )).unwrap();
  
}



fn lagragnge_graph_exp_0to10() {

  let xn: Vec<f64> = vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0];
  let fxn: Vec<f64> = xn.iter().map(|x: &f64| x.exp() ).collect();

  let xtche: Vec<f64> = tchebychev::roots_interval(6, 0.0, 10.0);
  let fxtche: Vec<f64> = xtche.iter().map(|x: &f64| x.exp() ).collect();

  let approx_exp =  lagrange::build_function(&xn , &fxn);
  let tche_exp = lagrange::build_function(&xtche , &fxtche);


  let root_drawing_area = BitMapBackend::new("images/lagrange_exp.png", (1024, 768))
  .into_drawing_area();
  
  root_drawing_area.fill(&WHITE).unwrap();
  
  let mut chart = ChartBuilder::on(&root_drawing_area)
  // enables Y axis, the size is 40 px
  .set_label_area_size(LabelAreaPosition::Left, 40)
  // enable X axis, the size is 40 px
  .set_label_area_size(LabelAreaPosition::Bottom, 40)
  .build_cartesian_2d(0.0..10.0 as f64, -400.0..14000.0)
  .unwrap();
  
  chart.configure_mesh().draw().unwrap();
  
  chart.draw_series(LineSeries::new(
  (0..1000).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, x.exp())),
  &RED
  )).unwrap();
  
  chart.draw_series(LineSeries::new(
  (0..1000).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, approx_exp(x))),
  &GREEN
  )).unwrap();

  chart.draw_series(LineSeries::new(
    (0..1000).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, tche_exp(x))),
    &BLUE
    )).unwrap();
  
}



fn main() {
// error_function::plot_from(-4, 4);
//  tchebychev::plot_reg(12);
//  lagragnge_graph_exp_0to10();
  fn f(x:f64)->f64{ 1f64 / (x*x + 5f64)}
  compare_uniform_tchebychev(f, -10.0, 10.0, 11)
}