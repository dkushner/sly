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
    fn apply<F>(&mut self, mut f: F)
    where
        F: FnMut(Self::Component) -> Self::Component;
}

macro_rules! define_channels {
    (@step $_idx:expr,) => {};

    (@step $idx:expr, $head:ident, $($tail:ident,)*) => {
        pub fn $head(&self, value: C) {
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
        #[repr(C)]
        #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
        pub struct $name<C: Component> {
            data: [C; $channels],
        }

        impl<C: Component> $name<C> {
            pub fn new($($channel: C),*) -> Self {
                $name {
                    data: [$($channel,)*]
                }
            }

            define_channels!($($channel),*);
        }

        impl<C: Component> Element for $name<C> {
            type Component = C;

            fn channels(&self) -> &[Self::Component] {
                &self.data
            }

            fn channels_mut(&mut self) -> &mut [Self::Component] {
                &mut self.data
            }

            fn num_channels() -> u8 {
                $channels
            }

            fn map<F>(&self, f: F) -> Self
            where
                F: FnMut(Self::Component) -> Self::Component
            {
                let mut clone = (*self).clone();
                clone.apply(f);
                clone
            }

            fn apply<F>(&mut self, mut f: F)
            where
                F: FnMut(Self::Component) -> Self::Component
            {
                for v in &mut self.data {
                    *v = f(*v);
                }
            }
        }
    }
}

define_color!(RGB, 3, (red, green, blue));
define_color!(RGBA, 4, (red, green, blue, alpha));