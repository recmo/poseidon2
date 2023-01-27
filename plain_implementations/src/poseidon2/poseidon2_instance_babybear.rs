use super::poseidon2_params::Poseidon2Params;
use crate::fields::babybear::FpBabyBear;
use crate::fields::utils::from_hex;

use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = FpBabyBear;

lazy_static! {
    pub static ref MAT_DIAG24_M_1: Vec<Scalar> = vec![
    from_hex("0x51310feb"),
    from_hex("0x6e029132"),
    from_hex("0x6498ba80"),
    from_hex("0x0bfb760c"),
    from_hex("0x70cd8cd1"),
    from_hex("0x00744ae5"),
    from_hex("0x43698015"),
    from_hex("0x6345e18d"),
    from_hex("0x6b7b89c2"),
    from_hex("0x3702b5f4"),
    from_hex("0x05d55881"),
    from_hex("0x07957f27"),
    from_hex("0x3d7f934d"),
    from_hex("0x101b60b2"),
    from_hex("0x6bece8db"),
    from_hex("0x513c2e62"),
    from_hex("0x3d741b5a"),
    from_hex("0x56285284"),
    from_hex("0x2b96573a"),
    from_hex("0x4d1848ff"),
    from_hex("0x0d7682b2"),
    from_hex("0x6bc70bc1"),
    from_hex("0x6f315f02"),
    from_hex("0x1574eea6"),
    ];

    pub static ref MAT_INTERNAL24: Vec<Vec<Scalar>> = vec![
    vec![from_hex("0x51310fec"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x6e029133"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x6498ba81"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x0bfb760d"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x70cd8cd2"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00744ae6"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x43698016"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x6345e18e"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x6b7b89c3"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x3702b5f5"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x05d55882"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x07957f28"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x3d7f934e"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x101b60b3"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x6bece8dc"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x513c2e63"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x3d741b5b"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x56285285"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x2b96573b"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x4d184900"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x0d7682b3"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x6bc70bc2"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x6f315f03"),
    from_hex("0x00000001"),
    ],
    vec![from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x00000001"),
    from_hex("0x1574eea7"),
    ],
    ];
    
    pub static ref RC24: Vec<Vec<Scalar>> = vec![
    vec![from_hex("0x75c89df8"),
    from_hex("0x0af90431"),
    from_hex("0x39e877bf"),
    from_hex("0x18a5a8cd"),
    from_hex("0x588e9a95"),
    from_hex("0x16760b26"),
    from_hex("0x026ff4a4"),
    from_hex("0x2326df32"),
    from_hex("0x5a0b7c6f"),
    from_hex("0x37d3973d"),
    from_hex("0x1e5bc883"),
    from_hex("0x59d3664a"),
    from_hex("0x677f8a14"),
    from_hex("0x64a3389e"),
    from_hex("0x0d81d503"),
    from_hex("0x77f51a78"),
    from_hex("0x1fc66dca"),
    from_hex("0x13a094e3"),
    from_hex("0x5d410ff1"),
    from_hex("0x2383e213"),
    from_hex("0x4f1144c3"),
    from_hex("0x1ee540c8"),
    from_hex("0x097edb05"),
    from_hex("0x2c17a521"),
    ],
    vec![from_hex("0x66927671"),
    from_hex("0x76626203"),
    from_hex("0x1de486e8"),
    from_hex("0x440c19f5"),
    from_hex("0x20bbb67a"),
    from_hex("0x2d4f79b0"),
    from_hex("0x55b64ceb"),
    from_hex("0x6712fbb4"),
    from_hex("0x24a3ec8a"),
    from_hex("0x3b6062d5"),
    from_hex("0x265abe23"),
    from_hex("0x3b6fb81b"),
    from_hex("0x43ddb4af"),
    from_hex("0x49a5c4a8"),
    from_hex("0x64581fc3"),
    from_hex("0x77b795f1"),
    from_hex("0x1e918db8"),
    from_hex("0x550e2c6a"),
    from_hex("0x6b3571c8"),
    from_hex("0x3609bf61"),
    from_hex("0x46ae6e8f"),
    from_hex("0x41fe0f80"),
    from_hex("0x20420c1a"),
    from_hex("0x0e95ad8a"),
    ],
    vec![from_hex("0x1b48deb1"),
    from_hex("0x122ac59f"),
    from_hex("0x0252c4a7"),
    from_hex("0x2696a2c4"),
    from_hex("0x01b232cf"),
    from_hex("0x5c26d56b"),
    from_hex("0x405535be"),
    from_hex("0x20cc2fe6"),
    from_hex("0x1c16804d"),
    from_hex("0x5fa5a4b8"),
    from_hex("0x579729ad"),
    from_hex("0x63e11b88"),
    from_hex("0x5601d544"),
    from_hex("0x3eeeaa1a"),
    from_hex("0x4dfd202b"),
    from_hex("0x4fa5da37"),
    from_hex("0x2118e7fc"),
    from_hex("0x3eb830dd"),
    from_hex("0x169ee87b"),
    from_hex("0x2bd97286"),
    from_hex("0x22865c3e"),
    from_hex("0x6aaa6429"),
    from_hex("0x29e0f68d"),
    from_hex("0x6aca023a"),
    ],
    vec![from_hex("0x4d02be5e"),
    from_hex("0x4f170e62"),
    from_hex("0x5cc04ba7"),
    from_hex("0x20655f92"),
    from_hex("0x6a216eea"),
    from_hex("0x7685bdbd"),
    from_hex("0x332f989c"),
    from_hex("0x57a66733"),
    from_hex("0x0719b431"),
    from_hex("0x2f19a5c2"),
    from_hex("0x5191f27f"),
    from_hex("0x0e38bd91"),
    from_hex("0x1c5abcda"),
    from_hex("0x1da9f1bf"),
    from_hex("0x42b01881"),
    from_hex("0x5bf52e0f"),
    from_hex("0x4dc17cb8"),
    from_hex("0x1d4688d6"),
    from_hex("0x18e0160d"),
    from_hex("0x54841116"),
    from_hex("0x6b381bc9"),
    from_hex("0x5c4a838c"),
    from_hex("0x0e3dc983"),
    from_hex("0x71c18d85"),
    ],
    vec![from_hex("0x71902c5e"),
    from_hex("0x1654566c"),
    from_hex("0x38b6f3a0"),
    from_hex("0x35a19789"),
    from_hex("0x2adf05e2"),
    from_hex("0x07d01f0d"),
    from_hex("0x02677f73"),
    from_hex("0x385f532a"),
    from_hex("0x3e2fb2ac"),
    from_hex("0x4e88c53b"),
    from_hex("0x1e370587"),
    from_hex("0x4e64e63c"),
    from_hex("0x5942f756"),
    from_hex("0x16944480"),
    from_hex("0x711cc845"),
    from_hex("0x76a93011"),
    from_hex("0x6c978579"),
    from_hex("0x68f3467f"),
    from_hex("0x47df3684"),
    from_hex("0x09001f22"),
    from_hex("0x3c1d9746"),
    from_hex("0x70796779"),
    from_hex("0x3b0bcbc9"),
    from_hex("0x64665dab"),
    ],
    vec![from_hex("0x55eb1733"),
    from_hex("0x3a240c5c"),
    from_hex("0x5a4445ac"),
    from_hex("0x3623eb7e"),
    from_hex("0x6c1a9a42"),
    from_hex("0x0a024e58"),
    from_hex("0x44097942"),
    from_hex("0x240878ef"),
    from_hex("0x3f507e57"),
    from_hex("0x68b9393f"),
    from_hex("0x43163825"),
    from_hex("0x6d71852f"),
    from_hex("0x5937ffff"),
    from_hex("0x564b7092"),
    from_hex("0x60d0dcb4"),
    from_hex("0x414e0bf2"),
    from_hex("0x754a4cd6"),
    from_hex("0x4edd0db2"),
    from_hex("0x341f2761"),
    from_hex("0x6da77d03"),
    from_hex("0x6f157bce"),
    from_hex("0x436d5397"),
    from_hex("0x5501d7ab"),
    from_hex("0x310b522f"),
    ],
    vec![from_hex("0x0ab56164"),
    from_hex("0x150e3635"),
    from_hex("0x0a7e3bc1"),
    from_hex("0x05fac2a2"),
    from_hex("0x68f51ab3"),
    from_hex("0x1f03a3cf"),
    from_hex("0x398e133a"),
    from_hex("0x0f5d1399"),
    from_hex("0x5f29a526"),
    from_hex("0x6cea6946"),
    from_hex("0x1dca5af8"),
    from_hex("0x5c06e542"),
    from_hex("0x1665a8f2"),
    from_hex("0x4b950256"),
    from_hex("0x6ad5b666"),
    from_hex("0x68e13aff"),
    from_hex("0x3c118567"),
    from_hex("0x70ef24e6"),
    from_hex("0x08fe6bb2"),
    from_hex("0x77610aee"),
    from_hex("0x140b4090"),
    from_hex("0x12a59e03"),
    from_hex("0x43376c38"),
    from_hex("0x5cc0ffc8"),
    ],
    vec![from_hex("0x1b05d461"),
    from_hex("0x50cd7eb1"),
    from_hex("0x320b07e2"),
    from_hex("0x24cfbcfc"),
    from_hex("0x12bb2eec"),
    from_hex("0x33169cde"),
    from_hex("0x5f7ea3ec"),
    from_hex("0x3f690e4c"),
    from_hex("0x2f47b8cd"),
    from_hex("0x32d5e526"),
    from_hex("0x3493e551"),
    from_hex("0x07eac5d0"),
    from_hex("0x111bfa27"),
    from_hex("0x2f667258"),
    from_hex("0x4c1c06d2"),
    from_hex("0x6cd4b86c"),
    from_hex("0x30702542"),
    from_hex("0x53d64e17"),
    from_hex("0x7143e320"),
    from_hex("0x47e038ee"),
    from_hex("0x6871b994"),
    from_hex("0x4113c042"),
    from_hex("0x240af147"),
    from_hex("0x5cde3214"),
    ],
    vec![from_hex("0x600fd4c2"),
    from_hex("0x17a79c40"),
    from_hex("0x43600bcc"),
    from_hex("0x158238e7"),
    from_hex("0x23082895"),
    from_hex("0x53b32037"),
    from_hex("0x583dd7e9"),
    from_hex("0x1c2966ec"),
    from_hex("0x4f1c4aa2"),
    from_hex("0x5e004513"),
    from_hex("0x3b80fd92"),
    from_hex("0x4cfed7fe"),
    from_hex("0x62679264"),
    from_hex("0x59abcafe"),
    from_hex("0x4ddde884"),
    from_hex("0x45535ec3"),
    from_hex("0x6a446628"),
    from_hex("0x238b231e"),
    from_hex("0x307d9f5a"),
    from_hex("0x1128d0a2"),
    from_hex("0x6ff378d4"),
    from_hex("0x2cd023e3"),
    from_hex("0x2f1d7c11"),
    from_hex("0x4c10f8f9"),
    ],
    vec![from_hex("0x6bd45d5c"),
    from_hex("0x74c89ea9"),
    from_hex("0x33cb1c8b"),
    from_hex("0x2c94a9ce"),
    from_hex("0x3816b43d"),
    from_hex("0x0e65a71e"),
    from_hex("0x5e38c082"),
    from_hex("0x6427f754"),
    from_hex("0x3bcce6ef"),
    from_hex("0x6ea9c00e"),
    from_hex("0x510a8434"),
    from_hex("0x30063591"),
    from_hex("0x06620981"),
    from_hex("0x4beaf035"),
    from_hex("0x4be4a698"),
    from_hex("0x5ca330ec"),
    from_hex("0x33d54bf7"),
    from_hex("0x316d226b"),
    from_hex("0x3fe68690"),
    from_hex("0x54e0d88e"),
    from_hex("0x197cc4e9"),
    from_hex("0x23c98f88"),
    from_hex("0x299beb0c"),
    from_hex("0x1c598f09"),
    ],
    vec![from_hex("0x772a2f2d"),
    from_hex("0x28f39a58"),
    from_hex("0x15c6d916"),
    from_hex("0x5482d960"),
    from_hex("0x1bcfe651"),
    from_hex("0x17da1dbc"),
    from_hex("0x4ae31a82"),
    from_hex("0x3d1fb87a"),
    from_hex("0x10e59612"),
    from_hex("0x59fff611"),
    from_hex("0x7151fd1f"),
    from_hex("0x04308067"),
    from_hex("0x538af856"),
    from_hex("0x31ec4b46"),
    from_hex("0x5ca7e0e1"),
    from_hex("0x014f24bc"),
    from_hex("0x34801c56"),
    from_hex("0x400f2575"),
    from_hex("0x42c0be20"),
    from_hex("0x3fef7fd5"),
    from_hex("0x40e16c35"),
    from_hex("0x39a29264"),
    from_hex("0x5139e92e"),
    from_hex("0x6243e0e4"),
    ],
    vec![from_hex("0x356f27cc"),
    from_hex("0x022c022e"),
    from_hex("0x47511780"),
    from_hex("0x28cc6078"),
    from_hex("0x358b8d3d"),
    from_hex("0x45f840fa"),
    from_hex("0x615d96d5"),
    from_hex("0x4e68d893"),
    from_hex("0x761e8520"),
    from_hex("0x3ae0bc8d"),
    from_hex("0x4d793ea9"),
    from_hex("0x461c98dd"),
    from_hex("0x0e6ae09e"),
    from_hex("0x2473818a"),
    from_hex("0x027d0ea1"),
    from_hex("0x1e37b4fc"),
    from_hex("0x5a9b918e"),
    from_hex("0x1af57d91"),
    from_hex("0x16b2e255"),
    from_hex("0x3e968bb0"),
    from_hex("0x025fff84"),
    from_hex("0x4a0ae3b6"),
    from_hex("0x196ea9cc"),
    from_hex("0x492f54d7"),
    ],
    vec![from_hex("0x21dd4a08"),
    from_hex("0x63d6a965"),
    from_hex("0x0d8d0c0c"),
    from_hex("0x4f797854"),
    from_hex("0x1ddd249b"),
    from_hex("0x48ea3e56"),
    from_hex("0x2028b394"),
    from_hex("0x59ba09f9"),
    from_hex("0x73e9f78e"),
    from_hex("0x14d3b6cb"),
    from_hex("0x095b67e2"),
    from_hex("0x5b97fccb"),
    from_hex("0x4ae9fd70"),
    from_hex("0x1c3d8daf"),
    from_hex("0x592bb255"),
    from_hex("0x459d8bb0"),
    from_hex("0x53284952"),
    from_hex("0x62d219ff"),
    from_hex("0x085b04e1"),
    from_hex("0x292a4f83"),
    from_hex("0x33ab42f3"),
    from_hex("0x3b4848d7"),
    from_hex("0x35b3b4e3"),
    from_hex("0x2b104210"),
    ],
    vec![from_hex("0x36bc80a0"),
    from_hex("0x23bc9027"),
    from_hex("0x5d3a8ec7"),
    from_hex("0x0b24aed6"),
    from_hex("0x0ede5278"),
    from_hex("0x5e5305a8"),
    from_hex("0x39ac900a"),
    from_hex("0x05dc1627"),
    from_hex("0x4294fabf"),
    from_hex("0x6de5d842"),
    from_hex("0x2a826d22"),
    from_hex("0x1f30553c"),
    from_hex("0x340b8c1b"),
    from_hex("0x05b2b5d4"),
    from_hex("0x752b7fd1"),
    from_hex("0x1ee6d240"),
    from_hex("0x63125d49"),
    from_hex("0x09fe5b3e"),
    from_hex("0x05f70546"),
    from_hex("0x13eb2cbd"),
    from_hex("0x0d833c10"),
    from_hex("0x5a629eec"),
    from_hex("0x5a7fd158"),
    from_hex("0x0eb52380"),
    ],
    vec![from_hex("0x560ad5c4"),
    from_hex("0x29645160"),
    from_hex("0x1170c734"),
    from_hex("0x1822aac9"),
    from_hex("0x217db6d3"),
    from_hex("0x2e21c8d3"),
    from_hex("0x606c5e6a"),
    from_hex("0x75faa8f5"),
    from_hex("0x3094fa7e"),
    from_hex("0x21b00a43"),
    from_hex("0x4bb30480"),
    from_hex("0x498991be"),
    from_hex("0x28b20b19"),
    from_hex("0x5c3d4d5f"),
    from_hex("0x0b992d6a"),
    from_hex("0x2607ef95"),
    from_hex("0x24188fae"),
    from_hex("0x3bf8a077"),
    from_hex("0x33f06cbf"),
    from_hex("0x2cd92b31"),
    from_hex("0x057d5b1b"),
    from_hex("0x344f5403"),
    from_hex("0x74b78582"),
    from_hex("0x1f3d6a54"),
    ],
    vec![from_hex("0x5f1e1ae8"),
    from_hex("0x6136a9a2"),
    from_hex("0x470c5e78"),
    from_hex("0x02ad2fca"),
    from_hex("0x4cd3a6bb"),
    from_hex("0x05b4f5c1"),
    from_hex("0x20436bea"),
    from_hex("0x1bed9ec7"),
    from_hex("0x1f29cc65"),
    from_hex("0x1417eaa3"),
    from_hex("0x0ba88347"),
    from_hex("0x0a65070c"),
    from_hex("0x03217e79"),
    from_hex("0x3e5f5253"),
    from_hex("0x60bb876e"),
    from_hex("0x320e4cec"),
    from_hex("0x53b2c52e"),
    from_hex("0x3e1d6eff"),
    from_hex("0x687a3e07"),
    from_hex("0x112d32bc"),
    from_hex("0x5af9296f"),
    from_hex("0x2cded641"),
    from_hex("0x6ecc492b"),
    from_hex("0x45bd2f2c"),
    ],
    vec![from_hex("0x5068b70b"),
    from_hex("0x2563dc0f"),
    from_hex("0x47e5ec52"),
    from_hex("0x6e316d9b"),
    from_hex("0x770839b4"),
    from_hex("0x4fecdeaa"),
    from_hex("0x20e2248a"),
    from_hex("0x257fa6ed"),
    from_hex("0x21abc9d3"),
    from_hex("0x4aaae270"),
    from_hex("0x17e21621"),
    from_hex("0x70d18ab2"),
    from_hex("0x4a23443e"),
    from_hex("0x169492c3"),
    from_hex("0x627575d6"),
    from_hex("0x4bcf6471"),
    from_hex("0x070d29e0"),
    from_hex("0x75fbfe05"),
    from_hex("0x08e3b575"),
    from_hex("0x582caf96"),
    from_hex("0x0ad360ea"),
    from_hex("0x53e6e93d"),
    from_hex("0x5ce70e60"),
    from_hex("0x68b4d9c8"),
    ],
    vec![from_hex("0x665572ef"),
    from_hex("0x02d7409a"),
    from_hex("0x645cdaf7"),
    from_hex("0x1a6ffc30"),
    from_hex("0x1cdb2768"),
    from_hex("0x681b647b"),
    from_hex("0x2744d691"),
    from_hex("0x51749894"),
    from_hex("0x1fa88ff1"),
    from_hex("0x3b11764e"),
    from_hex("0x71fec668"),
    from_hex("0x12ad4534"),
    from_hex("0x51ce85bf"),
    from_hex("0x2fa22845"),
    from_hex("0x4279a18a"),
    from_hex("0x6436c1c4"),
    from_hex("0x00ac710b"),
    from_hex("0x64aa70d4"),
    from_hex("0x6cb3f64d"),
    from_hex("0x73215a9e"),
    from_hex("0x1e0d8c6a"),
    from_hex("0x43cba3a4"),
    from_hex("0x207f2305"),
    from_hex("0x3b06e336"),
    ],
    vec![from_hex("0x3e1b5bb9"),
    from_hex("0x02392281"),
    from_hex("0x18b48bd7"),
    from_hex("0x0ed07feb"),
    from_hex("0x50fd2ce1"),
    from_hex("0x27016dbd"),
    from_hex("0x57328674"),
    from_hex("0x474cb312"),
    from_hex("0x77168659"),
    from_hex("0x6adbaa3a"),
    from_hex("0x6910fad6"),
    from_hex("0x1443e8c4"),
    from_hex("0x263c7e3e"),
    from_hex("0x034b3bcb"),
    from_hex("0x1bdfbbff"),
    from_hex("0x0d60038e"),
    from_hex("0x57283b50"),
    from_hex("0x06989f09"),
    from_hex("0x53e02e93"),
    from_hex("0x0f36babc"),
    from_hex("0x1d11c437"),
    from_hex("0x34c59005"),
    from_hex("0x0e84eb31"),
    from_hex("0x76503e52"),
    ],
    vec![from_hex("0x5172219b"),
    from_hex("0x187801e0"),
    from_hex("0x484aead8"),
    from_hex("0x27658b1c"),
    from_hex("0x29264a97"),
    from_hex("0x3dce14e1"),
    from_hex("0x35996177"),
    from_hex("0x5d5371b7"),
    from_hex("0x6ec08900"),
    from_hex("0x5d816715"),
    from_hex("0x20cfbffa"),
    from_hex("0x613f7c59"),
    from_hex("0x0569fcff"),
    from_hex("0x0761a9cb"),
    from_hex("0x2cfbcf11"),
    from_hex("0x085eea02"),
    from_hex("0x4860f9f0"),
    from_hex("0x4d205005"),
    from_hex("0x57f328bd"),
    from_hex("0x1a650f78"),
    from_hex("0x46ce7839"),
    from_hex("0x770a894b"),
    from_hex("0x1c0e9cb6"),
    from_hex("0x6e0ca41a"),
    ],
    vec![from_hex("0x0552da59"),
    from_hex("0x594dec6b"),
    from_hex("0x2aab23ed"),
    from_hex("0x60c6eb54"),
    from_hex("0x033206e3"),
    from_hex("0x0c7a8237"),
    from_hex("0x56a98019"),
    from_hex("0x5479f680"),
    from_hex("0x0c51522a"),
    from_hex("0x35d2c421"),
    from_hex("0x69a6d90b"),
    from_hex("0x5e7946e4"),
    from_hex("0x6b48ddca"),
    from_hex("0x728e2313"),
    from_hex("0x14dad2af"),
    from_hex("0x3947f321"),
    from_hex("0x67bd011f"),
    from_hex("0x60c4f777"),
    from_hex("0x472591d6"),
    from_hex("0x32731626"),
    from_hex("0x70a23d5b"),
    from_hex("0x151b6c47"),
    from_hex("0x6a67fb25"),
    from_hex("0x2fe0c49e"),
    ],
    vec![from_hex("0x54f4561b"),
    from_hex("0x15704ee4"),
    from_hex("0x16f90bb9"),
    from_hex("0x3be1690c"),
    from_hex("0x76c1febb"),
    from_hex("0x50a35164"),
    from_hex("0x509717e0"),
    from_hex("0x0355f8d3"),
    from_hex("0x668309fd"),
    from_hex("0x54efe9b0"),
    from_hex("0x49578bc5"),
    from_hex("0x02fa220b"),
    from_hex("0x2699c296"),
    from_hex("0x5b07ec6a"),
    from_hex("0x3df439cb"),
    from_hex("0x6fa4b39c"),
    from_hex("0x09e690d7"),
    from_hex("0x0bbce5c4"),
    from_hex("0x6a106fd9"),
    from_hex("0x1c13ac7f"),
    from_hex("0x1347d8fe"),
    from_hex("0x4f5e5b3e"),
    from_hex("0x6b5802b7"),
    from_hex("0x62d98682"),
    ],
    vec![from_hex("0x5a98822d"),
    from_hex("0x5f84702e"),
    from_hex("0x3126f240"),
    from_hex("0x5a1aff1a"),
    from_hex("0x77b04e66"),
    from_hex("0x48c685c3"),
    from_hex("0x355f8209"),
    from_hex("0x2e739ce1"),
    from_hex("0x1adf9544"),
    from_hex("0x1b327099"),
    from_hex("0x0ce1a5a0"),
    from_hex("0x1129ee7e"),
    from_hex("0x5b334254"),
    from_hex("0x4d1d8f66"),
    from_hex("0x23c29e16"),
    from_hex("0x63ac7ba7"),
    from_hex("0x513cb4e4"),
    from_hex("0x52edd435"),
    from_hex("0x4580818e"),
    from_hex("0x55516239"),
    from_hex("0x628cf778"),
    from_hex("0x5eebf087"),
    from_hex("0x15def2ec"),
    from_hex("0x581ee8ce"),
    ],
    vec![from_hex("0x6b4345c5"),
    from_hex("0x62ab4957"),
    from_hex("0x4af89e0f"),
    from_hex("0x4424f30c"),
    from_hex("0x23ad1768"),
    from_hex("0x665a9fd6"),
    from_hex("0x1c0b5ae1"),
    from_hex("0x65abddb8"),
    from_hex("0x7039a4b0"),
    from_hex("0x21c65f02"),
    from_hex("0x539f7cdc"),
    from_hex("0x05ad0446"),
    from_hex("0x4aae4ae6"),
    from_hex("0x258bc9e4"),
    from_hex("0x12e4cd47"),
    from_hex("0x5a8e8499"),
    from_hex("0x21ccbdef"),
    from_hex("0x1aee7336"),
    from_hex("0x622564eb"),
    from_hex("0x5e6d5bbc"),
    from_hex("0x48c2117a"),
    from_hex("0x418c1333"),
    from_hex("0x6afb386a"),
    from_hex("0x0c5b7812"),
    ],
    vec![from_hex("0x565753d9"),
    from_hex("0x59a7ea4f"),
    from_hex("0x37dd5860"),
    from_hex("0x5bf14251"),
    from_hex("0x56398160"),
    from_hex("0x20cbf536"),
    from_hex("0x1e49ee8c"),
    from_hex("0x30e66c4d"),
    from_hex("0x32357c88"),
    from_hex("0x595c244d"),
    from_hex("0x7464a56d"),
    from_hex("0x3a31384e"),
    from_hex("0x3d3273be"),
    from_hex("0x14d339cb"),
    from_hex("0x5044a65c"),
    from_hex("0x1cd80ac0"),
    from_hex("0x2d6ae173"),
    from_hex("0x5bfd70f8"),
    from_hex("0x05a66375"),
    from_hex("0x29500b8c"),
    from_hex("0x08d07343"),
    from_hex("0x45bec74b"),
    from_hex("0x045faaef"),
    from_hex("0x6e763fd3"),
    ],
    vec![from_hex("0x51aa9c1a"),
    from_hex("0x6acf4d51"),
    from_hex("0x35b040a0"),
    from_hex("0x73bf91ea"),
    from_hex("0x3177f4ab"),
    from_hex("0x579e48cc"),
    from_hex("0x241608ab"),
    from_hex("0x333e6a9e"),
    from_hex("0x0d495ddb"),
    from_hex("0x6987b141"),
    from_hex("0x18feffde"),
    from_hex("0x2c91e2c2"),
    from_hex("0x5dd6bff0"),
    from_hex("0x5ce61d67"),
    from_hex("0x779eb8b2"),
    from_hex("0x54826cbb"),
    from_hex("0x3001ecb4"),
    from_hex("0x154bcd9a"),
    from_hex("0x3be72d2e"),
    from_hex("0x5455873b"),
    from_hex("0x09ab84d6"),
    from_hex("0x438c6a8b"),
    from_hex("0x45ffda0e"),
    from_hex("0x1ce4c9c7"),
    ],
    vec![from_hex("0x35c8f2f0"),
    from_hex("0x1dc40835"),
    from_hex("0x756cc6f6"),
    from_hex("0x0e146d08"),
    from_hex("0x0ad9b699"),
    from_hex("0x0541f6fa"),
    from_hex("0x77b884d0"),
    from_hex("0x17db8c98"),
    from_hex("0x59c7d04e"),
    from_hex("0x3e0eba95"),
    from_hex("0x2a28a66e"),
    from_hex("0x60566879"),
    from_hex("0x1ae1e397"),
    from_hex("0x331e0750"),
    from_hex("0x38d0f91e"),
    from_hex("0x52fcf20d"),
    from_hex("0x4508cf06"),
    from_hex("0x4826d030"),
    from_hex("0x321f7db2"),
    from_hex("0x022452ff"),
    from_hex("0x375adcb4"),
    from_hex("0x69b19019"),
    from_hex("0x47442ecd"),
    from_hex("0x0222de3d"),
    ],
    vec![from_hex("0x3b30e99a"),
    from_hex("0x5b4c7207"),
    from_hex("0x71d04e99"),
    from_hex("0x1f02c78e"),
    from_hex("0x73fc0cdc"),
    from_hex("0x4bb751f9"),
    from_hex("0x2474d1bc"),
    from_hex("0x637b9b71"),
    from_hex("0x4766ad4f"),
    from_hex("0x33060411"),
    from_hex("0x2168b8b4"),
    from_hex("0x4f15816e"),
    from_hex("0x6ddacf2b"),
    from_hex("0x0057c6ad"),
    from_hex("0x5bfb91a9"),
    from_hex("0x6027e02f"),
    from_hex("0x5d967a56"),
    from_hex("0x139d4629"),
    from_hex("0x41f6cc04"),
    from_hex("0x240c7304"),
    from_hex("0x131eba30"),
    from_hex("0x0cd4affb"),
    from_hex("0x72b01bef"),
    from_hex("0x4ef7a632"),
    ],
    vec![from_hex("0x09478952"),
    from_hex("0x6814a0f0"),
    from_hex("0x72a61a71"),
    from_hex("0x39bd9d86"),
    from_hex("0x3ed1fbfc"),
    from_hex("0x653afc45"),
    from_hex("0x4c4ddb5d"),
    from_hex("0x656b8199"),
    from_hex("0x686caf27"),
    from_hex("0x5caf0289"),
    from_hex("0x20e047b0"),
    from_hex("0x736702e2"),
    from_hex("0x1b46d6ed"),
    from_hex("0x64cd8fdd"),
    from_hex("0x475354f5"),
    from_hex("0x59f199b8"),
    from_hex("0x32d5d81d"),
    from_hex("0x2732f5f5"),
    from_hex("0x09538182"),
    from_hex("0x1c3ac6b0"),
    from_hex("0x1f754ae0"),
    from_hex("0x442a0cb1"),
    from_hex("0x74ccefcd"),
    from_hex("0x43013839"),
    ],
    ];
    
    pub static ref POSEIDON2_BABYBEAR_PARAMS: Arc<Poseidon2Params<Scalar>> = Arc::new(Poseidon2Params::new(24, 7, 8, 21, &MAT_DIAG24_M_1, &MAT_INTERNAL24, &RC24));
    }
