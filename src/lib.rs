use core::fmt;
use std::error::Error;

// -1/e
const Z0: f64 = -0.36787944117144232160;
// sqrt(1/e)
const X0: f64 = 0.60653065971263342360;

/// The principal branch of the Lambert W function, W_0.
///
/// # Errors
///
/// Returns an error if `z` < -1/e.
pub fn lambert_w0(z: f64) -> Result<f64, LambertW0Error> {
    dw0c(z - Z0)
}

/// The secondary branch of the Lambert W function, W_-1.
///
/// # Errors
///
/// Returns an error if `z` is positive or if `z` < -1/e.
pub fn lambert_wm1(z: f64) -> Result<f64, LambertWm1Error> {
    dwm1c(z, z - Z0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LambertW0Error;

impl fmt::Display for LambertW0Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "argument out of range")
    }
}

impl Error for LambertW0Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LambertWm1Error {
    ArgumentOutOfRange,
    PositiveArgument,
}

impl fmt::Display for LambertWm1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArgumentOutOfRange => write!(f, "argument out of range"),
            Self::PositiveArgument => write!(f, "positive argument"),
        }
    }
}

impl Error for LambertWm1Error {}

/// 50-bit accuracy computation of principal branch of Lambert W function, W_0(z),
/// by piecewise minimax rational function approximation
///
/// NOTICE: Input arguement is NOT z but its complement defined as zc = z+1/e
///
/// Created by T. Fukushima <Toshio.Fukushima@nao.ac.jp>,
/// ported to Rust by Johanna Sörngård <jsorngard@gmail.com>
///
/// Reference: T. Fukushima (2020) to be submitted
///  "Precise and fast computation of Lambert W-functions by piecewise
///   rational function approximation with variable transformation"
#[rustfmt::skip]
fn dw0c(zc: f64) -> Result<f64, LambertW0Error> {
    if zc < 0.0 {
        Err(LambertW0Error)
    } else if zc <= 2.5498939065034735716 {
        // W <= 0.893, X_1
        let x = zc.sqrt();
        Ok((-0.9999999999999998890
            + x * (-2.7399668668203659304
                + x * (0.026164207726990399347
                    + x * (6.370916807894900917
                        + x * (7.101328651785402668
                            + x * (2.9800826783006852573
                                + x * (0.48819596813789865043
                                    + x * (0.023753035787333611915
                                        + x * (0.00007736576009377243100)))))))))
            / (1.
                + x * (5.071610848417428005
                    + x * (9.986838818354528337
                        + x * (9.660755192207886908
                            + x * (4.7943728991336119052
                                + x * (1.1629703477704522300
                                    + x * (0.11849462500733755233
                                        + x * (0.0034326525132402226488)))))))))
    } else if zc <= 43.613924462669367895 {
        // W <= 2.754, X_2
        let x = zc.sqrt();
        Ok((-0.99997801800578916749
            + x * (-0.70415751590483602272
                + x * (2.1232260832802529071
                    + x * (2.3896760702935718341
                        + x * (0.77765311805029175244
                            + x * (0.089686698993644741433
                                + x * (0.0033062485753746403559
                                    + x * (0.000025106760479132851033))))))))
            / (1.
                + x * (3.0356026828085410884
                    + x * (3.1434530151286777057
                        + x * (1.3723156566592447275
                            + x * (0.25844697415744211142
                                + x * (0.019551162251819044265
                                    + x * (0.00048775933244530123101
                                        + x * (2.3165116841073152717e-6)))))))))
    } else if zc <= 598.45353371878276946 {
        // W <= 4.821, X_3
        let x = zc.sqrt();
        Ok((-0.98967420337273506393
            + x * (0.59587680606394382748
                + x * (1.4225083018151943148
                    + x * (0.44882889168323809798
                        + x * (0.044504943332390033511
                            + x * (0.0015218794835419578554
                                + x * (0.000016072263556502220023
                                    + x * (3.3723373020306510843e-8))))))))
            / (1.
                + x * (1.6959402394626198052
                    + x * (0.80968573415500900896
                        + x * (0.14002034999817021955
                            + x * (0.0093571878493790164480
                                + x * (0.00023251487593389773464
                                    + x * (1.8060170751502988645e-6
                                        + x * (2.5750667337015924224e-9)))))))))
    } else if zc <= 8049.4919850757619109 {
        // W <= 7.041, X_4
        let x = zc.sqrt();
        Ok((-0.77316491997206225517
            + x * (1.1391333504296703783
                + x * (0.43116117255217074492
                    + x * (0.035773078319037507449
                        + x * (0.00096441640580559092740
                            + x * (8.9723854598675864757e-6
                                + x * (2.5623503144117723217e-8
                                    + x * (1.4348813778416631453e-11))))))))
            / (1.
                + x * (0.74657287456514418083
                    + x * (0.12629777033419350576
                        + x * (0.0069741512959563184881
                            + x * (0.00014089339244355354892
                                + x * (1.0257432883152943078e-6
                                    + x * (2.2902687190119230940e-9
                                        + x * (9.2794231013264501664e-13)))))))))
    } else if zc <= 111124.95412121781420 {
        // W <= 9.380, X_5
        let x = zc.sqrt();
        Ok((0.12007101671553688430
            + x * (0.83352640829912822896
                + x * (0.070142775916948337582
                    + x * (0.0014846357985475124849
                        + x * (0.000010478757366110155290
                            + x * (2.5715892987071038527e-8
                                + x * (1.9384214479606474749e-11
                                    + x * (2.8447049039139409652e-15))))))))
            / (1.
                + x * (0.25396738845619126630
                    + x * (0.012839238907330317393
                        + x * (0.00020275375632510997371
                            + x * (1.1482956073449141384e-6
                                + x * (2.3188370605674263647e-9
                                    + x * (1.4271994165742563419e-12
                                        + x * (1.5884836942394796961e-16)))))))))
    } else if zc <= 1.5870429812082297112e6 {
        // W <= 11.809, X_6
        let x = zc.sqrt();
        Ok((1.7221104439937710112
            + x * (0.39919594286484275605
                + x * (0.0079885540140685028937
                    + x * (0.000042889742253257920541
                        + x * (7.8146828180529864981e-8
                            + x * (4.9819638764354682359e-11
                                + x * (9.7650889714265294606e-15
                                    + x * (3.7052997281721724439e-19))))))))
            / (1.
                + x * (0.074007438118020543008
                    + x * (0.0010333501506697740545
                        + x * (4.4360858035727508506e-6
                            + x * (6.7822912316371041570e-9
                                + x * (3.6834356707639492021e-12
                                    + x * (6.0836159560266041168e-16
                                        + x * (1.8149869335981225316e-20)))))))))
    } else if zc <= 2.3414708401875459509e7 {
        // W <= 14.308, X_7
        let x = zc.sqrt();
        Ok((3.7529314023434544256
            + x * (0.15491342690357806525
                + x * (0.00075663140675900784505
                    + x * (1.0271609235969979059e-6
                        + x * (4.7853247675930066150e-10
                            + x * (7.8328040770275474410e-14
                                + x * (3.9433033758391036653e-18
                                    + x * (3.8232862205660283978e-23))))))))
            / (1.
                + x * (0.020112985338854443555
                    + x * (0.000074712286154830141768
                        + x * (8.4800598003693837469e-8
                            + x * (3.4182424130376911762e-11
                                + x * (4.8866259139690957899e-15
                                    + x * (2.1223373626834634178e-19
                                        + x * (1.6642985671260582515e-24)))))))))
    } else if zc <= 3.5576474308009965225e8 {
        // W <= 16.865, X_8
        let x = zc.sqrt();
        Ok((6.0196542055606555577
            + x * (0.053496672841797864762
                + x * (0.000064340849275316501519
                    + x * (2.1969090100095967485e-8
                        + x * (2.5927988937033061070e-12
                            + x * (1.0779198161801527308e-16
                                + x * (1.3780424091017898301e-21
                                    + x * (3.3768973150742552802e-27))))))))
            / (1.
                + x * (0.0052809683704233371675
                    + x * (5.1020501219389558082e-6
                        + x * (1.5018312292270832103e-9
                            + x * (1.5677706636413188379e-13
                                + x * (5.7992041238911878361e-18
                                    + x * (6.5133170770320780259e-23
                                        + x * (1.3205080139213406071e-28)))))))))
    } else if zc <= 5.5501716296163627854e9 {
        // W <= 19.468, X_9
        let x = zc.sqrt();
        Ok((8.4280268500989701597
            + x * (0.017155758546279713315
                + x * (5.0836620669829321508e-6
                    + x * (4.3354903691832581802e-10
                        + x * (1.2841017145645583385e-14
                            + x * (1.3419106769745885927e-19
                                + x * (4.3101698455492225750e-25
                                    + x * (2.6422433422088187549e-31))))))))
            / (1.
                + x * (0.0013572006754595300315
                    + x * (3.3535243481426203694e-7
                        + x * (2.5206969246421264128e-11
                            + x * (6.7136226273060530496e-16
                                + x * (6.3324226680854686574e-21
                                    + x * (1.8128167400013774194e-26
                                        + x * (9.3662030058136796889e-33)))))))))
    } else if zc <= 8.8674704839657775331e10 {
        // W <= 22.112, X_10
        let x = zc.sqrt();
        Ok((10.931063230472498189
            + x * (0.0052224234540245532982
                + x * (3.7996105711810129682e-7
                    + x * (8.0305793533410355824e-12
                        + x * (5.9139785627090605866e-17
                            + x * (1.5382020359533028724e-22
                                + x * (1.2288944126268109432e-28
                                    + x * (1.8665089270660122398e-35))))))))
            / (1.
                + x * (0.00034328702551197577797
                    + x * (2.1395351518538844476e-8
                        + x * (4.0524170186631594159e-13
                            + x * (2.7181424315335710420e-18
                                + x * (6.4538986638355490894e-24
                                    + x * (4.6494613785888987942e-30
                                        + x * (6.0442024367299387616e-37)))))))))
    } else if zc <= 1.4477791865272902816e12 {
        // W <= 24.791, X_11
        let x = zc.sqrt();
        Ok((13.502943080893871412
            + x * (0.0015284636506346264572
                + x * (2.7156967358262346166e-8
                    + x * (1.4110394051242161772e-13
                        + x * (2.5605734311219728461e-19
                            + x * (1.6421293724425337463e-25
                                + x * (3.2324944691435843553e-32
                                    + x * (1.2054662641251783155e-39))))))))
            / (1.
                + x * (0.000085701512879089462255
                    + x * (1.3311244435752691563e-9
                        + x * (6.2788924440385347269e-15
                            + x * (1.0483788152252204824e-20
                                + x * (6.1943499966249160886e-27
                                    + x * (1.1101567860340917294e-33
                                        + x * (3.5897381128308962590e-41)))))))))
    } else if zc <= 2.4111458632511851931e13 {
        // W <= 27.500, X_12
        let x = zc.sqrt();
        Ok((16.128076167439014775
            + x * (0.00043360385176467069131
                + x * (1.8696403871820916466e-9
                    + x * (2.3691795766901486045e-15
                        + x * (1.0503191826963154893e-21
                            + x * (1.6461927573606764263e-28
                                + x * (7.9138276083474522931e-36
                                    + x * (7.1845890343701668760e-44))))))))
            / (1.
                + x * (0.000021154255263102938752
                    + x * (8.1006115442323280538e-11
                        + x * (9.4155986022169905738e-17
                            + x * (3.8725127902295302254e-23
                                + x * (5.6344651115570565066e-30
                                    + x * (2.4860951084210029191e-37
                                        + x * (1.9788304737427787405e-45)))))))))
    } else if zc <= 4.0897036442600845564e14 {
        // W <= 30.236, X_13
        let x = zc.sqrt();
        Ok((18.796301105534486604
            + x * (0.00011989443339646469157
                + x * (1.2463377528676863250e-10
                    + x * (3.8219456858010368172e-17
                        + x * (4.1055693930252083265e-24
                            + x * (1.5595231456048464246e-31
                                + x * (1.8157173553077986962e-39
                                    + x * (3.9807997764326166245e-48))))))))
            / (1.
                + x * (5.1691031988359922329e-6
                    + x * (4.8325571823313711932e-12
                        + x * (1.3707888746916928107e-18
                            + x * (1.3754560850024480337e-25
                                + x * (4.8811882975661805184e-33
                                    + x * (5.2518641828170201894e-41
                                        + x * (1.0192119593134756440e-49)))))))))
    } else if zc <= 7.0555901476789972402e15 {
        // W <= 32.996, X_14
        let x = zc.sqrt();
        Ok((21.500582830667332906
            + x * (0.000032441943237735273768
                + x * (8.0764963416837559148e-12
                    + x * (5.9488445506122883523e-19
                        + x * (1.5364106187215861531e-26
                            + x * (1.4033231297002386995e-34
                                + x * (3.9259872712305770430e-43
                                    + x * (2.0629086382257737517e-52))))))))
            / (1.
                + x * (1.2515317642433850197e-6
                    + x * (2.8310314214817074806e-13
                        + x * (1.9423666416123637998e-20
                            + x * (4.7128616004157359714e-28
                                + x * (4.0433347391839945960e-36
                                    + x * (1.0515141443831187271e-44
                                        + x * (4.9316490935436927307e-54)))))))))
    } else if zc <= 1.2366607557976727287e17 {
        // W <= 35.779, X_15
        let x = zc.sqrt();
        Ok((24.235812532416977267
            + x * (8.6161505995776802509e-6
                + x * (5.1033431561868273692e-13
                    + x * (8.9642393665849638164e-21
                        + x * (5.5254364181097420777e-29
                            + x * (1.2045072724050605792e-37
                                + x * (8.0372997176526840184e-47
                                    + x * (1.0049140812146492611e-56))))))))
            / (1.
                + x * (3.0046761844749477987e-7
                    + x * (1.6309104270855463223e-14
                        + x * (2.6842271030298931329e-22
                            + x * (1.5619672632458881195e-30
                                + x * (3.2131689030397984274e-39
                                    + x * (2.0032396245307684134e-48
                                        + x * (2.2520274554676331938e-58)))))))))
    } else if zc <= 2.1999373487930999775e18 {
        // W <= 38.582, X_16
        let x = zc.sqrt();
        Ok((26.998134347987436511
            + x * (2.2512257767572285866e-6
                + x * (3.1521230759866963941e-14
                    + x * (1.3114035719790631541e-22
                        + x * (1.9156784033962366146e-31
                            + x * (9.8967003053444799163e-41
                                + x * (1.5640423898448433548e-50
                                    + x * (4.6216193040664872606e-61))))))))
            / (1.
                + x * (7.1572676370907573898e-8
                    + x * (9.2500506091115760826e-16
                        + x * (3.6239819582787573031e-24
                            + x * (5.0187712493800424118e-33
                                + x * (2.4565861988218069039e-42
                                    + x * (3.6435658433991660284e-52
                                        + x * (9.7432490640155346004e-63)))))))))
    } else if zc <= 3.9685392198344016155e19 {
        // W <= 41.404, X_17
        let x = zc.sqrt();
        Ok((29.784546702831970770
            + x * (5.7971764392171329944e-7
                + x * (1.9069872792601950808e-15
                    + x * (1.8668700870858763312e-24
                        + x * (6.4200510953370940075e-34
                            + x * (7.8076624650818968559e-44
                                + x * (2.9029638696956315654e-54
                                    + x * (2.0141870458566179853e-65))))))))
            / (1.
                + x * (1.6924463180469706372e-8
                    + x * (5.1703934311254540111e-17
                        + x * (4.7871532721560069095e-26
                            + x * (1.5664405832545149368e-35
                                + x * (1.8113137982381331398e-45
                                    + x * (6.3454150289495419529e-56
                                        + x * (4.0072964025244397967e-67)))))))))
    } else if zc <= 1.4127075145274652069e104 {
        // W <= 234.358, U_18
        let y = zc.ln();
        Ok((0.74413499460126776143
            + y * (0.41403243618005911160
                + y * (0.26012564166773416170
                    + y * (0.021450457095960295520
                        + y * (0.00051872377264705907577
                            + y * (4.3574693568319975996e-6
                                + y * (1.2363066058921706716e-8
                                    + y * (9.0194147766309957537e-12))))))))
            / (1.
                + y * (0.33487811067467010907
                    + y * (0.023756834394570626395
                        + y * (0.00054225633008907735160
                            + y * (4.4378980052579623037e-6
                                + y * (1.2436585497668099330e-8
                                    + y * (9.0225825867631852215e-12
                                        + y * (-4.2057836270109716654e-19)))))))))
    } else {
        //   U_19
        let y = zc.ln();
        Ok((-0.61514412812729761526
            + y * (0.67979310133630936580
                + y * (0.089685353704585808963
                    + y * (0.0015644941483989379249
                        + y * (7.7349901878176351162e-6
                            + y * (1.2891647546699435229e-8
                                + y * (7.0890325988973812656e-12
                                    + y * (9.8419790334279711453e-16))))))))
            / (1.
                + y * (0.097300263710401439315
                    + y * (0.0016103672748442058651
                        + y * (7.8247741003077000012e-6
                            + y * (1.2949261308971345209e-8
                                + y * (7.0986911219342827130e-12
                                    + y * (9.8426285042227044979e-16
                                        + y * (-1.5960147252606055352e-24)))))))))
    }
}

