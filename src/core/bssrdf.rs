//std
use std;
use std::f32::consts::PI;
// pbrt
use core::interpolation::integrate_catmull_rom;
use core::pbrt::INV_4_PI;
use core::pbrt::{Float, Spectrum};

pub struct BSSRDFTable {
    pub n_rho_samples: i32,
    pub n_radius_samples: i32,
    pub rho_samples: Vec<Float>,
    pub radius_samples: Vec<Float>,
    pub profile: Vec<Float>,
    pub rho_eff: Vec<Float>,
    pub profile_cdf: Vec<Float>,
}

impl BSSRDFTable {
    pub fn new(n_rho_samples: i32, n_radius_samples: i32) -> Self {
        BSSRDFTable {
            n_rho_samples: n_rho_samples,
            n_radius_samples: n_radius_samples,
            rho_samples: Vec::with_capacity(n_rho_samples as usize),
            radius_samples: Vec::with_capacity(n_radius_samples as usize),
            profile: Vec::with_capacity((n_radius_samples * n_rho_samples) as usize),
            rho_eff: Vec::with_capacity(n_rho_samples as usize),
            profile_cdf: Vec::with_capacity((n_radius_samples * n_rho_samples) as usize),
        }
    }
    pub fn eval_profile(&self, rho_index: i32, radius_index: i32) -> Float {
        self.profile[(rho_index * self.n_radius_samples + radius_index) as usize]
    }
}

pub fn fresnel_moment1(eta: Float) -> Float {
    // WORK
    0.0 as Float
}

pub fn fresnel_moment2(eta: Float) -> Float {
    // WORK
    0.0 as Float
}

pub fn beam_diffusion_ms(sigma_s: Float, sigma_a: Float, g: Float, eta: Float, r: Float) -> Float {
    let n_samples: i32 = 100;
    let mut ed: Float = 0.0;

    // precompute information for dipole integrand

    // compute reduced scattering coefficients $\sigmaps, \sigmapt$
    // and albedo $\rhop$
    let sigmap_s: Float = sigma_s * (1.0 as Float - g);
    let sigmap_t: Float = sigma_a + sigmap_s;
    let rhop: Float = sigmap_s / sigmap_t;
    // compute non-classical diffusion coefficient $D_\roman{G}$ using
    // Equation (15.24)
    let d_g: Float = (2.0 as Float * sigma_a + sigmap_s) / (3.0 as Float * sigmap_t * sigmap_t);
    // compute effective transport coefficient $\sigmatr$ based on $D_\roman{G}$
    let sigma_tr: Float = (sigma_a / d_g).sqrt();
    // determine linear extrapolation distance $\depthextrapolation$
    // using Equation (15.28)
    let fm1: Float = fresnel_moment1(eta);
    let fm2: Float = fresnel_moment2(eta);
    let ze: Float = -2.0 as Float * d_g * (1.0 as Float + 3.0 as Float * fm2)
        / (1.0 as Float - 2.0 as Float * fm1);
    // determine exitance scale factors using Equations (15.31) and (15.32)
    let c_phi: Float = 0.25 as Float * (1.0 as Float - 2.0 as Float * fm1);
    let c_e = 0.5 as Float * (1.0 as Float - 3.0 as Float * fm2);
    // for (int i = 0; i < n_samples; ++i) {
    for i in 0..n_samples {
        // sample real point source depth $\depthreal$
        let zr: Float =
            -(1.0 as Float - (i as Float + 0.5 as Float).ln() / n_samples as Float) / sigmap_t;
        // evaluate dipole integrand $E_{\roman{d}}$ at $\depthreal$ and add to _ed_
        let zv: Float = -zr + 2.0 as Float * ze;
        let dr: Float = (r * r + zr * zr).sqrt();
        let dv: Float = (r * r + zv * zv).sqrt();
        // compute dipole fluence rate $\dipole(r)$ using Equation (15.27)
        let phi_d: Float =
            INV_4_PI / d_g * ((-sigma_tr * dr).exp() / dr - (-sigma_tr * dv).exp() / dv);
        // compute dipole vector irradiance $-\N{}\cdot\dipoleE(r)$
        // using Equation (15.27)
        let ed_n: Float = INV_4_PI
            * (zr * (1.0 as Float + sigma_tr * dr) * (-sigma_tr * dr).exp() / (dr * dr * dr)
                - zv * (1.0 as Float + sigma_tr * dv) * (-sigma_tr * dv).exp() / (dv * dv * dv));
        // add contribution from dipole for depth $\depthreal$ to _ed_
        let e: Float = phi_d * c_phi + ed_n * c_e;
        let kappa: Float = 1.0 as Float - (-2.0 as Float * sigmap_t * (dr + zr)).exp();
        ed += kappa * rhop * rhop * e;
    }
    ed / n_samples as Float
}

