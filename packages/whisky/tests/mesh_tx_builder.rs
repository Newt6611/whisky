#[cfg(test)]
mod mesh_tx_builder_core_tests {
    use serde_json::{json, to_string};
    use sidan_csl_rs::model::{Asset, Budget};
    use whisky::{
        builder::{MeshTxBuilder, MeshTxBuilderParam, WData::JSON, WRedeemer},
        core::common::{builtin_byte_string, con_str0},
    };

    #[test]
    fn test_mesh_tx_builder_core() {
        MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
    }

    #[test]
    fn test_tx_in() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let asset = Asset::new_from_str("lovelace", "30000000");
        mesh.tx_in(
            "93fec6deaafabcc394a15552b57b1beca120d9ee90480d1e5cb42ff20118d40a",
            1,
            &[asset],
            "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6",
        );
    }

    #[test]
    fn test_script_tx_in() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let asset = Asset::new_from_str("lovelace", "30000000");

        let data = to_string(&json!({
            "constructor": 0,
            "fields": []
        }))
        .unwrap();

        mesh.spending_plutus_script_v2()
            .tx_in(
                "93fec6deaafabcc394a15552b57b1beca120d9ee90480d1e5cb42ff20118d40a",
                1,
                &[asset],
                "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6",
            )
            .spending_reference_tx_in_inline_datum_present()
            .spending_reference_tx_in_redeemer_value(&WRedeemer {
                data: JSON(data),
                ex_units: Budget {
                    mem: 3386819,
                    steps: 1048170931,
                },
            });
    }

    #[test]
    fn test_script_tx_in_with_datum_value() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let asset = Asset::new_from_str("lovelace", "30000000");

        let data = to_string(&json!({
            "constructor": 0,
            "fields": []
        }))
        .unwrap();

        mesh.spending_plutus_script_v2()
            .tx_in(
                "93fec6deaafabcc394a15552b57b1beca120d9ee90480d1e5cb42ff20118d40a",
                1,
                &[asset],
                "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6",
            )
            .tx_in_datum_value(&JSON(data.clone()))
            .spending_reference_tx_in_redeemer_value(&WRedeemer {
                data: JSON(data.clone()),
                ex_units: Budget {
                    mem: 3386819,
                    steps: 1048170931,
                },
            });
    }

    #[test]
    fn test_script_tx_in_with_ref_script() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let asset = Asset::new_from_str("lovelace", "30000000");

        let data = to_string(&json!({
            "constructor": 0,
            "fields": []
        }))
        .unwrap();

        mesh.spending_plutus_script_v2()
            .tx_in(
                "93fec6deaafabcc394a15552b57b1beca120d9ee90480d1e5cb42ff20118d40a",
                1,
                &[asset],
                "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6",
            )
            .spending_tx_in_reference(
                "bb712547a5abe3697f8aba72870e33a52fd2c0401715950197f9b7370d137998",
                0,
                "8be60057c65fbae6d5c0673f899fea68868b16aeba6ff06f2d7f3161",
                100,
            )
            .tx_in_datum_value(&JSON(data.clone()))
            .spending_reference_tx_in_redeemer_value(&WRedeemer {
                data: JSON(data.clone()),
                ex_units: Budget {
                    mem: 3386819,
                    steps: 1048170931,
                },
            });
    }

    #[test]
    fn test_script_tx_in_with_script_value() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let asset = Asset::new_from_str("lovelace", "30000000");

        let data = to_string(&json!({
            "constructor": 0,
            "fields": []
        }))
        .unwrap();

        let script_cbor = "5916465916430100003232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232222323232323232325335353357389211944617461206465636f646564207375636365737366756c6c790053335734608e609a004264646605e6eb4d5d08012999ab9a3049304f00113302f375a6ae84c138004cc0c9d71aba135744609c0020906ea8d5d09aba2001304c00204622350052253350082153355006213500122222222223500822533535533553353501522350022222222222223333500d205c205c205c233355306006603e2350012253355333573466e3cd4008198d40101984ccd5cd19b87350020683500406805e05f0671305000306700d2135001223500122223501c223500222222222222233355306306c2235002222253353304101800413306b006005100506100a153357389201024c660016130240252215335001100222130280292222533350020592133333335748002464a666ae68c180c1980044c8cccd55cf800902b1191999aab9f0012058233335573e6ae8800894cd4c0e4d5d08029099299a9999999aba40012325333573460d260de0022646666aae7c004817c8c8cccd55cf800903091999aab9f357440044a66a60846ae8401484c94cd4ccccccd5d20009032903291983c111999aab9f001230590672533530463574200442a66a60086ae8800c84c16ccc1b40080041c41c01c4dd60011032903283710982b1982b8018008361aba100406a206206b06a357440040d060dc0020ca6ea8008817081708170817019484c134cc13800c00418cd5d0802030902c8310309aba200205f306500105c375400440a640a640a640a60b840b4426a00244a66aa66a6a030446a004444444444444666aa60c40d0607a06846a00244666aa60ca0d6608006e46a00244666a0024660c69000000831119831800a400000266aa60c00d046a00244660a6004666a002466aa60c80d846a00244660ae004606e00200244666066088004002466aa60c80d846a00244660ae004607200200266605c07e010660c2660a0026660c2660a066e2807c051200205d05d05613357389201023341000501533553353304e015002135350142232323232323232323232323232323232323502722330570023330782225333573466e3c0052210006e13306b3307800200133003002333718004905f4d06000a41000266660f04444a66a00620044426a004446466e28cc1f800401ccdc599984080911299a801080091099299a999843809112999ab9a3371200490000999ab9a3084013370c002900203b03b89980199b8100248008cdc1800a40080060022002266e00004ccc21c048894ccd5cd19b8900248000520021337040026600666e04009200200100348010cc01400400c008cdc7003800999804802199b8c33700002900119b8133702016006900100399b8000348008cdc08010028019999aa98348379982b911180198010009000b8a00148810048000cccc1d488894cd400c54cd40041b0884cc1accc16801000c19c884d4008894ccd5cd19b870020061333008003006333307d222253350021330710030012215333573466e1c014008400c4ccc01801400400c00401401454ccd5cd19b8900200616153353335306f07406800506913330080070023306d00106913306d3305c0060053330080070023306d00106900153350011533573892010350543800162213500206b06133074225335001130023059062221533500110022213006305d00533500305e33064330533370601c900819b8600e48040cc190cc14ccdc1806a402066e180352010330643305333706018900819b8600c48040cc190cc14ccdc1805a402066e1802d2010330643305333706014900819b8600a48040cc190cc14ccdc1804a402066e180252010330643305333706010900819b8600848040cc190cc14ccdc1803a402066e1801d201033064330533370600c900819b8600648040cc190cc14ccdc1802a402066e180152010330643305333706008900819b8600448040180d4008888004d4004888008cc0c40c448888ccc01094cd40044c0d0488cc0081900048854cd40044c0d8488cc0080140048854ccd4cc1780100084ccc0180088cc1ac0140040044ccc01c008c0e0488cc0080180040044ccc0180088cc1ac0140040048894cd40044cc1a0cc1a000c008c0181908854ccd4cc1780140084cc1a8cc1a8014010c02000c4ccc01c008cc1a80140100044cc1a8cc1a8014010c02000c888c94cd40084c0040f88854ccd4cc17c0180084ccc01c0088c018cc1b001c0040044c00c1004ccc01c0088c018cc1b001c0040048cc1a4c00cc0dc488cc008018004c01c008ccc0c802d201600d33303100a48050030ccc0c0025201200b33302f00848040028ccc0b801d200e00933302d00648030020ccc0b0015200a00733302b00448020018ccc0a800d200600533302900248010010ccc0a000520020033728026446a00644a666ae68cdc38020010999a982c82f01a80180082e02c02b099ab9c49010233420005005815335738920102334300160481533550052135001222222222235006223500422533533061225335001056221350022235001222253353303100401513305b35008061300a0071300a00735016223500222222222222200c162215335001153355335533533306322253350011002221350022233007333306a22225335001100222135002225333573460d6002266601000e00c006266601000e660c42466600201000600400c00600400c00200609e6a0264444444444440100b04426a004444a66a0082a66a6606c0160062666ae68cdc3a40020020a80aa0ba440be0ac266ae71241023344000501533535013222222222222533533355306006603e25333573466e3c0400044c138004194010818c1901584cd5ce24810233450005005822160471120011533573892103505435001615335350022235002222222222222533533355304c05202a235001223500122225335333553053059302e032303002c335530510592350010570031304000705700b21303a35001052050215335350012222333553043049301e022302001c335530410492350010470031350012222533350020472133333335748002464a666ae68c138c1500044c8cccd55cf80090221191999aab9f001204623233335573e00240904646666aae7c00481288c8cccd55cf80090261191999aab9f001204e23233335573e00240a04646666aae7c00481488c8cccd55cf800902a1191999aab9f0012056233335573e6ae8800894cd4ccc0cc0d40d4d5d080a90a99a98191aba10142153353330350370373574202642a66a60686ae84048854cd4ccc0dc0e40e4d5d080890a99a981b1aba101021533533303903b03b3574201e42a66a607c6ae84038854cd4ccc0ec0f40f4d5d080690a99a981f1aba100c21305212333333333300100b00a00900800700600500400300206806706606506406306206106005f205706005f357440040ba6ae8800816cd5d100102c9aba2002057357440040aa6ae8800814cd5d10010289aba200204f3574400409a60a60020946ea80088104810481048104128812010c108dd500419ab9c4912353637269707420636f6e74657874206465636f646564207375636365737366756c6c7900533357346088609400826464642466002006004604a6ae84d5d118258032999ab9a3045304b001132323232323232323232323232323232323232323232321233333333333300101801601401201000e00c009007005003002303f357426ae88008ccc0e1d710009aba10013574400466606c06e40026ae84004d5d10011981fbae357420026ae8800d4ccd5cd182a182d00089919191909198008020012999ab9a3057305d0011323212330010030023030357426ae88c174008cc0c1d69aba1305c00105637546ae84d5d1182d801a999ab9a3055305b001132321233001003002302e357426ae88c16c008cc0b9d69aba1305a00105437546ae84c16400414cdd51aba10013574400466605e064eb4d5d08009aba20023303802f357420026ae88008ccc0add70151aba100135744004666052eb80a0d5d08009aba200233032024357420026ae88008cc0c0084d5d08009aba2304b0023302e01f3574260940020886ea8d5d0982480202199ab9c49011d52656465656d6572206465636f646564207375636365737366756c6c790053335734608800207e2a666ae68c10c0044c8488c00800cdd71aba13048002042304800137540066ea8004888ccd54c0d00f40d8cd54c0d40f48d400488cc0a0008c020004ccd54c0d00f488d4008894cd4ccd54c0ec104c0580688d400488cc028008014018400c4cc0e801000c0c8004cd54c0d40f48d400488cc0a0008cc124894cd40044c03000c884d4008894cd4cc03000802044888cc0080280104c01800c0100088cd40040cc0bc84888c00c01088c8c848ccc0048c00c888c00c0108c00c888c0080108c00c888c0040108d4c00800cd400c004c00800884888c004010888c8cdc199b803370466e380100052080043371c00866e000052002337069040400080119b820024801048cc004894cd400840040d40a894cd5ce2491b65787065637465642065786163746c79206f6e65206f7574707574001649888d400888d400c88c8cd40148cd401094ccd5cd19b8f00200100303820302335004203025333573466e3c00800400c0e054cd400c854cd400884cd40088cd40088cd40088cd40088cc05000800480cc8cd400880cc8cc0500080048880cc888cd401080cc8894ccd5cd19b8700600315333573466e1c0140084cc0cc0100040f40f40d854cd400480d80cc8ccccccd5d20009192999ab9a3035303b00113233335573e00240564646666aae7c00480b48cccd55cf9aba20022533530093574200a4264a66a6666666ae900048c8c94ccd5cd181f80089999aab9f30440022034233335573e6ae88c11400c8c94cd4ccccccd5d2000919192999ab9a3046001133335573e609400440744646666aae7c00480f08c8cccd55cf800901f11999aab9f357440044a66a603e6ae84c140020854cd4c080d5d080310a99a98109aba1005213033333036003002001049048047203f0480473574400408a6ae88c12c00c10c54ccd5cd182280089999aab9f304a002203a233335573e6ae88c12c00c94cd4c058d5d098260021098169816800821901d82202182098250009baa0022037203720372037040213028302800103e35742608c008406a07c07a2a666ae68c1000044cccd55cf98220011181281a101a01e81d98220009baa002203120312031203103a213022330230030010383574200806c405c06e06c6ae880080d0c0e80040c4dd5001101410141014101401891999999aba40012323253335734606c00226666aae7cc0e800880a88cccd55cf9aba2303b00325335300935742607800842603a603e00206640560680662a666ae68c0d40044cccd55cf981d001101511999aab9f3574460760064a66a60126ae84c0f001084c074c0740040cc80ac0d00cc0c4c0e8004dd500110139013901390138181111999999aba400123253335734606a60760022646666aae7c00480ac8c8cccd55cf800901691999aab9f357440044a66a60146ae84014854cd4c028d5d08021098109981100100081b81b101701b81b1aba2002034303a001031375400440504050405040500629111ca09fdd51688bf4095b25bed6b4ed916188e4d4c2cdf367aaaf4ac2460023333333574800240484048404840484602a6eb80080b48cc05088ccd400c0ac008004d40040ac8ccccccd5d2000901110111011118099bad002202202b22333573466e3c008004080084cc040894cd40088400c400407848cc004894cd400809840040788c94ccd5cd18158008120a999ab9a302a001026029302f3754002446464a666ae68c0b400406454ccd5cd18160008990911180180218021aba13030002153335734605600203405460600026ea80048c94ccd5cd1814181700089919091980080180118021aba135744605c00460166ae84c0b400409cdd50009192999ab9a3027302d00113232323232323232321233330010090070030023300b75c6ae84d5d10022999ab9a3030001132122230020043574260660042a666ae68c0bc0044c84888c004010dd71aba13033002153335734605c00203c05a60660026ea8d5d08009aba200233300875c00e6ae84004d5d11817001a999ab9a3028302e00113300e300a35742605a002660080126ae84d5d118168008139baa35742605800204c6ea800488c8c94ccd5cd18140008980698021aba1302d002153335734605200204804e605a0026ea8004cc005d73ad2223302c2233335573e0024036464660406601e600e605e002600c605c00260086ae8800cd5d08010129bab0012323253335734604e00226424444600800a60086ae84c0a400854ccd5cd181300089909111180100298029aba13029002153335734604a00226424444600200a600e6ae84c0a400854ccd5cd18120008990911118018029bae35742605200404660520026ea80048c8c94ccd5cd19b874803000444888806054ccd5cd19b874802800444888888801054ccd5cd19b87480200044c8c848888888cc004024020dd69aba13574460520066eb8d5d098140010a999ab9a3026001132321222222233002009008375c6ae84d5d118148019bae3574260500042a666ae68c0940044c8c848888888cc018024020dd71aba135744605200660086ae84c0a000854ccd5cd181200089909111111180380418021aba13028002153335734604600226424444444600a01060086ae84c0a0008088c0a0004dd5000919192999ab9a302300113233300b375a6ae84c0a000cdd69aba1001375a6ae84d5d10009aba2302700215333573460440022600e60086ae84c09c008084c09c004dd5000919192999ab9a302200113008375c6ae84c09800854ccd5cd1810800898031bae35742604c004040604c0026ea80048c94ccd5cd180f98128008991909198008018011bad357426ae88c094008c00cd5d0981200080f1baa00123253335734603c604800226eb8d5d0981180080e9baa001212230020032212330010030022122300100322212233300100500400322330202233335573e002401e466026600a6ae84008c00cd5d100100c9bac001301d22533500101222153350011003221330133330232225335002100122153350031004221533353300e00400213301a0043300700300513301a0023300700600113301a0043300700300500400230060012235002223500323225333533300800900500313330080090040010020021333006007003001235001222222220071233333333001006225333573466e1c00800401854ccd5cd19b8900200100400522333573466e2000800402402888ccd5cd19b8900200100900a22333573466e2400800402802488ccd5cd19b8800200100a009225333573466e2400800440044008894ccd5cd19b890020011002100111222001112220021100122200322333573466e1c00800400c01040204024c04888448894cd40044008884cc014008ccd54c01c040014010004c0448844894cd4004020884cc01cc010008cd54c01803801000488488cc00401000cc03c88448894cd40044d400c028884ccd4014028c010008ccd54c01c034014010004c0388848894cd400854cd4004018880288854cd400c0288854cd4cc01c0100084ccd4c02403801c00c004030440048800844004880044800454cd5ce249035054310016370e90001b8748008dc3a40086e1d2006371890002ab9e5573a464600200244660066004004003";
        mesh.spending_plutus_script_v2()
            .tx_in(
                "93fec6deaafabcc394a15552b57b1beca120d9ee90480d1e5cb42ff20118d40a",
                1,
                &[asset],
                "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6",
            )
            .tx_in_script(script_cbor)
            .tx_in_datum_value(&JSON(data.clone()))
            .spending_reference_tx_in_redeemer_value(&WRedeemer {
                data: JSON(data.clone()),
                ex_units: Budget {
                    mem: 3386819,
                    steps: 1048170931,
                },
            });
    }

    #[test]
    fn test_read_only_tx_in_reference() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let asset = Asset::new_from_str("lovelace", "30000000");

        let data = to_string(&json!({
            "constructor": 0,
            "fields": []
        }))
        .unwrap();

        mesh.spending_plutus_script_v2()
            .tx_in(
                "93fec6deaafabcc394a15552b57b1beca120d9ee90480d1e5cb42ff20118d40a",
                1,
                &[asset],
                "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6",
            )
            .spending_tx_in_reference(
                "bb712547a5abe3697f8aba72870e33a52fd2c0401715950197f9b7370d137998",
                0,
                "8be60057c65fbae6d5c0673f899fea68868b16aeba6ff06f2d7f3161",
                100,
            )
            .tx_in_datum_value(&JSON(data.clone()))
            .spending_reference_tx_in_redeemer_value(&WRedeemer {
                data: JSON(data.clone()),
                ex_units: Budget {
                    mem: 3386819,
                    steps: 1048170931,
                },
            })
            .read_only_tx_in_reference(
                "8b7ea04a142933b3d8005bf98be906bdba10978891593b383deac933497e2ea7",
                1,
            );
    }

    #[test]
    fn test_mint() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        mesh.tx_in(
            "fc1c806abc9981f4bee2ce259f61578c3341012f3d04f22e82e7e40c7e7e3c3c",
            3,
            &[Asset::new_from_str("lovelace", "9692479606")],
            "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
        )
        .mint_plutus_script_v2()
        .mint(
            1,
            "baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc81700",
            "test",
        )
        .mint_tx_in_reference(
            "63210437b543c8a11afbbc6765aa205eb2733cb74e2805afd4c1c8cb72bd8e22",
            0,
            "baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc81700",
            100,
        )
        .mint_redeemer_value(&WRedeemer {
            data: JSON(con_str0(json!([builtin_byte_string("1234abcd")])).to_string()),
            ex_units: Budget {
                mem: 3386819,
                steps: 1048170931,
            },
        });
    }
}
