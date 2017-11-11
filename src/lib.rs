#![no_std]

use core::ops::{Add, Sub};

// double precision
#[allow(dead_code)]
pub mod unsafe_cephes_double {
    extern "C" {
        // Floating point numeric utilities
        /// Round to nearest or event integer valued f64.
        pub fn round(x: f64) -> f64;
        /// Largest integer less than or equal to x.
        pub fn floor(x: f64) -> f64;
        /// Smallest integer greater than or equal to x.
        pub fn ceil(x: f64) -> f64;
        /// Return the significand between 0.5 and 1. Write exponent to expnt.
        /// x = y * 2**expn
        pub fn frexp(x: f64, expnt: &mut i32) -> f64;
        /// Multiply x by 2**n.
        pub fn ldexp(x: f64, n: i32) -> f64;
        /// Absolute value.
        pub fn fabs(x: f64) -> f64;
        /// Return 1 if the sign bit of x is 1, else 0.
        pub fn signbit(x: f64) -> i32;
        /// Return 1 if x is NaN, else 0.
        pub fn isnan(x: f64) -> i32;
        /// Return 1 if x is finite, else 0.
        pub fn isfinite(x: f64) -> i32;

        // Roots
        /// Cube root.
        pub fn cbrt(x: f64) -> f64;
        /// Square root.
        pub fn sqrt(x: f64) -> f64;
        /// Integer square root.
        pub fn lsqrt(x: i64) -> i64;

        // Exponential functions
        /// Exponential function.
        pub fn exp(x: f64) -> f64;
        /// Base 10 exponential function.
        pub fn exp10(x: f64) -> f64;
        /// Base 2 exponential function.
        pub fn exp2(x: f64) -> f64;
        /// Compute accurately exponential of squared argument.
        pub fn expm1(x: f64) -> f64;
        /// Compute accurately exp(x) - 1 for x close to 0.
        pub fn expx2(x: f64, sign: i32) -> f64;
        /// Exponential integral.
        pub fn ei(x: f64) -> f64;
        /// Error function.
        pub fn erf(x: f64) -> f64;
        /// Complementary error function.
        pub fn erfc(x: f64) -> f64;
        /// Power function.
        pub fn pow(x: f64, y: f64) -> f64;
        /// Integer power function.
        pub fn powi(x: f64, n: i32) -> f64;

        // Logarithmic functions
        /// Natural logarithm.
        pub fn log(x: f64) -> f64;
        /// Common logarithm.
        pub fn log10(x: f64) -> f64;
        /// Base 2 logarithm.
        pub fn log2(x: f64) -> f64;
        /// Compute accurately log(1 + x) for x close to 0.
        pub fn log1p(x: f64) -> f64;
        /// Dilogarithm (Spence's function).
        pub fn spence(x: f64) -> f64;

        // Trigonometric functions
        /// Circular sine.
        pub fn sin(x: f64) -> f64;
        /// Circular cosine.
        pub fn cos(x: f64) -> f64;
        /// Circular tangent.
        pub fn tan(x: f64) -> f64;
        /// Inverse circular sine.
        pub fn asin(x: f64) -> f64;
        /// Inverse circular cosine.
        pub fn acos(x: f64) -> f64;
        /// Inverse circular tangent.
        pub fn atan(x: f64) -> f64;
        /// Quadrant-correct inverse circular tangent.
        pub fn atan2(y: f64, x: f64) -> f64;
        /// Compute accurately cos(x) - 1 for x close to 0.
        pub fn cosm1(x: f64) -> f64;
        /// Sine and cosine integrals.
        pub fn sici(x: f64, si: &mut f64, ci: &mut f64) -> f64;

        // Hyperbolic functions
        /// Hyperbolic sine.
        pub fn sinh(x: f64) -> f64;
        /// Hyperbolic cosine.
        pub fn cosh(x: f64) -> f64;
        /// Hyperbolic tangent.
        pub fn tanh(x: f64) -> f64;
        /// Inverse hyperbolic sine.
        pub fn asinh(x: f64) -> f64;
        /// Inverse hyperbolic cosine.
        pub fn acosh(x: f64) -> f64;
        /// Inverse hyperbolic tangent.
        pub fn atanh(x: f64) -> f64;
        /// Hyperbolic sine and cosine integrals.
        pub fn shichi(x: f64, chi: &mut f64, shi: &mut f64);

        // Beta functions
        /// Beta function.
        pub fn beta(a: f64, b: f64) -> f64;
        /// Regularized incomplete beta function.
        pub fn incbet(a: f64, b: f64, x: f64) -> f64;
        /// Inverse of incomplete beta integral.
        pub fn incbi(a: f64, b: f64, y: f64) -> f64;

        // Gamma functions
        /// Gamma function.
        pub fn gamma(x: f64) -> f64;
        /// Reciprocal gamma function.
        pub fn rgamma(x: f64) -> f64;
        /// Natural logarithm of gamma function.
        pub fn lgam(x: f64) -> f64;
        /// Regularized incomplete gamma integral.
        pub fn igam(a: f64, x: f64) -> f64;
        /// Complemented incomplete gamma integral.
        pub fn igamc(a: f64, x: f64) -> f64;
        /// Inverse of complemented incomplete gamma integral.
        pub fn igami(a: f64, p: f64) -> f64;
        /// Psi (digamma) function.
        pub fn psi(x: f64) -> f64;
        /// Factorial function.
        pub fn fac(i: i32) -> f64;

        // Bessel functions
        /// Bessel function of order zero.
        pub fn j0(x: f64) -> f64;
        /// Bessel function of order one.
        pub fn j1(x: f64) -> f64;
        /// Bessel function of integer order.
        pub fn jn(n: i32, x: f64) -> f64;
        /// Bessel function of real order.
        pub fn jv(n: f64, x: f64) -> f64;

        /// Bessel function of the second kind, order zero.
        pub fn y0(x: f64) -> f64;
        /// Bessel function of the second kind, order one.
        pub fn y1(x: f64) -> f64;
        /// Bessel function of the second kind, integer order.
        pub fn yn(n: i32, x: f64) -> f64;
        /// Bessel function of the second kind, real order.
        pub fn yv(v: f64, x: f64) -> f64;

        /// Modified Bessel function of order zero.
        pub fn i0(x: f64) -> f64;
        /// Modified Bessel function of order zero, exponentially scaled.
        pub fn i0e(x: f64) -> f64;
        /// Modified Bessel function of order one.
        pub fn i1(x: f64) -> f64;
        /// Modified Bessel function of order one, exponentially scaled.
        pub fn i1e(x: f64) -> f64;
        /// Modified Bessel function of real order.
        pub fn iv(v: f64, x: f64) -> f64;

        /// Modified Bessel function of the third kind, order zero.
        pub fn k0(x: f64) -> f64;
        /// Modified Bessel function of the third kind, order zero,
        /// exponentially scaled.
        pub fn k0e(x: f64) -> f64;
        /// Modified Bessel function of the third kind, order one.
        pub fn k1(x: f64) -> f64;
        /// Modified Bessel function of the third kind, order one,
        /// exponentially scaled.
        pub fn k1e(x: f64) -> f64;
        /// Modified Bessel function of the third kind, integer order.
        pub fn kn(n: i32, x: f64) -> f64;

        // Elliptic functions
        /// Incomplete elliptic integral of the first kind.
        pub fn ellik(phi: f64, m: f64) -> f64;
        /// Incomplete elliptic integral of the second kind.
        pub fn ellie(phi: f64, m: f64) -> f64;
        /// Complete elliptic integral of the first kind.
        pub fn ellpk(m1: f64) -> f64;
        /// Complete elliptic integral of the second kind.
        pub fn ellpe(m1: f64) -> f64;
        /// Jacobian elliptic function.
        pub fn ellpj(u: f64, m: f64, sn: &mut f64, cn: &mut f64, dn: &mut f64, phi: &mut f64) -> i32;

        // Hypergeometric functions
        /// Confluent hypergeometric function 1F1.
        pub fn hyperg(a: f64, b: f64, x: f64) -> f64;
        /// Hypergeometric function 1F2.
        pub fn onef2(a: f64, b: f64, c: f64, x: f64, err: &mut f64) -> f64;
        /// Gauss hypergeometric function 2F1.
        pub fn hyp2f1(a: f64, b: f64, c: f64, x: f64) -> f64;
        /// Hypergeometric function 3F0.
        pub fn threef0(a: f64, b: f64, c: f64, x: f64, err: &mut f64) -> f64;

        // Distributions
        /// Binomial distribution.
        pub fn bdtr(k: i32, n: i32, p: f64) -> f64;
        /// Complemented binomial distribution.
        pub fn bdtrc(k: i32, n: i32, p: f64) -> f64;
        /// Inverse of binomial distribution.
        pub fn bdtri(k: i32, n: i32, y: f64) -> f64;

        /// Negative binomial distribution.
        pub fn nbdtr(k: i32, n: i32, p: f64) -> f64;
        /// Complemented negative binomial distribution.
        pub fn nbdtrc(k: i32, n: i32, p: f64) -> f64;
        /// Inverse of negative binomial distribution.
        pub fn nbdtri(k: i32, n: i32, p: f64) -> f64;

        /// Beta distribution.
        pub fn btdtr(a: f64, b: f64, x: f64) -> f64;

        /// Chi-square distribution.
        pub fn chdtr(df: f64, x: f64) -> f64;
        /// Complemented chi-square distribution.
        pub fn chdtrc(v: f64, x: f64) -> f64;
        /// Inverse of complemented chi-square distribution.
        pub fn chdtri(df: f64, y: f64) -> f64;

        /// F distribution.
        pub fn fdtr(df1: i32, df2: i32, x: f64) -> f64;
        /// Complemented F distribution.
        pub fn fdtrc(df1: i32, df2: i32, x: f64) -> f64;
        /// Inverse of complemented F distribution.
        pub fn fdtri(df1: i32, df2: i32, p: f64) -> f64;

        /// Gamma distribution.
        pub fn gdtr(a: f64, b: f64, x: f64) -> f64;
        /// Complemented gamma distribution.
        pub fn gdtrc(a: f64, b: f64, x: f64) -> f64;

        /// Normal distribution.
        pub fn ndtr(x: f64) -> f64;
        /// Inverse of normal distribution.
        pub fn ndtri(y: f64) -> f64;

        /// Poisson distribution.
        pub fn pdtr(k: i32, m: f64) -> f64;
        /// Complemented Poisson distribution.
        pub fn pdtrc(k: i32, m: f64) -> f64;
        /// Inverse of Poisson distribution.
        pub fn pdtri(k: i32, y: f64) -> f64;

        /// Student's t distribution.
        pub fn stdtr(k: i16, t: f64) -> f64;
        /// Inverse of Student's t distribution.
        pub fn stdtri(k: i32, p: f64) -> f64;

        // Misc special functions
        /// Airy function.
        pub fn airy(x: f64, ai: &mut f64, aip: &mut f64, bi: &mut f64, bip: &mut f64) -> i32;
        /// Dawson's integral.
        pub fn dawsn(x: f64) -> f64;
        /// Fresnel integral.
        pub fn fresnl(x: f64, s: &mut f64, c: &mut f64);
        /// Integral of Planck's black body radiation formula.
        pub fn plancki(lambda: f64, temperature: f64) -> f64;
        /// Struve function.
        pub fn struve(v: f64, x: f64) -> f64;
        /// Riemann zeta function.
        pub fn zetac(x: f64) -> f64;
        /// Riemann zeta function of two arguments.
        pub fn zeta(x: f64, q: f64) -> f64;
    }
}

