use super::poseidon_params::PoseidonParams;

use crate::fields::bls12::FpBLS12;
use crate::fields::utils::from_hex;

use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = FpBLS12;

lazy_static! {
    pub static ref MDS3: Vec<Vec<Scalar>> = vec![
        vec![
            from_hex("0x0be6d2562641a7f9161a0e519f4ad29dc2834b6261032527a7b09090f87448b5"),
            from_hex("0x4fdd0ed22815e84200e955e8f2b25aaf34f69c1a2e21a04537eed9643d700e31"),
            from_hex("0x4ab1e3a5a878258b2923d2c5a5a2359f2e0422e276f9ae9d8d2301bf6a926fa4"),
        ],
        vec![
            from_hex("0x00785f6a1b09e831f3c289b5f1cd1a81db94921747965d0e9972c8391e91b82d"),
            from_hex("0x004aad7aa272405e83c5669a4bb5b77522dcfc8926d769b3e19c4da7f9889f65"),
            from_hex("0x2f68507c0d7cc9a51c3aab39429957536ed2b88d9f8c8864a628ed26c7ffd543"),
        ],
        vec![
            from_hex("0x613e33b159118255d8bb81b05c5585da1f7c4f4b207600c933b1680c8926e345"),
            from_hex("0x05cff851a6d8661e95d58ffa6e4d993ca09c2cddabfc9a9e588387fbe63987d2"),
            from_hex("0x4ceee99819e22ad47a7c1fdf488aa7b243efc5c745e731c25ff5d61eec3f30c9"),
        ],
    ];
    pub static ref RC3: Vec<Vec<Scalar>> = vec![
        vec![
            from_hex("0x46458ff603a3ceca58a5b0f51ebf2af001145e02dadf8c016b05afd6f0ae0af4"),
            from_hex("0x2630c94c7a0d2dded30072e816cc9001945f799ac697e8d5f2fde213f2acf4cc"),
            from_hex("0x327bde9325365c26ffb001163a721e577084d28ac6c196694c73aa98cbf0241d"),
        ],
        vec![
            from_hex("0x72878022e5ee9a13d0fcbe1f825e536ed9f3bd959e9081d801e9d2861b19303e"),
            from_hex("0x19ecf6d5f3a70e528cd60e65fcdeeb4d02d925722cc2c38282efb42bb4688194"),
            from_hex("0x06ae544b048738fab2dd25fe238d8b300c07b982277559d50cddc0660c94ea6a"),
        ],
        vec![
            from_hex("0x6d3ed372db22f29ec869431326cf7a095f886eb61023f6f2d9cc4772f69f9aaf"),
            from_hex("0x4362977509ef2838df650150352c763f3d663094a643710795fe2361284349eb"),
            from_hex("0x43087ce1e0d2d41943953fa29c81fb55045ef53f7fda9829b308db8b78a26838"),
        ],
        vec![
            from_hex("0x2d7e8565174f8a470948d4d846477413bb65170a1624119d9c244acf66c376c4"),
            from_hex("0x3abd2284bdafefe77543e51ce1eb4fea37521b222f3dde0dcd507ed2d4552a7a"),
            from_hex("0x2959ed78237b31ad8c397bdfcd5311c1f821272a4fd0e05c27f290e6e1fd120f"),
        ],
        vec![
            from_hex("0x269cd3d904bdfde03829b8aea060b767593e07aa0d7be38a158aa74ff698c72f"),
            from_hex("0x444821d2c8efaf951c10cccb6bf7cc2d1cb8a68bebcbd670401cdf3cee2f54cf"),
            from_hex("0x1b321654fd4730fb020096f49130f57978de1969093128751383433ee491b00a"),
        ],
        vec![
            from_hex("0x35e37d53753f7b79ea2abc0e3d36bcea69c1dcd928383717da7710f0de297dc1"),
            from_hex("0x1213c358c3f79e40f6827e381ad6c3515ec2a1c13d03470846b925ccbacd84e5"),
            from_hex("0x5b48ef193cdd71b0626a7de3ef95c75f873628a2584a49e4830e567d1bb97949"),
        ],
        vec![
            from_hex("0x7328b48b0855badb9357833586e987847c2624fac8efe2240f47bb4b2acb30d1"),
            from_hex("0x35186a54c8a3daf7a1a46a8fe4171b39d694105ba53924f2ec31148fc6f109c5"),
            from_hex("0x034e8a30ac13977ea99a63c761eabf94d560efc505094682bb0cd88901d7db59"),
        ],
        vec![
            from_hex("0x235fa3eb07e8379386634ed3796f145d85fe14eb052726896ec6fb36edffa6d2"),
            from_hex("0x3d692a008bf5632c0dd02300cb65dc8314c40e125041644f3d304a39c9f7766d"),
            from_hex("0x053cc2b7ba6692ffe044620ece258bd9b5fa74ed4eb3abdc6f6b7b68fec05ecd"),
        ],
        vec![
            from_hex("0x46f8d8a16a9c76d407f2c149bfb694fad8a0cdb62f3d9b347c8f3b6fdaae3bb2"),
            from_hex("0x5fd1a1495b949d62b108d31a8b78588e59ad36bdd828cc98cd1d94feeceea0f6"),
            from_hex("0x5e3f1f2e5790a9b181ed32b6a2ea9a9df26b2bad373ed98a8d5d74c9b7fe8158"),
        ],
        vec![
            from_hex("0x22ab888f48dcc0e390f3a63dbd2975787eceddbc2358a14aabaa4e323aa6bb61"),
            from_hex("0x3328e4fe4d66c7c60a3e85bf18ba70f5a146a1cadfa3d47170f7d409c6c0c41d"),
            from_hex("0x5dbdeca9737093c23e891f6cb402b0af59bb4a3e1b3bd79f4ab9994f452f82a1"),
        ],
        vec![
            from_hex("0x3bb7ef2f0064f6405873cb1b501f0731260fa2e42a2f9ea12d7c72f2b7fe923c"),
            from_hex("0x29874ea41dc64e6161f5c4e347ecc0fbacc3dc4fc1afb6db5eeb258d5ae5bbe9"),
            from_hex("0x024ad11caa1cd96f127715bc86d2eda7f4ec069925795c2e1caaad5c1a072c46"),
        ],
        vec![
            from_hex("0x46aa52cc6b4eb800d717e72f50a198f1655894bb7ac7030043f8f282d69e1aad"),
            from_hex("0x61d2bbb1c2f9e1147ca83936f8470a41dbc67b4c69f486d09baf4c38d5f11e27"),
            from_hex("0x2e9b589758f10cfdc32bcb86c51c79c9e8c3717fdb23501077e276aba975ec8f"),
        ],
        vec![
            from_hex("0x4e42482e3a0279e52113b2564a314b264730846bb48391738881ad82a84b4214"),
            from_hex("0x6a2822e5f834faf563b0bf39cfb92d62fc4072b2e63273a1e7f597018930467b"),
            from_hex("0x4ec8d1f5bea04c1c9033e028ec2768ccd485833401f8fefda65858e431d723a0"),
        ],
        vec![
            from_hex("0x39e7e8c0d2704b8d2d37da82a3702d70152a082760573fd5655d86d6b083cc35"),
            from_hex("0x1f86c7ff87f801b19b67ae81e76cb71a4d5a642c53b73b1228999dac2347d3aa"),
            from_hex("0x285e16abb59d028ecf6a67bdf969877a3043c21afcb6fe430bc713e441d2df9c"),
        ],
        vec![
            from_hex("0x48a69d418b0b3edf764f0424bfbcc69ced2a3138484bbd30ef514c0391d2278a"),
            from_hex("0x58c431ad8ccc538274a3d462e8f229b275fbd9251a41af473107aa3647a65450"),
            from_hex("0x4f49448b92f7001dcf68c4c33a019607422cc0f4fd75e13e55e8875d277a9c8a"),
        ],
        vec![
            from_hex("0x1df210839a1ba5149e5c38fb567156f8c33e56db4b52eaccc9886e76aa3d75b3"),
            from_hex("0x00530c36d758beef72b85b107b22383f057a2d04cefb2fde37584d978fcda1ec"),
            from_hex("0x35c550bcb1589033827753c15e474eaf5df199eb8076bf057b849bc4de22e08d"),
        ],
        vec![
            from_hex("0x6a498912fdeba72236cc6cd66fd53d1b875f1cc83eb7af689eb801dfbb0960e2"),
            from_hex("0x6a406871e490b15ecbf2b9d76485f0938e21bb29920195bb368b4e113ca7a975"),
            from_hex("0x1828241200bce522981cb12acf20e9bbb79ed3cd049f9c208635910ef800e328"),
        ],
        vec![
            from_hex("0x195338d451516150385132d4487921c64df241ea42fa95263229d64c9d174bde"),
            from_hex("0x309219ceb9e12ba3324622e29fd1cec79820d09da8c12d53b7da6fcbcb4e6d49"),
            from_hex("0x72d2130470c6855c2b54e625bb34eaa340d8ad2f25c43959c7a4a47c601fa4c9"),
        ],
        vec![
            from_hex("0x1883d8e89ca678db9f00149c8a582cb350657d71b140d5e02da7766114abc1fc"),
            from_hex("0x5af0f335bd730a2fe8091a1f6b2fe6215cbfa93076501164e7dad7fb9e3e0199"),
            from_hex("0x21139d7532a87e34acb001632a75febb192a38c5d37fb56724a9015f4857970d"),
        ],
        vec![
            from_hex("0x11b70212e2294cfaa8f007e667ac27e93056e93f82e99843185430dc19c4c592"),
            from_hex("0x2d077613035f350d5ef48c1d890444d0f1ddb61fa46f87fd43337a41e01d1848"),
            from_hex("0x1258bd3d552a4b62f87449f56da9978140aee82e0347eeab94ab35fdeae0c12f"),
        ],
        vec![
            from_hex("0x437e960237b5de5117e0d347d5333b4562ccf3b91512a537467686bea429a947"),
            from_hex("0x536418fa097ce6cb400f91d612687d2b698c32111e86a33da19dc60407539f6a"),
            from_hex("0x5205100d33b55f3bbab9e8a15891e8684c4860d35b44457708f4d441e929cb0e"),
        ],
        vec![
            from_hex("0x2885797d7ef9cc4ff6282e6997572d27d70b9065f5dd5eaca1efc1e814c4c58a"),
            from_hex("0x1051f5bd9f6891940e418814184403b87e454ecd5765eaa236e05a48e78ab707"),
            from_hex("0x6e1dfb4ecfa16df5818920f1e10fafe465df7569c1d7a297804bf6528ee5237f"),
        ],
        vec![
            from_hex("0x133df401420df4a1e8f7efd7d3ab9473a83a38c377aaca453c153902ac935588"),
            from_hex("0x2d31673a47543e5ca73ab33073f981741e402e7933b65159cc0c4d8495619052"),
            from_hex("0x5b527aea078b85b24eb2a8a876bae1274fed1738dc6d290bafcd0fb74811acb7"),
        ],
        vec![
            from_hex("0x72628a2b66650a69b59f25a8bf3aff7413a2a04d9bb5ca0e4be28017d9dea5cf"),
            from_hex("0x61dc19a872f62b2a21c15d0441772dfb4edee26d52c6aa5d6f39b00b2f5f5c10"),
            from_hex("0x417652c1a479cbc4f237aedc36367c43dd20fe14ef7598e9b79b7efbf5fe4236"),
        ],
        vec![
            from_hex("0x4fb005ea2e8d33237ee3cb94a9d1057746eec3e67ab7114a80d59375b7714615"),
            from_hex("0x73e76b5ca3bc21b10076a9e1c429f5932890f41daa30259deb08c46d7d24136d"),
            from_hex("0x686c742501e7a682612b3118a95628117469623e90aecb94f0adfa470685e8e8"),
        ],
        vec![
            from_hex("0x429e68c6ad51b4c14a18bc45a3a67c72fb412293c48ed3234d8b768439e83e69"),
            from_hex("0x191b06649bae4baab81dcfdb3e187c765d65876fc590eadbbc6d570f1d724021"),
            from_hex("0x29f7fa91fc7f9b13638c50853890218ec1b45ec44469300b4d2cbd7559e3a94e"),
        ],
        vec![
            from_hex("0x2638ca3f3fb57f5c45ca91ee7e924d407a85ca2d07499a724f0ca0a316f56c60"),
            from_hex("0x33d629e0295e113d3f336a7d58b812600a177ac99e4bbd17f68d100ef7933e0d"),
            from_hex("0x0acfd5ee74b64f92426fd3a93d85295579b8c218748d9967e32127d0be44401d"),
        ],
        vec![
            from_hex("0x495a5c408c0320605041348621e8b49c4c5318062aa38f0ea24d759b43c9d97f"),
            from_hex("0x127872928056626d4f3f06017dd64d0cb6e3f87bfae5bc5cb823154386eb4f37"),
            from_hex("0x3fc01daf44c9d78d6b2487c4434fa24d6277c129ba05286fd3c1169b521aa8e0"),
        ],
        vec![
            from_hex("0x3403107a29144c2d21be05b4667ca8fb030d61a5eb0b620b2e2f0fffdce9ebab"),
            from_hex("0x2971939e8f8e021ff03868c16b26cecad7ae3eb6071b277da11d15c6ea8a1b4e"),
            from_hex("0x1896bcd0a75d8116f9534d343926da518fe7fccea714ca99d1e62ce952054b01"),
        ],
        vec![
            from_hex("0x297ec1d998e963a1bbaae698828083fef4c5f432884afca68fb2e034aebb92a6"),
            from_hex("0x57335867b35c83ee82364081e96d841f91d1ba6c62dade8f516cb2016a5b2ade"),
            from_hex("0x216978aed6bb1951888e1d47a152516a9af1852122826ed2c4699abde6c9f4c3"),
        ],
        vec![
            from_hex("0x2c7ff0831de0d4e2d1edd8a7596000ea907e02c25de63b88e5d1c596e042050f"),
            from_hex("0x112895fbc33720907c46ee08c632792ce5ac2414cc6e5f30ddd94a98455cdd12"),
            from_hex("0x694ac52410d6d78b2d8fa472b5ce73d43a678da34b764ff127ac55a7489ee12d"),
        ],
        vec![
            from_hex("0x2526f120caa388c18b0dbff7bc64d612ffbda8f4dbf68c0c6afe58267fb46f83"),
            from_hex("0x4a044b1890eacfb19048f20df725b2ed2fbca672f13bc72626e72680600698b1"),
            from_hex("0x48a9cf646323bd0e013de62d8f0c6a94ca627922e184f0dbb009364613af074e"),
        ],
        vec![
            from_hex("0x2b328294ee9fc5608ba8f9e9fc799ef927a54b8997ee95bd179cecbfd0e29aa7"),
            from_hex("0x43588f21dcf0f4f379c3831ff518a9346f75ad8d685222fc7a377127d74e9760"),
            from_hex("0x2890be2d3a211b019480d0ea005ea41fd93a00f4057d86f1e2434c7e96a82517"),
        ],
        vec![
            from_hex("0x4f725e91294408343c879ace676a667346f90c56ea031a5f6d21c18c90505cf8"),
            from_hex("0x4db62961a57310cfe9e4fd6652d1c916aeb280188f14514fdef716ace280d20d"),
            from_hex("0x1a4e3b44d5a3c33868324ed7229cc981f7e916cfadc6fb26d66484bda4d0a950"),
        ],
        vec![
            from_hex("0x36646f9bfcbb166a05df8c1fc8f22c6d299c5877700e2fdec1ca24404e4d4596"),
            from_hex("0x2287693cfd43746a9d11e6ef6f786a0c72ba7ddc5830d2a66b116280ad048f97"),
            from_hex("0x1e48eef122fe3a6e80ece12a60a08f547c656e2779636edfb8b15fc6b9d5f1a1"),
        ],
        vec![
            from_hex("0x147a884c76ad3605ee187229dc58db9980f1ba37fc2e0381afb11973bc9bb8b6"),
            from_hex("0x4d187b5769c03ec85284136d6ea20639ef58367e236006fae0bf82de605334f6"),
            from_hex("0x73856d5a6dd16cdf96dedd13db30c9fe5929a451a737ab300e1c4de4d673e75f"),
        ],
        vec![
            from_hex("0x19c47a802e2ae6da7f9a95397bbeb0e1ce9baaebda5f16460febae9ee0bc8aed"),
            from_hex("0x06def992ced25b78e847985bf292615519dafa3467aa29ee5b7ee5f3b67cd12e"),
            from_hex("0x4b7615c9be241168701fb388b368dcc408397eeb27dc9ecbbd03ff01b92d638a"),
        ],
        vec![
            from_hex("0x5dd7df3a5dff6565c6eb86d6cba671b614a7ff70e3e2d78ff675d98f85a94a18"),
            from_hex("0x540b0dfdec35133373a7321c395aa1e0a1372d144a11c0d6284cdf847983a723"),
            from_hex("0x58176dbeff850ce13d1629852d1507a8f7cada00738da3390ff3c29fb77bea7c"),
        ],
        vec![
            from_hex("0x56275f1d6387b948de167dd0102738cce5ced6aa9890b03b001d91a74e75571a"),
            from_hex("0x067fffaef61c98d0209ea5a5f567698dd6d9809605309637a10d4d469ed6167c"),
            from_hex("0x6839327baa463eb3bb8bcfa3ca4aa4ec1d0eb2fca814616ec3636e2b2352717d"),
        ],
        vec![
            from_hex("0x4ca4cc63ac5e8a9d6c18342f1e22cfb06f9b759e91e2a2019bb008ec9d2dc94b"),
            from_hex("0x69cb913f2ab9c10f64221e43b5e8c3652536b0dc6324055d95af590e31add55f"),
            from_hex("0x2953ca868290965e7de82c0e121c79e2d0a1872171d52fa05527651e624498a5"),
        ],
        vec![
            from_hex("0x2aa301bdfa3f410581bc4398b9f5c899c9cce38495d4419922539679bb92ad8d"),
            from_hex("0x103ef01c7c8768e785546e099df9d03632d65d23d9f94c3bb8cadb4fdfd1f367"),
            from_hex("0x660075effc8dae99502b1b5a27e5df4db6cf55597be89f08e5e62479ceafdea9"),
        ],
        vec![
            from_hex("0x5ef8f0775ae0432d874a0b39bf31ec37ccb2bdbb8cc60ac8c8280ec13c161bd1"),
            from_hex("0x2e57f220d0d344827291578b4cf9738782f6ee95fde8021cd0668cc243c74957"),
            from_hex("0x0083ba3a1e27588f5e7bcf349556dedd5a7b678a45c61935482034d76dcc7f94"),
        ],
        vec![
            from_hex("0x68da532ce1197837dd3be116579b50ea3cf04a83c05f283b761409ca97d2d1e2"),
            from_hex("0x250ca35b69c0adc3b56261a47fc7a1193d093e6b859b9c45e6bf1dea2faa623b"),
            from_hex("0x3990ef1da7f155c7f90ba7fd7f260cbbd031b1ae75ddebf2de0827fc0f1b1f3c"),
        ],
        vec![
            from_hex("0x0c14d4ccbb7832f2e39a6551015f1f56f4a4788df1102ffbce24f1f0fd1cc028"),
            from_hex("0x02f276ab6a075a9da67e20726ee9cbcb4c00876da103021a4553a3d86013b95a"),
            from_hex("0x4bc23de97778afccf3eab941e6bbb0fcf2d6c7516d9bccf0c39b150ea922df69"),
        ],
        vec![
            from_hex("0x4a440e1263a44b238165152ef44c27ab950f57698db19e656d2a3b41a4de6f09"),
            from_hex("0x6dd066079d336456096426680294f5fbbea1a08c52583e362ced779c06f1a372"),
            from_hex("0x06be5257bc3b73f8a9d8eb33ef70a9ebf6eacd94c62ba6c552c1acbb506305ca"),
        ],
        vec![
            from_hex("0x453c4ae08e77cea49a0d399ed8bd4971372e97e5b8627670669769d0d03301f3"),
            from_hex("0x49bca7dd947cc4d2aff3fda14f74f9e5c861bf6613381715c19408c0464e647e"),
            from_hex("0x429e71ece960261a53afd33cb401d2340cddf6242d51cd93f9f46b3bb9083227"),
        ],
        vec![
            from_hex("0x54b85a66072a3c1abea9473e1c64110dbfe15ba378defee12c405162055c0153"),
            from_hex("0x1641e8ffcceff15d2a1b61126d0f5a8e6a547e099efca74b865239941e0ce29d"),
            from_hex("0x0f130ddfdaa2489a19f1892b3f51344e6667d12677c7566d63be00a735946dbf"),
        ],
        vec![
            from_hex("0x41801cce4ff7c98025a3f556d792f51f9b4ae09805b388cfe1c104e967c8b2c9"),
            from_hex("0x5ead6f7d811b71b94282b089906621b7f932c3b2b9685de86d8a74ad78f95e6f"),
            from_hex("0x2a8a0d9341f75330d937936401cf0f36fb0e2c80e0180d51e07f7507214d5534"),
        ],
        vec![
            from_hex("0x5c5753de965ac7d34f5efc2e230a02f41f42d13c750d817231eb4cc4c86cb72c"),
            from_hex("0x2156af191023187ac70ceaef353f8fc06f4b78fb4e1d987a976ded455a1e2438"),
            from_hex("0x1faf7cd71fa18e7eba8e888486723da47acac38c21379b0714e0fcb3b435ef89"),
        ],
        vec![
            from_hex("0x2f9ad94cdc9597a11b500d1c8a48a70f068369134db88623a2ae786190796f66"),
            from_hex("0x0aa9701e9ab071702efde57c9ea3b2674fe9ae0ce0649f9c500ffe8299f54ad2"),
            from_hex("0x29c1c92fd32f543722089d35b0e0e4146c1f732d4ce21919cfca96ad08357d9f"),
        ],
        vec![
            from_hex("0x2ea45656d3d8e790665888fc89e5857bcacdd8df1b6d723d25eef58bd30f8558"),
            from_hex("0x0e08431273689242c516ae65b9a44d3e56c739268b9e70aa530bd27c8f264f18"),
            from_hex("0x19b93640f63362d3e08b341294bdccb2a7aab9033b481737139973949b2998ce"),
        ],
        vec![
            from_hex("0x39a079a60d0ff450f52d4f482afaa3976470e62976137711251bdbb25d98a5f3"),
            from_hex("0x7387b6ae30d8485acecde1d6631cf7404d526a170a8194032c69850f9279719f"),
            from_hex("0x69a16bce0fe73fdf65c4c11bef206a28291303129a785fc209bdffa4e788c746"),
        ],
        vec![
            from_hex("0x6417f7c17a335a7b112559154e8ee4a1b4c84ed0d4eba1fed0827cd9b1b6f168"),
            from_hex("0x2886d18b10e6c926c3cb9c9c16b785f68bb3b5e12210581ed2505dbbede8965e"),
            from_hex("0x4ad7522ba43507cc94d1fb5e3f5b5c73e4d882859b43be8b62ebc4093146dc99"),
        ],
        vec![
            from_hex("0x135c7f72b895513a3684966b040808252723adaecd2b2d2d44672016834f782d"),
            from_hex("0x1acb5fa1e1c9b698c2917961bc896ae1ceadce5813934d152de8d8ab3b80bd6b"),
            from_hex("0x4466df64c1cdce2f5ac7444798aab47fc7e52ccc0a44a8de65f3378555e747f1"),
        ],
        vec![
            from_hex("0x1ab0c9f310290df0e7865054ee4b4a78b938aa07fc457cddc7a7675a411a6fc2"),
            from_hex("0x6780f4f17721b2ae25a69544630d677ccb029302259aa0e4d20330e554648625"),
            from_hex("0x05f5225d8fedc1885a13fb8e68ca92d6c629a1f235e171ff4dcb427323b68da1"),
        ],
        vec![
            from_hex("0x3bbf64cb37d3626b231ef9bb6653fedaad65710d5906e73cee9daa313ef3c3c1"),
            from_hex("0x3ff036aefac928c16cb0b2edd62ddebbcad2d4bd398e2cb94243a925c32b046d"),
            from_hex("0x5e22bfa33bd03ebce51e5d3a94d03f527f8fed685c345792e2c21ed88dadf809"),
        ],
        vec![
            from_hex("0x54795e1111b786662c487a5390ed6dd5eedce373678cd0a9a154c7bc1695f11c"),
            from_hex("0x191fe0d97a253340e0390fdd0b2ab51bd328b2e02f10caef5d0a2cbc8e3dda8b"),
            from_hex("0x0585a9f9b01db9562b732cbb19ffa5fa3382ddf326189a29dd5c17b2eb2c4dff"),
        ],
        vec![
            from_hex("0x6fd6fa195f92c539911f551f25203f6267d79ea882e34e9e3085291e73bcf758"),
            from_hex("0x1660da7095fd0a813a57d049968f933c4927eb8846ef79330738bb182e443241"),
            from_hex("0x6a292747893bf1697a87fec46f56b9a03c419352488a280d840339ee3c344f06"),
        ],
        vec![
            from_hex("0x504a676815386196673b9a5e7b71578f6951eda6ff56efb5ff9762c2873a94ff"),
            from_hex("0x0c7022f52fe0da2c2427c90c85c657183fc20391ebe48d799b816b6fccacf2aa"),
            from_hex("0x6ce8f7bcfad437e20745bb9dd882dc51f860d856e3a56a9f82d69efc36ecde19"),
        ],
        vec![
            from_hex("0x0469a4e50041d08c266db6ab3ce8ee466e7258b3bf8747f4aebbfd5b5cf8b560"),
            from_hex("0x458e86e5b846e259ad8b8f9420ab245d32d68937af7624a328d081b423a85d01"),
            from_hex("0x00a12d81209a98dff4fccfdbd84355f94122a08fec68451a547ec89ccaffff11"),
        ],
        vec![
            from_hex("0x38e0f53b1cabd628ffe3d26cff699e3efe320f6698e56110818c29fa21716c01"),
            from_hex("0x6973d27c73688564912fd1b5ab2baac25dbbef765fdf13687cb5353f948b5e3c"),
            from_hex("0x0e2456fbdbad42eba28fefd2d6491c65d11ddd1f077edfc5c5bcfb9928c58d6b"),
        ],
        vec![
            from_hex("0x2a9a5a811cafc65601b9966e115a25402c63133716bc025f91d7c99921674f93"),
            from_hex("0x6f9cc43f2b9867ce0d5169cf209098f8d0ece387aabf1587d2a16725a9f3d5cb"),
            from_hex("0x6d74a5d4717714bbfe39f08c73be292f5b8d2b44d01184ac3ba0985c13cb497e"),
        ],
        vec![
            from_hex("0x3024cf4753aeacb4f9ebcaeaeb12fe7488614ec2b17d33f8e7e2853fff451d4d"),
            from_hex("0x6858fa70b4b0bc32cbc3c9bc066852c2f70f844061650fa10f4c2dbe2e73a830"),
            from_hex("0x31c7dbab8d38f8eee397e5729309b8dc591f6869809906883de6cebb8d3cdd10"),
        ],
        vec![
            from_hex("0x583abeefe435201daddcf8b44b827ada1b445b2e6eaae877d28e18962901bcc4"),
            from_hex("0x53bba68aca998c4db6c4c477770db24614be7c24d2494913095af784c6412f53"),
            from_hex("0x4048f366dc5ee465605d4e7c98b9196bd6d3dd7d287840edf1395f5642b83dc3"),
        ],
        vec![
            from_hex("0x182b0d5ce7a2dfc9af70dd9576dc4907cd4b3e22a1017f7b60b7085b0d0b4c20"),
            from_hex("0x6568df6c564a0e41fc7bc83d0a0db5eca5aee2dabd1ca1bd8a078bb57bda7667"),
            from_hex("0x36ea6937627f9d75e8e5759a8daea08ffb1ba59b581e5d113518d68a74b65136"),
        ],
    ];
    pub static ref POSEIDON_BLS_PARAMS: Arc<PoseidonParams<Scalar>> = Arc::new(
        PoseidonParams::new(3, 5, 8, 57, &MDS3, &RC3, &MDS3, &MDS3, &MDS3, &MDS3)
    );
}
