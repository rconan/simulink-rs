//! # GMT SIMULINK CONTROLLER BRIDGE
//!
//! This is an interface to build and to run a controller design with Simulink inside Rust
//! ## Example
//! A Simulink model named `SimControl` with 1 input `SimIn1` of size 6 and 1 output `SimOut1` of size 3 is imported into Rust with:
//! ```rust
//! import_simulink!(SimControl, U : (SimIn1,6), Y : (SimOut1,3))
//! build_inputs!(In1,6)
//! build_inputs!(Out1,3)
//! build_controller!(SimControl, U: (SimIn1 -> (In1,in1)), Y: (SimOut1 -> (Out1,out1)))
//! ```

pub trait Simulink {
    fn initialize(&mut self);
    fn __step__(&self);
    fn terminate(&self);
}

/// Import Simulink C definitions
///
/// An Simulink C import is written:  `(Simulink controller name, U : (<Simulink input name,size>,<...>,...), Y : (<Simulink output name,size>,<...>,...))`
#[macro_export]
macro_rules! import_simulink {
    ($controller:ident, U : ($($sim_u:ident, $size_u:expr),+), Y : ($($sim_y:ident, $size_y:expr),+)) => {
        paste::paste!{
            /// Simulink external input (U)
            #[repr(C)]
            #[allow(non_snake_case)]
            #[derive(Debug)]
            struct [<ExtU_ $controller _T>] {
            $($sim_u: [f64;$size_u],)+
        }}
        paste::paste!{
            /// Simulink external output (Y)
            #[repr(C)]
            #[allow(non_snake_case)]
            #[derive(Debug)]
            struct [<ExtY_ $controller _T>] {
            $($sim_y: [f64;$size_y],)+
        }}

        paste::paste!{
        extern "C" {
            fn [<$controller _initialize>]();
            fn [<$controller _step>]();
            fn [<$controller _terminate>]();
            static mut [<$controller _U>]: [<ExtU_ $controller _T>];
            static mut [<$controller _Y>]: [<ExtY_ $controller _T>];
        }}
    };
}

/// Build the controller inputs
///
/// An input definition is: `(<enum name,size>,<...>,...)` or `(<enum name,size,offset>,<...>,...)` with
///  - `enum name`: the name of the input enum variant (U::name)
///  - `size`: the size of the corresponding Simulink input
///  - `offset`: the pointer offset in the corresponding Simulink input
#[macro_export]
macro_rules! build_inputs {
    ($($name:ident, $size:expr),+) => {
        /// Controller inputs U
        #[derive(Debug)]
        pub enum U<'a> {
            $($name(&'a mut [f64; $size])),+
        }
        impl<'a> std::ops::Index<usize> for U<'a> {
            type Output = f64;
            fn index(&self, index: usize) -> &Self::Output {
                match self {
                    $(U::$name(data) => &data[index]),+
                }
            }
        }
        impl<'a> std::ops::IndexMut<usize> for U<'a> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match self {
                    $(U::$name(data) => &mut data[index]),+
                }
            }
        }
    };
    ($($name:ident, $size:expr,$offset:expr),+) => {
        /// Controller inputs U
        #[derive(Debug)]
        pub enum U<'a> {
            $($name(&'a mut [f64; $size])),+
        }
        impl<'a> std::ops::Index<usize> for U<'a> {
            type Output = f64;
            fn index(&self, index: usize) -> &Self::Output {
                match self {
                    $(U::$name(data) => &data[index + $offset]),+
                }
            }
        }
        impl<'a> std::ops::IndexMut<usize> for U<'a> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match self {
                    $(U::$name(data) => &mut data[index + $offset]),+
                }
            }
        }
    };
}
/// Build the controller outputs
///
///
/// An output definition is: `(<enum name,size>,<...>,...)` or `(<enum name,size,offset>,<...>,...)` with
///  - `enum name`: the name of the output enum variant (Y::name)
///  - `size`: the size of the corresponding Simulink output
///  - `offset`: the pointer offset in the corresponding Simulink output
#[macro_export]
macro_rules! build_outputs {
    ($($name:ident, $size:expr),+) => {
        /// Controller outputs Y
        #[derive(Debug)]
        pub enum Y<'a> {
            $($name(&'a mut [f64; $size])),+
        }
        impl<'a> std::ops::Index<usize> for Y<'a> {
            type Output = f64;
            fn index(&self, index: usize) -> &Self::Output {
                match self {
                    $(Y::$name(data) => &data[index]),+
                }
            }
        }
        impl<'a> From<&Y<'a>> for Vec<f64> {
            fn from(y: &Y<'a>) -> Vec<f64> {
                match y {
                    $(Y::$name(data) => data.to_vec()),+
                }
            }
        }
    };
    ($($name:ident, $size:expr,$subsize:expr,$offset:expr),+) => {
        /// Controller outputs Y
        #[derive(Debug)]
        pub enum Y<'a> {
            $($name(&'a mut [f64; $size])),+
        }
        impl<'a> std::ops::Index<usize> for Y<'a> {
            type Output = f64;
            fn index(&self, index: usize) -> &Self::Output {
                match self {
                    $(Y::$name(data) => &data[index + $offset]),+
                }
            }
        }
        impl<'a> From<&Y<'a>> for Vec<f64> {
            fn from(y: &Y<'a>) -> Vec<f64> {
                match y {
                    $(Y::$name(data) => data[$offset..$offset+$subsize].to_vec()),+
                }
            }
        }
    }
}
/// Build the controller
///
/// A controller definition is: `(Simulink controller name, U : (<Simulink input name -> (enum type,variable name)>,<...>,...), Y : (<Simulink output name -> (enum type,variable name)>,<...>,...))`
#[macro_export]
macro_rules! build_controller {
    ($controller:ident, U : ($($sim_u:ident -> ($enum_u:ident,$var_u:ident)),+) , Y : ($($sim_y:ident -> ($enum_y:ident,$var_y:ident)),+)) => {
        /// Controller
        pub struct Controller<'a> {
            $(pub $var_u: U<'a>,)+
            $(pub $var_y: Y<'a>,)+
        }
        paste::paste!{
        impl<'a> Controller<'a> {
            /// Creates a new controller
            pub fn new() -> Self {
                let mut this = unsafe {
                    Self {
                        $($var_u: U::$enum_u(&mut [<$controller _U>].$sim_u),)+
                        $($var_y: Y::$enum_y(&mut [<$controller _Y>].$sim_y),)+
                    }
                };
                this.initialize();
                this
            }
        }}
        use $crate::controllers::Simulink;
        paste::paste! {
        impl<'a> Simulink for Controller<'a> {
            fn initialize(&mut self) {
                unsafe {
                    [<$controller _initialize>]();
                }
            }
            fn __step__(&self) {
                unsafe {
                    [<$controller _step>]();
                }
            }
            fn terminate(&self) {
                unsafe {
                    [<$controller _terminate>]();
                }
            }
        }
        }
        impl<'a> Drop for Controller<'a> {
            fn drop(&mut self) {
                self.terminate()
            }
        }
        impl<'a> Iterator for &Controller<'a> {
            type Item = ();
            fn next(&mut self) -> Option<Self::Item> {
                self.__step__();
                Some(())
            }
        }
        impl<'a> Iterator for Controller<'a> {
            type Item = ();
            fn next(&mut self) -> Option<Self::Item> {
                self.__step__();
                Some(())
            }
        }
    };
}