pub mod cephes_double {
    use unsafe_cephes_double;

    /// Function to compute the base-10 exponential of x
    ///
    /// # Original Description from Stephen L. Moshier
    /// Returns 10 raised to the x power.
    ///
    /// Range reduction is accomplished by expressing the argument
    /// as 10^x = 2^n 10^f, with |f| < 0.5 log10(2).
    /// The Pade' form
    ///
    /// 10^x = 1 + 2x P( x^2)/( Q( x^2 ) - P( x^2 ) )
    ///
    /// is used to approximate 10^f.
    ///
    /// ACCURACY (Relative error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :-------: | :---: | :--: |
    /// | IEEE       | -307,+307 | 30000 | 2.2e-16 | 5.5e-17 |
    ///
    /// ERROR MESSAGES:
    ///
    /// | message | condition | value returned |
    /// | :-----: | :-------: | :------------: |
    /// | exp10 underflow | x < -MAXL10 | 0.0  |
    /// | exp10 overflow | x > MAXL10 | MAXNUM |
    ///
    /// IEEE arithmetic: MAXL10 = 308.2547155599167.
    ///
    ///
    /// # Arguments
    /// * `x`: A double precision number
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::exp10;
    /// let x = 1.0f64;
    /// exp10(x);
    /// ```
    pub fn exp10(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::exp10(x) }
    }

    /// Function to accurately compute `exp(x) - 1` for small x
    ///
    /// # Arguments
    /// * `x`: A double precision number
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::expm1;
    /// let x = 1.0f64;
    /// expm1(x);
    /// ```
    pub fn expm1(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::expm1(x) }
    }

    /// Function to accurately compute the exponential of a squared argument
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// Computes y = exp(x*x) while suppressing error amplification
    /// that would ordinarily arise from the inexactness of the
    /// exponential argument x*x.
    ///
    /// If sign < 0, the result is inverted; i.e., y = exp(-x*x) .
    ///
    /// ACCURACY (Relative error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :------: | :--: | :--:|
    /// | IEEE | -26.6, 26.6 | 10^7 | 3.9e-16 | 8.9e-17 |
    ///
    /// # Arguments
    /// * `x`: A double precision number
    /// * 'sign': An integer representing the sign of the exponent
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::expx2;
    /// let x = 1.0f64;
    /// expx2(x, -1);
    /// ```
    pub fn expx2(x: f64, sign: i32) -> f64 {
        unsafe { unsafe_cephes_double::expx2(x, sign) }
    }

    /// Function to accurately compute the exponential integral of x
    ///
    /// The exponential integral is given by:
    ///
    /// $Ei(x) = -\int^{\infty}_{-x} \frac{e^{-t}}{t} dt$
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// ACCURACY (Relative error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :------: | :--: | :--:|
    /// | IEEE | 0, 100 | 50000 | 8.6e-16 | 1.3e-16 |
    ///
    /// # Arguments
    /// * `x`: A double precision number
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::ei;
    /// let x = 1.0f64;
    /// ei(x);
    /// ```
    pub fn ei(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::ei(x) }
    }

    /// Function to accurately compute the error function of x
    ///
    /// The error function is given by:
    ///
    /// $Erf(x) = \frac{2}{\sqrt{pi}} -\int^{x}_{0} e^{-t^2} dt$
    ///
    /// # Arguments
    /// * `x`: A double precision number
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::erf;
    /// let x = 1.0f64;
    /// erf(x);
    /// ```
    pub fn erf(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::erf(x) }
    }

    /// Function to accurately compute the complementary error function of x
    ///
    /// The error function is given by:
    ///
    /// $Erf(x) = 1 + \frac{2}{\sqrt{pi}} -\int^{x}_{0} e^{-t^2} dt$
    ///
    /// # Arguments
    /// * `x`: A double precision number
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::erfc;
    /// let x = 1.0f64;
    /// erfc(x);
    /// ```
    pub fn erfc(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::erfc(x) }
    }

    /// Function to accurately compute `log(1 + x)`
    ///
    /// # Arguments
    /// * `x`: A double precision number
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::log1p;
    /// let x = 0.1f64;
    /// log1p(x);
    /// ```
    pub fn log1p(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::log1p(x) }
    }

    /// Function to accurately compute the dilogarithm function
    ///
    /// Spence's function is a special case of the polylogarithm function, and
    /// is defined as:
    ///
    /// $ Li_2(x) = \int^x_1 \frac{ln(t)}{t - 1} du
    ///
    /// Note, this implies that the domain has been shifted by 1 from the
    /// standard definition (as per wikipedia) of:
    ///
    /// $ Li_2(x) = - \int^x_0 \frac{ ln(1 - t) }{ t } dt
    ///
    /// # Original Description from Stephen L. Moshier
    /// Computes the integral for x >= 0.  A rational approximation gives the
    /// integral in the interval (0.5, 1.5).  Transformation formulas for 1/x
    /// and 1-x are employed outside the basic expansion range.
    ///
    /// ACCURACY(Relative error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :---: | :--: | :--: | :--: | :--: | :--: |
    /// | IEEE  | 0,4 | 30000 | 3.9e-15 | 5.4e-16 |
    ///
    /// # Arguments
    /// * `x`: A double precision number
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::spence;
    /// let x = 0.1f64;
    /// spence(x);
    /// ```
    pub fn spence(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::spence(x) }
    }

    /// Function to accurately compute `cos(x) - 1` for small x
    ///
    /// # Arguments
    /// * `x`: A double precision number
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::cosm1;
    /// let x = 0.1f64;
    /// cosm1(x);
    /// ```
    pub fn cosm1(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::cosm1(x) }
    }

    /// Function to accurately compute sine and cosine integrals
    ///
    /// The sine integral is defined as:
    ///
    /// $ Si(x) = \int_0^{\infty} \frac{ sin(t) }{t} dt $
    ///
    /// and the cosine integral is defined as:
    ///
    /// $ Ci(x) = -\int_x^{\infty} \frac{ cos(t) }{t} dt $
    ///
    /// which reduces to:
    ///
    /// $ Ci(x) = \gamma + ln(x) + \int_0^x \frac{cos(t) - 1}{t} dt $
    ///
    /// where $\gamma$ is the euler constant.
    ///
    /// # Original Description from Stephen L. Moshier
    /// The integrals are approximated by rational functions.
    /// For x > 8 auxiliary functions f(x) and g(x) are employed
    /// such that
    ///
    /// Ci(x) = f(x) sin(x) - g(x) cos(x)
    /// Si(x) = pi/2 - f(x) cos(x) - g(x) sin(x)
    ///
    /// ACCURACY(Absolute error, except relative when > 1):
    ///
    /// | arithmetic | Function | domain | # trials | peak | rms |
    /// | :---: | :--: | :--: | :--: | :--: | :--: | :--: |
    /// | IEEE | Si | 0,50 | 30000 | 4.4e-16 | 7.3e-17 |
    /// | IEEE | Ci | 0,50 | 30000 | 6.9e-16 | 5.1e-17 |
    ///
    /// # Arguments
    /// * `x`: A double precision number
    /// * `Si`: A mutable double precision number that will return the sine
    ///         integral value
    /// * `Ci`: A mutable double precision number that will return the cosine
    ///         integral value
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::sici;
    /// let x = 0.1f64;
    /// let mut si = 0.0_f64;
    /// let mut ci = 0.0_f64;
    /// let mut code = 0.0_f64;
    /// code = sici(0.5_f64, &mut si, &mut ci);
    /// ```
    pub fn sici(x: f64, si: &mut f64, ci: &mut f64) -> f64 {
        unsafe { unsafe_cephes_double::sici(x, si, ci) }
    }

    /// Function to accurately compute hyperbolic sine and cosine integrals
    ///
    /// The hyperbolic sine integral is defined as:
    ///
    /// $ Shi(x) = \int_0^{\infty} \frac{ sinh(t) }{t} dt $
    ///
    /// and the hyperbolic cosine integral is defined as:
    ///
    /// $ Chi(x) = -\int_x^{\infty} \frac{ cosh(t) }{t} dt $
    ///
    /// which reduces to:
    ///
    /// $ Chi(x) = \gamma + ln(x) + \int_0^x \frac{cosh(t) - 1}{t} dt $
    ///
    /// where $\gamma$ is the euler constant.
    ///
    /// # Original Description from Stephen L. Moshier
    /// The integrals are approximated by rational functions.
    /// For x > 8 auxiliary functions f(x) and g(x) are employed
    /// such that
    ///
    /// Ci(x) = f(x) sin(x) - g(x) cos(x)
    /// Si(x) = pi/2 - f(x) cos(x) - g(x) sin(x)
    ///
    /// ACCURACY(Relative error)):
    ///
    /// | arithmetic | Function | domain | # trials | peak | rms |
    /// | :---: | :--: | :--: | :--: | :--: | :--: | :--: |
    /// | IEEE | Shi | 0,88 | 30000 | 6.9-16 | 1.6e-16 |
    ///
    /// ACCURACY(Absolute error, except relative when |Chi| > 1):
    ///
    /// | arithmetic | Function | domain | # trials | peak | rms |
    /// | :---: | :--: | :--: | :--: | :--: | :--: | :--: |
    /// | IEEE | Chi | 0,88 | 30000 | 8.4e-16 | 1.4e-16 |
    ///
    /// # Arguments
    /// * `x`: A double precision number
    /// * `Shi`: A mutable double precision number that will return the
    ///          hyperbolic sine integral value
    /// * `Chi`: A mutable double precision number that will return the
    ///          hyperbolic cosine integral value
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::shichi;
    /// let x = 0.1f64;
    /// let mut shi = 0.0_f64;
    /// let mut chi = 0.0_f64;
    /// shichi(0.5_f64, &mut shi, &mut chi);
    /// ```
    pub fn shichi(x: f64, chi: &mut f64, shi: &mut f64){
        unsafe { unsafe_cephes_double::shichi(x, chi, shi) }
    }

    /// Function to compute the beta function.
    ///
    /// The beta function is given by:
    ///
    /// $B(a, b) = \int_0^1 t^{a - 1} ( 1 - t )^{b - 1} dt $
    ///
    /// which simplifies to
    ///
    /// $ B(a, b) = \frac{ \Gamma(a) \Gamma(b)}{ \Gamma(a + b) } $
    ///
    /// # Original Description from Stephen L. Moshier
    /// For large arguments the logarithm of the function is
    /// evaluated using lgam(), then exponentiated.
    ///
    /// ACCURACY (Relative error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :------: | :--: | :-: |
    /// | IEEE       | 0,30   | 30000    | 8.1e-14 | 1.1e-14 |
    ///
    /// ERROR MESSAGES:
    ///
    /// | message   | condition | value returned |
    /// | :-------: | :--------: | :----------: |
    /// | beta overflow | log(beta) > MAXLOG | 0.0 |
    /// | beta overflow | a or b <0 integer      | 0.0 |
    ///
    /// # Arguments
    /// * `a`: A double precision parameter, assumed to be greater than 0
    /// * `b`: A double precision parameter, assumed to be greater than 0
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::beta;
    /// let a = 0.1f64;
    /// let b = 0.2f64;
    /// beta(a, b);
    /// ```
    pub fn beta(a: f64, b: f64) -> f64 {
        unsafe { unsafe_cephes_double::beta(a, b) }
    }

    /// Function to compute the regularized incomplete beta function.
    ///
    /// The regularized incomplete beta function is given by:
    ///
    /// $I_x(a, b) = \frac{1}{B(a, b)} \int_0^x t^{a - 1} ( 1 - t )^{b - 1} dt $
    ///
    /// and for $x = 1$, the regularized incomplete beta function evaluates to
    /// exactly 1. Note, $ 1 - I_x(a, b) = I_{1 - x}(a, b)$.
    ///
    /// # Original Description from Stephen L. Moshier
    /// The integral is evaluated by a continued fraction expansion
    /// or, when b*x is small, by a power series.
    ///
    /// ACCURACY (Relative error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :------: | :--: | :-: |
    /// | IEEE       | 0,5    | 10000    | 6.9e-15 | 4.5e-16 |
    /// | IEEE | 0,85 | 250000 | 2.2e-13 | 1.7e-14 |
    /// | IEEE | 0,1000   |  30000  |  5.3e-12 | 6.3e-13 |
    /// | IEEE | 0,10000  | 250000  |  9.3e-11 | 7.1e-12 |
    /// | IEEE | 0,100000 |  10000  |  8.7e-10 | 4.8e-11 |
    ///
    /// ERROR MESSAGES:
    ///
    /// | message   | condition | value returned |
    /// | :-------: | :--------: | :----------: |
    /// | incbet domain | x<0, x>1 | 0.0 |
    /// | incbet underflow | - | 0.0 |
    ///
    /// # Arguments
    /// * `a`: A double precision parameter, assumed to be greater than 0
    /// * `b`: A double precision parameter, assumed to be greater than 0
    /// * `x`: A double precision parameter, assumed to be between 0 and 1
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::incbet;
    /// let a = 0.1f64;
    /// let b = 0.2f64;
    /// incbet(a, b, 0.5f64);
    /// ```
    pub fn incbet(a: f64, b: f64, x: f64) -> f64 {
        unsafe { unsafe_cephes_double::incbet(a, b, x) }
    }

    /// Computes the inverse of incomplete beta integral.
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// Given y, the function finds x such that
    ///
    ///  incbet( a, b, x ) = y .
    ///
    /// The routine performs interval halving or Newton iterations to find the
    /// root of incbet(a,b,x) - y = 0.
    ///
    ///
    /// ACCURACY (Relative error):
    ///
    /// | arithmetic | domain (x) | domain (a, b) | # trials | peak | rms |
    /// | :--------: | :--------: | :-----------: | :------: | :--: | :-: |
    /// |   IEEE     | 0,1        | .5,10000      | 50000    | 5.8e-12 | 1.3e-13 |
    /// |  IEEE      | 0,1        | .25,100       | 100000   | 1.8e-13 | 3.9e-15 |
    /// |  IEEE      | 0,1        | 0,5           | 50000    | 1.1e-12 | 5.5e-15 |
    ///
    /// With a and b constrained to half-integer or integer values:
    ///
    /// | arithmetic | domain (x) | domain (a, b) | # trials | peak | rms |
    /// | :--------: | :--------: | :-----------: | :------: | :--: | :-: |
    /// |  IEEE      |  0,1       | .5,10000      | 50000    | 5.8e-12 | 1.1e-13 |
    /// |  IEEE      |  0,1       | .5,100        | 100000   | 1.7e-14 | 7.9e-16 |
    ///
    /// With a = .5, b constrained to half-integer or integer values:
    ///
    /// | arithmetic | domain (x) | domain (a, b) | # trials | peak | rms |
    /// | :--------: | :--------: | :-----------: | :------: | :--: | :-: |
    /// |   IEEE     | 0,1        | .5,10000      | 10000    | 8.3e-11 | 1.0e-11 |
    ///
    /// # Arguments
    /// * `a`: A double precision parameter, assumed to be greater than 0
    /// * `b`: A double precision parameter, assumed to be greater than 0
    /// * `y`: A double precision parameter, assumed to be between 0 and 1
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::incbi;
    /// let a = 0.1f64;
    /// let b = 0.2f64;
    /// incbi(a, b, 0.5f64);
    /// ```
    pub fn incbi(a: f64, b: f64, y: f64) -> f64 {
        unsafe { unsafe_cephes_double::incbi(a, b, y) }
    }

    /// Computes the gamma function.
    ///
    /// The gamma function, $\Gamma(x)$, is defined as:
    ///
    /// $ \Gamma(t) = \int_0^{\infty} x^{t - 1} e^{-x} dx $.
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// Arguments |x| <= 34 are reduced by recurrence and the function
    /// approximated by a rational function of degree 6/7 in the
    /// interval (2,3).  Large arguments are handled by Stirling's
    /// formula. Large negative arguments are made positive using
    /// a reflection formula.
    ///
    /// ACCURACY (Relative error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :------: | :--: | :-: |
    /// |    IEEE  | -170,-33    | 20000 | 2.3e-15 | 3.3e-16 |
    /// |    IEEE  |  -33,  33   | 20000 | 9.4e-16 | 2.2e-16 |
    /// |    IEEE  |   33, 171.6 | 20000 | 2.3e-15 | 3.2e-16 |
    ///
    /// Error for arguments outside the test range will be larger
    /// owing to error amplification by the exponential function.
    ///
    /// # Arguments
    /// * `x`: A double precision parameter
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::gamma;
    /// gamma(1.0f64);
    /// ```
    pub fn gamma(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::gamma(x) }
    }

    /// Computes the reciprocal gamma function.
    ///
    /// The reciprocal gamma function is given by:
    ///
    /// $ \frac{1}{\Gamma(x)} $
    ///
    /// where the gamma function, $\Gamma(x)$, is defined as:
    ///
    /// $ \Gamma(t) = \int_0^{\infty} x^{t - 1} e^{-x} dx $.
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// The function is approximated by a Chebyshev expansion in
    /// the interval [0,1].  Range reduction is by recurrence
    /// for arguments between -34.034 and +34.84425627277176174.
    /// 1/MAXNUM is returned for positive arguments outside this
    /// range.  For arguments less than -34.034 the cosecant
    /// reflection formula is applied; lograrithms are employed
    /// to avoid unnecessary overflow.
    ///
    /// The reciprocal gamma function has no singularities,
    /// but overflow and underflow may occur for large arguments.
    /// These conditions return either MAXNUM or 1/MAXNUM with
    /// appropriate sign.
    ///
    /// ACCURACY (Relative error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :------: | :--: | :-: |
    ///    IEEE     -30,+30      30000       1.1e-15     2.0e-16
    ///
    /// For arguments less than -34.034 the peak error is on the
    /// order of 5e-15 (DEC), excepting overflow or underflow.
    /// Arguments |x| <= 34 are reduced by recurrence and the function
    /// approximated by a rational function of degree 6/7 in the
    /// interval (2,3).  Large arguments are handled by Stirling's
    /// formula. Large negative arguments are made positive using
    /// a reflection formula.
    ///
    /// # Arguments
    /// * `x`: A double precision parameter
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::rgamma;
    /// rgamma(1.0f64);
    /// ```
    pub fn rgamma(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::rgamma(x) }
    }

    /// Computes the natural logarithm of gamma function.
    ///
    /// The log of the gamma function is given by:
    ///
    /// $ \ln\left( \Gamma(x) \right) $
    ///
    /// where the gamma function, $\Gamma(x)$, is defined as:
    ///
    /// $ \Gamma(t) = \int_0^{\infty} x^{t - 1} e^{-x} dx $.
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// For arguments greater than 13, the logarithm of the gamma
    /// function is approximated by the logarithmic version of
    /// Stirling's formula using a polynomial approximation of
    /// degree 4. Arguments between -33 and +33 are reduced by
    /// recurrence to the interval [2,3] of a rational approximation.
    /// The cosecant reflection formula is employed for arguments
    /// less than -33.
    ///
    /// Arguments greater than MAXLGM return MAXNUM and an error
    /// message.  MAXLGM = 2.035093e36 for DEC
    /// arithmetic or 2.556348e305 for IEEE arithmetic.
    ///
    /// ACCURACY:
    ///
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :------: | :--: | :-: |
    /// |  IEEE  | 0, 3             |   28000  |  5.4e-16  |  1.1e-16 |
    /// |  IEEE  | 2.718, 2.556e305 |   40000  |  3.5e-16  |  8.3e-17 |
    ///
    /// The error criterion was relative when the function magnitude
    /// was greater than one but absolute when it was less than one.
    ///
    /// The following test used the relative error criterion, though
    /// at certain points the relative error could be much higher than
    /// indicated.
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :------: | :--: | :-: |
    /// | IEEE |   -200, -4   |   10000  | 4.8e-16  |   1.3e-16 |
    ///
    /// # Arguments
    /// * `x`: A double precision parameter
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::lgam;
    /// lgam(1.0f64);
    /// ```
    pub fn lgam(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::lgam(x) }
    }

    /// Computes the regularized incomplete gamma integral.
    ///
    /// The regularized incomplete gamma function is given by:
    ///
    /// $ \gamma(a, x) = \frac{1}{\Gamma{a}} \int_0^{x} t^{a - 1} e^{-t} dt $
    ///
    /// where the gamma function, $\Gamma(a)$, is defined as:
    ///
    /// $ \Gamma(a) = \int_0^{\infty} x^{a - 1} e^{-x} dx $.
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// In this implementation both arguments must be positive.
    /// The integral is evaluated by either a power series or
    /// continued fraction expansion, depending on the relative
    /// values of a and x.
    ///
    /// ACCURACY (Relative error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--------: | :----: | :------: | :--: | :-: |
    /// |  IEEE      |  0,30  |  200000  | 3.6e-14 | 2.9e-15 |
    /// |  IEEE      |  0,100 |  300000  | 9.9e-14 | 1.5e-14 |
    ///
    /// # Arguments
    /// * `a`: a double precision parameter, assumed to be > 0
    /// * `x`: A double precision parameter, assumed to be > 0
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::igam;
    /// igam(0.5f64, 1.0f64);
    /// ```
    pub fn igam(a: f64, x: f64) -> f64 {
        unsafe { unsafe_cephes_double::igam(a, x) }
    }

    /// Computes the complemented regularized incomplete gamma integral.
    ///
    /// The regularized incomplete gamma function is given by:
    ///
    /// $ \Gamma(a, x) = \frac{1}{\Gamma{a}} \int_x^{\infty} t^{a - 1} e^{-t} dt $
    ///
    /// which reduces to:
    ///
    /// $ \Gamma(a, x) = 1 - \frac{1}{\Gamma{a}} \int_0^{x} t^{a - 1} e^{-t} dt $
    ///
    /// where the gamma function, $\Gamma(a)$, is defined as:
    ///
    /// $ \Gamma(a) = \int_0^{\infty} x^{a - 1} e^{-x} dx $.
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// In this implementation both arguments must be positive.
    /// The integral is evaluated by either a power series or
    /// continued fraction expansion, depending on the relative
    /// values of a and x.
    ///
    /// ACCURACY (Relative error, tested at random a, x):
    ///
    /// | arithmetic | domain (a) | domain (x) | # trials | peak | rms |
    /// | :--------: | :----: | :----------: | :------: | :--: | :-: |
    /// |  IEEE      | 0.5,100 |  0,100  |   200000 | 1.9e-14 | 1.7e-15 |
    /// |  IEEE      | 0.01,0.5 | 0,100  |   200000 | 1.4e-13 | 1.6e-15 |
    ///
    /// # Arguments
    /// * `a`: a double precision parameter, assumed to be > 0
    /// * `x`: A double precision parameter, assumed to be > 0
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::igamc;
    /// igamc(0.5f64, 1.0f64);
    /// ```
    pub fn igamc(a: f64, x: f64) -> f64 {
        unsafe { unsafe_cephes_double::igamc(a, x) }
    }

    /// Computes the inverse of complemented incomplete gamma integral.
    ///
    /// This function finds the value `x` for a given p where
    /// `p = igamc(a, p)`, where `igamc` is the regularized incomplete gamma
    /// function, which is given by
    ///
    /// $ \Gamma(a, x) = \frac{1}{\Gamma{a}} \int_x^{\infty} t^{a - 1} e^{-t} dt $
    ///
    /// where the gamma function, $\Gamma(a)$, is defined as:
    ///
    /// $ \Gamma(a) = \int_0^{\infty} x^{a - 1} e^{-x} dx $.
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// It is valid in the right-hand tail of the distribution, p < 0.5.
    /// Starting with the approximate value
    ///
    /// $ x = a t^3 $
    ///
    ///  where
    ///
    ///  $t = 1 - d - ndtri(p) \sqrt{d}$
    ///
    /// and
    ///
    ///  $d = 1/9a$,
    ///
    /// the routine performs up to 10 Newton iterations to find the
    /// root of igamc(a,x) - p = 0.
    ///
    /// ACCURACY (Relative Error):
    ///
    /// Tested at random a, p in the intervals indicated.
    ///
    /// | arithmetic | domain (a) | domain (x) | # trials | peak | rms |
    /// | :--------: | :----: | :----------: | :------: | :--: | :-: |
    /// |  IEEE      | 0.5,100 |  0,100  |   200000 | 1.0e-14 | 1.7e-15 |
    /// |  IEEE      | 0.01,0.5 | 0,100  |   200000 | 9.0e-14 | 3.4e-15 |
    /// |  IEEE      | 0.5,10000 | 0,0.5  |  20000  | 2.3e-13 | 3.8e-14 |
    ///
    /// # Arguments
    /// * `a`: a double precision parameter, assumed to be > 0
    /// * `p`: A double precision parameter, assumed to be > 0
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::igami;
    /// igami(0.5f64, 1.0f64);
    /// ```
    pub fn igami(a: f64, p: f64) -> f64 {
        unsafe { unsafe_cephes_double::igami(a, p) }
    }

    /// Computes the psi (digamma) function.
    ///
    /// The digamma function is simply the derivative of the natural log of the
    /// gamma function:
    ///
    /// $\psi(x) = \frac{d \ln\left(\Gamma(x)\right\right)}{dx}$
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// For integer x,
    ///
    /// $ \psi(x) = -\gamma + \sum_{k=1}^{n-1}\frac{1}{k} $
    ///
    /// This formula is used for $0 < x \le 10$.  If x is negative, it
    /// is transformed to a positive argument by the reflection
    /// formula  $\psi(1-x) = \psi(x) + \pi \cot(\pi x)$.
    ///
    /// For general positive x, the argument is made greater than 10
    /// using the recurrence  $\psi(x+1) = \psi(x) + 1/x$.
    /// Then the following asymptotic expansion is applied:
    ///
    /// $\psi(x) = \ln(x) - \frac{1}{2x} \sum^{\infty}_{k=1} \frac{B_{2k}}{2k x^{2k}} $
    ///
    /// where the $B_{2k}$ are Bernoulli numbers.
    ///
    /// ACCURACY (Relative error (except absolute when |psi| < 1)):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :----: | :---:     | :---:    | :---: | :--: |
    /// |   IEEE |  0,30  | 30000 | 1.3e-15 | 1.4e-16 |
    /// |   IEEE |  -30,0 | 40000 | 1.5e-15 | 2.2e-16 |
    ///
    /// ERROR MESSAGES:
    ///
    /// | message | condition | value returned |
    /// | :------: | :-----: | :-----: |
    /// | psi singularity | x integer <=0 | MAXNUM|
    ///
    /// # Arguments
    /// * `x`: a double precision parameter
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::psi;
    /// psi(4.0f64);
    /// ```
    pub fn psi(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::psi(x) }
    }

    /// Computes the factorial function.
    ///
    /// The factorial function returns the factorial of i, the input argument
    /// where the factorial is defined as:
    ///
    /// $ i! = \Pi_k^i k $
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// Due to machine arithmetic bounds the largest value of
    /// i 170 in IEEE arithmetic.  Greater values, or negative ones,
    /// produce an error message and return MAXNUM.
    ///
    /// ACCURACY (Relative Error):
    ///
    /// For i < 34 the values are simply tabulated, and have
    /// full machine accuracy.  If i > 55, fac(i) = gamma(i+1);
    /// see gam for more information.
    ///
    /// | arithmetic  | domain    |  peak|
    /// | :---: | :---: | :---: |
    /// | IEEE  | 0, 170 | 1.4e-15 |
    ///
    /// # Arguments
    /// * `i`: an integer parameter
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::fac;
    /// fac(4i32);
    /// ```
    pub fn fac(i: i32) -> f64 {
        unsafe { unsafe_cephes_double::fac(i) }
    }

    /// Computes the Bessel function of the first kind of order zero.
    ///
    /// The Bessel of the first kind of order zero has many possible
    /// representations but a convenient integral representation is the
    /// following:
    ///
    /// $ \J_0(z) = \frac{1}{\pi} \int_0^{\pi} \cos(z \sin(\theta)) d \theta $
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// The domain is divided into the intervals [0, 5] and
    /// (5, infinity). In the first interval the following rational
    /// approximation is used:
    ///
    /// $ \frac{\left(w - r_1^2\right) \left( w - r^2_2 \right) P_3(w)}{Q_8(w)}$
    ///
    /// where $w = x^2$  and the two r's are zeros of the function.
    ///
    /// In the second interval, the Hankel asymptotic expansion
    /// is employed with two rational functions of degree 6/6
    /// and 7/7.
    ///
    /// ACCURACY (Absolute error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--:       | :---:  | :---:    | :--:| :--: |
    /// | IEEE | 0, 30 | 60000 | 4.2e-16 | 1.1e-16 |
    ///
    /// # Arguments
    /// * `x`: a double precision parameter
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::j0;
    /// j0(1.0f64);
    /// ```
    pub fn j0(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::j0(x) }
    }

    /// Computes the Bessel function of the first kind of order one.
    ///
    /// The Bessel of the first kind of order one has many possible
    /// representations but a convenient integral representation is the
    /// following:
    ///
    /// $ \J_1(z) = \frac{1}{\pi} \int_0^{\pi} \cos(z \sin(\theta) - \theta) d \theta $
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// The domain is divided into the intervals [0, 8] and
    /// (8, infinity). In the first interval a 24 term Chebyshev
    /// expansion is used. In the second, the asymptotic
    /// trigonometric representation is employed using two
    /// rational functions of degree 5/5.
    ///
    /// ACCURACY (Absolute error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--:       | :---:  | :---:    | :--:| :--: |
    /// | IEEE | 0, 30 | 30000 | 2.6e-16 | 1.1e-16 |
    ///
    /// # Arguments
    /// * `x`: a double precision parameter
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::j1;
    /// j1(1.0f64);
    /// ```
    pub fn j1(x: f64) -> f64 {
        unsafe { unsafe_cephes_double::j1(x) }
    }

    /// Computes the Bessel function of the first kind of nth integer order.
    ///
    /// The Bessel of the first kind of nth order has many possible
    /// representations but a convenient integral representation is the
    /// following:
    ///
    /// $ \J_n(z) = \frac{1}{\pi} \int_0^{\pi} \cos(z \sin(\theta) - n \theta) d \theta $
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// Returns Bessel function of order n, where n is a
    /// (possibly negative) integer.
    ///
    /// The ratio of $N_n(x)$ to $J_0(x)$ is computed by backward
    /// recurrence.  First the ratio $J_n/J_{n-1}$ is found by a
    /// continued fraction expansion.  Then the recurrence
    /// relating successive orders is applied until $j_0$ or $j_1$ is
    /// reached.
    ///
    /// If n = 0 or 1 the routine for j0 or j1 is called
    /// directly.
    ///
    /// ACCURACY (Absolute error):
    ///
    /// | arithmetic | domain | # trials | peak | rms |
    /// | :--:       | :---:  | :---:    | :--:| :--: |
    /// | IEEE | 0, 30 | 5000 | 4.4e-16 | 7.9e-17 |
    ///
    /// Not suitable for large n or x. Use jv() instead.
    ///
    /// # Arguments
    /// * `n`: an integer parameter
    /// * `x`: a double precision parameter
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::jn;
    /// jn(2i32, 1.0f64);
    /// ```
    pub fn jn(n: i32, x: f64) -> f64 {
        unsafe { unsafe_cephes_double::jn(n, x) }
    }

    /// Computes the Bessel function of the first kind of vth real order.
    ///
    /// The Bessel of the first kind of vth order has many possible
    /// representations but a series representation is the following:
    ///
    /// $ J_v(x) = \sum_{m=0}^{\infty} \frac{(-1)^m}{ m! \Gamma(m + v + 1)} \left( \frac{x}{2} \right)^{2m + v}$
    ///
    /// where $\Gamma$ is the gamma function.
    ///
    /// # Original Description from Stephen L. Moshier
    ///
    /// Returns Bessel function of order v of the argument,
    /// where v is real.  Negative x is allowed if v is an integer.
    ///
    /// Several expansions are included: the ascending power
    /// series, the Hankel expansion, and two transitional
    /// expansions for large v.  If v is not too large, it
    /// is reduced by recurrence to a region of best accuracy.
    /// The transitional expansions give 12D accuracy for v > 500.
    ///
    /// ACCURACY:
    /// Results for integer v are indicated by *, where x and v
    /// both vary from -125 to +125.  Otherwise,
    /// x ranges from 0 to 125, v ranges as indicated by "domain."
    /// Error criterion is absolute, except relative when |jv()| > 1.
    ///
    /// | arithmetic | v domain | x domain | # trials | peak | rms |
    /// | :------:   | :-----:  | :----:   | :----:   | :---: | :--: |
    /// | IEEE       | 0,125    | 0,125    | 100000   | 4.6e-15 | 2.2e-16 |
    /// | IEEE | -125,0 | 0,125 | 40000 | 5.4e-11 | 3.7e-13 |
    /// | IEEE | 0,500 | 0,500 | 20000 | 4.4e-15 | 4.0e-16 |
    ///
    /// Integer v:
    ///
    /// | arithmetic | v domain | x domain | # trials | peak | rms |
    /// | :------:   | :-----:  | :----:   | :----:   | :---: | :--: |
    /// | IEEE | -125,125 | -125,125 | 50000 | 3.5e-15* | 1.9e-16* |
    ///
    /// # Arguments
    /// * `v`: a double precision parameter
    /// * `x`: a double precision parameter
    ///
    /// # Example
    /// ```
    /// use special_fun::cephes_double::jv;
    /// jv(2.0f64, 1.0f64);
    /// ```
    pub fn jv(n: f64, x: f64) -> f64 {
        unsafe { unsafe_cephes_double::jv(n, x) }
    }

    /*
        /// Bessel function of the second kind, order zero.
        pub fn y0(x: f64) -> f64;
        /// Bessel function of the second kind, order one.
        pub fn y1(x: f64) -> f64;
        /// Bessel function of the second kind, integer order.
        pub fn yn(n: i32, x: f64) -> f64;
        /// Bessel function of the second kind, real order.
        pub fn yv(v: f64, x: f64) -> f64;

        /// Modified Bessel function of order zero.
        pub fn i0(x: f64) -> f64;
        /// Modified Bessel function of order zero, exponentially scaled.
        pub fn i0e(x: f64) -> f64;
        /// Modified Bessel function of order one.
        pub fn i1(x: f64) -> f64;
        /// Modified Bessel function of order one, exponentially scaled.
        pub fn i1e(x: f64) -> f64;
        /// Modified Bessel function of real order.
        pub fn iv(v: f64, x: f64) -> f64;

        /// Modified Bessel function of the third kind, order zero.
        pub fn k0(x: f64) -> f64;
        /// Modified Bessel function of the third kind, order zero,
        /// exponentially scaled.
        pub fn k0e(x: f64) -> f64;
        /// Modified Bessel function of the third kind, order one.
        pub fn k1(x: f64) -> f64;
        /// Modified Bessel function of the third kind, order one,
        /// exponentially scaled.
        pub fn k1e(x: f64) -> f64;
        /// Modified Bessel function of the third kind, integer order.
        pub fn kn(n: i32, x: f64) -> f64;

        // Elliptic functions
        /// Incomplete elliptic integral of the first kind.
        pub fn ellik(phi: f64, m: f64) -> f64;
        /// Incomplete elliptic integral of the second kind.
        pub fn ellie(phi: f64, m: f64) -> f64;
        /// Complete elliptic integral of the first kind.
        pub fn ellpk(m1: f64) -> f64;
        /// Complete elliptic integral of the second kind.
        pub fn ellpe(m1: f64) -> f64;
        /// Jacobian elliptic function.
        pub fn ellpj(u: f64, m: f64, sn: &mut f64, cn: &mut f64, dn: &mut f64, phi: &mut f64) -> i32;

        // Hypergeometric functions
        /// Confluent hypergeometric function 1F1.
        pub fn hyperg(a: f64, b: f64, x: f64) -> f64;
        /// Hypergeometric function 1F2.
        pub fn onef2(a: f64, b: f64, c: f64, x: f64, err: &mut f64) -> f64;
        /// Gauss hypergeometric function 2F1.
        pub fn hyp2f1(a: f64, b: f64, c: f64, x: f64) -> f64;
        /// Hypergeometric function 3F0.
        pub fn threef0(a: f64, b: f64, c: f64, x: f64, err: &mut f64) -> f64;

        // Distributions
        /// Binomial distribution.
        pub fn bdtr(k: i32, n: i32, p: f64) -> f64;
        /// Complemented binomial distribution.
        pub fn bdtrc(k: i32, n: i32, p: f64) -> f64;
        /// Inverse of binomial distribution.
        pub fn bdtri(k: i32, n: i32, y: f64) -> f64;

        /// Negative binomial distribution.
        pub fn nbdtr(k: i32, n: i32, p: f64) -> f64;
        /// Complemented negative binomial distribution.
        pub fn nbdtrc(k: i32, n: i32, p: f64) -> f64;
        /// Inverse of negative binomial distribution.
        pub fn nbdtri(k: i32, n: i32, p: f64) -> f64;

        /// Beta distribution.
        pub fn btdtr(a: f64, b: f64, x: f64) -> f64;

        /// Chi-square distribution.
        pub fn chdtr(df: f64, x: f64) -> f64;
        /// Complemented chi-square distribution.
        pub fn chdtrc(v: f64, x: f64) -> f64;
        /// Inverse of complemented chi-square distribution.
        pub fn chdtri(df: f64, y: f64) -> f64;

        /// F distribution.
        pub fn fdtr(df1: i32, df2: i32, x: f64) -> f64;
        /// Complemented F distribution.
        pub fn fdtrc(df1: i32, df2: i32, x: f64) -> f64;
        /// Inverse of complemented F distribution.
        pub fn fdtri(df1: i32, df2: i32, p: f64) -> f64;

        /// Gamma distribution.
        pub fn gdtr(a: f64, b: f64, x: f64) -> f64;
        /// Complemented gamma distribution.
        pub fn gdtrc(a: f64, b: f64, x: f64) -> f64;

        /// Normal distribution.
        pub fn ndtr(x: f64) -> f64;
        /// Inverse of normal distribution.
        pub fn ndtri(y: f64) -> f64;

        /// Poisson distribution.
        pub fn pdtr(k: i32, m: f64) -> f64;
        /// Complemented Poisson distribution.
        pub fn pdtrc(k: i32, m: f64) -> f64;
        /// Inverse of Poisson distribution.
        pub fn pdtri(k: i32, y: f64) -> f64;

        /// Student's t distribution.
        pub fn stdtr(k: i16, t: f64) -> f64;
        /// Inverse of Student's t distribution.
        pub fn stdtri(k: i32, p: f64) -> f64;

        // Misc special functions
        /// Airy function.
        pub fn airy(x: f64, ai: &mut f64, aip: &mut f64, bi: &mut f64, bip: &mut f64) -> i32;
        /// Dawson's integral.
        pub fn dawsn(x: f64) -> f64;
        /// Fresnel integral.
        pub fn fresnl(x: f64, s: &mut f64, c: &mut f64);
        /// Integral of Planck's black body radiation formula.
        pub fn plancki(lambda: f64, temperature: f64) -> f64;
        /// Struve function.
        pub fn struve(v: f64, x: f64) -> f64;
        /// Riemann zeta function.
        pub fn zetac(x: f64) -> f64;
        /// Riemann zeta function of two arguments.
        pub fn zeta(x: f64, q: f64) -> f64;
    */
}

