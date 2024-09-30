/// Compute principal and secondary branches of the Lambert W function.
pub trait LambertW {
    /// The principal branch of the Lambert W funciton.
    fn lambert_w0(self) -> Self;
    /// The secondary branch of the Lambert W funciton.
    fn lambert_wm1(self) -> Self;
}

#[cfg(feature = "24bits")]
impl LambertW for f32 {
    #[inline]
    fn lambert_w0(self) -> Self {
        super::lambert_w0f(self)
    }
    #[inline]
    fn lambert_wm1(self) -> Self {
        super::lambert_wm1f(self)
    }
}

#[cfg(feature = "50bits")]
impl LambertW for f64 {
    #[inline]
    fn lambert_w0(self) -> Self {
        super::lambert_w0(self)
    }
    #[inline]
    fn lambert_wm1(self) -> Self {
        super::lambert_wm1(self)
    }
}

#[cfg(test)]
mod test {
    use super::LambertW;
    use approx::assert_abs_diff_eq;

    #[cfg(feature = "50bits")]
    #[test]
    fn test_trait_impl_on_f64() {
        assert_abs_diff_eq!(
            (-2.678_794_411_714_424e-1_f64).lambert_w0(),
            -3.993_824_525_397_807e-1
        );
        assert_abs_diff_eq!(
            (-3.578_794_411_714_423e-1_f64).lambert_wm1(),
            -1.253493791367214,
            epsilon = 1e-14
        );
    }

    #[cfg(feature = "24bits")]
    #[test]
    fn test_trait_impl_on_f32() {
        assert_abs_diff_eq!(6.321_205_5e-1_f32.lambert_w0(), 4.167_04e-1, epsilon = 1e-7);
        assert_abs_diff_eq!(
            (-3.578_794_3e-1_f32).lambert_wm1(),
            -1.253_493_8,
            epsilon = 1e-6
        );
    }
}
