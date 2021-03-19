/**
The Gaussian MKS unit system

Note: this system is incomplete. More derived units and constants are coming.





Following, we list all of the [base units](#base-units), [derived units](#derived-units), and
[constants](#constants) that are defined in this unit system.
# Base Units
Constant | Unit | Print Token | Dimensionn
---|---|---|---
SQRTM | SqrtMeter | sqrtm | 
SQRTKG | SqrtKilogram | sqrtkg | 
S | Second | s | Time
# Derived Units
Constant | Unit | Unit Definition | Dimension
---|---|---|---
M | Meter | SqrtMeter * SqrtMeter | Length
KG | Kilogram | SqrtKilogram * SqrtKilogram | Mass
MPS | MeterPerSecond | Meter / Second | Velocity
# Constants
Name | Constant | Value | Unit | Dimension 
---|---|---|---|---
*/
pub mod mks {
    make_units! {
        MKS;
        ONE: Unitless;

        base {
            SQRTM: SqrtMeter, "sqrtm";
            SQRTKG: SqrtKilogram, "sqrtkg";
            S: Second, "s", Time;
        }

        derived {
            M: Meter = (SqrtMeter * SqrtMeter), Length;
            KG: Kilogram = (SqrtKilogram * SqrtKilogram), Mass;
            MPS: MeterPerSecond = (Meter / Second), Velocity;
        }

        constants {
        }

        fmt = false;
    }

    pub use self::f64consts::*;


    #[test]
    fn test_mks_constants() {
        #[allow(unused_imports)]
        use f64prefixes::*;
        #[allow(unused_imports)]
        use core::f64::consts;

    }
}