// single precision
#[allow(dead_code)]
extern "C" {
    // Floating point numeric utilities
    /// Round to nearest or event integer valued f32.
    fn roundf(x: f32) -> f32;
    /// Largest integer less than or equal to x.
    fn floorf(x: f32) -> f32;
    /// Smallest integer greater than or equal to x.
    fn ceilf(x: f32) -> f32;
    /// Return the significand between 0.5 and 1. Write exponent to expnt.
    /// x = y * 2**expn
    fn frexpf(x: f32, expnt: &mut i32) -> f32;
    /// Multiply x by 2**n.
    fn ldexpf(x: f32, n: i32) -> f32;
    /// Absolute value.
    fn fabsf(x: f32) -> f32;
    /// Return 1 if the sign bit of x is 1, else 0.
    fn signbitf(x: f32) -> i32;
    /// Return 1 if x is NaN, else 0.
    fn isnanf(x: f32) -> i32;
    /// Return 1 if x is finite, else 0.
    fn isfinitef(x: f32) -> i32;

    // Roots
    /// Cube root.
    fn cbrtf(x: f32) -> f32;
    /// Square root.
    fn sqrtf(x: f32) -> f32;
    /// Integer square root.
    fn lsqrtf(x: i64) -> i64;

    // Exponential functions
    /// Exponential function.
    fn expf(x: f32) -> f32;
    /// Base 10 exponential function.
    fn exp10f(x: f32) -> f32;
    /// Base 2 exponential function.
    fn exp2f(x: f32) -> f32;
    /// Compute accurately exponential of squared argument.
    fn expm1f(x: f32) -> f32;
    /// Compute accurately exp(x) - 1 for x close to 0.
    fn expx2f(x: f32, sign: i32) -> f32;
    /// Exponential integral.
    fn eif(x: f32) -> f32;
    /// Error function.
    fn erff(x: f32) -> f32;
    /// Complementary error function.
    fn erfcf(x: f32) -> f32;
    /// Power function.
    fn powf(x: f32, y: f32) -> f32;
    /// Integer power function.
    fn powif(x: f32, n: i32) -> f32;

    // Logarithmic functions
    /// Natural logarithm.
    fn logf(x: f32) -> f32;
    /// Common logarithm.
    fn log10f(x: f32) -> f32;
    /// Base 2 logarithm.
    fn log2f(x: f32) -> f32;
    /// Compute accurately log(1 + x) for x close to 0.
    fn log1pf(x: f32) -> f32;
    /// Dilogarithm (Spence's function).
    fn spencef(x: f32) -> f32;

    // Trigonometric functions
    /// Circular sine.
    fn sinf(x: f32) -> f32;
    /// Circular cosine.
    fn cosf(x: f32) -> f32;
    /// Circular tangent.
    fn tanf(x: f32) -> f32;
    /// Inverse circular sine.
    fn asinf(x: f32) -> f32;
    /// Inverse circular cosine.
    fn acosf(x: f32) -> f32;
    /// Inverse circular tangent.
    fn atanf(x: f32) -> f32;
    /// Quadrant-correct inverse circular tangent.
    fn atan2f(y: f32, x: f32) -> f32;
    /// Compute accurately cos(x) - 1 for x close to 0.
    fn cosm1f(x: f32) -> f32;
    /// Sine and cosine integrals.
    fn sicif(x: f32, si: &mut f32, ci: &mut f32) -> f32;

    // Hyperbolic functions
    /// Hyperbolic sine.
    fn sinhf(x: f32) -> f32;
    /// Hyperbolic cosine.
    fn coshf(x: f32) -> f32;
    /// Hyperbolic tangent.
    fn tanhf(x: f32) -> f32;
    /// Inverse hyperbolic sine.
    fn asinhf(x: f32) -> f32;
    /// Inverse hyperbolic cosine.
    fn acoshf(x: f32) -> f32;
    /// Inverse hyperbolic tangent.
    fn atanhf(x: f32) -> f32;
    /// Hyperbolic sine and cosine integrals.
    fn shichif(x: f32, chi: &mut f32, shi: &mut f32);

    // Beta functions
    /// Beta function.
    fn betaf(a: f32, b: f32) -> f32;
    /// Regularized incomplete beta function.
    fn incbetf(a: f32, b: f32, x: f32) -> f32;
    /// Inverse of incomplete beta integral.
    fn incbif(a: f32, b: f32, y: f32) -> f32;

    // Gamma functions
    /// Gamma function.
    fn gammaf(x: f32) -> f32;
    /// Reciprocal gamma function.
    fn rgammaf(x: f32) -> f32;
    /// Natural logarithm of gamma function.
    fn lgamf(x: f32) -> f32;
    /// Regularized incomplete gamma integral.
    fn igamf(a: f32, x: f32) -> f32;
    /// Complemented incomplete gamma integral.
    fn igamcf(a: f32, x: f32) -> f32;
    /// Inverse of complemented incomplete gamma integral.
    fn igamif(a: f32, p: f32) -> f32;
    /// Psi (digamma) function.
    fn psif(x: f32) -> f32;
    /// Factorial function.
    fn facf(i: i32) -> f32;

    // Bessel functions
    /// Bessel function of order zero.
    fn j0f(x: f32) -> f32;
    /// Bessel function of order one.
    fn j1f(x: f32) -> f32;
    /// Bessel function of integer order.
    fn jnf(n: i32, x: f32) -> f32;
    /// Bessel function of real order.
    fn jvf(n: f32, x: f32) -> f32;

    /// Bessel function of the second kind, order zero.
    fn y0f(x: f32) -> f32;
    /// Bessel function of the second kind, order one.
    fn y1f(x: f32) -> f32;
    /// Bessel function of the second kind, integer order.
    fn ynf(n: i32, x: f32) -> f32;
    /// Bessel function of the second kind, real order.
    fn yvf(v: f32, x: f32) -> f32;

    /// Modified Bessel function of order zero.
    fn i0f(x: f32) -> f32;
    /// Modified Bessel function of order zero, exponentially scaled.
    fn i0ef(x: f32) -> f32;
    /// Modified Bessel function of order one.
    fn i1f(x: f32) -> f32;
    /// Modified Bessel function of order one, exponentially scaled.
    fn i1ef(x: f32) -> f32;
    /// Modified Bessel function of real order.
    fn ivf(v: f32, x: f32) -> f32;

    /// Modified Bessel function of the third kind, order zero.
    fn k0f(x: f32) -> f32;
    /// Modified Bessel function of the third kind, order zero,
    /// exponentially scaled.
    fn k0ef(x: f32) -> f32;
    /// Modified Bessel function of the third kind, order one.
    fn k1f(x: f32) -> f32;
    /// Modified Bessel function of the third kind, order one,
    /// exponentially scaled.
    fn k1ef(x: f32) -> f32;
    /// Modified Bessel function of the third kind, integer order.
    fn knf(n: i32, x: f32) -> f32;

    // Elliptic functions
    /// Incomplete elliptic integral of the first kind.
    fn ellikf(phi: f32, m: f32) -> f32;
    /// Incomplete elliptic integral of the second kind.
    fn ellief(phi: f32, m: f32) -> f32;
    /// Complete elliptic integral of the first kind.
    fn ellpkf(m1: f32) -> f32;
    /// Complete elliptic integral of the second kind.
    fn ellpef(m1: f32) -> f32;
    /// Jacobian elliptic function.
    fn ellpjf(u: f32, m: f32, sn: &mut f32, cn: &mut f32, dn: &mut f32, phi: &mut f32) -> i32;

    // Hypergeometric functions
    /// Confluent hypergeometric function 1F1.
    fn hypergf(a: f32, b: f32, x: f32) -> f32;
    /// Hypergeometric function 1F2.
    fn onef2f(a: f32, b: f32, c: f32, x: f32, err: &mut f32) -> f32;
    /// Gauss hypergeometric function 2F1.
    fn hyp2f1f(a: f32, b: f32, c: f32, x: f32) -> f32;
    /// Hypergeometric function 3F0.
    fn threef0f(a: f32, b: f32, c: f32, x: f32, err: &mut f32) -> f32;

    // Distributions
    /// Binomial distribution.
    fn bdtrf(k: i32, n: i32, p: f32) -> f32;
    /// Complemented binomial distribution.
    fn bdtrcf(k: i32, n: i32, p: f32) -> f32;
    /// Inverse of binomial distribution.
    fn bdtrif(k: i32, n: i32, y: f32) -> f32;

    /// Negative binomial distribution.
    fn nbdtrf(k: i32, n: i32, p: f32) -> f32;
    /// Complemented negative binomial distribution.
    fn nbdtrcf(k: i32, n: i32, p: f32) -> f32;
    /// Inverse of negative binomial distribution.
    fn nbdtrif(k: i32, n: i32, p: f32) -> f32;

    /// Beta distribution.
    fn btdtrf(a: f32, b: f32, x: f32) -> f32;

    /// Chi-square distribution.
    fn chdtrf(df: f32, x: f32) -> f32;
    /// Complemented chi-square distribution.
    fn chdtrcf(v: f32, x: f32) -> f32;
    /// Inverse of complemented chi-square distribution.
    fn chdtrif(df: f32, y: f32) -> f32;

    /// F distribution.
    fn fdtrf(df1: i32, df2: i32, x: f32) -> f32;
    /// Complemented F distribution.
    fn fdtrcf(df1: i32, df2: i32, x: f32) -> f32;
    /// Inverse of complemented F distribution.
    fn fdtrif(df1: i32, df2: i32, p: f32) -> f32;

    /// Gamma distribution.
    fn gdtrf(a: f32, b: f32, x: f32) -> f32;
    /// Complemented gamma distribution.
    fn gdtrcf(a: f32, b: f32, x: f32) -> f32;

    /// Normal distribution.
    fn ndtrf(x: f32) -> f32;
    /// Inverse of normal distribution.
    fn ndtrif(y: f32) -> f32;

    /// Poisson distribution.
    fn pdtrf(k: i32, m: f32) -> f32;
    /// Complemented Poisson distribution.
    fn pdtrcf(k: i32, m: f32) -> f32;
    /// Inverse of Poisson distribution.
    fn pdtrif(k: i32, y: f32) -> f32;

    /// Student's t distribution.
    fn stdtrf(k: i16, t: f32) -> f32;
    /// Inverse of Student's t distribution.
    fn stdtrif(k: i32, p: f32) -> f32;

    // Misc special functions
    /// Airy function.
    fn airyf(x: f32, ai: &mut f32, aip: &mut f32, bi: &mut f32, bip: &mut f32) -> i32;
    /// Dawson's integral.
    fn dawsnf(x: f32) -> f32;
    /// Fresnel integral.
    fn fresnlf(x: f32, s: &mut f32, c: &mut f32);
    /// Integral of Planck's black body radiation formula.
    fn planckif(lambda: f32, temperature: f32) -> f32;
    /// Struve function.
    fn struvef(v: f32, x: f32) -> f32;
    /// Riemann zeta function.
    fn zetacf(x: f32) -> f32;
    /// Riemann zeta function of two arguments.
    fn zetaf(x: f32, q: f32) -> f32;
}

