use std::f64::consts::PI;
use plotters::prelude::*;


pub fn polynomial(n:u32, x:f64) -> f64 {
  //Creates the Tchebychev polynomial of degree n, defined for x in [-1, 1]
  //The roots of this polynomial are the Tchebychev interpolation points
  //In the worst case, these points give a better approximation of a function defined continuously on [-1,1]
  return (x.acos() * n as f64).cos();

}


pub fn recursive_polynomial(n:u32, x:f64) -> f64 {
  //Recursive alternative to the above function 
  //using the fact that the polynomial of degree 0  t0(x) = 1, degree 1 t1(x) = x
  //and degree n tn(x) = 2x * tn-1(x) - tn-2(x)
  match n {
    0 => 1f64,
    1 => x,
    _ => 2f64 * x * recursive_polynomial(n-1, x) - recursive_polynomial(n-2, x)
  }
}


pub fn roots(n:u32) -> Vec<f64>{
  //Finds the roots between -1 and 1 (the only roots) of the Tchebychev polynomial of degree n
  //The roots are given by cos(pi*(2i + 1)/n) where i is an integer between 0 and n-1
  let mut result: Vec<f64> = vec![0f64; n as usize]; //Initialises the vector
  let const_term: f64 = PI / ((2 * n) as f64);
  for i in 0..n {
    result[i as usize] = (((2 * i + 1) as f64) * const_term).cos();
  }
  return result;
}

pub fn roots_interval (n:u32, a:f64, b:f64) -> Vec<f64>{
  return roots(n).iter().map(|x: &f64| *x * (b - a) / 2f64 + (b+a)/2f64).collect::<Vec<f64>>();
}


pub fn plot_reg(n:u32) {
  let root_drawing_area = BitMapBackend::new("images/basic.png", (1024, 768))
    .into_drawing_area();
  
  root_drawing_area.fill(&WHITE).unwrap();
  
  let mut chart = ChartBuilder::on(&root_drawing_area)
    // enables Y axis, the size is 40 px
    .set_label_area_size(LabelAreaPosition::Left, 40)
    // enable X axis, the size is 40 px
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .build_cartesian_2d(-1.0..1.0 as f64, -2.0..2.0)
    .unwrap();
  
  chart.configure_mesh().draw().unwrap();
  
  chart.draw_series(LineSeries::new(
    (-100..101).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, polynomial(n ,x))),
    &RED
)).unwrap();

  chart.draw_series(roots(n).iter().map(|x:&f64| Circle::new((*x, 0f64), 2, &BLUE)),
  ).unwrap();
}

