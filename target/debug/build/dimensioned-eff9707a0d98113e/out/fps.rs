/**
The foot, pound, second system, using the mass pound as a base unit.

Note: this system is incomplete. More derived units and constants are coming.





Following, we list all of the [base units](#base-units), [derived units](#derived-units), and
[constants](#constants) that are defined in this unit system.
# Base Units
Constant | Unit | Print Token | Dimensionn
---|---|---|---
SQRTFT | SqrtFoot | sqrtft | 
SQRTLB | SqrtPound | sqrtlb | 
S | Second | s | Time
# Derived Units
Constant | Unit | Unit Definition | Dimension
---|---|---|---
FT | Foot | SqrtFoot * SqrtFoot | Length
LB | Pound | SqrtPound * SqrtPound | Mass
# Constants
Name | Constant | Value | Unit | Dimension 
---|---|---|---|---
*/
pub mod fps {
    make_units! {
        FPS;
        ONE: Unitless;

        base {
            SQRTFT: SqrtFoot, "sqrtft";
            SQRTLB: SqrtPound, "sqrtlb";
            S: Second, "s", Time;
        }

        derived {
            FT: Foot = (SqrtFoot * SqrtFoot), Length;
            LB: Pound = (SqrtPound * SqrtPound), Mass;
        }

        constants {
        }

        fmt = false;
    }

    pub use self::f64consts::*;


    #[test]
    fn test_fps_constants() {
        #[allow(unused_imports)]
        use f64prefixes::*;
        #[allow(unused_imports)]
        use core::f64::consts;

    }
}