/// Special functions on primitive floating point numbers.
pub trait FloatSpecial: Copy + Add<Output=Self> + Sub<Output=Self> {
    /// Beta function.
    fn beta(self, b: Self) -> Self;
    /// Logarithm of beta function.
    fn logbeta(self, b: Self) -> Self {
        self.loggamma() + b.loggamma() - (self + b).loggamma()
    }
    /// Regularized incomplete beta function.
    fn betainc(self, a: Self, b: Self) -> Self;
    /// Inverse of incomplete beta integral.
    fn betainc_inv(self, a: Self, b: Self) -> Self;

    /// Factorial.
    fn factorial(self) -> Self;
    /// Gamma function.
    fn gamma(self) -> Self;
    /// Reciprocal gamma function.
    fn rgamma(self) -> Self;
    /// Logarithm of gamma function.
    fn loggamma(self) -> Self;

    /// Regularized incomplete gamma integral.
    fn gammainc(self, a: Self) -> Self;
    /// Complemented incomplete gamma integral.
    fn gammac(self, a: Self) -> Self;
    /// Inverse of complemented incomplete gamma integral.
    fn gammac_inv(self, a: Self) -> Self;

    /// Digamma function.
    fn digamma(self) -> Self;

    /// Error function.
    fn erf(self) -> Self;
    /// Complementary error function.
    fn erfc(self) -> Self;

