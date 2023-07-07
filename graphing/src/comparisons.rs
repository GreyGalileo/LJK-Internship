use plotters::prelude::*;
use crate::tchebychev;
use crate::lagrange;


pub fn uniform_tchebychev(f: impl Fn(f64) -> f64, a: f64, b: f64, n: u32) {
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
  
  
    let root_drawing_area = BitMapBackend::new("images/normal_interpolate2.png", (1024, 768))
    .into_drawing_area();
    
    root_drawing_area.fill(&WHITE).unwrap();
    
    let mut chart = ChartBuilder::on(&root_drawing_area)
    // enables Y axis, the size is 40 px
    .set_all_label_area_size(50)
    .caption(format!("Polynomial Interploation of e^(-x^2) Using n={} Uniform/Tchebychev Points from {} to {}", n, a ,b), ("sans-serif", 30))
    .build_cartesian_2d::<std::ops::Range<f64>,std::ops::Range<f64>>(a..b, 0.0..1.0)
    .unwrap();
    
    chart.configure_mesh().draw().unwrap();
    
    chart.draw_series(LineSeries::new(
    (a as i32 * 100.. b as i32 * 100).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, f(x))),
    &RED
    )).unwrap()
    .label("Initial Function").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));;;
    
    chart.draw_series(LineSeries::new(
    (a as i32 * 100.. b as i32 * 100).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, approx_f(x))),
    &GREEN
    )).unwrap()
    .label("Uniform Interpolation").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));;;
  
    chart.draw_series(LineSeries::new(
    (a as i32 * 100.. b as i32 * 100).map(|x:i32| x as f64 / 100.0).map(|x:f64| (x, tche_f(x))),
    &BLUE
    )).unwrap()
    .label("Tchebychev Interpolation").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));;

    chart.configure_series_labels().border_style(&BLACK).draw().unwrap();
    
  }
  
  
  
pub fn lagragnge_graph_exp_0to10() {
  /*
  Specific case of comparisons::uniform_tchebychev for x.exp() function 
  evaluated between 0 and 10 with n = 6
  equivalent to uniform_tchebychev(f: |x| x.exp(), a: 0, b: 10, n: 6)
  */
 
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
  


fn float_succ(u:f64) -> f64 { u + u / 2f64.powf(52.0) }

pub fn numerical_stability(f: impl Fn(f64) -> f64, a: f64, b: f64, n: u32) {
  /*
  Generates and plots the polynomials generated from 
  the lagrange interpolation using uniformly distributed points (Green)
  and the Tchebychev roots (Blue)
  approximating the function f (Red) on the interval [a,b]
  */
  
    let s: f64 = (b-a)/((n - 1)  as f64);
    let xn: Vec<f64> = (0..n).map(|x| a + s * x as f64).collect::<Vec<f64>>();
    let fxn: Vec<f64> = xn.iter().map(|x| f(*x)).collect();
    let fxerr: Vec<f64> = fxn.iter().map(|u| float_succ(*u)).collect::<Vec<f64>>();
  
    let xtche: Vec<f64> = tchebychev::roots_interval(n, a, b);
    let fxtche: Vec<f64> = xtche.iter().map(|x| f(*x)).collect();
    let fxtcherr = xtche.iter().map(|u| float_succ(*u)).collect::<Vec<f64>>();

    let approx_f =  lagrange::build_function(&xn , &fxn);
    let approx_err =  lagrange::build_function(&xn , &fxerr);
    let tche_f = lagrange::build_function(&xtche , &fxtche);
    let tche_err = lagrange::build_function(&xtche , &fxtcherr);
  
  
    let root_drawing_area = BitMapBackend::new("images/numerical_stability.png", (1024, 768))
    .into_drawing_area();
    
    root_drawing_area.fill(&WHITE).unwrap();
    


    
  }