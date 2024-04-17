/* ===================================================
   Copyright 2010 VIRES Simulationstechnologie GmbH

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.


   NOTE:
   The methods have been realized using the CEPHES library

       http://www.netlib.org/cephes/

   and do neither constitute the only nor the exclusive way of implementing
   spirals for OpenDRIVE applications. Their sole purpose is to facilitate
   the interpretation of OpenDRIVE spiral data.
*/

use std::f64::consts::PI;

/* S(x) for small x */
// SINE_NUMERATOR
const SN: [f64; 6] = [
    -2.99181919401019853726e3,
    7.08840045257738576863e5,
    -6.29741486205862506537e7,
    2.54890880573376359104e9,
    -4.42979518059697779103e10,
    3.18016297876567817986e11,
];
// SINE_DENOMINATOR
const SD: [f64; 6] = [
    2.81376268889994315696e2,
    4.55847810806532581675e4,
    5.17343888770096400730e6,
    4.19320245898111231129e8,
    2.24411795645340920940e10,
    6.07366389490084639049e11,
];

/* C(x) for small x */
// COSINE_NUMERATOR
const CN: [f64; 6] = [
    -4.98843114573573548651e-8,
    9.50428062829859605134e-6,
    -6.45191435683965050962e-4,
    1.88843319396703850064e-2,
    -2.05525900955013891793e-1,
    9.99999999999999998822e-1,
];
// COSINE_DENOMINATOR
const CD: [f64; 7] = [
    3.99982968972495980367e-12,
    9.15439215774657478799e-10,
    1.25001862479598821474e-7,
    1.22262789024179030997e-5,
    8.68029542941784300606e-4,
    4.12142090722199792936e-2,
    1.00000000000000000118e0,
];

/* Auxiliary function f(x) */
// FRESNEL_F_NUMERATOR
const FN: [f64; 10] = [
    4.21543555043677546506e-1,
    1.43407919780758885261e-1,
    1.15220955073585758835e-2,
    3.45017939782574027900e-4,
    4.63613749287867322088e-6,
    3.05568983790257605827e-8,
    1.02304514164907233465e-10,
    1.72010743268161828879e-13,
    1.34283276233062758925e-16,
    3.76329711269987889006e-20,
];
// FRESNEL_F_DENOMINATOR
const FD: [f64; 10] = [
    7.51586398353378947175e-1,
    1.16888925859191382142e-1,
    6.44051526508858611005e-3,
    1.55934409164153020873e-4,
    1.84627567348930545870e-6,
    1.12699224763999035261e-8,
    3.60140029589371370404e-11,
    5.88754533621578410010e-14,
    4.52001434074129701496e-17,
    1.25443237090011264384e-20,
];

/* Auxiliary function g(x) */
// FRESNEL_G_NUMERATOR
const GN: [f64; 11] = [
    5.04442073643383265887e-1,
    1.97102833525523411709e-1,
    1.87648584092575249293e-2,
    6.84079380915393090172e-4,
    1.15138826111884280931e-5,
    9.82852443688422223854e-8,
    4.45344415861750144738e-10,
    1.08268041139020870318e-12,
    1.37555460633261799868e-15,
    8.36354435630677421531e-19,
    1.86958710162783235106e-22,
];
// FRESNEL_G_DENOMINATOR
const GD: [f64; 11] = [
    1.47495759925128324529e0,
    3.37748989120019970451e-1,
    2.53603741420338795122e-2,
    8.14679107184306179049e-4,
    1.27545075667729118702e-5,
    1.04314589657571990585e-7,
    4.60680728146520428211e-10,
    1.10273215066240270757e-12,
    1.38796531259578871258e-15,
    8.39158816283118707363e-19,
    1.86958710162783236342e-22,
];

fn poly_eval(x: f64, coefs: &[f64], init: f64) -> f64 {
    coefs.iter().fold(init, |acc, &c| acc * x + c)
}

fn fresnel(x: f64) -> (f64, f64) {
    let xa = x.abs();
    let xsq = xa.powi(2);

    let (sine, cosine) = if xsq < 2.5625 {
        let t = xsq.powi(2);
        let s = xa * xsq * poly_eval(t, &SN, 0.0) / poly_eval(t, &SD, t);
        let c = xa * poly_eval(t, &CN, 0.0) / poly_eval(t, &CD, 0.0);
        (s, c)
    } else if xa > 36974.0 {
        (0.5, 0.5)
    } else {
        let t = xsq * PI;
        let u = t.powi(2).recip();
        let t_inv = t.recip();
        let f = 1.0 - u * poly_eval(u, &FN, 0.0) / poly_eval(u, &FD, u);
        let g = t_inv * poly_eval(u, &GN, 0.0) / poly_eval(u, &GD, u);

        let trig = PI * 0.5 * xsq;
        let (sin_t, cos_t) = trig.sin_cos();

        let trig_x = PI * xa;
        let c = 0.5 + (f * sin_t - g * cos_t) / trig_x;
        let s = 0.5 - (f * cos_t + g * sin_t) / trig_x;
        (s, c)
    };

    (sine * x.signum(), cosine * x.signum())
}

/**
 * Compute the coordinates and tangent angle of a point on a clothoid (Euler spiral) given its arc length and curvature rate.
 * @param s     run-length along the spiral
 * @param dk    first derivative of curvature (1/m2)
 */
pub fn odr_spiral(s: f64, dk: f64) -> (f64, f64, f64) {
    let a = dk.abs().sqrt().recip() * PI.sqrt();
    let t_ang = s.powi(2) * dk * 0.5;

    let (y, x) = fresnel(s / a);

    (x * a, y * dk.signum() * a, t_ang)
}