    /// Confluent hypergeometric function 1F1.
    fn hyp1f1(self, a: Self, b: Self) -> Self;
    /// Hypergeometric function 1F2.
    fn hyp1f2(self, a: Self, b: Self, c: Self) -> Self;
    /// Gauss hypergeometric function 2F1.
    fn hyp2f1(self, a: Self, b: Self, c: Self) -> Self;
    /// Hypergeometric function 3F0.
    fn hyp3f0(self, a: Self, b: Self, c: Self) -> Self;

    /// Normal distribution function.
    fn norm(self) -> Self;
    /// Inverse of Normal distribution function.
    fn norm_inv(self) -> Self;

    /// Bessel function of real order of the first kind.
    fn besselj(self, v: Self) -> Self;
    /// Bessel function of real order of the second kind.
    fn bessely(self, v: Self) -> Self;
    /// Modified bessel function of real order of the first kind.
    fn besseli(self, v: Self) -> Self;
    /// Modified bessel function of integer order of the second kind.
    fn besselk(self, v: i32) -> Self;

    /// Riemann zeta function.
    fn riemann_zeta(self) -> Self;
    /// Hurwitz zeta function.
    fn hurwitz_zeta(self, q: Self) -> Self;
}

impl FloatSpecial for f64 {
    fn beta(self, b: f64) -> f64 {
        unsafe { unsafe_cephes_double::beta(self, b) }
    }
    fn betainc(self, a: f64, b: f64) -> f64 {
        unsafe { unsafe_cephes_double::incbet(a, b, self) }
    }
    fn betainc_inv(self, a: f64, b: f64) -> f64 {
        unsafe { unsafe_cephes_double::incbi(a, b, self) }
    }

