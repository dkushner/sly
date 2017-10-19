use num::traits::{Bounded, Num, Zero};

pub trait Component: Copy + Clone + Bounded + Num + Zero {}

impl Component for usize {}
impl Component for u8 {}
impl Component for u16 {}
impl Component for u32 {}
impl Component for u64 {}
impl Component for isize {}
impl Component for i8 {}
impl Component for i16 {}
impl Component for i32 {}
impl Component for i64 {}
impl Component for f32 {}
impl Component for f64 {}

pub trait Element: Copy + Clone {
    type Component: Component;

    fn channels(&self) -> &[Self::Component];
    fn channels_mut(&mut self) -> &mut [Self::Component];
    fn num_channels() -> u8;
    fn map<F>(&self, f: F) -> Self
    where
        F: FnMut(Self::Component) -> Self::Component;
    fn apply<F>(&mut self, f: F)
    where
        F: FnMut(Self::Component) -> Self::Component;
}

macro_rules! define_channels {
    (@step $_idx:expr,) => {};

    (@step $idx:expr, $head:ident, $($tail:ident,)*) => {
        pub fn $ident(&self, value: C) {
            self.data[$idx]
        }

        define_channels!(@step $idx + 1usize, $($tail,)*);
    };

    ($($name:ident),*) => {
        define_channels!(@step 0usize, $($name,)*);
    };
}

macro_rules! define_color {
    (
        $name:ident, $channels:expr, ($($channel:ident),*)
    ) => {
        #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
        #[repr(C)]
        pub struct $name<C: Component> {
            data: [C; $channels],
        }

        impl<C: Component> $name<C> {
            pub fn new($($channel: C),*) -> Self {
                $name {
                    data: [$($channel,)*]
                }
            }
        }
    }
}

define_color!(RGB, 3, (red, green, blue));