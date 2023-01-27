use super::poseidon2_params::Poseidon2Params;
use crate::fields::bn256::FpBN256;
use crate::fields::utils::from_hex;

use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = FpBN256;

lazy_static! {
    pub static ref MAT_DIAG3_M_1: Vec<Scalar> = vec![
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001"),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001"),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000002"),
    ];

    pub static ref MAT_INTERNAL3: Vec<Vec<Scalar>> = vec![
        vec![from_hex("0x0000000000000000000000000000000000000000000000000000000000000002"),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001"),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001"),
        ],
        vec![from_hex("0x0000000000000000000000000000000000000000000000000000000000000001"),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000002"),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001"),
        ],
        vec![from_hex("0x0000000000000000000000000000000000000000000000000000000000000001"),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001"),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000003"),
        ],
    ];
    
    pub static ref RC3: Vec<Vec<Scalar>> = vec![
        vec![from_hex("0x2b149bee29246b2661a408d4cc9742d81dcadd4f169e3b741c64e11a27b5309d"),
        from_hex("0x1cd2b41e2e56eccf053e7644c63293449620fa6b1f744e4970d30de26b42a2cc"),
        from_hex("0x07d112dd77cf168e92a068a76744c93e493ded90272a951eeddfcc2dbe9d0dee"),
        ],
        vec![from_hex("0x1d29689f35a0ddfdd6d3e8858754a29ac7cd97d6bb8a4c8712b7cbe8cc94852b"),
        from_hex("0x14ba68d571feecece470e3fd915e9defbfd450b439437a03c64a9e514268c999"),
        from_hex("0x23b914cef9cbdc2da4ee0465c891c2833abe7daab4821d0a27316b763d2b059e"),
        ],
        vec![from_hex("0x0794cd048294dcd0a5eb22d63a0e75b4e2840734e76be0362347c0a030fa6606"),
        from_hex("0x2232d139172b9537cff0c67a39432af4465679ed7c533622f8805f189ecf29a2"),
        from_hex("0x021c92cb42f87adb66463fbf894a1ae39290153e62f1e12173e48dd66a488904"),
        ],
        vec![from_hex("0x2c71511891be78ee73d91a257c32daa0192ac8aaaa13c58740ac488c352680aa"),
        from_hex("0x2cb8e266669d27fe161a4616b6973cd498a75ed87d088c2fca679e4088a9acd7"),
        from_hex("0x026fce415d2cbadfd9dc6974bcf6a3203ab3573e1d78507158291ee70bc54085"),
        ],
        vec![from_hex("0x12b775a70049dac8e04d358d4b7c2e6ca6ad69816502de10a630680d3356ebbd"),
        from_hex("0x29b8f8e01ad9c6695c20ecc242fe5513359b864e6b35027b426fcde3cce04d1d"),
        from_hex("0x21d1d3a45510c943674e3ddd80dae7d832b948f3733a388d786fa5718acf2c41"),
        ],
        vec![from_hex("0x1f1ce33a66fc0f1300c9260e7c071ae8bd5fa8fe8bab84c321fa9351970887b0"),
        from_hex("0x273f542c4636970876b0fa898346bc932e7925f8ad58208e5ae1e2bfbd7cbe53"),
        from_hex("0x2b9c9d25d34ac23481aa1d32b0bc79266ef4b025d1c30019006b5a4728252594"),
        ],
        vec![from_hex("0x0bf47dc5955752a1af9f125b878defca7f85c35df551b31886f00a3b81fec765"),
        from_hex("0x1b8db70f8c96217672cfdb11ec4bf23ea88708a08229a2e606d6a9ade68b0df3"),
        from_hex("0x0be2fe2351d7cd24055ee4a8b1593d6d8471941dad5cf08d2b1bf358952caea1"),
        ],
        vec![from_hex("0x13aef9d2bec458e3f6d254b7461a7d19d50bdc7b1953ac3aaa8dd7f335cb4c3f"),
        from_hex("0x02acbad4780cfeee2fc1204d6a8320b685b54e014a02d32a11adf4a5377d26b0"),
        from_hex("0x1240637c372623453435c28bb66941cde05a7ddd9eb132268d964ad74dafb84e"),
        ],
        vec![from_hex("0x005bc31fbd4fd878b89beb885da1c928af0154e9f3afe96c6e37fec09db18287"),
        from_hex("0x18b400374be5cf395a122fb0cc138e9961f7de85052319ff8c2bd9531cf483de"),
        from_hex("0x0f73d5fb9b5ccab20f55b1e765cdaed3e14c105620345a33f0ad8d7aa829d4fa"),
        ],
        vec![from_hex("0x286f4a3789f5a544e25a7cd50386be281bfe6a5fccb8194366f8c6fb66e0ad51"),
        from_hex("0x0f410e95849a2af1ffccfc0e061234cfab325aaa39cef3165faafe4c2e4cc4d3"),
        from_hex("0x20b7319df0195ff417d53f50318d56bfd9f5551d218abb574ede8bde284be1b2"),
        ],
        vec![from_hex("0x2efa517786ea832366855a88341b90cefbfc77239d6b6c8b731ef8ee93ac5840"),
        from_hex("0x0cc9c3e5d6464d5eed2104724596ce2797d0034e9cabf2129c10991857283b33"),
        from_hex("0x0c8f00fb53eeae79054dace85b413e1ea360f8db42a3c021b2606e7cf1171ef6"),
        ],
        vec![from_hex("0x29f16ebbed7d3341f2e32d28944b6bfc3eb43d181c8ea929e8febde3ad314419"),
        from_hex("0x03efe1ff07653b96fd624d6c36924ad2228f29a30ce57ec47db45ec1e5d5f857"),
        from_hex("0x19297309218f7b144a71410d101c635f364b4c9e14024a67fb80db254add3fb7"),
        ],
        vec![from_hex("0x0141a3a89bce0e1ea36eed09e1ccd4ba03e89160e4a0489c6b52f2b6ea8ded48"),
        from_hex("0x031e4bdd1c73a4de529914c2a7ddeb7863c94ee747a687c9b06d2b338889760f"),
        from_hex("0x115d4dc1433afe12343c1353bb6705417bff6f4ae6e2a79de91f600d729098be"),
        ],
        vec![from_hex("0x1e6400b97933b663c3d826de3026c5cda9e46a96273d357b11f4afff418e5368"),
        from_hex("0x108ec7f5dcaf96a33736703aeeb13a1e1578b32e3d3fb20ca0a44b59647f8dc8"),
        from_hex("0x1a7daa41a83f73e9e738707617cacd808c9a6dcb1b4c1562a85413f4d4eab2b3"),
        ],
        vec![from_hex("0x1944671516c9a421087502ed882cdc18568b99f3f43cf81fe3a911601ec184d3"),
        from_hex("0x172b235153f3562bac9ae7f64606d6ec13ac28b856c37dce0acd4fd72b48a3a5"),
        from_hex("0x0eafbc94639b36aeffe967101a261ce45c5f30d3968b2b7831017dda5c4ef992"),
        ],
        vec![from_hex("0x12d0693c6ab63b2d904647cfa90fb0ead5e70dc6a71a7a4de007734f0074e36b"),
        from_hex("0x0ccedab3824c44cb26c0a1e3adb4d361f56d0bc571470e8fca3fa8a2809ad8f5"),
        from_hex("0x08352cd586bc904116aa5e8c895114e2e2ceb356a295973e9d49aed344d0ca07"),
        ],
        vec![from_hex("0x245bda9bcd59c13e59092e64a3a96b477dd19b09a1b19c17d236673494363581"),
        from_hex("0x2ebdba8dd58b83bba83f487de5ad6f3453681c1714a6583ae9187792624cd71e"),
        from_hex("0x0756973d00f43609bde9df1b0a63a8382ced1163a33529c35b369d593f8383c1"),
        ],
        vec![from_hex("0x2d362bff73b6581513958f53bd65963e321ad3e6a3bc31cf6253fc5a73c01f93"),
        from_hex("0x1bf6b111b09e31b0d344c20963e40f6ea1f3c0043b548b69dae905ddecf80410"),
        from_hex("0x186e393d9f668b4f86848033e4c28c79202fd7de2b37dadd694e8f4ccd2b8db2"),
        ],
        vec![from_hex("0x195930897c9e66f6d09b89b8486b69cc394eab46e7c5b87bedaa5c7294dbd451"),
        from_hex("0x2132799ffb54ca5707c35dfe270c2cbccddb8be54f86fd44d5eba7d919bc0be3"),
        from_hex("0x238435e8c2a2d48f2347ae7c9778d1e9decb149d9b1dd87fbd15d5681f592b7b"),
        ],
        vec![from_hex("0x1a8e10afecf14412d0ba144d5057e382bfebdc62ccbf696170c1f0ccf561bcbe"),
        from_hex("0x0d11cc9f45cee15355712396831988b3ff3a312c18e884dd4b130bfc65230890"),
        from_hex("0x026124f5486bf534054b288560726aa949ee15d943a0b29d4b8d01920d46262d"),
        ],
        vec![from_hex("0x2e57241ffc3c8440c9b10c423f2db2d962f54d24eecd1e772f9a028447e93312"),
        from_hex("0x283346536c4847328045ce08b3d2ac822528d1b3f424b490efba24ac8949c9e5"),
        from_hex("0x10430984596462e05111794d71177c6cb21a06540c6642a90d8e00fe6d91b22c"),
        ],
        vec![from_hex("0x1dc54b406d2f2f424074ae4cce6ec03ae4fa8da5656290d3fdd4e64761e5f8ea"),
        from_hex("0x10655b9b66305cbe5568fef3d1caaa0f93e64260b0eb32d6764f1997114c6927"),
        from_hex("0x234f6e54c276767ff668f2f43654a13034132beadf17cfc695f0720fd67362b0"),
        ],
        vec![from_hex("0x11c5ad0f24d30e2872924ab57b06a988bb68d70a4e4c089c9bb13bea0326fe3b"),
        from_hex("0x2103957c2e782f4ac3e7f191257b0b8fca2770a44262ac4e8afb6ef59dc19c80"),
        from_hex("0x0d3d2afd4f878b328799d0511b6cb8613ff115708275658553803a50896b7165"),
        ],
        vec![from_hex("0x223d4bacccb09e618d0e0229ee69445c9e4a640c24ea640837c21fba6d91b75d"),
        from_hex("0x03c0e4df6338b01a12fb491c38dfff7912f2081b614d93d8d998d5d93faa64b0"),
        from_hex("0x24936c0a68fe6cd4cb16158fe1efaa999c0088fa64f5e7684b9ebb343e597403"),
        ],
        vec![from_hex("0x18e0cf8852a8df77887c1dd31cdc9a5cac626aa12d0e42476a6dd62bb82d619c"),
        from_hex("0x0f4e18d6dc7384087c2ae4d9aa57c4bb7859262268514d655ea3980b28b769d8"),
        from_hex("0x26ce01fac6806525b3202c4426b67e275af0e97bbc77ea25aecd7766db66b399"),
        ],
        vec![from_hex("0x127bb5fbbb7e48eb0235324212b619c0715599e570105b9d99ec96c3a42a47bf"),
        from_hex("0x2c0413553f2307aadf4f9cf23eefc1a2818a7ef68647647afe9ed1394bb00b75"),
        from_hex("0x09eec9bf2355771227c986772750e9c36c821433a60272d50690da23e999072e"),
        ],
        vec![from_hex("0x20df71f2ff7f8807e703a8766efad340e62d24c7be8594c2d4f7ec186ec0cc10"),
        from_hex("0x1ce9b1cd55cab33a3379c6f0c17136565a37bd2c0ea27aef5cc2f457a7738a3d"),
        from_hex("0x10cac56db45af1e78e52b37ae3f4306818c53fe82d67a6027739c87aa0d4b194"),
        ],
        vec![from_hex("0x0ecc99e479baf18d7bdb5b4f796e675facb6f79f0d0a9b1a475c6af182c27546"),
        from_hex("0x2c587a7a247c917db33d0d099991fd99114f2016b566a43f0c4ea58cf50d40ec"),
        from_hex("0x2f276d75477e0af076f693b1b5fcecebf9df784fd06fae5547bcfd49ea185c87"),
        ],
        vec![from_hex("0x0b8a8348d338bbc854d5aadb898f7a55c61f9325c1521bff1332ee7d75532bad"),
        from_hex("0x02c4549075997626fa4b6945e04f6ba773d80f1a4f7c6a6f93e879baca1bec7b"),
        from_hex("0x15016548171b3ea3cbb688c75312577227d4a8acc6b27f76a65fe8be162745a0"),
        ],
        vec![from_hex("0x287c1e3e95ae1f273d5c513f2bbb4a65c1d09607411356413fde27aafcf53cdc"),
        from_hex("0x0ed668511750862381afd781078ee0697cf3daa9e839274f4b4682e8108d0878"),
        from_hex("0x013a1d88c6c28fcd13978d517406a97090d85fcd2bbf4efaa35877ff10445a3a"),
        ],
        vec![from_hex("0x09e496649d1a01185f68183da808ad11c3400d515ee0cdc821953ca5253ee81d"),
        from_hex("0x08bcb31b45f1b2c8eb4d3ce77bbb754c73f3e50afe93e942cc750d71de1ebb93"),
        from_hex("0x00f328ccd3ea8bced27e2369192b2bd54e2c776572e6bebf642a8f49aaeb3bba"),
        ],
        vec![from_hex("0x18226b27d3cf150d6d8ca6314058de779228eb38adedf7c391b4fb60c2114988"),
        from_hex("0x00934d32de2e0aaf2566de933c2f0ce4826309be7cae2548d12606c929a5e39f"),
        from_hex("0x2a2e2df8520dccb9a83031cb4f3859afe10890ddb3bd9421f63a2b4a50928354"),
        ],
        vec![from_hex("0x048933838062747921555d7bb13ac470d66e5a6b41e6a98f63fa0592b1a0e23a"),
        from_hex("0x1e63f2e2e2ef02d9a6981cd4ada263aaa3b04ec77c8686fc710ee39f3256f3c6"),
        from_hex("0x28e4bdba6de3cfce9dc16b06d0147a8646d2d05667b50a2a1df26b3c59ba2c4d"),
        ],
        vec![from_hex("0x22035fef7737851c20652a150281f4b8a72885031d374f8fa0189b99f3f5fd2b"),
        from_hex("0x07c57a0025dcc723083e0140dbf6e5aa70558aacca01593bd3845e3309a9425c"),
        from_hex("0x2cb343b9142be92d413a1b35bcf7de029c8f1f991260ce047e5b76e734a14b56"),
        ],
        vec![from_hex("0x207a607bc13271e6f165f58f0b3a35ac6c9ac6c59ec750a50d28a517a3324459"),
        from_hex("0x2e7454e37b235cfc45c364a132153db71adbc726b8ac8d7b0cd18faf445a72fd"),
        from_hex("0x04860adb476613c2353f4206d82f58fcc6df9d7b49f1cd52dd7785e38140e5b9"),
        ],
        vec![from_hex("0x104bb09cfcc468901299793ffd1dcd01c7b985a24c25fbc32f50b54961855cde"),
        from_hex("0x0248bb3186c1d9a957fee479d94d2927601bf99240a07d9d7ac9f3e179cd0417"),
        from_hex("0x1d9a07bea9980b70f6c8c7d6aeef65d442816998396d3c1c7d371c6fbb368a2d"),
        ],
        vec![from_hex("0x288e334fefbf958803e5f1a4b7d2ff36ceda1a3f6beaae174657dee77f42b37e"),
        from_hex("0x2a62369f6642e58f4992b192af234550fc3eba43f15580b55f62432b40a3a402"),
        from_hex("0x1e5d7df998e5041015380473b1a83596ca4a5c764da40810df620a4b644a90f6"),
        ],
        vec![from_hex("0x014024729f267392e3cde38b6c4e231101ae822116afc7e49902656813acea7b"),
        from_hex("0x04746429a4c43d6cfdebff58a3d24c958cc1546ffce476c6e19e56f1dc2ce0ac"),
        from_hex("0x10b478f67d4a67509f896b8060af0f258f336e39fae79c691df19e9742d537f5"),
        ],
        vec![from_hex("0x13c127251227cc112da0dd02a47ea2c9155ce6bca88918aa4c88021f5848acb9"),
        from_hex("0x1889e993b6cbfa276780d8605593a84cfea941c1230199ebeb420e97c8128f50"),
        from_hex("0x213859ec3d6cb453110a15a594b52998ad3f6d919b8089934bfd614e18726fb2"),
        ],
        vec![from_hex("0x2348090c67ecb16f49192404716f947224572e27880d3cec9233f72029beb437"),
        from_hex("0x0c550feda9bb519e996996646b213d2ff9e6c039f3ed7ce4811c4636502e1e69"),
        from_hex("0x01398f371233b6a2495126c2ffc72f3f94191139c49dcba2d54a3e0d34f6859d"),
        ],
        vec![from_hex("0x1e36c510395e83781b6194ce24864dd4b36c42e9b2b9b4c8008d873425e0c1e7"),
        from_hex("0x2cb142c1de2b2429cfe9507c5a8fc70dfce573e362ff41290cabcec53d1ef9a5"),
        from_hex("0x1bc3d85b880d5bbf50ea931fb4f0b50ff25cdbe6296a5a88625f03858856d80e"),
        ],
        vec![from_hex("0x08419c1c3360bb9dc8329fb2b5adf6973deee2b515786efef8c5567e67031119"),
        from_hex("0x186a6a5c77968d91ea8492f62ecdd49b69bea9d4d61353b38d687df45216d5fc"),
        from_hex("0x2852732f61313754643f065b97744e1e799bc29d98c8b66f8452abcee580171c"),
        ],
        vec![from_hex("0x125bbc048b3a4ed00800f474398a2d5d29a2ad8d554cd001b1c7e7d36e5f498f"),
        from_hex("0x2986f6db4e0e3550290ba0724ccdb0546ce1f23a8a51915ca25405a692ce1727"),
        from_hex("0x264b51221311447d2e1c469626de96dc497e10daa8410b09d435b8ecdb828b81"),
        ],
        vec![from_hex("0x27e9e2e796298eb1ef66e266e770b48f1c91d62aaf6e31ce577033bc78ba787e"),
        from_hex("0x2031e52e8599127b9d64d1435188586d1b6d3ec95012fe81cf544d6bb471aa7f"),
        from_hex("0x2cc0d543a15b671e4388ae49b81ef9d12eed006ef05d9527c93b18620b425ed9"),
        ],
        vec![from_hex("0x16985b461d5f9bce9ea9a7483d2c600c19bb9eba2602a4c4150205a27744a8bd"),
        from_hex("0x22cc4bcfa003868a8d0abd581d851016a60ceedd0bd9ada05d42b32445702462"),
        from_hex("0x299c63b08886408ddae189d8456a5f2706faed81eeb64cbb6486fe5fb2563698"),
        ],
        vec![from_hex("0x24b5ad8def5218c60a246dd6f06d4fa85f9b149aba2765d0c465f5136b39223a"),
        from_hex("0x0d0a42eb352568f65725d1c2b8da3e4219386cc86c78cb660bfcdfcb07d3b767"),
        from_hex("0x03417dcd9761c546ef248d3a52eac5d63818e92e5b0e65480d809b51d292aaf0"),
        ],
        vec![from_hex("0x2c33367a42fd6e153f087da4c11a5d352c7f0679fb2171d52ab0bb3f12fcf865"),
        from_hex("0x2c2ac89913f1bb1f31a9c954a877d508097f4c659de48af7d74db41d9ffdf9e8"),
        from_hex("0x1a86905259e2517c5e5b5a47e42afc79e56a8f12ba3f00fc6f71a39068405a38"),
        ],
        vec![from_hex("0x191bd530b5dc0633d2b188cbc88c17678179dbb24ef18a12eff989d3f5682109"),
        from_hex("0x1309e0d7e9b3375517509636ca2965edac8ecf95becfbaa1a284f0258eabfded"),
        from_hex("0x10b6a6be8e5a21768650ef508c33629bc6dfba773d9b8c4e853f9ac131557121"),
        ],
        vec![from_hex("0x2fa530b290a4be33bdbc9bb84ac854ba16c5d4df56e16db5847abb582b28bbb4"),
        from_hex("0x2354b60c4423da9a05ef4d47837b5ffa9a972e44bce25c703c51a8a76e681de4"),
        from_hex("0x1021784644e92e0c1d0da144f7434cd8fe4a3ffefe13ce4c86af7d34c9291e5e"),
        ],
        vec![from_hex("0x14d04c9d854bde0f2371fa5d08ebc338cd8463a58392672be44380ef2d2be22f"),
        from_hex("0x2eb9b2a11c1cb01d0e2ebeca09cc7b4d804aa2798f56fb6cfa45720f333b601c"),
        from_hex("0x22bf31ac4672a4cf48cb0519251bb1354d5b26a5b8d4c2deb2db426def816d36"),
        ],
        vec![from_hex("0x1ca104345ea47e5b2af3f204cdcf17c6a6a69a43b718af18a9ba105bd88fd29a"),
        from_hex("0x03ea2fd4e151c3297d62c3b93c23d7c71e46fc4725342adcfe9ca4cdd9fc4ca0"),
        from_hex("0x13689cc750829f7e767409c37d94a8eea3208c8ffdfe05116c340f9ad28b9ea3"),
        ],
        vec![from_hex("0x05201fe9b61bbcd92de2afe5208204f26d36a8236e46a5060658aa67b6bb2305"),
        from_hex("0x211fe45dbc27073bf8ce43b22fc39e2597e1c7918f1f6280ae20f5c9aa08d669"),
        from_hex("0x043b2ae7f42a64bc48f377739e51ebafccf96553045cf36dc98f3336d4585240"),
        ],
        vec![from_hex("0x01094fe9e0e81cf91ce25989cb04b6aa48cdb204fdb53c3bd462e4e018105e0a"),
        from_hex("0x23fb8fcbab80da82e73c2ce03e93c1d34e1c6bfb93b460829c2b441ba5af1538"),
        from_hex("0x04f3638f7125a82e667eff9235d1c10df4b5983f2473eb5ebddce08b6c35d5a2"),
        ],
        vec![from_hex("0x1460a6e770e5f6dc3071093f86e17c875b9461706e3a5caf609816bdfa938bac"),
        from_hex("0x1d20f2797a2a5c801aeb9e2ae105c852a47fa349ee7b10f64640aad41ebf12aa"),
        from_hex("0x025512618578e295a09e06482ff9f928eaf6ac01f06b0c392e2ad5ef6f5c226b"),
        ],
        vec![from_hex("0x1272f6ab38955a0e643440e995fe933297b5c6503b09bf1b97d1e1634f2329f4"),
        from_hex("0x06d90516d82a696c5dc3dbec7c4fde33d8c90cb6401b836565ac8c962eb239da"),
        from_hex("0x11fb73034401b7d98fb43b5a00b26344bd2cc45451c40f521783606c6fa89418"),
        ],
        vec![from_hex("0x25228dc7ff3560602ab004f406d32e3399bda278e146d32bf9e02f3621639a73"),
        from_hex("0x14502070df48ef23a6db6af84da7327cf10e818e7358d467ff172d351bdc9d96"),
        from_hex("0x21320cddcf586ae02e96e7f4976f4345571a27ad9f803710e1de3baf5cc1bf89"),
        ],
        vec![from_hex("0x0f7b9555a972bb65cabc43b1a478942589ae6af7ee898ae77c61fc32803f696f"),
        from_hex("0x223eb16f08fa92fef07461f9d15bc6112fe4e59389297d6f64eb6747b50b51d5"),
        from_hex("0x10b667024f54d769b2a2d03c460afda55f85d33dca6e74a55b214969e7af2128"),
        ],
        vec![from_hex("0x1e07519c0cbccc13b9bbe28abc98acd48d4a4333e0b59a0830fe03e9438c1f94"),
        from_hex("0x0ff81bff9cf2461a46100f190f81b9ab9a882612da85152c9e3aa59d58081161"),
        from_hex("0x09dfdf41f12801a9b1722623c422cf0e3a441bfb680fba3b58076f07c5c8518e"),
        ],
        vec![from_hex("0x2bffec21f321b2910f154c39ee7b68967279b6e21bb2e71119f90527aec531cf"),
        from_hex("0x238d14f479c8fc5731356be9d3210c4aa209625298dff273ea7e06e4000c6c6e"),
        from_hex("0x18db91c87d15277b1baa2df09b6ed2d6eee6f5c6465f3533baca32695ad69d07"),
        ],
        vec![from_hex("0x22b12b47214b0b394be0c27278cb3f134b9ad1824e0dcef34de40b50ad1c61fe"),
        from_hex("0x18093c5b371688d824adf8f4ed945e3aeb1e61fc45b2c6de5c0f1c0499b69ac7"),
        from_hex("0x2d622a8ec76cfd1e345797bf1f41d3524911753b9eeff5b162b19fa743678abd"),
        ],
        vec![from_hex("0x1548f60fef00ff6f79add2a7bca2a164f67e9197a580224a45c6c4886f30e449"),
        from_hex("0x2b3dd5cf48e7fed93883a7980a5b43953218933b0f594847a199bb9dca025f80"),
        from_hex("0x012f4c0e61d1e99c57aabccfa5d25cb454ff9ba618b3dddef5d5b0d58b433a43"),
        ],
        vec![from_hex("0x0611ede3440b2990c430819a1b38282534c31fcddd39b010c701ccd634cf7c2b"),
        from_hex("0x12c844ada0a9233676a6534e69865290ab02e1aa0210094130b8a4bbdd5c5125"),
        from_hex("0x0532450255f81e6d5c2c1be73eceb37627b98b48d4cfbc07f112996f18417d64"),
        ],
        vec![from_hex("0x081e0a742f90abd5859e040c73b7d079e6cfdbc2655eeadad23402f62e4873e5"),
        from_hex("0x0602a33f8b9c022451d0d2658218049f88f3794931a2f0d9ccf14de310fa7e23"),
        from_hex("0x2252925173c3ac3a1ace47387f9357a75da225854b1372c7d0173f6e6107c0c3"),
        ],
        vec![from_hex("0x25cd5ed66d1880ed0da78b793f559c96b278cdeff655c7b6fd0a0be90eb6b723"),
        from_hex("0x1ec8331f9cdf4a0da88f86fc24d4d3f0b6c15881aa79a48e0c6a993e5c0ac7cc"),
        from_hex("0x0f140b183d4060fe0e9315e0d02ac7ecd8bfbba67f240f2b1a741c676dbf55f1"),
        ],
        vec![from_hex("0x2bf24dffaad7444c5a0eb0cace067f73c5affc1620a7dfa8b6308772d9226c64"),
        from_hex("0x17b92957bff045bf2c6bb48629d2dd2c2b8d1dfa5706dbd5f19c78189737bb8f"),
        from_hex("0x2f1dfe0d41a67ddf8032e11e5bb553670997440545494079c7e5e630a067f07b"),
        ],
    ];
    
    pub static ref POSEIDON2_BN256_PARAMS: Arc<Poseidon2Params<Scalar>> = Arc::new(Poseidon2Params::new(3, 5, 8, 57, &MAT_DIAG3_M_1, &MAT_INTERNAL3, &RC3));
}