    fn factorial(self) -> f64 {
        unsafe { unsafe_cephes_double::gamma(self + 1.0) }
    }
    fn gamma(self) -> f64 {
        unsafe { unsafe_cephes_double::gamma(self) }
    }
    fn rgamma(self) -> f64 {
        unsafe { unsafe_cephes_double::rgamma(self) }
    }
    fn loggamma(self) -> f64 {
        unsafe { unsafe_cephes_double::lgam(self) }
    }

    fn gammainc(self, a: f64) -> f64 {
        unsafe { unsafe_cephes_double::igam(a, self) }
    }
    fn gammac(self, a: f64) -> f64 {
        unsafe { unsafe_cephes_double::igamc(a, self) }
    }
    fn gammac_inv(self, a: f64) -> f64 {
        unsafe { unsafe_cephes_double::igami(a, self) }
    }

    fn digamma(self) -> f64 {
        unsafe { unsafe_cephes_double::psi(self) }
    }

    fn erf(self) -> f64 {
        unsafe { unsafe_cephes_double::erf(self) }
    }
    fn erfc(self) -> f64 {
        unsafe { unsafe_cephes_double::erfc(self) }
    }

    fn hyp1f1(self, a: f64, b: f64) -> f64 {
        unsafe { unsafe_cephes_double::hyperg(a, b, self) }
    }
    fn hyp1f2(self, a: f64, b: f64, c: f64) -> f64 {
        let mut err = 0.0;
        unsafe { unsafe_cephes_double::onef2(a, b, c, self, &mut err) }
    }
    fn hyp2f1(self, a: f64, b: f64, c: f64) -> f64 {
        unsafe { unsafe_cephes_double::hyp2f1(a, b, c, self) }
    }
    fn hyp3f0(self, a: f64, b: f64, c: f64) -> f64 {
        let mut err = 0.0;
        unsafe { unsafe_cephes_double::threef0(a, b, c, self, &mut err) }
    }