pub fn beam_diffusion_ss(sigma_s: Float, sigma_a: Float, g: Float, eta: Float, r: Float) -> Float {
    // compute material parameters and minimum $t$ below the critical angle
    // Float sigma_t = sigma_a + sigma_s, rho = sigma_s / sigma_t;
    // Float t_crit = r * std::sqrt(eta * eta - 1);
    // Float ess = 0;
    // const int n_samples = 100;
    // for (int i = 0; i < n_samples; ++i) {
    //     // Evaluate single scattering integrand and add to _ess_
    //     Float ti = t_crit - std::log(1 - (i + .5f) / n_samples) / sigma_t;

    //     // Determine length $d$ of connecting segment and $\cos\theta_\roman{o}$
    //     Float d = std::sqrt(r * r + ti * ti);
    //     Float cos_theta_o = ti / d;

    //     // Add contribution of single scattering at depth $t$
    //     ess += rho * std::exp(-sigma_t * (d + t_crit)) / (d * d) *
    //            PhaseHG(cos_theta_o, g) * (1 - FrDielectric(-cos_theta_o, 1, eta)) *
    //            std::abs(cos_theta_o);
    // }
    // return ess / n_samples;
    // WORK
    0.0 as Float
}

pub fn compute_beam_diffusion_bssrdf(g: Float, eta: Float, t: &mut BSSRDFTable) {
    // choose radius values of the diffusion profile discretization
    t.radius_samples[0] = 0.0 as Float;
    t.radius_samples[1] = 2.5e-3 as Float;
    for i in 2..t.n_radius_samples as usize {
        t.radius_samples[i] = t.radius_samples[i - 1] * 1.2 as Float;
    }
    // choose albedo values of the diffusion profile discretization
    for i in 0..t.n_rho_samples as usize {
        t.rho_samples[i] = (1.0 as Float
            - (-8.0 as Float * i as Float / (t.n_rho_samples as Float - 1.0 as Float)).exp())
            / (1.0 as Float - (-8.0 as Float).exp());
    }
    // ParallelFor([&](int i) {
    for i in 0..t.n_rho_samples as usize {
        // compute the diffusion profile for the _i_th albedo sample

        // compute scattering profile for chosen albedo $\rho$
        for j in 0..t.n_radius_samples as usize {
            //         Float rho = t.rho_samples[i], r = t.radius_samples[j];
            let rho: Float = t.rho_samples[i];
            let r: Float = t.radius_samples[j];
            t.profile[i * t.n_radius_samples as usize + j] = 2.0 as Float
                * PI
                * r
                * (beam_diffusion_ss(rho, 1.0 as Float - rho, g, eta, r)
                    + beam_diffusion_ms(rho, 1.0 as Float - rho, g, eta, r));
        }
        // compute effective albedo $\rho_{\roman{eff}}$ and CDF for
        // importance sampling
        t.rho_eff[i] = integrate_catmull_rom(
            t.n_radius_samples,
            &t.radius_samples,
            &t.profile,
            i * t.n_radius_samples as usize,
            &mut t.profile_cdf,
            i * t.n_radius_samples as usize,
        );
    }
    // }, t.n_rho_samples);
}
