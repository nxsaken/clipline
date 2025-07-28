use clipline::*;
use rand::distr::{Uniform, uniform, uniform::SampleUniform};
use rand::prelude::*;

const RAND_SEED: u64 = 0;
const NUM_SAMPLES: usize = 10_000;

struct LineGenC<T: SampleUniform> {
    x_i: Uniform<T>,
    y_i: Uniform<T>,

    x_omax_ex: Uniform<T>,
    x_omax_in: Uniform<T>,
    y_omax_ex: Uniform<T>,
    y_omax_in: Uniform<T>,
}

macro_rules! gen_clip {
    ($($T:ty),+) => {
        $(impl LineGenC<$T> {
            fn new(x_max: $T, y_max: $T) -> Result<Self, uniform::Error> {
                Ok(Self {
                    x_i: Uniform::new_inclusive(<$T>::MIN, x_max)?,
                    y_i: Uniform::new_inclusive(<$T>::MIN, y_max)?,
                    x_omax_ex: Uniform::new_inclusive(x_max + 1, <$T>::MAX)?,
                    x_omax_in: Uniform::new_inclusive(x_max, <$T>::MAX)?,
                    y_omax_ex: Uniform::new_inclusive(y_max + 1, <$T>::MAX)?,
                    y_omax_in: Uniform::new_inclusive(y_max, <$T>::MAX)?,
                })
            }

            fn from_clip(clip: &Clip<$T>) -> Result<Self, uniform::Error> {
                Self::new(clip.x_max(), clip.y_max())
            }

            fn reject<R: Rng>(&self, rng: &mut R) -> [$T; 4] {
                let [x0, y0, x1, y1] = match rng.random() {
                    false => [
                        rng.random(),
                        rng.random(),
                        self.y_omax_ex.sample(rng),
                        self.y_omax_in.sample(rng),
                    ],
                    true => [
                        self.x_omax_ex.sample(rng),
                        self.x_omax_in.sample(rng),
                        rng.random(),
                        rng.random(),
                    ],
                };
                [x0, y0, x1, y1]
            }

            fn accept<R: Rng>(&self, rng: &mut R) -> [$T; 4] {
                let [x0, y0, x1, y1] = [
                    self.x_i.sample(rng),
                    self.x_i.sample(rng),
                    self.y_i.sample(rng),
                    self.y_i.sample(rng),
                ];
                [x0, y0, x1, y1]
            }

            fn complex<R: Rng>(&self, rng: &mut R) -> [$T; 4] {
                let [ox, oy] = rng.random();
                let x0 = self.x_i.sample(rng);
                let y0 = self.y_i.sample(rng);
                let x1 = if ox { self.x_omax_ex.sample(rng) } else { self.x_i.sample(rng) };
                let y1 = if oy { self.y_omax_ex.sample(rng) } else { self.y_i.sample(rng) };
                [x0, y0, x1, y1]
            }
        })+
    };
}

gen_clip!(u8, i8, u16, i16);

struct LineGenV<T: SampleUniform> {
    x_i: Uniform<T>,
    y_i: Uniform<T>,

    x_omin_ex: Uniform<T>,
    x_omin_in: Uniform<T>,
    y_omin_ex: Uniform<T>,
    y_omin_in: Uniform<T>,
    x_omax_ex: Uniform<T>,
    x_omax_in: Uniform<T>,
    y_omax_ex: Uniform<T>,
    y_omax_in: Uniform<T>,
}

macro_rules! gen_viewport {
    ($($T:ty),+) => {
        $(impl LineGenV<$T> {
            fn new(x_min: $T, y_min: $T, x_max: $T, y_max: $T) -> Result<Self, uniform::Error> {
                Ok(Self {
                    x_i: Uniform::new_inclusive(x_min, x_max)?,
                    y_i: Uniform::new_inclusive(y_min, y_max)?,
                    x_omin_ex: Uniform::new(<$T>::MIN, x_min)?,
                    x_omin_in: Uniform::new_inclusive(<$T>::MIN, x_min)?,
                    y_omin_ex: Uniform::new(<$T>::MIN, y_min)?,
                    y_omin_in: Uniform::new_inclusive(<$T>::MIN, y_min)?,
                    x_omax_ex: Uniform::new_inclusive(x_max + 1, <$T>::MAX)?,
                    x_omax_in: Uniform::new_inclusive(x_max, <$T>::MAX)?,
                    y_omax_ex: Uniform::new_inclusive(y_max + 1, <$T>::MAX)?,
                    y_omax_in: Uniform::new_inclusive(y_max, <$T>::MAX)?,
                })
            }

            fn from_viewport(clip: &Viewport<$T>) -> Result<Self, uniform::Error> {
                Self::new(clip.x_min(), clip.y_min(), clip.x_max(), clip.y_max())
            }

            fn reject<R: Rng>(&self, rng: &mut R) -> [$T; 4] {
                let [x0, y0, x1, y1] = match rng.random::<[bool; 2]>() {
                    [false, false] => [
                        rng.random(),
                        rng.random(),
                        self.y_omin_ex.sample(rng),
                        self.y_omin_in.sample(rng),
                    ],
                    [false, true] => [
                        rng.random(),
                        rng.random(),
                        self.y_omax_ex.sample(rng),
                        self.y_omax_in.sample(rng),
                    ],
                    [true, false] => [
                        self.x_omin_ex.sample(rng),
                        self.x_omin_in.sample(rng),
                        rng.random(),
                        rng.random(),
                    ],
                    [true, true] => [
                        self.x_omax_ex.sample(rng),
                        self.x_omax_in.sample(rng),
                        rng.random(),
                        rng.random(),
                    ],
                };
                [x0, y0, x1, y1]
            }

            fn accept<R: Rng>(&self, rng: &mut R) -> [$T; 4] {
                let [x0, y0, x1, y1] = [
                    self.x_i.sample(rng),
                    self.x_i.sample(rng),
                    self.y_i.sample(rng),
                    self.y_i.sample(rng),
                ];
                [x0, y0, x1, y1]
            }

            fn complex<R: Rng>(&self, rng: &mut R) -> [$T; 4] {
                let [ix, iy, ox, oy] = rng.random();
                let x0 = if ix { self.x_omin_ex.sample(rng) } else { self.x_i.sample(rng) };
                let y0 = if iy { self.y_omin_ex.sample(rng) } else { self.y_i.sample(rng) };
                let x1 = if ox { self.x_omax_ex.sample(rng) } else { self.x_i.sample(rng) };
                let y1 = if oy { self.y_omax_ex.sample(rng) } else { self.y_i.sample(rng) };
                [x0, y0, x1, y1]
            }
        })+
    };
}

gen_viewport!(u8, i8, u16, i16);
