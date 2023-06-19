use plotters::prelude::*;
mod error_function;
mod tchebychev;
mod lagrange;


fn make_lagrange_function<'a>(xn: &'a Vec<f64> ,fxn: &'a Vec<f64>) -> impl Fn(f64) -> f64 + 'a {
  let poly = |x: f64|  lagrange::divided_differences(xn, fxn, x);
  return poly;
}



fn lagragnge_graph_exp_0to10() {  
  let xn: Vec<f64> = vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0];
  let fxn: Vec<f64> = xn.iter().map(|x: &f64| x.exp() ).collect();

  let xtche: Vec<f64> = tchebychev::roots_interval(5, 0.0, 10.0);
  let fxtche: Vec<f64> = xtche.iter().map(|x: &f64| x.exp() ).collect();

  let approx_exp =  make_lagrange_function(&xn , &fxn);
  let tche_exp = make_lagrange_function(&xtche , &fxtche);


  let root_drawing_area = BitMapBackend::new("images/lagrange_exp.png", (1024, 768))
  .into_drawing_area();
  
  root_drawing_area.fill(&WHITE).unwrap();
  
  let mut chart = ChartBuilder::on(&root_drawing_area)
  // enables Y axis, the size is 40 px
  .set_label_area_size(LabelAreaPosition::Left, 40)
  // enable X axis, the size is 40 px
  .set_label_area_size(LabelAreaPosition::Bottom, 40)
  .build_cartesian_2d(0.0..9.0 as f64, -400.0..9000.0)
  .unwrap();
  
  chart.configure_mesh().draw().unwrap();
  
  chart.draw_series(LineSeries::new(
  (0..900).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, x.exp())),
  &RED
  )).unwrap();
  
  chart.draw_series(LineSeries::new(
  (0..900).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, approx_exp(x))),
  &GREEN
  )).unwrap();

  chart.draw_series(LineSeries::new(
    (0..900).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, tche_exp(x))),
    &BLUE
    )).unwrap();
  
}



fn main() {
  error_function::plot_from(-4, 4);
  tchebychev::plot_reg(12);
  lagragnge_graph_exp_0to10();
}