    fn norm(self) -> f64 {
        unsafe { unsafe_cephes_double::ndtr(self) }
    }
    fn norm_inv(self) -> f64 {
        unsafe { unsafe_cephes_double::ndtri(self) }
    }

    fn besselj(self, v: f64) -> f64 {
        unsafe { unsafe_cephes_double::jv(v, self) }
    }
    fn bessely(self, v: f64) -> f64 {
        unsafe { unsafe_cephes_double::yv(v, self) }
    }
    fn besseli(self, v: f64) -> f64 {
        unsafe { unsafe_cephes_double::iv(v, self) }
    }
    fn besselk(self, n: i32) -> f64 {
        unsafe { unsafe_cephes_double::kn(n, self) }
    }

    fn riemann_zeta(self) -> f64 {
        unsafe { 1.0 + unsafe_cephes_double::zetac(self) }
    }
    fn hurwitz_zeta(self, q: f64) -> f64 {
        unsafe { unsafe_cephes_double::zeta(self, q) }
    }
}

impl FloatSpecial for f32 {
    fn beta(self, b: f32) -> f32 {
        unsafe { betaf(self, b) }
    }
    fn betainc(self, a: f32, b: f32) -> f32 {
        unsafe { incbetf(a, b, self) }
    }
    fn betainc_inv(self, a: f32, b: f32) -> f32 {
        unsafe { incbif(a, b, self) }
    }

