use plotters::prelude::*;
mod error_function;
mod tchebychev;
mod lagrange;
mod comparisons;


fn main() {
// error_function::plot_from(-4, 4);
// tchebychev::plot_reg(12);
//  lagragnge_graph_exp_0to10();
  fn f(x:f64)->f64{ (-x*x).exp()}
  comparisons::uniform_tchebychev(f, -2.0, 2.0, 9);
}