/// 50-bit accuracy computation of secondary branch of Lambert W function, W_-1(z),
/// by piecewise minimax rational function approximation
/// 
/// NOTICE: Required are two input arguements z and its complement defined as zc = z+1/e
/// 
/// Created by T. Fukushima <Toshio.Fukushima@nao.ac.jp>,
/// ported to Rust by Johanna Sörngård
/// 
/// Reference: T. Fukushima (2020) to be submitted
///  "Precise and fast computation of Lambert W-functions by piecewise
///   rational function approximation with variable transformation"
#[rustfmt::skip]
fn dwm1c(z: f64, zc: f64) -> Result<f64, LambertWm1Error> {
    if zc < 0.0 {
        Err(LambertWm1Error::ArgumentOutOfRange)
    } else if z <= -0.3542913309442164 {
        // W >= -1.3, X_-1
        let x = zc.sqrt();
        Ok((-1.0000000000000001110
            + x * (4.2963016178777127009
                + x * (-4.0991407924007457612
                    + x * (-6.8442842200833309724
                        + x * (17.084773793345271001
                            + x * (-13.015133123886661124
                                + x * (3.9303608629539851049 + x * (-0.34636746512247457319))))))))
            / (1.
                + x * (-6.6279455994747624059
                    + x * (17.740962374121397994
                        + x * (-24.446872319343475890
                            + x * (18.249006287190617068
                                + x * (-7.0580758756624790550
                                    + x * (1.1978786762794003545
                                        + x * (-0.053875778140352599789)))))))))
    } else if z <= -0.18872688282289434049 {
        // W >= -2.637, Y_-1
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-8.2253155264446844854
            + x * (-813.20706732001487178
                + x * (-15270.113237678509000
                    + x * (-79971.585089674149237
                        + x * (-103667.54215808376511
                            + x * (42284.755505061257427
                                + x * (74953.525397605484884 + x * (10554.369146366736811))))))))
            / (1.
                + x * (146.36315161669567659
                    + x * (3912.4761372539240712
                        + x * (31912.693749754847460
                            + x * (92441.293717108619527
                                + x * (94918.733120470346165
                                    + x * (29531.165406571745340
                                        + x * (1641.6808960330370987)))))))))
    } else if z <= -0.060497597226958343647 {
        // W >= -4.253, Y_-2
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-9.6184127443354024295
            + x * (-3557.8569043018004121
                + x * (-254015.59311284381043
                    + x * (-5.3923893630670639391e6
                        + x * (-3.6638257417536896798e7
                            + x * (-6.1484319486226966213e7
                                + x * (3.0421690377446134451e7
                                    + x * (3.9728139054879320452e7))))))))
            / (1.
                + x * (507.40525628523300801
                    + x * (46852.747159777876192
                        + x * (1.3168304640091436297e6
                            + x * (1.3111690693712415242e7
                                + x * (4.6142116445258015195e7
                                    + x * (4.8982268956208830876e7
                                        + x * (9.1959100987983855122e6)))))))))
    } else if z <= -0.017105334740676008194 {
        // W >= -5.832, Y_-3
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-11.038489462297466388
            + x * (-15575.812882656619195
                + x * (-4.2492947304897773433e6
                    + x * (-3.5170245938803423768e8
                        + x * (-9.8659163036611364640e9
                            + x * (-8.6195372303305003908e10
                                + x * (-1.3286335574027616000e11
                                    + x * (1.5989546434420660462e11))))))))
            / (1.
                + x * (1837.0770693017166818
                    + x * (612840.97585595092761
                        + x * (6.2149181398465483037e7
                            + x * (2.2304011314443083969e9
                                + x * (2.8254232485273698021e10
                                    + x * (1.0770866639543156165e11
                                        + x * (7.1964698876049131992e10)))))))))
    } else if z <= -0.0045954962127943706433 {
        // W >= -7.382, Y_-4
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-12.474405916395746052
            + x * (-68180.335575543773385
                + x * (-7.1846599845620093278e7
                    + x * (-2.3142688221759181151e10
                        + x * (-2.5801378337945295130e12
                            + x * (-9.5182748161386314616e13
                                + x * (-8.6073250986210321766e14
                                    + x * (1.4041941853339961439e14))))))))
            / (1.
                + x * (6852.5813734431100971
                    + x * (8.5153001025466544379e6
                        + x * (3.2146028239685694655e9
                            + x * (4.2929807417453196113e11
                                + x * (2.0234381161638084359e13
                                    + x * (2.8699933268233923842e14
                                        + x * (7.1210136651525477096e14)))))))))
    } else if z <= -0.0012001610672197724173 {
        // W >= -8.913, Y_-5
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-13.921651376890072595
            + x * (-298789.56482388065526
                + x * (-1.2313019937322092334e9
                    + x * (-1.5556149081899508970e12
                        + x * (-6.8685341106772708734e14
                            + x * (-1.0290616275933266835e17
                                + x * (-4.1404683701619648471e18
                                    + x * (-1.4423309998006368397e19))))))))
            / (1.
                + x * (26154.955236499142433
                    + x * (1.2393087277442041494e8
                        + x * (1.7832922702470761113e11
                            + x * (9.0772608163810850446e13
                                + x * (1.6314734740054252741e16
                                    + x * (8.8371323861233504533e17
                                        + x * (8.4166620643385013384e18)))))))))
    } else if z <= -0.00030728805932191499844 {
        // W >= -10.433, Y_-6
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-15.377894224591557534
            + x * (-1.3122312005096979952e6
                + x * (-2.1408157022111737888e10
                    + x * (-1.0718287431557811808e14
                        + x * (-1.8849353524027734456e17
                            + x * (-1.1394858607309311995e20
                                + x * (-1.9261555088729141590e22
                                    + x * (-3.9978452086676901296e23))))))))
            / (1.
                + x * (101712.86771760620046
                    + x * (1.8728545945050381188e9
                        + x * (1.0469617416664402757e13
                            + x * (2.0704349060120443049e16
                                + x * (1.4464907902386074496e19
                                    + x * (3.0510432205608900949e21
                                        + x * (1.1397589139790739717e23)))))))))
    } else if z <= -0.000077447159838062184354 {
        // W >= -11.946, Y_-7
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-16.841701411264981596
            + x * (-5.7790823257577138416e6
                + x * (-3.7757230791256404116e11
                    + x * (-7.5712133742589860941e15
                        + x * (-5.3479338916011465685e19
                            + x * (-1.3082711732297865476e23
                                + x * (-9.1462777004521427440e25
                                    + x * (-8.9602768119263629340e27))))))))
            / (1.
                + x * (401820.46666230725328
                    + x * (2.9211518136900492046e10
                        + x * (6.4456135373410289079e14
                            + x * (5.0311809576499530281e18
                                + x * (1.3879041239716289478e22
                                    + x * (1.1575146167513516225e25
                                        + x * (1.7199220185947756654e27)))))))))
    } else if z <= -4.5808119698158173174e-17 {
        // W >= -41.344, V_-8
        let u = (-z).ln();
        Ok((-2.0836260384016439265
            + u * (1.6122436242271495710
                + u * (5.4464264959637207619
                    + u * (-3.0886331128317160105
                        + u * (0.46107829155370137880
                            + u * (-0.023553839118456381330
                                + u * (0.00040538904170253404780
                                    + u * (-1.7948156922516825458e-6))))))))
            / (1.
                + u * (2.3699648912703015610
                    + u * (-2.1249449707404812847
                        + u * (0.38480980098588483913
                            + u * (-0.021720009380176605969
                                + u * (0.00039405862890608636876
                                    + u * (-1.7909312066865957905e-6
                                        + u * (3.1153673308133671452e-12)))))))))
    } else if z <= -6.1073672236594792982e-79 {
        // W >= -185.316, V_-9
        let u = (-z).ln();
        Ok((0.16045383766570541409
            + u * (2.2214182524461514029
                + u * (-0.94119662492050892971
                    + u * (0.091921523818747869300
                        + u * (-0.0029069760533171663224
                            + u * (0.000032707247990255961149
                                + u * (-1.2486672336889893018e-7
                                    + u * (1.2247438279861785291e-10))))))))
            / (1.
                + u * (-0.70254996087870332289
                    + u * (0.080974347786703195026
                        + u * (-0.0027469850029563153939
                            + u * (0.000031943362385183657062
                                + u * (-1.2390620687321666439e-7
                                    + u * (1.2241636115168201999e-10
                                        + u * (-1.0275718020546765400e-17)))))))))
    } else if z < 0.0 {
        // V_-10
        let u = (-z).ln();
        Ok((-1.2742179703075440564
            + u * (1.3696658805421383765
                + u * (-0.12519345387558783223
                    + u * (0.0025155722460763844737
                        + u * (-0.000015748033750499977208
                            + u * (3.4316085386913786410e-8
                                + u * (-2.5025242885340438533e-11
                                    + u * (4.6423885014099583351e-15))))))))
            / (1.
                + u * (-0.11420006474152465694
                    + u * (0.0024285233832122595942
                        + u * (-0.000015520907512751723152
                            + u * (3.4120534760396002260e-8
                                + u * (-2.4981056186450274587e-11
                                    + u * (4.6419768093059706079e-15
                                        + u * (-1.3608713936942602985e-23)))))))))
    } else {
        Err(LambertWm1Error::PositiveArgument)
    }
}