    fn factorial(self) -> f32 {
        unsafe { gammaf(self + 1.0) }
    }
    fn gamma(self) -> f32 {
        unsafe { gammaf(self) }
    }
    fn rgamma(self) -> f32 {
        unsafe { rgammaf(self) }
    }
    fn loggamma(self) -> f32 {
        unsafe { lgamf(self) }
    }

    fn gammainc(self, a: f32) -> f32 {
        unsafe { igamf(a, self) }
    }
    fn gammac(self, a: f32) -> f32 {
        unsafe { igamcf(a, self) }
    }
    fn gammac_inv(self, a: f32) -> f32 {
        unsafe { igamif(a, self) }
    }

    fn digamma(self) -> f32 {
        unsafe { psif(self) }
    }

    fn erf(self) -> f32 {
        unsafe { erff(self) }
    }
    fn erfc(self) -> f32 {
        unsafe { erfcf(self) }
    }

    fn hyp1f1(self, a: f32, b: f32) -> f32 {
        unsafe { hypergf(a, b, self) }
    }
    fn hyp1f2(self, a: f32, b: f32, c: f32) -> f32 {
        let mut err = 0.0;
        unsafe { onef2f(a, b, c, self, &mut err) }
    }
    fn hyp2f1(self, a: f32, b: f32, c: f32) -> f32 {
        unsafe { hyp2f1f(a, b, c, self) }
    }
    fn hyp3f0(self, a: f32, b: f32, c: f32) -> f32 {
        let mut err = 0.0;
        unsafe { threef0f(a, b, c, self, &mut err) }
    }

    fn norm(self) -> f32 {
        unsafe { ndtrf(self) }
    }
    fn norm_inv(self) -> f32 {
        unsafe { ndtrif(self) }
    }

    fn besselj(self, v: f32) -> f32 {
        unsafe { jvf(v, self) }
    }
    fn bessely(self, v: f32) -> f32 {
        unsafe { yvf(v, self) }
    }
    fn besseli(self, v: f32) -> f32 {
        unsafe { ivf(v, self) }
    }
    fn besselk(self, n: i32) -> f32 {
        unsafe { knf(n, self) }
    }

    fn riemann_zeta(self) -> f32 {
        unsafe { 1.0 + zetacf(self) }
    }
    fn hurwitz_zeta(self, q: f32) -> f32 {
        unsafe { zetaf(self, q) }
    }
}

