use std::collections::HashMap;

use crate::gotypes::{Player, Point};

pub type HashCodes = HashMap<(Point, Option<Player>), u64>;

pub struct ZoobristHash {
    pub hash_codes: HashCodes,
    pub empty_board: u64,
}

impl ZoobristHash {
    pub fn new() -> ZoobristHash {
        let hash_code = HashMap::from([
            ((Point { row: 1, col: 1 }, None), 6402364705153495313),
            (
                (Point { row: 1, col: 1 }, Some(Player::Black)),
                444191475187629924,
            ),
            (
                (Point { row: 1, col: 1 }, Some(Player::White)),
                3180807544946524599,
            ),
            ((Point { row: 1, col: 2 }, None), 3474352554071515715),
            (
                (Point { row: 1, col: 2 }, Some(Player::Black)),
                4452486373405451057,
            ),
            (
                (Point { row: 1, col: 2 }, Some(Player::White)),
                2625197482182378633,
            ),
            ((Point { row: 1, col: 3 }, None), 4301395653233812397),
            (
                (Point { row: 1, col: 3 }, Some(Player::Black)),
                1052743445463685251,
            ),
            (
                (Point { row: 1, col: 3 }, Some(Player::White)),
                4333922193950527979,
            ),
            ((Point { row: 1, col: 4 }, None), 579019433439089653),
            (
                (Point { row: 1, col: 4 }, Some(Player::Black)),
                6933861647704254864,
            ),
            (
                (Point { row: 1, col: 4 }, Some(Player::White)),
                7471883531373616369,
            ),
            ((Point { row: 1, col: 5 }, None), 6620703135527919578),
            (
                (Point { row: 1, col: 5 }, Some(Player::Black)),
                266726669874565249,
            ),
            (
                (Point { row: 1, col: 5 }, Some(Player::White)),
                971215397166039728,
            ),
            ((Point { row: 1, col: 6 }, None), 4901139204652546593),
            (
                (Point { row: 1, col: 6 }, Some(Player::Black)),
                3661296273230053320,
            ),
            (
                (Point { row: 1, col: 6 }, Some(Player::White)),
                2091383563393534946,
            ),
            ((Point { row: 1, col: 7 }, None), 5841479823883894579),
            (
                (Point { row: 1, col: 7 }, Some(Player::Black)),
                1343795425674087019,
            ),
            (
                (Point { row: 1, col: 7 }, Some(Player::White)),
                7027629093182269949,
            ),
            ((Point { row: 1, col: 8 }, None), 6974526363558785031),
            (
                (Point { row: 1, col: 8 }, Some(Player::Black)),
                3308889409935951416,
            ),
            (
                (Point { row: 1, col: 8 }, Some(Player::White)),
                8001375756973022700,
            ),
            ((Point { row: 1, col: 9 }, None), 3326669231944330056),
            (
                (Point { row: 1, col: 9 }, Some(Player::Black)),
                182527627536217527,
            ),
            (
                (Point { row: 1, col: 9 }, Some(Player::White)),
                647055117537790819,
            ),
            ((Point { row: 1, col: 10 }, None), 6580632857085978264),
            (
                (Point { row: 1, col: 10 }, Some(Player::Black)),
                6659120627973877329,
            ),
            (
                (Point { row: 1, col: 10 }, Some(Player::White)),
                6957515196384025093,
            ),
            ((Point { row: 1, col: 11 }, None), 9041661876709162103),
            (
                (Point { row: 1, col: 11 }, Some(Player::Black)),
                9191630649994762430,
            ),
            (
                (Point { row: 1, col: 11 }, Some(Player::White)),
                7667317760915857375,
            ),
            ((Point { row: 1, col: 12 }, None), 1146547653441411500),
            (
                (Point { row: 1, col: 12 }, Some(Player::Black)),
                4291861776441109562,
            ),
            (
                (Point { row: 1, col: 12 }, Some(Player::White)),
                5261049327471094678,
            ),
            ((Point { row: 1, col: 13 }, None), 2778587950653512497),
            (
                (Point { row: 1, col: 13 }, Some(Player::Black)),
                465534554861083261,
            ),
            (
                (Point { row: 1, col: 13 }, Some(Player::White)),
                3865103907397000387,
            ),
            ((Point { row: 1, col: 14 }, None), 4402854031209572266),
            (
                (Point { row: 1, col: 14 }, Some(Player::Black)),
                2607843465059610177,
            ),
            (
                (Point { row: 1, col: 14 }, Some(Player::White)),
                5449307550544736426,
            ),
            ((Point { row: 1, col: 15 }, None), 5205560982133491164),
            (
                (Point { row: 1, col: 15 }, Some(Player::Black)),
                1757319192084209670,
            ),
            (
                (Point { row: 1, col: 15 }, Some(Player::White)),
                2204336175311806893,
            ),
            ((Point { row: 1, col: 16 }, None), 6377548462923823584),
            (
                (Point { row: 1, col: 16 }, Some(Player::Black)),
                5370945101679060223,
            ),
            (
                (Point { row: 1, col: 16 }, Some(Player::White)),
                2750848819294718078,
            ),
            ((Point { row: 1, col: 17 }, None), 1922634541266613353),
            (
                (Point { row: 1, col: 17 }, Some(Player::Black)),
                3035185049417693555,
            ),
            (
                (Point { row: 1, col: 17 }, Some(Player::White)),
                9116106208682090693,
            ),
            ((Point { row: 1, col: 18 }, None), 9132932969976328558),
            (
                (Point { row: 1, col: 18 }, Some(Player::Black)),
                9192639340869719841,
            ),
            (
                (Point { row: 1, col: 18 }, Some(Player::White)),
                5306307051004961905,
            ),
            ((Point { row: 1, col: 19 }, None), 1273340342344806167),
            (
                (Point { row: 1, col: 19 }, Some(Player::Black)),
                4527122623697748155,
            ),
            (
                (Point { row: 1, col: 19 }, Some(Player::White)),
                3428488706066419650,
            ),
            ((Point { row: 2, col: 1 }, None), 6449118728834868339),
            (
                (Point { row: 2, col: 1 }, Some(Player::Black)),
                1585640068930406174,
            ),
            (
                (Point { row: 2, col: 1 }, Some(Player::White)),
                9134544091463577840,
            ),
            ((Point { row: 2, col: 2 }, None), 8328734357465609043),
            (
                (Point { row: 2, col: 2 }, Some(Player::Black)),
                3825472901644543968,
            ),
            (
                (Point { row: 2, col: 2 }, Some(Player::White)),
                4331833334881670434,
            ),
            ((Point { row: 2, col: 3 }, None), 7939176487324947983),
            (
                (Point { row: 2, col: 3 }, Some(Player::Black)),
                8678508756355018553,
            ),
            (
                (Point { row: 2, col: 3 }, Some(Player::White)),
                5235979586169535941,
            ),
            ((Point { row: 2, col: 4 }, None), 7263218545755892903),
            (
                (Point { row: 2, col: 4 }, Some(Player::Black)),
                4448713833350756341,
            ),
            (
                (Point { row: 2, col: 4 }, Some(Player::White)),
                4465157666470120555,
            ),
            ((Point { row: 2, col: 5 }, None), 3239965565957128280),
            (
                (Point { row: 2, col: 5 }, Some(Player::Black)),
                4557763796869350033,
            ),
            (
                (Point { row: 2, col: 5 }, Some(Player::White)),
                8842890046215258149,
            ),
            ((Point { row: 2, col: 6 }, None), 3769355537864191299),
            (
                (Point { row: 2, col: 6 }, Some(Player::Black)),
                6885240998881489694,
            ),
            (
                (Point { row: 2, col: 6 }, Some(Player::White)),
                4415950635220041552,
            ),
            ((Point { row: 2, col: 7 }, None), 7621263210603790530),
            (
                (Point { row: 2, col: 7 }, Some(Player::Black)),
                7256234720016582174,
            ),
            (
                (Point { row: 2, col: 7 }, Some(Player::White)),
                499138474184864094,
            ),
            ((Point { row: 2, col: 8 }, None), 5025506371434802894),
            (
                (Point { row: 2, col: 8 }, Some(Player::Black)),
                2778062765892278994,
            ),
            (
                (Point { row: 2, col: 8 }, Some(Player::White)),
                3712877991078905475,
            ),
            ((Point { row: 2, col: 9 }, None), 840509165269696992),
            (
                (Point { row: 2, col: 9 }, Some(Player::Black)),
                8647746146718318171,
            ),
            (
                (Point { row: 2, col: 9 }, Some(Player::White)),
                535387047113016018,
            ),
            ((Point { row: 2, col: 10 }, None), 8708635127468887087),
            (
                (Point { row: 2, col: 10 }, Some(Player::Black)),
                975639018856529464,
            ),
            (
                (Point { row: 2, col: 10 }, Some(Player::White)),
                12436055714459141,
            ),
            ((Point { row: 2, col: 11 }, None), 8895395780204337051),
            (
                (Point { row: 2, col: 11 }, Some(Player::Black)),
                4412612752235080739,
            ),
            (
                (Point { row: 2, col: 11 }, Some(Player::White)),
                3784840302641714006,
            ),
            ((Point { row: 2, col: 12 }, None), 577424424841274797),
            (
                (Point { row: 2, col: 12 }, Some(Player::Black)),
                4746030903634755766,
            ),
            (
                (Point { row: 2, col: 12 }, Some(Player::White)),
                6316652347533303429,
            ),
            ((Point { row: 2, col: 13 }, None), 6441043060651851472),
            (
                (Point { row: 2, col: 13 }, Some(Player::Black)),
                8551628441633633049,
            ),
            (
                (Point { row: 2, col: 13 }, Some(Player::White)),
                6158877703276773492,
            ),
            ((Point { row: 2, col: 14 }, None), 7740666914240334555),
            (
                (Point { row: 2, col: 14 }, Some(Player::Black)),
                6148556273867368376,
            ),
            (
                (Point { row: 2, col: 14 }, Some(Player::White)),
                2671810666463534590,
            ),
            ((Point { row: 2, col: 15 }, None), 3756484191730342703),
            (
                (Point { row: 2, col: 15 }, Some(Player::Black)),
                3556222011996989557,
            ),
            (
                (Point { row: 2, col: 15 }, Some(Player::White)),
                3861437304505402972,
            ),
            ((Point { row: 2, col: 16 }, None), 236496246506091899),
            (
                (Point { row: 2, col: 16 }, Some(Player::Black)),
                818014148096696419,
            ),
            (
                (Point { row: 2, col: 16 }, Some(Player::White)),
                6468780300745379509,
            ),
            ((Point { row: 2, col: 17 }, None), 399155553254586720),
            (
                (Point { row: 2, col: 17 }, Some(Player::Black)),
                3702070310203979897,
            ),
            (
                (Point { row: 2, col: 17 }, Some(Player::White)),
                593549611371714409,
            ),
            ((Point { row: 2, col: 18 }, None), 8767893947804512198),
            (
                (Point { row: 2, col: 18 }, Some(Player::Black)),
                5133383106902779321,
            ),
            (
                (Point { row: 2, col: 18 }, Some(Player::White)),
                6828977111791572903,
            ),
            ((Point { row: 2, col: 19 }, None), 4592011223739099001),
            (
                (Point { row: 2, col: 19 }, Some(Player::Black)),
                7614885855104742178,
            ),
            (
                (Point { row: 2, col: 19 }, Some(Player::White)),
                1065785641268353102,
            ),
            ((Point { row: 3, col: 1 }, None), 3214339223890817952),
            (
                (Point { row: 3, col: 1 }, Some(Player::Black)),
                1804735232890689478,
            ),
            (
                (Point { row: 3, col: 1 }, Some(Player::White)),
                5530191433834147475,
            ),
            ((Point { row: 3, col: 2 }, None), 2459542802743141529),
            (
                (Point { row: 3, col: 2 }, Some(Player::Black)),
                7430710132699508883,
            ),
            (
                (Point { row: 3, col: 2 }, Some(Player::White)),
                5227922030234559936,
            ),
            ((Point { row: 3, col: 3 }, None), 8310487749316464853),
            (
                (Point { row: 3, col: 3 }, Some(Player::Black)),
                6539250671230328204,
            ),
            (
                (Point { row: 3, col: 3 }, Some(Player::White)),
                1636391802322895807,
            ),
            ((Point { row: 3, col: 4 }, None), 3201605223513475804),
            (
                (Point { row: 3, col: 4 }, Some(Player::Black)),
                4693922249704781826,
            ),
            (
                (Point { row: 3, col: 4 }, Some(Player::White)),
                1779229419151637822,
            ),
            ((Point { row: 3, col: 5 }, None), 558201485475450064),
            (
                (Point { row: 3, col: 5 }, Some(Player::Black)),
                4865685331756308707,
            ),
            (
                (Point { row: 3, col: 5 }, Some(Player::White)),
                7052208577881940555,
            ),
            ((Point { row: 3, col: 6 }, None), 4805567071864461468),
            (
                (Point { row: 3, col: 6 }, Some(Player::Black)),
                7559097505437019193,
            ),
            (
                (Point { row: 3, col: 6 }, Some(Player::White)),
                4643844473324317561,
            ),
            ((Point { row: 3, col: 7 }, None), 1519687966931990918),
            (
                (Point { row: 3, col: 7 }, Some(Player::Black)),
                6786702668528296933,
            ),
            (
                (Point { row: 3, col: 7 }, Some(Player::White)),
                3684977632621123172,
            ),
            ((Point { row: 3, col: 8 }, None), 2610234951903747791),
            (
                (Point { row: 3, col: 8 }, Some(Player::Black)),
                7104169818985152575,
            ),
            (
                (Point { row: 3, col: 8 }, Some(Player::White)),
                4560982823209099388,
            ),
            ((Point { row: 3, col: 9 }, None), 159142264817205456),
            (
                (Point { row: 3, col: 9 }, Some(Player::Black)),
                781602136172868089,
            ),
            (
                (Point { row: 3, col: 9 }, Some(Player::White)),
                5569741163936435548,
            ),
            ((Point { row: 3, col: 10 }, None), 9067722428313912262),
            (
                (Point { row: 3, col: 10 }, Some(Player::Black)),
                4723369690040452091,
            ),
            (
                (Point { row: 3, col: 10 }, Some(Player::White)),
                6378019894347085307,
            ),
            ((Point { row: 3, col: 11 }, None), 460746804383889904),
            (
                (Point { row: 3, col: 11 }, Some(Player::Black)),
                8100714703851785038,
            ),
            (
                (Point { row: 3, col: 11 }, Some(Player::White)),
                305499469080745306,
            ),
            ((Point { row: 3, col: 12 }, None), 5297676611173878612),
            (
                (Point { row: 3, col: 12 }, Some(Player::Black)),
                1927546534823338275,
            ),
            (
                (Point { row: 3, col: 12 }, Some(Player::White)),
                896019098429197548,
            ),
            ((Point { row: 3, col: 13 }, None), 1286564752928684810),
            (
                (Point { row: 3, col: 13 }, Some(Player::Black)),
                3212624103038956378,
            ),
            (
                (Point { row: 3, col: 13 }, Some(Player::White)),
                1020352911642252781,
            ),
            ((Point { row: 3, col: 14 }, None), 5449294960749378567),
            (
                (Point { row: 3, col: 14 }, Some(Player::Black)),
                1189228143402220965,
            ),
            (
                (Point { row: 3, col: 14 }, Some(Player::White)),
                7591720174038768320,
            ),
            ((Point { row: 3, col: 15 }, None), 6931960571125079076),
            (
                (Point { row: 3, col: 15 }, Some(Player::Black)),
                1055891884634482278,
            ),
            (
                (Point { row: 3, col: 15 }, Some(Player::White)),
                3800778189645971447,
            ),
            ((Point { row: 3, col: 16 }, None), 715891830713545573),
            (
                (Point { row: 3, col: 16 }, Some(Player::Black)),
                7852768451293332371,
            ),
            (
                (Point { row: 3, col: 16 }, Some(Player::White)),
                5078429455126308573,
            ),
            ((Point { row: 3, col: 17 }, None), 1778533326402989039),
            (
                (Point { row: 3, col: 17 }, Some(Player::Black)),
                2458344364044189676,
            ),
            (
                (Point { row: 3, col: 17 }, Some(Player::White)),
                3689804324801256999,
            ),
            ((Point { row: 3, col: 18 }, None), 2170672545712705912),
            (
                (Point { row: 3, col: 18 }, Some(Player::Black)),
                898230024106306560,
            ),
            (
                (Point { row: 3, col: 18 }, Some(Player::White)),
                1336093135229730915,
            ),
            ((Point { row: 3, col: 19 }, None), 8702635158959698093),
            (
                (Point { row: 3, col: 19 }, Some(Player::Black)),
                8425171920202584706,
            ),
            (
                (Point { row: 3, col: 19 }, Some(Player::White)),
                915181043986995702,
            ),
            ((Point { row: 4, col: 1 }, None), 4004871713182262070),
            (
                (Point { row: 4, col: 1 }, Some(Player::Black)),
                2821066776118968440,
            ),
            (
                (Point { row: 4, col: 1 }, Some(Player::White)),
                606591794771243444,
            ),
            ((Point { row: 4, col: 2 }, None), 5230740409529799347),
            (
                (Point { row: 4, col: 2 }, Some(Player::Black)),
                2209315354228125698,
            ),
            (
                (Point { row: 4, col: 2 }, Some(Player::White)),
                4054461639177786028,
            ),
            ((Point { row: 4, col: 3 }, None), 7152330581273339063),
            (
                (Point { row: 4, col: 3 }, Some(Player::Black)),
                1725271973127949257,
            ),
            (
                (Point { row: 4, col: 3 }, Some(Player::White)),
                8476625611840964372,
            ),
            ((Point { row: 4, col: 4 }, None), 7083050387819501721),
            (
                (Point { row: 4, col: 4 }, Some(Player::Black)),
                8531110550199435366,
            ),
            (
                (Point { row: 4, col: 4 }, Some(Player::White)),
                8808199961093427400,
            ),
            ((Point { row: 4, col: 5 }, None), 1011681971776576307),
            (
                (Point { row: 4, col: 5 }, Some(Player::Black)),
                7907947182356614813,
            ),
            (
                (Point { row: 4, col: 5 }, Some(Player::White)),
                5256367253333525286,
            ),
            ((Point { row: 4, col: 6 }, None), 8090533373807174698),
            (
                (Point { row: 4, col: 6 }, Some(Player::Black)),
                7592576832065363597,
            ),
            (
                (Point { row: 4, col: 6 }, Some(Player::White)),
                9108827166927302462,
            ),
            ((Point { row: 4, col: 7 }, None), 148442356257871513),
            (
                (Point { row: 4, col: 7 }, Some(Player::Black)),
                5610751411300577227,
            ),
            (
                (Point { row: 4, col: 7 }, Some(Player::White)),
                7691118390552077387,
            ),
            ((Point { row: 4, col: 8 }, None), 1342630896765287083),
            (
                (Point { row: 4, col: 8 }, Some(Player::Black)),
                6123849063435974300,
            ),
            (
                (Point { row: 4, col: 8 }, Some(Player::White)),
                8196583757814147075,
            ),
            ((Point { row: 4, col: 9 }, None), 2129147695078284758),
            (
                (Point { row: 4, col: 9 }, Some(Player::Black)),
                9111181793229849285,
            ),
            (
                (Point { row: 4, col: 9 }, Some(Player::White)),
                5479420289394836701,
            ),
            ((Point { row: 4, col: 10 }, None), 5923407029508913502),
            (
                (Point { row: 4, col: 10 }, Some(Player::Black)),
                8044078599318789644,
            ),
            (
                (Point { row: 4, col: 10 }, Some(Player::White)),
                5479153792641138286,
            ),
            ((Point { row: 4, col: 11 }, None), 393887881715599288),
            (
                (Point { row: 4, col: 11 }, Some(Player::Black)),
                726705351862230621,
            ),
            (
                (Point { row: 4, col: 11 }, Some(Player::White)),
                8139510337319572969,
            ),
            ((Point { row: 4, col: 12 }, None), 2822056917918869985),
            (
                (Point { row: 4, col: 12 }, Some(Player::Black)),
                2433442482806939237,
            ),
            (
                (Point { row: 4, col: 12 }, Some(Player::White)),
                3125525600398009853,
            ),
            ((Point { row: 4, col: 13 }, None), 5684229964252598350),
            (
                (Point { row: 4, col: 13 }, Some(Player::Black)),
                4686403392452973091,
            ),
            (
                (Point { row: 4, col: 13 }, Some(Player::White)),
                6085779330032826349,
            ),
            ((Point { row: 4, col: 14 }, None), 7826091629117900271),
            (
                (Point { row: 4, col: 14 }, Some(Player::Black)),
                8931528487227229778,
            ),
            (
                (Point { row: 4, col: 14 }, Some(Player::White)),
                4535458209118169084,
            ),
            ((Point { row: 4, col: 15 }, None), 1833797249778996213),
            (
                (Point { row: 4, col: 15 }, Some(Player::Black)),
                2965594857841836131,
            ),
            (
                (Point { row: 4, col: 15 }, Some(Player::White)),
                4097061679336533589,
            ),
            ((Point { row: 4, col: 16 }, None), 943390866590328013),
            (
                (Point { row: 4, col: 16 }, Some(Player::Black)),
                6711478166371935177,
            ),
            (
                (Point { row: 4, col: 16 }, Some(Player::White)),
                3101246156323461394,
            ),
            ((Point { row: 4, col: 17 }, None), 3173861742912989385),
            (
                (Point { row: 4, col: 17 }, Some(Player::Black)),
                8548142168299395065,
            ),
            (
                (Point { row: 4, col: 17 }, Some(Player::White)),
                5335277406341172729,
            ),
            ((Point { row: 4, col: 18 }, None), 2203934612495904377),
            (
                (Point { row: 4, col: 18 }, Some(Player::Black)),
                4062952182080696242,
            ),
            (
                (Point { row: 4, col: 18 }, Some(Player::White)),
                4159099634239066734,
            ),
            ((Point { row: 4, col: 19 }, None), 1279838196939269452),
            (
                (Point { row: 4, col: 19 }, Some(Player::Black)),
                8838438865276977743,
            ),
            (
                (Point { row: 4, col: 19 }, Some(Player::White)),
                9213692649117641996,
            ),
            ((Point { row: 5, col: 1 }, None), 3970787424252681183),
            (
                (Point { row: 5, col: 1 }, Some(Player::Black)),
                9121673643611892697,
            ),
            (
                (Point { row: 5, col: 1 }, Some(Player::White)),
                5357186677382163771,
            ),
            ((Point { row: 5, col: 2 }, None), 4932472531976974592),
            (
                (Point { row: 5, col: 2 }, Some(Player::Black)),
                714832167672743793,
            ),
            (
                (Point { row: 5, col: 2 }, Some(Player::White)),
                2437081593430763803,
            ),
            ((Point { row: 5, col: 3 }, None), 8256778408589110131),
            (
                (Point { row: 5, col: 3 }, Some(Player::Black)),
                6080470963214502273,
            ),
            (
                (Point { row: 5, col: 3 }, Some(Player::White)),
                2495757575503324631,
            ),
            ((Point { row: 5, col: 4 }, None), 89160742088962376),
            (
                (Point { row: 5, col: 4 }, Some(Player::Black)),
                2416021242026214464,
            ),
            (
                (Point { row: 5, col: 4 }, Some(Player::White)),
                8327480479404806966,
            ),
            ((Point { row: 5, col: 5 }, None), 6540335895073663871),
            (
                (Point { row: 5, col: 5 }, Some(Player::Black)),
                1122643827088857012,
            ),
            (
                (Point { row: 5, col: 5 }, Some(Player::White)),
                482721879645106227,
            ),
            ((Point { row: 5, col: 6 }, None), 3289361158875287472),
            (
                (Point { row: 5, col: 6 }, Some(Player::Black)),
                8704118699776142494,
            ),
            (
                (Point { row: 5, col: 6 }, Some(Player::White)),
                2996745166287033854,
            ),
            ((Point { row: 5, col: 7 }, None), 2377104713468886038),
            (
                (Point { row: 5, col: 7 }, Some(Player::Black)),
                4141699207365748082,
            ),
            (
                (Point { row: 5, col: 7 }, Some(Player::White)),
                862169453568162505,
            ),
            ((Point { row: 5, col: 8 }, None), 4917939387271042997),
            (
                (Point { row: 5, col: 8 }, Some(Player::Black)),
                1484651023567990862,
            ),
            (
                (Point { row: 5, col: 8 }, Some(Player::White)),
                5673769096719125195,
            ),
            ((Point { row: 5, col: 9 }, None), 3020492033375902908),
            (
                (Point { row: 5, col: 9 }, Some(Player::Black)),
                1775183889184447121,
            ),
            (
                (Point { row: 5, col: 9 }, Some(Player::White)),
                631398328521238939,
            ),
            ((Point { row: 5, col: 10 }, None), 6981674218688302866),
            (
                (Point { row: 5, col: 10 }, Some(Player::Black)),
                2273351100975277247,
            ),
            (
                (Point { row: 5, col: 10 }, Some(Player::White)),
                5777225017176976571,
            ),
            ((Point { row: 5, col: 11 }, None), 4330705311243731973),
            (
                (Point { row: 5, col: 11 }, Some(Player::Black)),
                3963482564690695107,
            ),
            (
                (Point { row: 5, col: 11 }, Some(Player::White)),
                2288727862300371672,
            ),
            ((Point { row: 5, col: 12 }, None), 9014811060966004298),
            (
                (Point { row: 5, col: 12 }, Some(Player::Black)),
                4289286217307604131,
            ),
            (
                (Point { row: 5, col: 12 }, Some(Player::White)),
                697355111142392607,
            ),
            ((Point { row: 5, col: 13 }, None), 8235685956105459657),
            (
                (Point { row: 5, col: 13 }, Some(Player::Black)),
                8168007848538534157,
            ),
            (
                (Point { row: 5, col: 13 }, Some(Player::White)),
                2804272276044443686,
            ),
            ((Point { row: 5, col: 14 }, None), 8225900234506491415),
            (
                (Point { row: 5, col: 14 }, Some(Player::Black)),
                2200277323150620183,
            ),
            (
                (Point { row: 5, col: 14 }, Some(Player::White)),
                203974965768980816,
            ),
            ((Point { row: 5, col: 15 }, None), 2752315560833329864),
            (
                (Point { row: 5, col: 15 }, Some(Player::Black)),
                1237373329060552168,
            ),
            (
                (Point { row: 5, col: 15 }, Some(Player::White)),
                2457801072451683571,
            ),
            ((Point { row: 5, col: 16 }, None), 9206495611621710488),
            (
                (Point { row: 5, col: 16 }, Some(Player::Black)),
                7479150707332996833,
            ),
            (
                (Point { row: 5, col: 16 }, Some(Player::White)),
                653894659358868314,
            ),
            ((Point { row: 5, col: 17 }, None), 6953138104722064453),
            (
                (Point { row: 5, col: 17 }, Some(Player::Black)),
                6400962948817612487,
            ),
            (
                (Point { row: 5, col: 17 }, Some(Player::White)),
                2768989623510194233,
            ),
            ((Point { row: 5, col: 18 }, None), 7417012331464802806),
            (
                (Point { row: 5, col: 18 }, Some(Player::Black)),
                3980447420236150362,
            ),
            (
                (Point { row: 5, col: 18 }, Some(Player::White)),
                6919602418177107430,
            ),
            ((Point { row: 5, col: 19 }, None), 5668464032013017166),
            (
                (Point { row: 5, col: 19 }, Some(Player::Black)),
                8757112649996924998,
            ),
            (
                (Point { row: 5, col: 19 }, Some(Player::White)),
                6290723394473004617,
            ),
            ((Point { row: 6, col: 1 }, None), 7238406206765822665),
            (
                (Point { row: 6, col: 1 }, Some(Player::Black)),
                423812617168567217,
            ),
            (
                (Point { row: 6, col: 1 }, Some(Player::White)),
                3104019267489386807,
            ),
            ((Point { row: 6, col: 2 }, None), 2765613177652849486),
            (
                (Point { row: 6, col: 2 }, Some(Player::Black)),
                4362573117234489564,
            ),
            (
                (Point { row: 6, col: 2 }, Some(Player::White)),
                4916634655561473640,
            ),
            ((Point { row: 6, col: 3 }, None), 2657467858928810457),
            (
                (Point { row: 6, col: 3 }, Some(Player::Black)),
                3735127098437132220,
            ),
            (
                (Point { row: 6, col: 3 }, Some(Player::White)),
                170273614138368740,
            ),
            ((Point { row: 6, col: 4 }, None), 7029981726712463215),
            (
                (Point { row: 6, col: 4 }, Some(Player::Black)),
                8526383184719402860,
            ),
            (
                (Point { row: 6, col: 4 }, Some(Player::White)),
                7823091606639214527,
            ),
            ((Point { row: 6, col: 5 }, None), 2860601882956863443),
            (
                (Point { row: 6, col: 5 }, Some(Player::Black)),
                3122201562173091301,
            ),
            (
                (Point { row: 6, col: 5 }, Some(Player::White)),
                2822640494234751573,
            ),
            ((Point { row: 6, col: 6 }, None), 8157066395471718803),
            (
                (Point { row: 6, col: 6 }, Some(Player::Black)),
                5551543367155764251,
            ),
            (
                (Point { row: 6, col: 6 }, Some(Player::White)),
                3323265190680832478,
            ),
            ((Point { row: 6, col: 7 }, None), 6286132655359072642),
            (
                (Point { row: 6, col: 7 }, Some(Player::Black)),
                6092326308893543831,
            ),
            (
                (Point { row: 6, col: 7 }, Some(Player::White)),
                5832654969028746701,
            ),
            ((Point { row: 6, col: 8 }, None), 1211077220123420512),
            (
                (Point { row: 6, col: 8 }, Some(Player::Black)),
                627083288764982946,
            ),
            (
                (Point { row: 6, col: 8 }, Some(Player::White)),
                5563068161605367514,
            ),
            ((Point { row: 6, col: 9 }, None), 2611182155893117641),
            (
                (Point { row: 6, col: 9 }, Some(Player::Black)),
                553160645565891622,
            ),
            (
                (Point { row: 6, col: 9 }, Some(Player::White)),
                2201330596883065552,
            ),
            ((Point { row: 6, col: 10 }, None), 4003057723226907800),
            (
                (Point { row: 6, col: 10 }, Some(Player::Black)),
                4216348254493350495,
            ),
            (
                (Point { row: 6, col: 10 }, Some(Player::White)),
                6796279568887386894,
            ),
            ((Point { row: 6, col: 11 }, None), 5340047233161030150),
            (
                (Point { row: 6, col: 11 }, Some(Player::Black)),
                776091550552755915,
            ),
            (
                (Point { row: 6, col: 11 }, Some(Player::White)),
                6969021693827360652,
            ),
            ((Point { row: 6, col: 12 }, None), 6197898450384910086),
            (
                (Point { row: 6, col: 12 }, Some(Player::Black)),
                8829570073548991273,
            ),
            (
                (Point { row: 6, col: 12 }, Some(Player::White)),
                8769820237202793209,
            ),
            ((Point { row: 6, col: 13 }, None), 5291242650150271421),
            (
                (Point { row: 6, col: 13 }, Some(Player::Black)),
                6662086761910395834,
            ),
            (
                (Point { row: 6, col: 13 }, Some(Player::White)),
                3457949163291068790,
            ),
            ((Point { row: 6, col: 14 }, None), 4232629379719922985),
            (
                (Point { row: 6, col: 14 }, Some(Player::Black)),
                6544339295308139893,
            ),
            (
                (Point { row: 6, col: 14 }, Some(Player::White)),
                5399631532724418690,
            ),
            ((Point { row: 6, col: 15 }, None), 2613162652827432819),
            (
                (Point { row: 6, col: 15 }, Some(Player::Black)),
                2049677909257218173,
            ),
            (
                (Point { row: 6, col: 15 }, Some(Player::White)),
                1464310709531746579,
            ),
            ((Point { row: 6, col: 16 }, None), 5464082436111998573),
            (
                (Point { row: 6, col: 16 }, Some(Player::Black)),
                8795629608094445377,
            ),
            (
                (Point { row: 6, col: 16 }, Some(Player::White)),
                4842434302155018932,
            ),
            ((Point { row: 6, col: 17 }, None), 6727888049076736664),
            (
                (Point { row: 6, col: 17 }, Some(Player::Black)),
                5100997362160864456,
            ),
            (
                (Point { row: 6, col: 17 }, Some(Player::White)),
                5540122784933976079,
            ),
            ((Point { row: 6, col: 18 }, None), 3891535108343851883),
            (
                (Point { row: 6, col: 18 }, Some(Player::Black)),
                6932220238214217651,
            ),
            (
                (Point { row: 6, col: 18 }, Some(Player::White)),
                3024710041380052487,
            ),
            ((Point { row: 6, col: 19 }, None), 5157851649025310803),
            (
                (Point { row: 6, col: 19 }, Some(Player::Black)),
                3859403834721659588,
            ),
            (
                (Point { row: 6, col: 19 }, Some(Player::White)),
                5734093285794028742,
            ),
            ((Point { row: 7, col: 1 }, None), 8754311069462869784),
            (
                (Point { row: 7, col: 1 }, Some(Player::Black)),
                4135756281096723130,
            ),
            (
                (Point { row: 7, col: 1 }, Some(Player::White)),
                2660262621554400597,
            ),
            ((Point { row: 7, col: 2 }, None), 3186047861180874773),
            (
                (Point { row: 7, col: 2 }, Some(Player::Black)),
                8437617441112518392,
            ),
            (
                (Point { row: 7, col: 2 }, Some(Player::White)),
                5700847880115193089,
            ),
            ((Point { row: 7, col: 3 }, None), 2451349687867204270),
            (
                (Point { row: 7, col: 3 }, Some(Player::Black)),
                5024753648402413878,
            ),
            (
                (Point { row: 7, col: 3 }, Some(Player::White)),
                4707342013474535257,
            ),
            ((Point { row: 7, col: 4 }, None), 894458264737334111),
            (
                (Point { row: 7, col: 4 }, Some(Player::Black)),
                7400549340701852518,
            ),
            (
                (Point { row: 7, col: 4 }, Some(Player::White)),
                4982178515371346946,
            ),
            ((Point { row: 7, col: 5 }, None), 7889078960323713327),
            (
                (Point { row: 7, col: 5 }, Some(Player::Black)),
                3463688502071051813,
            ),
            (
                (Point { row: 7, col: 5 }, Some(Player::White)),
                5439043411360527203,
            ),
            ((Point { row: 7, col: 6 }, None), 8158999343965029092),
            (
                (Point { row: 7, col: 6 }, Some(Player::Black)),
                4417991440989120207,
            ),
            (
                (Point { row: 7, col: 6 }, Some(Player::White)),
                7911029905557355535,
            ),
            ((Point { row: 7, col: 7 }, None), 1269318264525121990),
            (
                (Point { row: 7, col: 7 }, Some(Player::Black)),
                5526412712629111132,
            ),
            (
                (Point { row: 7, col: 7 }, Some(Player::White)),
                7056989575271047877,
            ),
            ((Point { row: 7, col: 8 }, None), 7310640724609976681),
            (
                (Point { row: 7, col: 8 }, Some(Player::Black)),
                8450643432045855842,
            ),
            (
                (Point { row: 7, col: 8 }, Some(Player::White)),
                2142109716255387536,
            ),
            ((Point { row: 7, col: 9 }, None), 2372815664597218093),
            (
                (Point { row: 7, col: 9 }, Some(Player::Black)),
                5788558840708187518,
            ),
            (
                (Point { row: 7, col: 9 }, Some(Player::White)),
                3942803655429791644,
            ),
            ((Point { row: 7, col: 10 }, None), 6132824217092640719),
            (
                (Point { row: 7, col: 10 }, Some(Player::Black)),
                3172679522121416254,
            ),
            (
                (Point { row: 7, col: 10 }, Some(Player::White)),
                1516842868586377042,
            ),
            ((Point { row: 7, col: 11 }, None), 4781290152295674513),
            (
                (Point { row: 7, col: 11 }, Some(Player::Black)),
                4930726143183960084,
            ),
            (
                (Point { row: 7, col: 11 }, Some(Player::White)),
                2266423932357799570,
            ),
            ((Point { row: 7, col: 12 }, None), 7235698355734347131),
            (
                (Point { row: 7, col: 12 }, Some(Player::Black)),
                165973395613352972,
            ),
            (
                (Point { row: 7, col: 12 }, Some(Player::White)),
                4371681950815139985,
            ),
            ((Point { row: 7, col: 13 }, None), 8393902247179788641),
            (
                (Point { row: 7, col: 13 }, Some(Player::Black)),
                7590340859497507303,
            ),
            (
                (Point { row: 7, col: 13 }, Some(Player::White)),
                4206211316467976619,
            ),
            ((Point { row: 7, col: 14 }, None), 8229545792753994899),
            (
                (Point { row: 7, col: 14 }, Some(Player::Black)),
                838711893588053696,
            ),
            (
                (Point { row: 7, col: 14 }, Some(Player::White)),
                4477295381157072518,
            ),
            ((Point { row: 7, col: 15 }, None), 4173090253482017435),
            (
                (Point { row: 7, col: 15 }, Some(Player::Black)),
                4072906910742500792,
            ),
            (
                (Point { row: 7, col: 15 }, Some(Player::White)),
                4907262873264099313,
            ),
            ((Point { row: 7, col: 16 }, None), 5867380728191338030),
            (
                (Point { row: 7, col: 16 }, Some(Player::Black)),
                8135365657022021506,
            ),
            (
                (Point { row: 7, col: 16 }, Some(Player::White)),
                4189409989034502083,
            ),
            ((Point { row: 7, col: 17 }, None), 5874705238031490949),
            (
                (Point { row: 7, col: 17 }, Some(Player::Black)),
                1337494314650889511,
            ),
            (
                (Point { row: 7, col: 17 }, Some(Player::White)),
                3148591770173058627,
            ),
            ((Point { row: 7, col: 18 }, None), 3722649922881024055),
            (
                (Point { row: 7, col: 18 }, Some(Player::Black)),
                3934330035566479726,
            ),
            (
                (Point { row: 7, col: 18 }, Some(Player::White)),
                6860352147551392319,
            ),
            ((Point { row: 7, col: 19 }, None), 4259121313222633299),
            (
                (Point { row: 7, col: 19 }, Some(Player::Black)),
                6710991919328058302,
            ),
            (
                (Point { row: 7, col: 19 }, Some(Player::White)),
                729487318318649743,
            ),
            ((Point { row: 8, col: 1 }, None), 2708867513683618113),
            (
                (Point { row: 8, col: 1 }, Some(Player::Black)),
                8284401402821228502,
            ),
            (
                (Point { row: 8, col: 1 }, Some(Player::White)),
                266774759783490574,
            ),
            ((Point { row: 8, col: 2 }, None), 5918755146055395138),
            (
                (Point { row: 8, col: 2 }, Some(Player::Black)),
                5702689780266635942,
            ),
            (
                (Point { row: 8, col: 2 }, Some(Player::White)),
                2220258171102127975,
            ),
            ((Point { row: 8, col: 3 }, None), 6653466860005395853),
            (
                (Point { row: 8, col: 3 }, Some(Player::Black)),
                5694510591981484870,
            ),
            (
                (Point { row: 8, col: 3 }, Some(Player::White)),
                2556856366938374080,
            ),
            ((Point { row: 8, col: 4 }, None), 8020600426333158407),
            (
                (Point { row: 8, col: 4 }, Some(Player::Black)),
                6356921100593396180,
            ),
            (
                (Point { row: 8, col: 4 }, Some(Player::White)),
                5149748205727967954,
            ),
            ((Point { row: 8, col: 5 }, None), 3051160807432229960),
            (
                (Point { row: 8, col: 5 }, Some(Player::Black)),
                260378505723678285,
            ),
            (
                (Point { row: 8, col: 5 }, Some(Player::White)),
                1822942542127719912,
            ),
            ((Point { row: 8, col: 6 }, None), 2059368881907087243),
            (
                (Point { row: 8, col: 6 }, Some(Player::Black)),
                7630446262559191057,
            ),
            (
                (Point { row: 8, col: 6 }, Some(Player::White)),
                2352065658131437765,
            ),
            ((Point { row: 8, col: 7 }, None), 1000694156378281597),
            (
                (Point { row: 8, col: 7 }, Some(Player::Black)),
                6211828436070597931,
            ),
            (
                (Point { row: 8, col: 7 }, Some(Player::White)),
                9013248265871745105,
            ),
            ((Point { row: 8, col: 8 }, None), 2815401307423159003),
            (
                (Point { row: 8, col: 8 }, Some(Player::Black)),
                7483337698674146232,
            ),
            (
                (Point { row: 8, col: 8 }, Some(Player::White)),
                9052402633085182507,
            ),
            ((Point { row: 8, col: 9 }, None), 6384879156631677794),
            (
                (Point { row: 8, col: 9 }, Some(Player::Black)),
                8426441507681753452,
            ),
            (
                (Point { row: 8, col: 9 }, Some(Player::White)),
                1854388785471876700,
            ),
            ((Point { row: 8, col: 10 }, None), 142227175561852656),
            (
                (Point { row: 8, col: 10 }, Some(Player::Black)),
                65994339143027192,
            ),
            (
                (Point { row: 8, col: 10 }, Some(Player::White)),
                1426413243849234911,
            ),
            ((Point { row: 8, col: 11 }, None), 8430986503980830376),
            (
                (Point { row: 8, col: 11 }, Some(Player::Black)),
                619817367324323140,
            ),
            (
                (Point { row: 8, col: 11 }, Some(Player::White)),
                4677987248292530730,
            ),
            ((Point { row: 8, col: 12 }, None), 1508615909331847777),
            (
                (Point { row: 8, col: 12 }, Some(Player::Black)),
                8162333180096663164,
            ),
            (
                (Point { row: 8, col: 12 }, Some(Player::White)),
                5852637502964969314,
            ),
            ((Point { row: 8, col: 13 }, None), 3275974798452305241),
            (
                (Point { row: 8, col: 13 }, Some(Player::Black)),
                631782669438325441,
            ),
            (
                (Point { row: 8, col: 13 }, Some(Player::White)),
                8555593582322512786,
            ),
            ((Point { row: 8, col: 14 }, None), 4134515947666502889),
            (
                (Point { row: 8, col: 14 }, Some(Player::Black)),
                4197594044100261270,
            ),
            (
                (Point { row: 8, col: 14 }, Some(Player::White)),
                2117711969623158939,
            ),
            ((Point { row: 8, col: 15 }, None), 2704155668255596809),
            (
                (Point { row: 8, col: 15 }, Some(Player::Black)),
                8386704845222266124,
            ),
            (
                (Point { row: 8, col: 15 }, Some(Player::White)),
                6995258630292617909,
            ),
            ((Point { row: 8, col: 16 }, None), 4620356030723102193),
            (
                (Point { row: 8, col: 16 }, Some(Player::Black)),
                8844257309346066382,
            ),
            (
                (Point { row: 8, col: 16 }, Some(Player::White)),
                4397164639691657310,
            ),
            ((Point { row: 8, col: 17 }, None), 6924436026869042932),
            (
                (Point { row: 8, col: 17 }, Some(Player::Black)),
                7384357565442669286,
            ),
            (
                (Point { row: 8, col: 17 }, Some(Player::White)),
                7926866768428985043,
            ),
            ((Point { row: 8, col: 18 }, None), 1215192570725879370),
            (
                (Point { row: 8, col: 18 }, Some(Player::Black)),
                5352654741175191994,
            ),
            (
                (Point { row: 8, col: 18 }, Some(Player::White)),
                7230246477789996181,
            ),
            ((Point { row: 8, col: 19 }, None), 4928595744231691382),
            (
                (Point { row: 8, col: 19 }, Some(Player::Black)),
                8616104833462014511,
            ),
            (
                (Point { row: 8, col: 19 }, Some(Player::White)),
                1892938770974011968,
            ),
            ((Point { row: 9, col: 1 }, None), 6743386700188933609),
            (
                (Point { row: 9, col: 1 }, Some(Player::Black)),
                3950154523875636246,
            ),
            (
                (Point { row: 9, col: 1 }, Some(Player::White)),
                6192177334781799846,
            ),
            ((Point { row: 9, col: 2 }, None), 1158215549499412747),
            (
                (Point { row: 9, col: 2 }, Some(Player::Black)),
                5196418744428607937,
            ),
            (
                (Point { row: 9, col: 2 }, Some(Player::White)),
                2762022000374018832,
            ),
            ((Point { row: 9, col: 3 }, None), 3437874067203692545),
            (
                (Point { row: 9, col: 3 }, Some(Player::Black)),
                3176712490511609694,
            ),
            (
                (Point { row: 9, col: 3 }, Some(Player::White)),
                5532505359918782509,
            ),
            ((Point { row: 9, col: 4 }, None), 1681133576289494651),
            (
                (Point { row: 9, col: 4 }, Some(Player::Black)),
                3265322080508945804,
            ),
            (
                (Point { row: 9, col: 4 }, Some(Player::White)),
                8813436625971083059,
            ),
            ((Point { row: 9, col: 5 }, None), 4809082433745526062),
            (
                (Point { row: 9, col: 5 }, Some(Player::Black)),
                8937671368406148786,
            ),
            (
                (Point { row: 9, col: 5 }, Some(Player::White)),
                7053786125698232633,
            ),
            ((Point { row: 9, col: 6 }, None), 6949320065309673474),
            (
                (Point { row: 9, col: 6 }, Some(Player::Black)),
                8712247726931233578,
            ),
            (
                (Point { row: 9, col: 6 }, Some(Player::White)),
                4800931047663682160,
            ),
            ((Point { row: 9, col: 7 }, None), 5528666175740154755),
            (
                (Point { row: 9, col: 7 }, Some(Player::Black)),
                1327409937908817374,
            ),
            (
                (Point { row: 9, col: 7 }, Some(Player::White)),
                4968243361936629567,
            ),
            ((Point { row: 9, col: 8 }, None), 1766252635293279171),
            (
                (Point { row: 9, col: 8 }, Some(Player::Black)),
                8371827633496203896,
            ),
            (
                (Point { row: 9, col: 8 }, Some(Player::White)),
                7664536919037887122,
            ),
            ((Point { row: 9, col: 9 }, None), 834007951221734505),
            (
                (Point { row: 9, col: 9 }, Some(Player::Black)),
                1372011264285931700,
            ),
            (
                (Point { row: 9, col: 9 }, Some(Player::White)),
                6958136041648042869,
            ),
            ((Point { row: 9, col: 10 }, None), 8335642646378469991),
            (
                (Point { row: 9, col: 10 }, Some(Player::Black)),
                8846790262314288081,
            ),
            (
                (Point { row: 9, col: 10 }, Some(Player::White)),
                4754132456638923854,
            ),
            ((Point { row: 9, col: 11 }, None), 728530736027322184),
            (
                (Point { row: 9, col: 11 }, Some(Player::Black)),
                1223080923982888743,
            ),
            (
                (Point { row: 9, col: 11 }, Some(Player::White)),
                9083073265780692130,
            ),
            ((Point { row: 9, col: 12 }, None), 8398287296122274253),
            (
                (Point { row: 9, col: 12 }, Some(Player::Black)),
                917843401066074734,
            ),
            (
                (Point { row: 9, col: 12 }, Some(Player::White)),
                3892412035384001864,
            ),
            ((Point { row: 9, col: 13 }, None), 5810603813584124513),
            (
                (Point { row: 9, col: 13 }, Some(Player::Black)),
                2044483860325592702,
            ),
            (
                (Point { row: 9, col: 13 }, Some(Player::White)),
                3568871320809511757,
            ),
            ((Point { row: 9, col: 14 }, None), 7084073977275282610),
            (
                (Point { row: 9, col: 14 }, Some(Player::Black)),
                308871768499422692,
            ),
            (
                (Point { row: 9, col: 14 }, Some(Player::White)),
                2975360071744976769,
            ),
            ((Point { row: 9, col: 15 }, None), 965035502713851221),
            (
                (Point { row: 9, col: 15 }, Some(Player::Black)),
                4686998987103768104,
            ),
            (
                (Point { row: 9, col: 15 }, Some(Player::White)),
                6763319247795544372,
            ),
            ((Point { row: 9, col: 16 }, None), 335543540674458061),
            (
                (Point { row: 9, col: 16 }, Some(Player::Black)),
                8858140030017112705,
            ),
            (
                (Point { row: 9, col: 16 }, Some(Player::White)),
                7051082800217888273,
            ),
            ((Point { row: 9, col: 17 }, None), 5822265270841595340),
            (
                (Point { row: 9, col: 17 }, Some(Player::Black)),
                1571417258931187188,
            ),
            (
                (Point { row: 9, col: 17 }, Some(Player::White)),
                6177082688753647598,
            ),
            ((Point { row: 9, col: 18 }, None), 8031848734766952378),
            (
                (Point { row: 9, col: 18 }, Some(Player::Black)),
                5983730123854807911,
            ),
            (
                (Point { row: 9, col: 18 }, Some(Player::White)),
                696389077836533043,
            ),
            ((Point { row: 9, col: 19 }, None), 2882479314766007597),
            (
                (Point { row: 9, col: 19 }, Some(Player::Black)),
                8713061394733110502,
            ),
            (
                (Point { row: 9, col: 19 }, Some(Player::White)),
                1050853813804221676,
            ),
            ((Point { row: 10, col: 1 }, None), 1950234272191381469),
            (
                (Point { row: 10, col: 1 }, Some(Player::Black)),
                5175079759805257525,
            ),
            (
                (Point { row: 10, col: 1 }, Some(Player::White)),
                4757374552741113331,
            ),
            ((Point { row: 10, col: 2 }, None), 8813779413874243703),
            (
                (Point { row: 10, col: 2 }, Some(Player::Black)),
                2093855662644194547,
            ),
            (
                (Point { row: 10, col: 2 }, Some(Player::White)),
                7326110078467404228,
            ),
            ((Point { row: 10, col: 3 }, None), 1896199623460553578),
            (
                (Point { row: 10, col: 3 }, Some(Player::Black)),
                3494786890845576880,
            ),
            (
                (Point { row: 10, col: 3 }, Some(Player::White)),
                4822579479625204366,
            ),
            ((Point { row: 10, col: 4 }, None), 4110133747305616117),
            (
                (Point { row: 10, col: 4 }, Some(Player::Black)),
                6809121397298136095,
            ),
            (
                (Point { row: 10, col: 4 }, Some(Player::White)),
                1142547570472052062,
            ),
            ((Point { row: 10, col: 5 }, None), 3920574724383796057),
            (
                (Point { row: 10, col: 5 }, Some(Player::Black)),
                8490022816923582618,
            ),
            (
                (Point { row: 10, col: 5 }, Some(Player::White)),
                7912568750051101101,
            ),
            ((Point { row: 10, col: 6 }, None), 2611484310505569377),
            (
                (Point { row: 10, col: 6 }, Some(Player::Black)),
                8026232239820375548,
            ),
            (
                (Point { row: 10, col: 6 }, Some(Player::White)),
                8243417593632187738,
            ),
            ((Point { row: 10, col: 7 }, None), 4606569151749803495),
            (
                (Point { row: 10, col: 7 }, Some(Player::Black)),
                7986741646505653418,
            ),
            (
                (Point { row: 10, col: 7 }, Some(Player::White)),
                4082810198274077169,
            ),
            ((Point { row: 10, col: 8 }, None), 2580857269279378932),
            (
                (Point { row: 10, col: 8 }, Some(Player::Black)),
                8706569330971570630,
            ),
            (
                (Point { row: 10, col: 8 }, Some(Player::White)),
                5645076841868909104,
            ),
            ((Point { row: 10, col: 9 }, None), 7424587179509651130),
            (
                (Point { row: 10, col: 9 }, Some(Player::Black)),
                2184698807655211705,
            ),
            (
                (Point { row: 10, col: 9 }, Some(Player::White)),
                3156600238538578747,
            ),
            ((Point { row: 10, col: 10 }, None), 3310293293796591705),
            (
                (Point { row: 10, col: 10 }, Some(Player::Black)),
                2282523187695085649,
            ),
            (
                (Point { row: 10, col: 10 }, Some(Player::White)),
                1907651172290516840,
            ),
            ((Point { row: 10, col: 11 }, None), 4081014091650373916),
            (
                (Point { row: 10, col: 11 }, Some(Player::Black)),
                860252683717739433,
            ),
            (
                (Point { row: 10, col: 11 }, Some(Player::White)),
                3113425945782685885,
            ),
            ((Point { row: 10, col: 12 }, None), 8496032273814956766),
            (
                (Point { row: 10, col: 12 }, Some(Player::Black)),
                8112914414093410853,
            ),
            (
                (Point { row: 10, col: 12 }, Some(Player::White)),
                5674386181867331442,
            ),
            ((Point { row: 10, col: 13 }, None), 460960048795654073),
            (
                (Point { row: 10, col: 13 }, Some(Player::Black)),
                2096970978170442551,
            ),
            (
                (Point { row: 10, col: 13 }, Some(Player::White)),
                65888111676722484,
            ),
            ((Point { row: 10, col: 14 }, None), 2150527785864224937),
            (
                (Point { row: 10, col: 14 }, Some(Player::Black)),
                6933109289082883821,
            ),
            (
                (Point { row: 10, col: 14 }, Some(Player::White)),
                5299541951288090098,
            ),
            ((Point { row: 10, col: 15 }, None), 2156046386939312224),
            (
                (Point { row: 10, col: 15 }, Some(Player::Black)),
                1577851428011824440,
            ),
            (
                (Point { row: 10, col: 15 }, Some(Player::White)),
                5883930708901612338,
            ),
            ((Point { row: 10, col: 16 }, None), 5894456861931156456),
            (
                (Point { row: 10, col: 16 }, Some(Player::Black)),
                1789633680523736723,
            ),
            (
                (Point { row: 10, col: 16 }, Some(Player::White)),
                2060351134511504306,
            ),
            ((Point { row: 10, col: 17 }, None), 6528956843065007787),
            (
                (Point { row: 10, col: 17 }, Some(Player::Black)),
                1750562304957394079,
            ),
            (
                (Point { row: 10, col: 17 }, Some(Player::White)),
                2614619476442229857,
            ),
            ((Point { row: 10, col: 18 }, None), 4285372643475982199),
            (
                (Point { row: 10, col: 18 }, Some(Player::Black)),
                1253633533403755994,
            ),
            (
                (Point { row: 10, col: 18 }, Some(Player::White)),
                7708574052894177945,
            ),
            ((Point { row: 10, col: 19 }, None), 1146181610758603993),
            (
                (Point { row: 10, col: 19 }, Some(Player::Black)),
                2907147452008658037,
            ),
            (
                (Point { row: 10, col: 19 }, Some(Player::White)),
                1624893857574645652,
            ),
            ((Point { row: 11, col: 1 }, None), 6600889724937353275),
            (
                (Point { row: 11, col: 1 }, Some(Player::Black)),
                5251013867839258866,
            ),
            (
                (Point { row: 11, col: 1 }, Some(Player::White)),
                182706400080058445,
            ),
            ((Point { row: 11, col: 2 }, None), 8452575339349872181),
            (
                (Point { row: 11, col: 2 }, Some(Player::Black)),
                1016978115270879205,
            ),
            (
                (Point { row: 11, col: 2 }, Some(Player::White)),
                3675955787731333716,
            ),
            ((Point { row: 11, col: 3 }, None), 7988035518794270861),
            (
                (Point { row: 11, col: 3 }, Some(Player::Black)),
                8013990723634243579,
            ),
            (
                (Point { row: 11, col: 3 }, Some(Player::White)),
                5188540287965720978,
            ),
            ((Point { row: 11, col: 4 }, None), 2626010928596301180),
            (
                (Point { row: 11, col: 4 }, Some(Player::Black)),
                8676164363279579784,
            ),
            (
                (Point { row: 11, col: 4 }, Some(Player::White)),
                6136845334252258012,
            ),
            ((Point { row: 11, col: 5 }, None), 7739883564023534804),
            (
                (Point { row: 11, col: 5 }, Some(Player::Black)),
                3870292117782701049,
            ),
            (
                (Point { row: 11, col: 5 }, Some(Player::White)),
                3379418719542278107,
            ),
            ((Point { row: 11, col: 6 }, None), 3191367889339035055),
            (
                (Point { row: 11, col: 6 }, Some(Player::Black)),
                3887270346160412488,
            ),
            (
                (Point { row: 11, col: 6 }, Some(Player::White)),
                8311930232282244556,
            ),
            ((Point { row: 11, col: 7 }, None), 1057014878116115124),
            (
                (Point { row: 11, col: 7 }, Some(Player::Black)),
                117733489483515854,
            ),
            (
                (Point { row: 11, col: 7 }, Some(Player::White)),
                283931416477096805,
            ),
            ((Point { row: 11, col: 8 }, None), 4288592364095064501),
            (
                (Point { row: 11, col: 8 }, Some(Player::Black)),
                4374889157940996903,
            ),
            (
                (Point { row: 11, col: 8 }, Some(Player::White)),
                5253619226708660761,
            ),
            ((Point { row: 11, col: 9 }, None), 462087282971616580),
            (
                (Point { row: 11, col: 9 }, Some(Player::Black)),
                6706345442552767344,
            ),
            (
                (Point { row: 11, col: 9 }, Some(Player::White)),
                7091181745596794369,
            ),
            ((Point { row: 11, col: 10 }, None), 4934056123847552657),
            (
                (Point { row: 11, col: 10 }, Some(Player::Black)),
                5070506725134408311,
            ),
            (
                (Point { row: 11, col: 10 }, Some(Player::White)),
                3795547590811991174,
            ),
            ((Point { row: 11, col: 11 }, None), 1297554240296777509),
            (
                (Point { row: 11, col: 11 }, Some(Player::Black)),
                1523276519119352578,
            ),
            (
                (Point { row: 11, col: 11 }, Some(Player::White)),
                8129907947090939767,
            ),
            ((Point { row: 11, col: 12 }, None), 9071521583323485530),
            (
                (Point { row: 11, col: 12 }, Some(Player::Black)),
                5078701995274613057,
            ),
            (
                (Point { row: 11, col: 12 }, Some(Player::White)),
                3789082397402468533,
            ),
            ((Point { row: 11, col: 13 }, None), 8203112540552071704),
            (
                (Point { row: 11, col: 13 }, Some(Player::Black)),
                1862648691335708078,
            ),
            (
                (Point { row: 11, col: 13 }, Some(Player::White)),
                3563230535496567664,
            ),
            ((Point { row: 11, col: 14 }, None), 5294618759427954761),
            (
                (Point { row: 11, col: 14 }, Some(Player::Black)),
                146067844120509946,
            ),
            (
                (Point { row: 11, col: 14 }, Some(Player::White)),
                3458225018555978949,
            ),
            ((Point { row: 11, col: 15 }, None), 8339043895005418257),
            (
                (Point { row: 11, col: 15 }, Some(Player::Black)),
                2177022254647070435,
            ),
            (
                (Point { row: 11, col: 15 }, Some(Player::White)),
                4164108108099808472,
            ),
            ((Point { row: 11, col: 16 }, None), 8992767240077238223),
            (
                (Point { row: 11, col: 16 }, Some(Player::Black)),
                8266732303263193660,
            ),
            (
                (Point { row: 11, col: 16 }, Some(Player::White)),
                7769342006007064958,
            ),
            ((Point { row: 11, col: 17 }, None), 8150564971404507664),
            (
                (Point { row: 11, col: 17 }, Some(Player::Black)),
                6165614977864587183,
            ),
            (
                (Point { row: 11, col: 17 }, Some(Player::White)),
                877272186653260412,
            ),
            ((Point { row: 11, col: 18 }, None), 7327898938204419879),
            (
                (Point { row: 11, col: 18 }, Some(Player::Black)),
                329182253602334078,
            ),
            (
                (Point { row: 11, col: 18 }, Some(Player::White)),
                2400841112928608,
            ),
            ((Point { row: 11, col: 19 }, None), 3977123413101628477),
            (
                (Point { row: 11, col: 19 }, Some(Player::Black)),
                5062038999931368386,
            ),
            (
                (Point { row: 11, col: 19 }, Some(Player::White)),
                2721390189772895402,
            ),
            ((Point { row: 12, col: 1 }, None), 4360033836765261028),
            (
                (Point { row: 12, col: 1 }, Some(Player::Black)),
                8204114360619351780,
            ),
            (
                (Point { row: 12, col: 1 }, Some(Player::White)),
                6954811528197685000,
            ),
            ((Point { row: 12, col: 2 }, None), 9036247333791858215),
            (
                (Point { row: 12, col: 2 }, Some(Player::Black)),
                2199614172029597010,
            ),
            (
                (Point { row: 12, col: 2 }, Some(Player::White)),
                1856723800642548680,
            ),
            ((Point { row: 12, col: 3 }, None), 5674729209171125804),
            (
                (Point { row: 12, col: 3 }, Some(Player::Black)),
                7883502993642673307,
            ),
            (
                (Point { row: 12, col: 3 }, Some(Player::White)),
                9110809727914459295,
            ),
            ((Point { row: 12, col: 4 }, None), 8083477365681075767),
            (
                (Point { row: 12, col: 4 }, Some(Player::Black)),
                7653855125822988063,
            ),
            (
                (Point { row: 12, col: 4 }, Some(Player::White)),
                2550946436039360190,
            ),
            ((Point { row: 12, col: 5 }, None), 3349324930458867787),
            (
                (Point { row: 12, col: 5 }, Some(Player::Black)),
                3252660232637555005,
            ),
            (
                (Point { row: 12, col: 5 }, Some(Player::White)),
                1234973871658377379,
            ),
            ((Point { row: 12, col: 6 }, None), 2372796904463625457),
            (
                (Point { row: 12, col: 6 }, Some(Player::Black)),
                2714474110718573658,
            ),
            (
                (Point { row: 12, col: 6 }, Some(Player::White)),
                2922932858700824842,
            ),
            ((Point { row: 12, col: 7 }, None), 6984466958382389975),
            (
                (Point { row: 12, col: 7 }, Some(Player::Black)),
                1304272032505844829,
            ),
            (
                (Point { row: 12, col: 7 }, Some(Player::White)),
                4412464546816746704,
            ),
            ((Point { row: 12, col: 8 }, None), 3476365136404014733),
            (
                (Point { row: 12, col: 8 }, Some(Player::Black)),
                6371091448955140245,
            ),
            (
                (Point { row: 12, col: 8 }, Some(Player::White)),
                1466560588593637266,
            ),
            ((Point { row: 12, col: 9 }, None), 4642717582336135617),
            (
                (Point { row: 12, col: 9 }, Some(Player::Black)),
                8948855720666141590,
            ),
            (
                (Point { row: 12, col: 9 }, Some(Player::White)),
                6213597555189260145,
            ),
            ((Point { row: 12, col: 10 }, None), 2671207131631953832),
            (
                (Point { row: 12, col: 10 }, Some(Player::Black)),
                2289855430531536781,
            ),
            (
                (Point { row: 12, col: 10 }, Some(Player::White)),
                1079994396639798172,
            ),
            ((Point { row: 12, col: 11 }, None), 3677910639930301919),
            (
                (Point { row: 12, col: 11 }, Some(Player::Black)),
                264717285611233850,
            ),
            (
                (Point { row: 12, col: 11 }, Some(Player::White)),
                6132419899442887956,
            ),
            ((Point { row: 12, col: 12 }, None), 5036790561745105113),
            (
                (Point { row: 12, col: 12 }, Some(Player::Black)),
                3894831455188040975,
            ),
            (
                (Point { row: 12, col: 12 }, Some(Player::White)),
                8397680417073351336,
            ),
            ((Point { row: 12, col: 13 }, None), 7486952213357644434),
            (
                (Point { row: 12, col: 13 }, Some(Player::Black)),
                8019055082110826633,
            ),
            (
                (Point { row: 12, col: 13 }, Some(Player::White)),
                2763271402461037383,
            ),
            ((Point { row: 12, col: 14 }, None), 4187848061922147931),
            (
                (Point { row: 12, col: 14 }, Some(Player::Black)),
                8212920524602598463,
            ),
            (
                (Point { row: 12, col: 14 }, Some(Player::White)),
                6059940661878438388,
            ),
            ((Point { row: 12, col: 15 }, None), 1706069461311095556),
            (
                (Point { row: 12, col: 15 }, Some(Player::Black)),
                800160352722785413,
            ),
            (
                (Point { row: 12, col: 15 }, Some(Player::White)),
                3325247341665160990,
            ),
            ((Point { row: 12, col: 16 }, None), 8027599743141047580),
            (
                (Point { row: 12, col: 16 }, Some(Player::Black)),
                445641289634904717,
            ),
            (
                (Point { row: 12, col: 16 }, Some(Player::White)),
                3567511526301907961,
            ),
            ((Point { row: 12, col: 17 }, None), 7385537632455963735),
            (
                (Point { row: 12, col: 17 }, Some(Player::Black)),
                62410261542762687,
            ),
            (
                (Point { row: 12, col: 17 }, Some(Player::White)),
                2167299545513438560,
            ),
            ((Point { row: 12, col: 18 }, None), 1985116491241899714),
            (
                (Point { row: 12, col: 18 }, Some(Player::Black)),
                6255202715009013310,
            ),
            (
                (Point { row: 12, col: 18 }, Some(Player::White)),
                8414900668028793600,
            ),
            ((Point { row: 12, col: 19 }, None), 2429276353133396490),
            (
                (Point { row: 12, col: 19 }, Some(Player::Black)),
                1847682822850556028,
            ),
            (
                (Point { row: 12, col: 19 }, Some(Player::White)),
                7348162734895443377,
            ),
            ((Point { row: 13, col: 1 }, None), 2740644359063616243),
            (
                (Point { row: 13, col: 1 }, Some(Player::Black)),
                4300965642275840365,
            ),
            (
                (Point { row: 13, col: 1 }, Some(Player::White)),
                7818198876660425920,
            ),
            ((Point { row: 13, col: 2 }, None), 7691740946004035704),
            (
                (Point { row: 13, col: 2 }, Some(Player::Black)),
                1739396463365117424,
            ),
            (
                (Point { row: 13, col: 2 }, Some(Player::White)),
                1205206435048225510,
            ),
            ((Point { row: 13, col: 3 }, None), 6071201435843963197),
            (
                (Point { row: 13, col: 3 }, Some(Player::Black)),
                1859468397498404977,
            ),
            (
                (Point { row: 13, col: 3 }, Some(Player::White)),
                8918623117509719449,
            ),
            ((Point { row: 13, col: 4 }, None), 1821748043159364911),
            (
                (Point { row: 13, col: 4 }, Some(Player::Black)),
                4631942953520085482,
            ),
            (
                (Point { row: 13, col: 4 }, Some(Player::White)),
                5013417302799730077,
            ),
            ((Point { row: 13, col: 5 }, None), 7862578350480619139),
            (
                (Point { row: 13, col: 5 }, Some(Player::Black)),
                1727990950976565183,
            ),
            (
                (Point { row: 13, col: 5 }, Some(Player::White)),
                6945730708205859564,
            ),
            ((Point { row: 13, col: 6 }, None), 1241167448281569216),
            (
                (Point { row: 13, col: 6 }, Some(Player::Black)),
                7159801058727899484,
            ),
            (
                (Point { row: 13, col: 6 }, Some(Player::White)),
                550198735512664108,
            ),
            ((Point { row: 13, col: 7 }, None), 1293671948176065378),
            (
                (Point { row: 13, col: 7 }, Some(Player::Black)),
                8954381719324200566,
            ),
            (
                (Point { row: 13, col: 7 }, Some(Player::White)),
                6469536102592732480,
            ),
            ((Point { row: 13, col: 8 }, None), 7309379715586934845),
            (
                (Point { row: 13, col: 8 }, Some(Player::Black)),
                3942431050182997112,
            ),
            (
                (Point { row: 13, col: 8 }, Some(Player::White)),
                3562871107319467957,
            ),
            ((Point { row: 13, col: 9 }, None), 3554368399876695750),
            (
                (Point { row: 13, col: 9 }, Some(Player::Black)),
                4820981967630837995,
            ),
            (
                (Point { row: 13, col: 9 }, Some(Player::White)),
                6506003644779812706,
            ),
            ((Point { row: 13, col: 10 }, None), 75165547083973152),
            (
                (Point { row: 13, col: 10 }, Some(Player::Black)),
                2726396203636676247,
            ),
            (
                (Point { row: 13, col: 10 }, Some(Player::White)),
                3528833049631923826,
            ),
            ((Point { row: 13, col: 11 }, None), 1312496585560927648),
            (
                (Point { row: 13, col: 11 }, Some(Player::Black)),
                1552363990659818621,
            ),
            (
                (Point { row: 13, col: 11 }, Some(Player::White)),
                3092638618649416205,
            ),
            ((Point { row: 13, col: 12 }, None), 7293160852328703843),
            (
                (Point { row: 13, col: 12 }, Some(Player::Black)),
                3028478959388987959,
            ),
            (
                (Point { row: 13, col: 12 }, Some(Player::White)),
                135459696849454266,
            ),
            ((Point { row: 13, col: 13 }, None), 8240027998143891221),
            (
                (Point { row: 13, col: 13 }, Some(Player::Black)),
                6832571697712871192,
            ),
            (
                (Point { row: 13, col: 13 }, Some(Player::White)),
                6670610197927269334,
            ),
            ((Point { row: 13, col: 14 }, None), 7897243161474883760),
            (
                (Point { row: 13, col: 14 }, Some(Player::Black)),
                4194661492642604232,
            ),
            (
                (Point { row: 13, col: 14 }, Some(Player::White)),
                8897469759656018470,
            ),
            ((Point { row: 13, col: 15 }, None), 3628909237245382490),
            (
                (Point { row: 13, col: 15 }, Some(Player::Black)),
                3621083416792383737,
            ),
            (
                (Point { row: 13, col: 15 }, Some(Player::White)),
                1044951076593964374,
            ),
            ((Point { row: 13, col: 16 }, None), 5074519954533631651),
            (
                (Point { row: 13, col: 16 }, Some(Player::Black)),
                4785826263584180796,
            ),
            (
                (Point { row: 13, col: 16 }, Some(Player::White)),
                2896995813186829821,
            ),
            ((Point { row: 13, col: 17 }, None), 2521897184292838739),
            (
                (Point { row: 13, col: 17 }, Some(Player::Black)),
                3726710280247918589,
            ),
            (
                (Point { row: 13, col: 17 }, Some(Player::White)),
                1346132077020756898,
            ),
            ((Point { row: 13, col: 18 }, None), 362426564225769883),
            (
                (Point { row: 13, col: 18 }, Some(Player::Black)),
                6262172930722527705,
            ),
            (
                (Point { row: 13, col: 18 }, Some(Player::White)),
                971261805966150392,
            ),
            ((Point { row: 13, col: 19 }, None), 2719861366189057701),
            (
                (Point { row: 13, col: 19 }, Some(Player::Black)),
                308866103452104365,
            ),
            (
                (Point { row: 13, col: 19 }, Some(Player::White)),
                6218592345396876308,
            ),
            ((Point { row: 14, col: 1 }, None), 8924232850905585413),
            (
                (Point { row: 14, col: 1 }, Some(Player::Black)),
                5694058760963514477,
            ),
            (
                (Point { row: 14, col: 1 }, Some(Player::White)),
                2244012173554539786,
            ),
            ((Point { row: 14, col: 2 }, None), 5591784495217181424),
            (
                (Point { row: 14, col: 2 }, Some(Player::Black)),
                3889975433169933029,
            ),
            (
                (Point { row: 14, col: 2 }, Some(Player::White)),
                5849393950258914488,
            ),
            ((Point { row: 14, col: 3 }, None), 2133780521550489293),
            (
                (Point { row: 14, col: 3 }, Some(Player::Black)),
                1396880423203366649,
            ),
            (
                (Point { row: 14, col: 3 }, Some(Player::White)),
                4110208832263932145,
            ),
            ((Point { row: 14, col: 4 }, None), 1117238335814674381),
            (
                (Point { row: 14, col: 4 }, Some(Player::Black)),
                8749019886330822436,
            ),
            (
                (Point { row: 14, col: 4 }, Some(Player::White)),
                5088905729799233472,
            ),
            ((Point { row: 14, col: 5 }, None), 5334171861263963358),
            (
                (Point { row: 14, col: 5 }, Some(Player::Black)),
                734689735421136974,
            ),
            (
                (Point { row: 14, col: 5 }, Some(Player::White)),
                5026682058626472335,
            ),
            ((Point { row: 14, col: 6 }, None), 2884149941149311262),
            (
                (Point { row: 14, col: 6 }, Some(Player::Black)),
                5148069458024981520,
            ),
            (
                (Point { row: 14, col: 6 }, Some(Player::White)),
                3775111677789921901,
            ),
            ((Point { row: 14, col: 7 }, None), 268536115391065705),
            (
                (Point { row: 14, col: 7 }, Some(Player::Black)),
                1584289617759234736,
            ),
            (
                (Point { row: 14, col: 7 }, Some(Player::White)),
                6110944001437383435,
            ),
            ((Point { row: 14, col: 8 }, None), 4431372681799391870),
            (
                (Point { row: 14, col: 8 }, Some(Player::Black)),
                8495880876011258463,
            ),
            (
                (Point { row: 14, col: 8 }, Some(Player::White)),
                1779102438903325334,
            ),
            ((Point { row: 14, col: 9 }, None), 8457042363905943411),
            (
                (Point { row: 14, col: 9 }, Some(Player::Black)),
                5991947578814289866,
            ),
            (
                (Point { row: 14, col: 9 }, Some(Player::White)),
                1819755688896508506,
            ),
            ((Point { row: 14, col: 10 }, None), 8020269451533821453),
            (
                (Point { row: 14, col: 10 }, Some(Player::Black)),
                76807380073689101,
            ),
            (
                (Point { row: 14, col: 10 }, Some(Player::White)),
                4875460073468700149,
            ),
            ((Point { row: 14, col: 11 }, None), 649950192155371578),
            (
                (Point { row: 14, col: 11 }, Some(Player::Black)),
                8420286392003412464,
            ),
            (
                (Point { row: 14, col: 11 }, Some(Player::White)),
                7330202131997174971,
            ),
            ((Point { row: 14, col: 12 }, None), 3437898181839925521),
            (
                (Point { row: 14, col: 12 }, Some(Player::Black)),
                4579844043636482751,
            ),
            (
                (Point { row: 14, col: 12 }, Some(Player::White)),
                5675298027485555095,
            ),
            ((Point { row: 14, col: 13 }, None), 2782341101524300525),
            (
                (Point { row: 14, col: 13 }, Some(Player::Black)),
                4475387273802659322,
            ),
            (
                (Point { row: 14, col: 13 }, Some(Player::White)),
                5102347129208604848,
            ),
            ((Point { row: 14, col: 14 }, None), 1833468644417825643),
            (
                (Point { row: 14, col: 14 }, Some(Player::Black)),
                6290874629857104695,
            ),
            (
                (Point { row: 14, col: 14 }, Some(Player::White)),
                8561294844716308608,
            ),
            ((Point { row: 14, col: 15 }, None), 254966696020520283),
            (
                (Point { row: 14, col: 15 }, Some(Player::Black)),
                7367677728278944549,
            ),
            (
                (Point { row: 14, col: 15 }, Some(Player::White)),
                1824843138712013229,
            ),
            ((Point { row: 14, col: 16 }, None), 1055401592349859307),
            (
                (Point { row: 14, col: 16 }, Some(Player::Black)),
                7827546914560571253,
            ),
            (
                (Point { row: 14, col: 16 }, Some(Player::White)),
                5667720198339318907,
            ),
            ((Point { row: 14, col: 17 }, None), 1408031708744953358),
            (
                (Point { row: 14, col: 17 }, Some(Player::Black)),
                2861774344483343859,
            ),
            (
                (Point { row: 14, col: 17 }, Some(Player::White)),
                3381675511593998274,
            ),
            ((Point { row: 14, col: 18 }, None), 7336561305202704811),
            (
                (Point { row: 14, col: 18 }, Some(Player::Black)),
                6556318742702303239,
            ),
            (
                (Point { row: 14, col: 18 }, Some(Player::White)),
                1324000411936927448,
            ),
            ((Point { row: 14, col: 19 }, None), 8954819160184438559),
            (
                (Point { row: 14, col: 19 }, Some(Player::Black)),
                8052349090182791458,
            ),
            (
                (Point { row: 14, col: 19 }, Some(Player::White)),
                7239229860948448453,
            ),
            ((Point { row: 15, col: 1 }, None), 8291809624538792221),
            (
                (Point { row: 15, col: 1 }, Some(Player::Black)),
                2538083883736520226,
            ),
            (
                (Point { row: 15, col: 1 }, Some(Player::White)),
                8274655105676915846,
            ),
            ((Point { row: 15, col: 2 }, None), 6931566474393815647),
            (
                (Point { row: 15, col: 2 }, Some(Player::Black)),
                4545000441550109050,
            ),
            (
                (Point { row: 15, col: 2 }, Some(Player::White)),
                5873594683098786847,
            ),
            ((Point { row: 15, col: 3 }, None), 8315099980556599349),
            (
                (Point { row: 15, col: 3 }, Some(Player::Black)),
                8464849721411205594,
            ),
            (
                (Point { row: 15, col: 3 }, Some(Player::White)),
                2458216153093656083,
            ),
            ((Point { row: 15, col: 4 }, None), 479004347673471424),
            (
                (Point { row: 15, col: 4 }, Some(Player::Black)),
                1244855509357968671,
            ),
            (
                (Point { row: 15, col: 4 }, Some(Player::White)),
                4575261232844901683,
            ),
            ((Point { row: 15, col: 5 }, None), 5715414256790813417),
            (
                (Point { row: 15, col: 5 }, Some(Player::Black)),
                1250921919857759889,
            ),
            (
                (Point { row: 15, col: 5 }, Some(Player::White)),
                1081204847824126909,
            ),
            ((Point { row: 15, col: 6 }, None), 6624630350275454462),
            (
                (Point { row: 15, col: 6 }, Some(Player::Black)),
                5516454750686759462,
            ),
            (
                (Point { row: 15, col: 6 }, Some(Player::White)),
                2756644124819897588,
            ),
            ((Point { row: 15, col: 7 }, None), 3340176754373011181),
            (
                (Point { row: 15, col: 7 }, Some(Player::Black)),
                3206204824346623496,
            ),
            (
                (Point { row: 15, col: 7 }, Some(Player::White)),
                26430015510255603,
            ),
            ((Point { row: 15, col: 8 }, None), 4930751566536492850),
            (
                (Point { row: 15, col: 8 }, Some(Player::Black)),
                597522933322154360,
            ),
            (
                (Point { row: 15, col: 8 }, Some(Player::White)),
                371967473456796557,
            ),
            ((Point { row: 15, col: 9 }, None), 3841041044456046036),
            (
                (Point { row: 15, col: 9 }, Some(Player::Black)),
                4311573471646846043,
            ),
            (
                (Point { row: 15, col: 9 }, Some(Player::White)),
                6618550024651892578,
            ),
            ((Point { row: 15, col: 10 }, None), 6025152064570605573),
            (
                (Point { row: 15, col: 10 }, Some(Player::Black)),
                4840379054011716996,
            ),
            (
                (Point { row: 15, col: 10 }, Some(Player::White)),
                412450886724929897,
            ),
            ((Point { row: 15, col: 11 }, None), 4728798248329881085),
            (
                (Point { row: 15, col: 11 }, Some(Player::Black)),
                518079527071036378,
            ),
            (
                (Point { row: 15, col: 11 }, Some(Player::White)),
                4288178157707188241,
            ),
            ((Point { row: 15, col: 12 }, None), 7092648339647305837),
            (
                (Point { row: 15, col: 12 }, Some(Player::Black)),
                6594680907138298189,
            ),
            (
                (Point { row: 15, col: 12 }, Some(Player::White)),
                3384010508749114041,
            ),
            ((Point { row: 15, col: 13 }, None), 3175325380222444724),
            (
                (Point { row: 15, col: 13 }, Some(Player::Black)),
                4042200394647250377,
            ),
            (
                (Point { row: 15, col: 13 }, Some(Player::White)),
                989323389652347970,
            ),
            ((Point { row: 15, col: 14 }, None), 6992105915936304959),
            (
                (Point { row: 15, col: 14 }, Some(Player::Black)),
                5167926881444962887,
            ),
            (
                (Point { row: 15, col: 14 }, Some(Player::White)),
                1880109307190105705,
            ),
            ((Point { row: 15, col: 15 }, None), 6777615500034728810),
            (
                (Point { row: 15, col: 15 }, Some(Player::Black)),
                5501299262012918754,
            ),
            (
                (Point { row: 15, col: 15 }, Some(Player::White)),
                3399344572582604150,
            ),
            ((Point { row: 15, col: 16 }, None), 7672486937793083068),
            (
                (Point { row: 15, col: 16 }, Some(Player::Black)),
                4590561347678556578,
            ),
            (
                (Point { row: 15, col: 16 }, Some(Player::White)),
                2744835452180995638,
            ),
            ((Point { row: 15, col: 17 }, None), 579802781452674641),
            (
                (Point { row: 15, col: 17 }, Some(Player::Black)),
                7815051084238234798,
            ),
            (
                (Point { row: 15, col: 17 }, Some(Player::White)),
                8825071524309684872,
            ),
            ((Point { row: 15, col: 18 }, None), 7856181308947372757),
            (
                (Point { row: 15, col: 18 }, Some(Player::Black)),
                4870193313166468144,
            ),
            (
                (Point { row: 15, col: 18 }, Some(Player::White)),
                3688724226392407154,
            ),
            ((Point { row: 15, col: 19 }, None), 8922173895957909887),
            (
                (Point { row: 15, col: 19 }, Some(Player::Black)),
                1877065985697984506,
            ),
            (
                (Point { row: 15, col: 19 }, Some(Player::White)),
                8655554446381557936,
            ),
            ((Point { row: 16, col: 1 }, None), 6556031236969286246),
            (
                (Point { row: 16, col: 1 }, Some(Player::Black)),
                6443645201743780202,
            ),
            (
                (Point { row: 16, col: 1 }, Some(Player::White)),
                2762067543878927108,
            ),
            ((Point { row: 16, col: 2 }, None), 2366571831079956419),
            (
                (Point { row: 16, col: 2 }, Some(Player::Black)),
                939033849504052381,
            ),
            (
                (Point { row: 16, col: 2 }, Some(Player::White)),
                846003791900350306,
            ),
            ((Point { row: 16, col: 3 }, None), 7165211158568592344),
            (
                (Point { row: 16, col: 3 }, Some(Player::Black)),
                5129770743968664015,
            ),
            (
                (Point { row: 16, col: 3 }, Some(Player::White)),
                7275730683128642676,
            ),
            ((Point { row: 16, col: 4 }, None), 6581020631265903357),
            (
                (Point { row: 16, col: 4 }, Some(Player::Black)),
                2951028199540884145,
            ),
            (
                (Point { row: 16, col: 4 }, Some(Player::White)),
                7031281878075417899,
            ),
            ((Point { row: 16, col: 5 }, None), 4950460232062986255),
            (
                (Point { row: 16, col: 5 }, Some(Player::Black)),
                7911970555751691695,
            ),
            (
                (Point { row: 16, col: 5 }, Some(Player::White)),
                6538335192289137380,
            ),
            ((Point { row: 16, col: 6 }, None), 7562886280055929503),
            (
                (Point { row: 16, col: 6 }, Some(Player::Black)),
                5515623507188643909,
            ),
            (
                (Point { row: 16, col: 6 }, Some(Player::White)),
                486243866438770865,
            ),
            ((Point { row: 16, col: 7 }, None), 370952491074592131),
            (
                (Point { row: 16, col: 7 }, Some(Player::Black)),
                6186966918632387361,
            ),
            (
                (Point { row: 16, col: 7 }, Some(Player::White)),
                8367573962684752055,
            ),
            ((Point { row: 16, col: 8 }, None), 5111164465840843253),
            (
                (Point { row: 16, col: 8 }, Some(Player::Black)),
                6843500356969554304,
            ),
            (
                (Point { row: 16, col: 8 }, Some(Player::White)),
                1861655187442495092,
            ),
            ((Point { row: 16, col: 9 }, None), 8513801854010801898),
            (
                (Point { row: 16, col: 9 }, Some(Player::Black)),
                3769943096339235807,
            ),
            (
                (Point { row: 16, col: 9 }, Some(Player::White)),
                7700883061110114305,
            ),
            ((Point { row: 16, col: 10 }, None), 6036858580882119589),
            (
                (Point { row: 16, col: 10 }, Some(Player::Black)),
                6235654762231292950,
            ),
            (
                (Point { row: 16, col: 10 }, Some(Player::White)),
                2837846151478443838,
            ),
            ((Point { row: 16, col: 11 }, None), 1541044112500056336),
            (
                (Point { row: 16, col: 11 }, Some(Player::Black)),
                7770808223579816188,
            ),
            (
                (Point { row: 16, col: 11 }, Some(Player::White)),
                3650766475271992051,
            ),
            ((Point { row: 16, col: 12 }, None), 5616154398682881462),
            (
                (Point { row: 16, col: 12 }, Some(Player::Black)),
                7534628418839371086,
            ),
            (
                (Point { row: 16, col: 12 }, Some(Player::White)),
                7511668397119318463,
            ),
            ((Point { row: 16, col: 13 }, None), 6814315894443601661),
            (
                (Point { row: 16, col: 13 }, Some(Player::Black)),
                2269838876424946526,
            ),
            (
                (Point { row: 16, col: 13 }, Some(Player::White)),
                28215113756749003,
            ),
            ((Point { row: 16, col: 14 }, None), 5048258071989020202),
            (
                (Point { row: 16, col: 14 }, Some(Player::Black)),
                1210825246322736323,
            ),
            (
                (Point { row: 16, col: 14 }, Some(Player::White)),
                7284838970644067464,
            ),
            ((Point { row: 16, col: 15 }, None), 881739829658612186),
            (
                (Point { row: 16, col: 15 }, Some(Player::Black)),
                4941573376550531472,
            ),
            (
                (Point { row: 16, col: 15 }, Some(Player::White)),
                3712836831209236121,
            ),
            ((Point { row: 16, col: 16 }, None), 8268238307895306528),
            (
                (Point { row: 16, col: 16 }, Some(Player::Black)),
                8124387305920822925,
            ),
            (
                (Point { row: 16, col: 16 }, Some(Player::White)),
                4386167911205470553,
            ),
            ((Point { row: 16, col: 17 }, None), 3853246836506317104),
            (
                (Point { row: 16, col: 17 }, Some(Player::Black)),
                8901133131138247364,
            ),
            (
                (Point { row: 16, col: 17 }, Some(Player::White)),
                2821377795133081773,
            ),
            ((Point { row: 16, col: 18 }, None), 5160743941508258720),
            (
                (Point { row: 16, col: 18 }, Some(Player::Black)),
                5018262052375639436,
            ),
            (
                (Point { row: 16, col: 18 }, Some(Player::White)),
                597087507416802053,
            ),
            ((Point { row: 16, col: 19 }, None), 7990071123641771138),
            (
                (Point { row: 16, col: 19 }, Some(Player::Black)),
                2647034409199430891,
            ),
            (
                (Point { row: 16, col: 19 }, Some(Player::White)),
                2295830542032634937,
            ),
            ((Point { row: 17, col: 1 }, None), 7737401394819595242),
            (
                (Point { row: 17, col: 1 }, Some(Player::Black)),
                8875600302167034488,
            ),
            (
                (Point { row: 17, col: 1 }, Some(Player::White)),
                7731527998706009486,
            ),
            ((Point { row: 17, col: 2 }, None), 715720492641185994),
            (
                (Point { row: 17, col: 2 }, Some(Player::Black)),
                7688568884192057030,
            ),
            (
                (Point { row: 17, col: 2 }, Some(Player::White)),
                7030239749929949703,
            ),
            ((Point { row: 17, col: 3 }, None), 4253369128350027031),
            (
                (Point { row: 17, col: 3 }, Some(Player::Black)),
                8432096447296282118,
            ),
            (
                (Point { row: 17, col: 3 }, Some(Player::White)),
                2824769679627582916,
            ),
            ((Point { row: 17, col: 4 }, None), 6834520328610039054),
            (
                (Point { row: 17, col: 4 }, Some(Player::Black)),
                7274073934562428887,
            ),
            (
                (Point { row: 17, col: 4 }, Some(Player::White)),
                1256090539020774254,
            ),
            ((Point { row: 17, col: 5 }, None), 8892327929102139278),
            (
                (Point { row: 17, col: 5 }, Some(Player::Black)),
                1496962392350061870,
            ),
            (
                (Point { row: 17, col: 5 }, Some(Player::White)),
                1796572505418791468,
            ),
            ((Point { row: 17, col: 6 }, None), 2021874047425398325),
            (
                (Point { row: 17, col: 6 }, Some(Player::Black)),
                9143740157124297494,
            ),
            (
                (Point { row: 17, col: 6 }, Some(Player::White)),
                7561920636246740497,
            ),
            ((Point { row: 17, col: 7 }, None), 2784943382009283738),
            (
                (Point { row: 17, col: 7 }, Some(Player::Black)),
                799530292803360910,
            ),
            (
                (Point { row: 17, col: 7 }, Some(Player::White)),
                5507890572167645904,
            ),
            ((Point { row: 17, col: 8 }, None), 8480778420742478626),
            (
                (Point { row: 17, col: 8 }, Some(Player::Black)),
                2205835665899408082,
            ),
            (
                (Point { row: 17, col: 8 }, Some(Player::White)),
                7406349870558287808,
            ),
            ((Point { row: 17, col: 9 }, None), 5950713963206029900),
            (
                (Point { row: 17, col: 9 }, Some(Player::Black)),
                335759533405404998,
            ),
            (
                (Point { row: 17, col: 9 }, Some(Player::White)),
                5530597664334721975,
            ),
            ((Point { row: 17, col: 10 }, None), 3659090874879786197),
            (
                (Point { row: 17, col: 10 }, Some(Player::Black)),
                8488238512991604691,
            ),
            (
                (Point { row: 17, col: 10 }, Some(Player::White)),
                5910244816057548126,
            ),
            ((Point { row: 17, col: 11 }, None), 2076767893426238081),
            (
                (Point { row: 17, col: 11 }, Some(Player::Black)),
                2617844579794470220,
            ),
            (
                (Point { row: 17, col: 11 }, Some(Player::White)),
                7885087251066544813,
            ),
            ((Point { row: 17, col: 12 }, None), 3761481934439569715),
            (
                (Point { row: 17, col: 12 }, Some(Player::Black)),
                8094303581367369323,
            ),
            (
                (Point { row: 17, col: 12 }, Some(Player::White)),
                2974339592992130823,
            ),
            ((Point { row: 17, col: 13 }, None), 5442299735028481451),
            (
                (Point { row: 17, col: 13 }, Some(Player::Black)),
                1325647306435444227,
            ),
            (
                (Point { row: 17, col: 13 }, Some(Player::White)),
                7326156650190010263,
            ),
            ((Point { row: 17, col: 14 }, None), 6260299019469353142),
            (
                (Point { row: 17, col: 14 }, Some(Player::Black)),
                3910520021654792223,
            ),
            (
                (Point { row: 17, col: 14 }, Some(Player::White)),
                6330832963346145639,
            ),
            ((Point { row: 17, col: 15 }, None), 7715541794810238533),
            (
                (Point { row: 17, col: 15 }, Some(Player::Black)),
                5259959809963739837,
            ),
            (
                (Point { row: 17, col: 15 }, Some(Player::White)),
                9096504778234209399,
            ),
            ((Point { row: 17, col: 16 }, None), 4645917779744010099),
            (
                (Point { row: 17, col: 16 }, Some(Player::Black)),
                1023310894315491780,
            ),
            (
                (Point { row: 17, col: 16 }, Some(Player::White)),
                2079758702188377299,
            ),
            ((Point { row: 17, col: 17 }, None), 2507490802149009596),
            (
                (Point { row: 17, col: 17 }, Some(Player::Black)),
                8907235051503820861,
            ),
            (
                (Point { row: 17, col: 17 }, Some(Player::White)),
                6203853682362073138,
            ),
            ((Point { row: 17, col: 18 }, None), 6317701673188674690),
            (
                (Point { row: 17, col: 18 }, Some(Player::Black)),
                2902888034615956431,
            ),
            (
                (Point { row: 17, col: 18 }, Some(Player::White)),
                425619146347537885,
            ),
            ((Point { row: 17, col: 19 }, None), 508022599436169859),
            (
                (Point { row: 17, col: 19 }, Some(Player::Black)),
                1108210081162658515,
            ),
            (
                (Point { row: 17, col: 19 }, Some(Player::White)),
                741525160349831667,
            ),
            ((Point { row: 18, col: 1 }, None), 7275655697059903574),
            (
                (Point { row: 18, col: 1 }, Some(Player::Black)),
                3800636911197042933,
            ),
            (
                (Point { row: 18, col: 1 }, Some(Player::White)),
                7946539222518770125,
            ),
            ((Point { row: 18, col: 2 }, None), 901341352621114628),
            (
                (Point { row: 18, col: 2 }, Some(Player::Black)),
                5028186539960225960,
            ),
            (
                (Point { row: 18, col: 2 }, Some(Player::White)),
                3628128000369005397,
            ),
            ((Point { row: 18, col: 3 }, None), 3978631719280723405),
            (
                (Point { row: 18, col: 3 }, Some(Player::Black)),
                4782727626474374488,
            ),
            (
                (Point { row: 18, col: 3 }, Some(Player::White)),
                2384708518039177872,
            ),
            ((Point { row: 18, col: 4 }, None), 7200653546508687593),
            (
                (Point { row: 18, col: 4 }, Some(Player::Black)),
                4372207454289246127,
            ),
            (
                (Point { row: 18, col: 4 }, Some(Player::White)),
                7090970891832093476,
            ),
            ((Point { row: 18, col: 5 }, None), 8829578388259516991),
            (
                (Point { row: 18, col: 5 }, Some(Player::Black)),
                9016280767763678920,
            ),
            (
                (Point { row: 18, col: 5 }, Some(Player::White)),
                6656385173221657964,
            ),
            ((Point { row: 18, col: 6 }, None), 3134115697796137611),
            (
                (Point { row: 18, col: 6 }, Some(Player::Black)),
                2888191479333958188,
            ),
            (
                (Point { row: 18, col: 6 }, Some(Player::White)),
                4514119210060198337,
            ),
            ((Point { row: 18, col: 7 }, None), 7189652192474924525),
            (
                (Point { row: 18, col: 7 }, Some(Player::Black)),
                2100162033911112442,
            ),
            (
                (Point { row: 18, col: 7 }, Some(Player::White)),
                4913440858593268517,
            ),
            ((Point { row: 18, col: 8 }, None), 4755153895757425922),
            (
                (Point { row: 18, col: 8 }, Some(Player::Black)),
                7473725857882692362,
            ),
            (
                (Point { row: 18, col: 8 }, Some(Player::White)),
                3228343618994448971,
            ),
            ((Point { row: 18, col: 9 }, None), 5459321318135163507),
            (
                (Point { row: 18, col: 9 }, Some(Player::Black)),
                5449260555466291456,
            ),
            (
                (Point { row: 18, col: 9 }, Some(Player::White)),
                4923923022378492258,
            ),
            ((Point { row: 18, col: 10 }, None), 1172134112848671359),
            (
                (Point { row: 18, col: 10 }, Some(Player::Black)),
                5411904721734930808,
            ),
            (
                (Point { row: 18, col: 10 }, Some(Player::White)),
                6256527668732175076,
            ),
            ((Point { row: 18, col: 11 }, None), 1524737865180826028),
            (
                (Point { row: 18, col: 11 }, Some(Player::Black)),
                2436537771157275258,
            ),
            (
                (Point { row: 18, col: 11 }, Some(Player::White)),
                2534150629171808577,
            ),
            ((Point { row: 18, col: 12 }, None), 3034951175408650286),
            (
                (Point { row: 18, col: 12 }, Some(Player::Black)),
                7881315621222457549,
            ),
            (
                (Point { row: 18, col: 12 }, Some(Player::White)),
                4275762378733093568,
            ),
            ((Point { row: 18, col: 13 }, None), 3719558475377279183),
            (
                (Point { row: 18, col: 13 }, Some(Player::Black)),
                9165385662215474483,
            ),
            (
                (Point { row: 18, col: 13 }, Some(Player::White)),
                1318870392083905240,
            ),
            ((Point { row: 18, col: 14 }, None), 751064176175730088),
            (
                (Point { row: 18, col: 14 }, Some(Player::Black)),
                5179285693550576190,
            ),
            (
                (Point { row: 18, col: 14 }, Some(Player::White)),
                605057698816664948,
            ),
            ((Point { row: 18, col: 15 }, None), 2067130805696423017),
            (
                (Point { row: 18, col: 15 }, Some(Player::Black)),
                3558350152874909125,
            ),
            (
                (Point { row: 18, col: 15 }, Some(Player::White)),
                6656234009731888208,
            ),
            ((Point { row: 18, col: 16 }, None), 840041880362035869),
            (
                (Point { row: 18, col: 16 }, Some(Player::Black)),
                8012761561191369061,
            ),
            (
                (Point { row: 18, col: 16 }, Some(Player::White)),
                8429314462758985191,
            ),
            ((Point { row: 18, col: 17 }, None), 6731722567304497159),
            (
                (Point { row: 18, col: 17 }, Some(Player::Black)),
                3259245969865068356,
            ),
            (
                (Point { row: 18, col: 17 }, Some(Player::White)),
                1400501091130114268,
            ),
            ((Point { row: 18, col: 18 }, None), 5645967359194989658),
            (
                (Point { row: 18, col: 18 }, Some(Player::Black)),
                8593741605046955819,
            ),
            (
                (Point { row: 18, col: 18 }, Some(Player::White)),
                7069982885326507153,
            ),
            ((Point { row: 18, col: 19 }, None), 4491673721437735112),
            (
                (Point { row: 18, col: 19 }, Some(Player::Black)),
                8152184449595371282,
            ),
            (
                (Point { row: 18, col: 19 }, Some(Player::White)),
                6216354364232214808,
            ),
            ((Point { row: 19, col: 1 }, None), 6923287377723231576),
            (
                (Point { row: 19, col: 1 }, Some(Player::Black)),
                875198948840092384,
            ),
            (
                (Point { row: 19, col: 1 }, Some(Player::White)),
                2189944624992509414,
            ),
            ((Point { row: 19, col: 2 }, None), 4838986340159026209),
            (
                (Point { row: 19, col: 2 }, Some(Player::Black)),
                1452987734301440572,
            ),
            (
                (Point { row: 19, col: 2 }, Some(Player::White)),
                2438199841949623401,
            ),
            ((Point { row: 19, col: 3 }, None), 6371277773154485271),
            (
                (Point { row: 19, col: 3 }, Some(Player::Black)),
                7704298800219576468,
            ),
            (
                (Point { row: 19, col: 3 }, Some(Player::White)),
                1800359798299526174,
            ),
            ((Point { row: 19, col: 4 }, None), 4492737498613936055),
            (
                (Point { row: 19, col: 4 }, Some(Player::Black)),
                8836099552578613572,
            ),
            (
                (Point { row: 19, col: 4 }, Some(Player::White)),
                6654771487561268466,
            ),
            ((Point { row: 19, col: 5 }, None), 7265682302720645301),
            (
                (Point { row: 19, col: 5 }, Some(Player::Black)),
                7456435998204002475,
            ),
            (
                (Point { row: 19, col: 5 }, Some(Player::White)),
                7291672331108211290,
            ),
            ((Point { row: 19, col: 6 }, None), 4009822984326004538),
            (
                (Point { row: 19, col: 6 }, Some(Player::Black)),
                6223256581350032628,
            ),
            (
                (Point { row: 19, col: 6 }, Some(Player::White)),
                843022137189959518,
            ),
            ((Point { row: 19, col: 7 }, None), 7981743819094088811),
            (
                (Point { row: 19, col: 7 }, Some(Player::Black)),
                3263333610358065810,
            ),
            (
                (Point { row: 19, col: 7 }, Some(Player::White)),
                8410858849089095611,
            ),
            ((Point { row: 19, col: 8 }, None), 5933912452747329075),
            (
                (Point { row: 19, col: 8 }, Some(Player::Black)),
                7330118765495484200,
            ),
            (
                (Point { row: 19, col: 8 }, Some(Player::White)),
                8493290295455668103,
            ),
            ((Point { row: 19, col: 9 }, None), 5526948185105544359),
            (
                (Point { row: 19, col: 9 }, Some(Player::Black)),
                6277944094504532807,
            ),
            (
                (Point { row: 19, col: 9 }, Some(Player::White)),
                2840735193097970021,
            ),
            ((Point { row: 19, col: 10 }, None), 2443238057827578677),
            (
                (Point { row: 19, col: 10 }, Some(Player::Black)),
                1147469396919968445,
            ),
            (
                (Point { row: 19, col: 10 }, Some(Player::White)),
                8077137366222441296,
            ),
            ((Point { row: 19, col: 11 }, None), 8769091502172322499),
            (
                (Point { row: 19, col: 11 }, Some(Player::Black)),
                350128161095099125,
            ),
            (
                (Point { row: 19, col: 11 }, Some(Player::White)),
                4998452354131013285,
            ),
            ((Point { row: 19, col: 12 }, None), 1424786266620724313),
            (
                (Point { row: 19, col: 12 }, Some(Player::Black)),
                1453083478883669469,
            ),
            (
                (Point { row: 19, col: 12 }, Some(Player::White)),
                37094861777903360,
            ),
            ((Point { row: 19, col: 13 }, None), 6245487595187324970),
            (
                (Point { row: 19, col: 13 }, Some(Player::Black)),
                4470875344147563095,
            ),
            (
                (Point { row: 19, col: 13 }, Some(Player::White)),
                262354482543045354,
            ),
            ((Point { row: 19, col: 14 }, None), 287114867358884965),
            (
                (Point { row: 19, col: 14 }, Some(Player::Black)),
                8248181847107938759,
            ),
            (
                (Point { row: 19, col: 14 }, Some(Player::White)),
                2328836216498553040,
            ),
            ((Point { row: 19, col: 15 }, None), 6383643812066851140),
            (
                (Point { row: 19, col: 15 }, Some(Player::Black)),
                8067987557991805378,
            ),
            (
                (Point { row: 19, col: 15 }, Some(Player::White)),
                4032532876641780841,
            ),
            ((Point { row: 19, col: 16 }, None), 5320120584009006168),
            (
                (Point { row: 19, col: 16 }, Some(Player::Black)),
                1391203785520871408,
            ),
            (
                (Point { row: 19, col: 16 }, Some(Player::White)),
                7837035815476897278,
            ),
            ((Point { row: 19, col: 17 }, None), 2348947234525505974),
            (
                (Point { row: 19, col: 17 }, Some(Player::Black)),
                3459665267267208229,
            ),
            (
                (Point { row: 19, col: 17 }, Some(Player::White)),
                4050012508868328251,
            ),
            ((Point { row: 19, col: 18 }, None), 1756046589728040341),
            (
                (Point { row: 19, col: 18 }, Some(Player::Black)),
                7464872111700182003,
            ),
            (
                (Point { row: 19, col: 18 }, Some(Player::White)),
                2016174654088690195,
            ),
            ((Point { row: 19, col: 19 }, None), 5927758607513907973),
            (
                (Point { row: 19, col: 19 }, Some(Player::Black)),
                5859898915817033326,
            ),
            (
                (Point { row: 19, col: 19 }, Some(Player::White)),
                6217718608968288470,
            ),
        ]);
        let empty_board = 9181944435492932548;

        ZoobristHash {
            hash_codes: hash_code,
            empty_board: empty_board,
        }
    }
}
