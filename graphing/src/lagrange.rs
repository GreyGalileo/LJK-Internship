pub fn divided_differences (xn:&Vec<f64>, fxn:&Vec<f64>, x:f64) -> f64 {
  assert!(xn.len() == fxn.len());
  let n: usize = fxn.len();
  let mut fbracket: Vec<f64> = fxn.clone();
  for i in 0..n {
    for j in 1..(n-i){
      fbracket[n-j] = (fbracket[n-j] - fbracket[n-j-1]) / (xn[n-j] - xn[n-i-j-1])
    }
  }


  let mut result: f64 = 0f64;
  for i in 1..n+1{
    result = fbracket[n-i] + (x - xn[n-i]) * result;
  }
  
  return result;

}


pub fn build_function<'a>(xn: &'a Vec<f64> ,fxn: &'a Vec<f64>) -> impl Fn(f64) -> f64 + 'a {
  let poly = |x: f64|  divided_differences(xn, fxn, x);
  return poly;
}