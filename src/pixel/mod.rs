//!Pixel encodings.

use num::Float;

pub use self::srgb::Srgb;

mod srgb;

///A conversion trait for RGB pixel types.
///
///It makes conversion from `Rgb` to various pixel representations easy and
///extensible.
pub trait RgbPixel<T: Float = f32> {
    ///Create an instance of `Self` from red, green, blue and alpha values.
    ///These can be assumed to already be gamma corrected and belongs to the
    ///range [0.0, 1.0].
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> Self;

    ///Convert the red, green, blue and alpha values of `self` to values in
    ///the range [0.0, 1.0]. No gamma correction should be performed.
    fn to_rgba(&self) -> (T, T, T, T);
}

impl<T: Float> RgbPixel<T> for (f32, f32, f32, f32) {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> (f32, f32, f32, f32) {
        ( red.to_f32().unwrap(), green.to_f32().unwrap(), blue.to_f32().unwrap(), alpha.to_f32().unwrap() )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b, a) = *self;
        ( T::from(r).unwrap(), T::from(g).unwrap(), T::from(b).unwrap(), T::from(a).unwrap() )
    }
}

impl<T: Float> RgbPixel<T> for (f32, f32, f32) {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> (f32, f32, f32) {
        ( red.to_f32().unwrap(), green.to_f32().unwrap(), blue.to_f32().unwrap() )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b) = *self;
        ( T::from(r).unwrap(), T::from(g).unwrap(), T::from(b).unwrap(), T::one() )
    }
}
impl<T: Float> RgbPixel<T> for (f64, f64, f64, f64) {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> (f64, f64, f64, f64) {
        ( red.to_f64().unwrap(), green.to_f64().unwrap(), blue.to_f64().unwrap(), alpha.to_f64().unwrap() )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b, a) = *self;
        ( T::from(r).unwrap(), T::from(g).unwrap(), T::from(b).unwrap(), T::from(a).unwrap() )
    }
}

impl<T: Float> RgbPixel<T> for (f64, f64, f64) {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> (f64, f64, f64) {
        (red.to_f64().unwrap(), green.to_f64().unwrap(), blue.to_f64().unwrap())
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b) = *self;
        ( T::from(r).unwrap(), T::from(g).unwrap(), T::from(b).unwrap(), T::one() )
    }
}

impl<T: Float> RgbPixel<T> for (u8, u8, u8, u8) {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> (u8, u8, u8, u8) {
        (
            (red * T::from(255.0).unwrap()).to_u8().unwrap(),
            (green * T::from(255.0).unwrap()).to_u8().unwrap(),
            (blue * T::from(255.0).unwrap()).to_u8().unwrap(),
            (alpha * T::from(255.0).unwrap()).to_u8().unwrap(),
        )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b, a) = *self;
        (
            T::from(r).unwrap() / T::from(255.0).unwrap(),
            T::from(g).unwrap() / T::from(255.0).unwrap(),
            T::from(b).unwrap() / T::from(255.0).unwrap(),
            T::from(a).unwrap() / T::from(255.0).unwrap(),
        )
    }
}

impl<T: Float> RgbPixel<T> for (u8, u8, u8) {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> (u8, u8, u8) {
        (
            (red * T::from(255.0).unwrap()).to_u8().unwrap(),
            (green * T::from(255.0).unwrap()).to_u8().unwrap(),
            (blue * T::from(255.0).unwrap()).to_u8().unwrap(),
        )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b) = *self;
        (
            T::from(r).unwrap() / T::from(255.0).unwrap(),
            T::from(g).unwrap() / T::from(255.0).unwrap(),
            T::from(b).unwrap() / T::from(255.0).unwrap(),
            T::one(),
        )
    }
}

impl<T: Float> RgbPixel<T> for [f32; 4] {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> [f32; 4] {
        [ red.to_f32().unwrap(), green.to_f32().unwrap(), blue.to_f32().unwrap(), alpha.to_f32().unwrap() ]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        ( T::from(self[0]).unwrap(), T::from(self[1]).unwrap(), T::from(self[2]).unwrap(), T::from(self[3]).unwrap() )
    }
}

impl<T: Float> RgbPixel<T> for [f32; 3] {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> [f32; 3] {
        [red.to_f32().unwrap(), green.to_f32().unwrap(), blue.to_f32().unwrap()]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (T::from(self[0]).unwrap(), T::from(self[1]).unwrap(), T::from(self[2]).unwrap(), T::one())
    }
}
impl<T: Float> RgbPixel<T> for [f64; 4] {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> [f64; 4] {
        [red.to_f64().unwrap(), green.to_f64().unwrap(), blue.to_f64().unwrap(), alpha.to_f64().unwrap()]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (T::from(self[0]).unwrap(), T::from(self[1]).unwrap(), T::from(self[2]).unwrap(), T::from(self[3]).unwrap())
    }
}

impl<T: Float> RgbPixel<T> for [f64; 3] {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> [f64; 3] {
        [red.to_f64().unwrap(), green.to_f64().unwrap(), blue.to_f64().unwrap()]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (T::from(self[0]).unwrap(), T::from(self[1]).unwrap(), T::from(self[2]).unwrap(), T::one())
    }
}

impl<T: Float> RgbPixel<T> for [u8; 4] {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> [u8; 4] {
        [
            (red * T::from(255.0).unwrap()).to_u8().unwrap(),
            (green * T::from(255.0).unwrap()).to_u8().unwrap(),
            (blue * T::from(255.0).unwrap()).to_u8().unwrap(),
            (alpha * T::from(255.0).unwrap()).to_u8().unwrap(),
        ]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (
            T::from(self[0]).unwrap() / T::from(255.0).unwrap(),
            T::from(self[1]).unwrap() / T::from(255.0).unwrap(),
            T::from(self[2]).unwrap() / T::from(255.0).unwrap(),
            T::from(self[3]).unwrap() / T::from(255.0).unwrap(),
        )
    }
}

impl<T: Float> RgbPixel<T> for [u8; 3] {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> [u8; 3] {
        [
            (red * T::from(255.0).unwrap()).to_u8().unwrap(),
            (green * T::from(255.0).unwrap()).to_u8().unwrap(),
            (blue * T::from(255.0).unwrap()).to_u8().unwrap(),
        ]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (
            T::from(self[0]).unwrap() / T::from(255.0).unwrap(),
            T::from(self[1]).unwrap() / T::from(255.0).unwrap(),
            T::from(self[2]).unwrap() / T::from(255.0).unwrap(),
            T::one(),
        )
    }
}
