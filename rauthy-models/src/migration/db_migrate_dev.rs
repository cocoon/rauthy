use crate::app_state::DbPool;
use crate::entity::jwk::Jwk;
use crate::entity::magic_links::{MagicLink, MagicLinkUsage};
use rauthy_common::error_response::ErrorResponse;
use std::ops::Add;
use time::OffsetDateTime;
use tracing::warn;

pub async fn migrate_dev_data(db: &DbPool) -> Result<(), ErrorResponse> {
    warn!("Migrating DEV DATA - DO NOT USE IN PRODUCTION!");

    let needs_migration = match sqlx::query_as::<_, Jwk>("select * from jwks")
        .fetch_all(db)
        .await
    {
        Ok(res) => res.is_empty(),
        Err(_) => true,
    };
    if !needs_migration {
        return Ok(());
    }

    let rs256hex = "1800000000000000535379694751507877334f57764550485076774c57617a322c9431640000000000000000100000000000000062564379547347616767567935797151dc040000000000006ebf6f9bbd6a1975d6d03f31ef8670edd2604bd565dd3aaae599d4c28f8e4a534eb1498a23b537b7ddea6cd1487020fa218810595011541a32932ee6f5aecb76069e44c6f3ae14807f670cff2c25bcb5f25432d300ef464e20022bef5efb3f5f50132b0ac2d54830e971a87f3907637d8c6b52b208ddbb83ccbcfe4a748092c6fc0187bfc5fc8d1dcd8aff98a0b6563bd2a4444af7f1f8185c5dd51bf824849d0fc9e174d2281f341b43963e2893cd28a3edd842528e16fb4612e63c0a847c6fde1ed55b4f545e9e345dc12c06ce9f98bf59e83e4730e7c4ff4eba3c281df7b33f358adf72423c12d8ad102e197075c13715670902b61b24d2dec2803d66f8a48764975d610c99de6d9251c31761bd55994e0bb33e5d3aa4076f1a2ec7d3ff4b9bd97deda18b1d9f8c4c195e199722b21af6fb53a02712b3f89734315c785680d203bc38aa9c849a6d90be880e35e97037ed74ec82d30031b68134cec355ce41821bda355bd99e190c9f06ef0c972f6d990850a22b34f7bf654552a711c715ace71162ea11a4581f1b95268de340d1edc353ead2dac1863d1f7ce8635f321a172848d8a3b6bd3b31db169369cc8e40809ace1960792361391332208e5269ed46b94d1f62db82bac26ed2a461e43b085ade2032cfa8e9733471f7c86fe54cc2bfac9f48374e33f5c23d5d3ba8a6f09d6b77d64e126668510e213a4407ae6cea9cc16c5a0ffffcdbfbac00d4d5faba2c5f8b168d7e13aa70591f0395e3e3683ef103d1a40d5cd10ba53b6f5a561623e69026b14e3a78cb80413eccf0afa8bcadfb3ee66e8ab181292909964cfd18c0759195a3a7d4a799830b4a7ae322be245689e4abfc6d643bf7f837d3a8b8f4d34d60da5e8880a99a41300485a7b9b2823e18d061d78df94c83f77f51978cb805f7faa68259c52e7559abf6db593afdb10b2df555b972a35b939ec2040850a10df3c79021b16022f446fe8d0f647ea20112df17ae11071c98d1fe5e86ce674b362c61945a279a46f9a3fd760a2039e2e4450b960ad477ba97fb1613b6e476ed11bc2a637b47ee325fe54ba1a90025248ab1b8f09cbac0b9d6c9863a74a6a403e5da3e8b1c6f27757fb49b15fd089fa91ed70887357c6beecdfaf39f88c62550daeaf720cd8052131037ff6732d6f9ae9e0716bab791e0fe06ed324b344c501b3d5edb8e1c2118cbcb853e12d192a8be2c049e28f142242ee2e5ec0947298332549f1c6d2ba50f99f9c53f4e0346fa5ff21bbbd37af0d3c9e5ed40f1ba918c15658b0cf2bd35e6fae5484a80170af402903860345c47ea1226032ef61a1fba58df1f74c8132dd56155fd104b53d848cf6de0ec78caf618188226e78d6108deed1a500dc7f3d3b885df3f5a1b1949fc0f56d9522f7015f5a3df048459a7bb3a7896090cec798b8f2bdce1ebc4ff6bc64e7e8ceffca076e383d132bf66cdaa7d8af73df72974637df04db4919340f9f91ba652e50a353feaab588eeb7cacf8b768cb406b1e9a845024939856f833fd0c8a75b25d8a24cf3e39fc1a4c39db326f2cad13ce041500f6fcde84206073378d4632f525266b1f925cefae5d36e515297eb7e120ab1cb69a9e91fd961ecd859f853145786757e8df8cc0da8499a23ea0af6e9a877788d4f502bcd93c6bc6201c7cc17112ebba810bcd7dab28dc1dea8015e14464c9a2b97fa4827cbf07ce2693c1bf5ece4e03424578995f78aa655e067eef9634d93668f2635aad6db1762b8d";
    let rs384hex = "1800000000000000345541663679754669796d656565736a544e64614e5845374294316400000000010000001000000000000000625643795473476167675679357971511f070000000000002f3bdbc4230f8323c2f96e88db358ddf06d54deca8c53772bc9cfd1cd50157c30221cb579387049b732f79a24e90b51542738d53aa5c602df932d1b1dcd22a2ceeb04701bb359a1b6f015bdd413c8e6f65b40bc1c19730c5a5badd77be80aae530a007725d3d50d46cf47a333e72404e6a020d1fe9f756a1eb551a0cd4ca1836c7db977ae85095bc280dabb14c6afdc08cc3951376dafd14150832b74bba88132ce1200aa497b5d05465f3484872cd35bead6298450a6699750563b723003850800c5f155a6b124a6ffbd7a7501897fa4ad9773801fb3ce73811d2d3c13abc74caec6254a08efb6fc603603982afd282b76a3e11b24e279f360906531ef7307c4db62b524cd07132ae178b8e7cf081979bd5ed216997eeda02f8f8125c0c56109cb13ddb86631ca41722c5d7a34c45061cdefb914ca3aee5c0218a1d2beae9e0af8fead6bd5c2313659565c2b5194381b20fb0bb7859294169942473c09ccf54acb429bb7eea57e15015282ca9af297aecdd68e26f458af0574de868966dd1c86e41b856b6442e1f0ed2ef2d2c7fe3084dbdab48f1b1e4521169f8b5316adcbd41130f6e187ba92efd13807f87916e43df62a6533678ce4fcc8951aaed15dfc493aec2323ea98c784c4218582f3c2d6f2790b740a4678e61bc4ec0f72a2af62dde76e6075293143cdf0531720d7b110e618b8fe50169b94f4fd4f2a22e9c33270442f7bd0a781abe28a6048fd49c3922a0bb0352d56f5c64accb46d97a4b4ebe67ebba51e1eb1f01121b7663e78c1ccf7fda58b9a9d7f06e215dcf5d804d5fda7302e3b77137081f9c2cdf2adbf62fdb798063f77025afb096baaf79f534104fbae0fb68ba42f456446f2ede5c0f897a8893a984c0da13a630dc723109267973d46e1e7806c9cb80fb7c3667fa1ede2e71df6323f86038c2d65d95ca2c516aa489500796f14ddc6012f73a340a678986e54fd90e56ec6c486897028b5451eecf0f94adf247b1fd47d225bf0d80ffb5b7fd46e1797113e96158c0cad99c39a8a193897c6ea0c3f5030fd81e17dbf656df64d65ab352f20d5c6e09851c27dc5e847ea4118037147c9156e132d9b57b84fba87aa1cb8132f047db2c1ea0a3547afdcaf8e6f58ae58a89a60889df60e86680489b99cac3c1429835025a5e63f1b0e1168556884404c28f4ff861c3d875e85247a77ab9c439506b0ed137e6d007431623a00f91e2f307cf8c0d329d804a517fd20509657ade59d7043c287b38350b067a5159b0a1b11d135cd88904092b6495d6faf247bbc4c396cf0219f54c0dc8ee5b736001a13477f6b05727ec7370e691e778e7623fd89cc267efab5d81f484ad922d9ecbb3bd3ac4d3717c16111dce0fa2b186e164978a6288c96eab79e8357fdbb7522c4e87d8180cf84e28d1858af69e4a62cc6804ea49c81199256f301853b02121acbb473e6d95ee4be9ed96752562d8f09406c3491c16ae7df3d173f4c506bf296c585e65e03622cfb885bcbe4b4234d62cc5af723f6ebf93ecf6c6cdd4125e249656e5a80693481e34ce16c0b8ab0f2eb0d4474d7bea0c3e03b1e99f0823c3b888353cef5597636a0a939fb976f060ce50edb3c74ac476170530d133a0d3fefa1a8e47adbb85e14cd4c4f8b9443d2211e4f18a67e35fee1d6f78c6fe6590e3f4f073810a658a0cbe0c994b36f59f282860da3b26d50b13e71682a8972f6cb0a54c83e03daa542e4e44169407d5842ad449a23a80f8dcaaba225a29fc65181c97b1c69b0d660baa2dcd4c5e8ab86fb8eed5ba84ec18cc15abd58df92bea09f5510530a00fc242687b6ada6b98468d62ed4a76dfb9dce9d448b7525c1676d7a2b0ba03483ad7580d766302e17a9f4ac5a037919d2a2977f8428dcfe330b8c4d2280093271f0c56434054a5f5e5518013eec2411cc0ce3758de78eef253e4b1447484bc6a8e121b5678fa77977716d35bc45f796dab1eb0a08d3597def43a5a7f91b3e0e335ebd82106362c5f786bc00af1358104a640f1826b3567633e98ef91027b8498b3101f21a56d010ed16ad4179c9f9b0497af4a17adc4584f253e745ea4b291ceb4a5f5a35721fef41f5a104ab3026ccb5809603add1948f2805ad43d696f1fdfba7bb3f3773c79341452c8350fa27831d4412eeccfe20542c635aa94ef0ce9587e8dda1b22d0ce339daa2df36dd4834782e051c0052ab47800a21fa30b6b5a87b41ae90a7201f37502eda634235ad959d7a45efe7808444cf9df654f7fafa7b25cb1a6c5f49967fa9a54a9f2438d26735f39a9737f4216a3ce461ede71ab405b60629dc882d65bb63973c85fd8cf2aa00d5a19f0225a02a633038ba9dd84e3c31b00770271cecc2dd71c619a44ff2fbd67ebbef8a8549b3aefbf00deaeb9ece8cf2226e45ca6bede5da989cd99dafcd4bf42b0f43e11108d06c1b0ff4e09b96f6a631027fc49ed3a6695fb11203cadbff4d925b53a69e950002991859f7c882c2549e0d90d0de1589ad9d337eecb2e8d50143c8e8c52b57b4bb829c2d7ca65ed786cbe01a0939aefceec04097b74c02292ba96108c2618a5db";
    let rs512hex = "1800000000000000766b734958583741715942456c68444854646b4775725959849431640000000002000000100000000000000062564379547347616767567935797151620900000000000013536617b5b99fba069cee85eabe38efe63649eeac2169321229350162aeadcb6cf9b86bf56845c41e76fa2ac782f2d5cd4b570986d80e03cf6304533138aa4d8338eef9f53b81a660160df7e84d5ae160956bb3a6b7ee0514336b89f80194b539591ecfb12ff3906bccf115c04364ec62260337ffbc0f9725f4d4b0f35ce36a287bc785292577d04fa45d21651c84bae82fac366aaf82df866d108d0b066eddb11c9b85b23677ddd9030ee28be63fc879b67e7925222c70f4d9e8b46033fed4a3f9c54116fbcd4a5f4ce59e34740c9183566ba3b3a2a45cb72176bb3d9a6c87bf7b0126af7937ee7db026aeccf1f0c56f90c2e45b2d94b9655acc0fb65f236861a807e345bde870fccd0f6cb297ef852c3498df0f8fb4c2395c93cd6197bc69dc754b456fd770f0eeb8c480253f6f4f6d43c966b3c006e5873c8f2b1b98b3f4af69b484b3191674decd1e497262890275077a82c5b3dd07bee667649a483055329e5c6b92233ce1e1c80752d828e30801c34bc0b2bbbb10c51648e5489233a9566f4e3f1735ed77d2706876c6621af0974e796b35015151cee5936f8cb48cd37822508ed9d1abba7f49318cfbddd40f6b1fc9e06d283ecca0086e09996317b9fe6201a1e13bfd652013a4ce277abed1052577df7fa2e8c40a122d9fd05723abf11242a73d6098f4ef964c63a2c27936f908e3f4915dfc1b76916fd192a02d992f38e1ec72975e2bc90dc5f185f9b62aa42d072688b2961b65884204a8ef1d226cc075d02851b3df60c0e30f34118bdf323c087b334c189612df5e9a66136ef08716cd53a7aca6fab10d2b07e2f1c0542e93085a718915718ec81bfa8024a7ca9e3fda1e151b2ed72f140ccade9ec1267e4da889e3e87aa1c2271ac723caa60c451110b549daef7114a7224430fb6a5bcbb4e0c7d95b8ed0dad54cde90f0fb8785b8ea77baeb114c0ee2c31861ffe1775d9858fba2e2d01bed159604db1824c2f8b6f1a03e33126dbab5dd004bfe79d038e4e2977cec76ab0ee894bf2bf7d62d48d9a56e7d75ed636841bfed3e094806ea70c72b4b9cfab0186728fd58950e54af3807239fea6bbc9be6a839baa53da0aea7dbc585bfc1789018e7108c645e5331505f75af3b9a181cb7d1e5c87ec481cb12088219e153e03ffd000410a30b74c4115cbf946bb9c2fcf28a945b136ddb222771165c496fda8e9028dda74f8c549d0263080f72ae15ccd76029aaf1b9127e8de64e30da441ba49f73f26db36b650604ad5466c41f7c8557cb6fe196e9b336a48f1d4059277fd607d460d73e54dff57348ed0f91a66f7a119cc24718ba2dedd0199ef1370ef90b36e37dd7f77402faeffe54dbdc588636ea75bb24c4315c3cce16b8122d01133bcbd1f6c9b9f9cb1379d81c71c08385786cb0e364c02e3175bf4e183589e211a32d896eb6169b791a23126f8de821a4f8e289f28c289b7dd2d93b65a634e9e4eaa210f720f8f40c06239b5c8308082d96c3133c6dc5d47ff3caed47842171b8e3ce1056bdbb4ef24111189f91b5930ce648c669b3b5f4831d9fef41e277722b461238f9d63ad3ab5b967e69d183b1a697d30f71057e404f67b49955181b7b56fee2d9162b4e953eb29e8675e388d3d6e2b99c67cc38a61b8c2d4563404935253b90b2d9b795f3dd6cbb99c104492101d6a06b1da50c64a926310506773d5f4014ed06c49c5d507f5e0902d7a7eb8ebb4ad22c25ad9aed723fd6d29cb120b6dc5b2a6a7404d8976247445694d270a6d9360960d68cbf8c4552f9c5cc5bb77ef005f5abc51251f78b63fb7cd6bdd56057fd116c85a4f4f6f34d9a8aa545976d7215d8658715edfb13a75b698d299db8e271d2b24ce5dc5106230c9b5a1fe7a45913e9e9798c04d58021ec01d7851c90f2fe5c4110970c61122a7dae2a41b0d4163403a54457745e026573cb72fb6d0b78cc0fe0b51e58f71c60f7f4f3ff8ae291523f528bf45c1ef01c2abffc8a5bcd2ff90ec6835375c61f5cd674f73372826ae10f1dc9525662b31f1e0e55508505c89078351ac5fc5750756b28bc16e2fdfecc4dda066e7acca80158511aea24f3f1d3d2e616bd5534e1785c7eb8b8c0fff76e4df371e6933fce2fb5186ea67cda94b71f239f688c4e5557e7318c45a9a5a2463e546221cd5e6929e9129f7e31bf81c051cc23abf17d3effb75a42212c1a3304205c17e72c7a4e4188981b45c51b08b2b20a9d54139d7e07839d276a7913dbe2bc5933d4edf5823522b56bbd6ea59256faa6bad6fa70611aed7f5f78bc524f2bcedee4f552b05f50ad7977bd4bee2ba0409284f12e38ed1ca0d407cc4b19956b8910996992977d4458e53cb058659b5c7e19cf036a25f5d32208dfb679dd9563f226aedfcc6cbd217d14e5d14859f44fd2fb3a1deee2b3885cea6a70b2281ba20af1928bf0e001fcf89bbffe60fb4ca2dd76f68b2878cbd5044302c6cf7ee4f9dbe1b84644a866060379c1dfd75bbaedf8a305098e9427c322e9af6e80997a713618bfe93ed980d8d4eadb447b2df13d6c9bda1bd9ce62f8d87ac5ac19a8a2bb851ea1ae76f9e1fc3581ef708e67059aaf165c32ac5d69fbed14527746a7493b56328dbc4460ac9028bdcdfe99ef28b3bdec0d27eb42e13866db0a063fb28d52f36c51497d871302712957ebdcf3aeb15457074a8fed2854e75b97455207c8ac4c17b87332ca17484250918c697f5480bb4850de6c237f2b7feeb9c02ff758a987622a2ea2081899e5ef9b76141bbc09ac046c04ec681f2b9be9e600382de701e3bbd054b1a9b95fa1e1e366a0b94f09a0fc85247878df5a2a7d97e2bc9f3d5690c03625049107230ebb17d2e40d1e4557197402b574d074bc4d19140e6400a4660037bc09c356d39533b7d0397822052aca9a78a44fe2279fca3d7ef51becaea32ad5c9c62ef193af1af81e36fd33c5ab759f4efc361adbecb654ce2ea572a17ca98a1c07c5a575767c99926f7d1ffcd9a67cd8831ee9d9e6818fd530f89fbf924411b4e4ea29e50b2eb4b5866dd811a65b1b0e447dcb0d5d878796c72bb5d36893cad5bb6899a42f11075c2d040665d52195bb1ce35c86e459205ed63b131f4e525083a9673bf7fb5d775d511dc91ee0dfd9d40d987e90e3f51c916db9a33324f90a6e86602e24c462118d92b6d1b12f560d0a83fccb9505c3a89a793b87ec6979d2ebb6c96de1620075c83e223be76a3a65f3f027fdf84cbb1ba76e4e94b0b4337ec80b779e708464354bf7622bc502c502932b980705086c35979ff968fec0345c82759a878a66f59f974fff35bad4ece748912c2bf32ffa20cd3ae470a68dbd7730e202a9b56eb7651d934fb2d35941146671d00b45b24f4d69db13a92beeddba3bb3df6324b8";
    let eddsahex = "18000000000000003951366b44764a7931555153424f783343714c4c615835698494316400000000030000001000000000000000625643795473476167675679357971514c0000000000000013a3f16a1da3db536d3a952fd348e66912197bb4dee0b00befa3aa2a76abb778d058c9c351fd977c99fd8bb63e14434e5024ba9c185dcfacc101c9bbb2ca9b02b35b17dc294ec59716e18c45";

    let mut jwks = Vec::with_capacity(4);
    let entity = bincode::deserialize::<Jwk>(hex::decode(rs256hex).unwrap().as_slice()).unwrap();
    jwks.push(entity);
    let entity = bincode::deserialize::<Jwk>(hex::decode(rs384hex).unwrap().as_slice()).unwrap();
    jwks.push(entity);
    let entity = bincode::deserialize::<Jwk>(hex::decode(rs512hex).unwrap().as_slice()).unwrap();
    jwks.push(entity);
    let entity = bincode::deserialize::<Jwk>(hex::decode(eddsahex).unwrap().as_slice()).unwrap();
    jwks.push(entity);

    for jwk in jwks {
        let _ = sqlx::query(
            r#"insert into jwks (kid, created_at, signature, enc_key_id, jwk)
            values ($1, $2, $3, $4, $5)"#,
        )
        .bind(&jwk.kid)
        .bind(jwk.created_at)
        .bind(jwk.signature.as_str())
        .bind(&jwk.enc_key_id)
        .bind(&jwk.jwk)
        .execute(db)
        .await;
    }

    let ml = MagicLink {
        id: "2qqdUOcXECQeypBNTs7Pnp7A2zAwr0VzynyzJiIjNR1Ua9KA95dTewM56JaPIoyj".to_string(),
        user_id: "2PYV3STNz3MN7VnPjJVcPQap".to_string(),
        csrf_token: "8jINPnFznLF9o905QuE2n9CD4rTraQO4E4fOWPZTAkbgNHqM".to_string(),
        cookie: None,
        exp: OffsetDateTime::now_utc()
            .add(::time::Duration::days(1))
            .unix_timestamp(),
        used: false,
        usage: MagicLinkUsage::PasswordReset.to_string(),
    };
    let _ = sqlx::query(
        r#"insert into magic_links (id, user_id, csrf_token, exp, used, usage)
        values ($1, $2, $3, $4, $5, $6)"#,
    )
    .bind(&ml.id)
    .bind(&ml.user_id)
    .bind(&ml.csrf_token)
    .bind(ml.exp)
    .bind(false)
    .bind(ml.usage)
    .execute(db)
    .await;

    Ok(())
}
