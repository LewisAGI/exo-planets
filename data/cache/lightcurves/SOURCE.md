# Light-curve cache (MAST / Kepler / K2 / TESS public HTTP)

Pulled 2026-08-27. **No secrets.** These are small PDCSAP extracts from real
Kepler / K2 long-cadence FITS and a TESS SPOC 2-min FITS on the public STScI
archive (the same bits MAST serves). Full mission light curves are huge;
this is a cached slice.

Re-fetch (needs network; Rust FITS reader, no Python):

```bash
cargo run -- fetch --cache data/cache
```

TESS TIME is BTJD (BJD−2457000). Cached TESS times are shifted to BKJD
(+2167 d = 2457000−2454833). TESS S14 is mid-span 2500 PDCSAP points.

| Cache CSV | Host | ID | File | URL |
|---|---|---|---|---|
| `kepler10b_kic11904151_q1_llc.csv` | Kepler-10 b | KIC 11904151 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0119/011904151/kplr011904151-2009166043257_llc.fits |
| `kepler10b_tic377780790_tess_s14_pdcsap.csv` | Kepler-10 b | TIC 377780790 | TESS S14 SPOC 2-min | https://archive.stsci.edu/missions/tess/tid/s0014/0000/0003/7778/0790/tess2019198215352-s0014-0000000377780790-0150-s_lc.fits |
| `kepler1b_kic11446443_q1_llc.csv` | Kepler-1 b (TrES-2b) | KIC 11446443 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0114/011446443/kplr011446443-2009166043257_llc.fits |
| `kepler11b_kic6541920_q1_llc.csv` | Kepler-11 b | KIC 6541920 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0065/006541920/kplr006541920-2009166043257_llc.fits |
| `kepler2b_kic10666592_q1_llc.csv` | Kepler-2 b (HAT-P-7b) | KIC 10666592 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0106/010666592/kplr010666592-2009166043257_llc.fits |
| `kepler3b_kic10748390_q1_llc.csv` | Kepler-3 b (HAT-P-11b) | KIC 10748390 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0107/010748390/kplr010748390-2009166043257_llc.fits |
| `kepler4b_kic11853905_q1_llc.csv` | Kepler-4 b | KIC 11853905 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0118/011853905/kplr011853905-2009166043257_llc.fits |
| `kepler5b_kic8191672_q1_llc.csv` | Kepler-5 b | KIC 8191672 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0081/008191672/kplr008191672-2009166043257_llc.fits |
| `kepler6b_kic10874614_q1_llc.csv` | Kepler-6 b | KIC 10874614 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0108/010874614/kplr010874614-2009166043257_llc.fits |
| `kepler7b_kic5780885_q1_llc.csv` | Kepler-7 b | KIC 5780885 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0057/005780885/kplr005780885-2009166043257_llc.fits |
| `kepler8b_kic6922244_q1_llc.csv` | Kepler-8 b | KIC 6922244 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0069/006922244/kplr006922244-2009166043257_llc.fits |
| `kepler9b_kic3323887_q1_llc.csv` | Kepler-9 b | KIC 3323887 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0033/003323887/kplr003323887-2009166043257_llc.fits |
| `kepler22b_kic10593626_q1_llc.csv` | Kepler-22 b | KIC 10593626 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0105/010593626/kplr010593626-2009166043257_llc.fits |
| `kepler18b_kic8644288_q1_llc.csv` | Kepler-18 b/c/d | KIC 8644288 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0086/008644288/kplr008644288-2009166043257_llc.fits |
| `kepler19b_kic2571238_q1_llc.csv` | Kepler-19 b | KIC 2571238 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0025/002571238/kplr002571238-2009166043257_llc.fits |
| `kepler20b_kic6850504_q1_llc.csv` | Kepler-20 b–f | KIC 6850504 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0068/006850504/kplr006850504-2009166043257_llc.fits |
| `kepler21b_kic3632418_q1_llc.csv` | Kepler-21 b | KIC 3632418 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0036/003632418/kplr003632418-2009166043257_llc.fits |
| `kepler30b_kic3832474_q1_llc.csv` | Kepler-30 b/d | KIC 3832474 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0038/003832474/kplr003832474-2009166043257_llc.fits |
| `kepler36b_kic11401755_q1_llc.csv` | Kepler-36 b/c | KIC 11401755 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0114/011401755/kplr011401755-2009166043257_llc.fits |
| `kepler48b_kic5735762_q1_llc.csv` | Kepler-48 b–d | KIC 5735762 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0057/005735762/kplr005735762-2009166043257_llc.fits |
| `kepler51b_kic11773022_q1_llc.csv` | Kepler-51 b | KIC 11773022 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0117/011773022/kplr011773022-2009166043257_llc.fits |
| `kepler79b_kic8394721_q1_llc.csv` | Kepler-79 b–e | KIC 8394721 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0083/008394721/kplr008394721-2009166043257_llc.fits |
| `kepler68c_kic11295426_q1_llc.csv` | Kepler-68 c | KIC 11295426 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0112/011295426/kplr011295426-2009166043257_llc.fits |
| `kepler89b_kic6462863_q1_llc.csv` | Kepler-89 b–e | KIC 6462863 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0064/006462863/kplr006462863-2009166043257_llc.fits |
| `kepler102b_kic10187017_q1_llc.csv` | Kepler-102 b–f | KIC 10187017 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0101/010187017/kplr010187017-2009166043257_llc.fits |
| `kepler62c_kic9002278_q1_llc.csv` | Kepler-62 c/d/e | KIC 9002278 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0090/009002278/kplr009002278-2009166043257_llc.fits |
| `kepler37b_kic8478994_q1_llc.csv` | Kepler-37 b/c/d | KIC 8478994 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0084/008478994/kplr008478994-2009166043257_llc.fits |
| `kepler444b_kic6278762_q1_llc.csv` | Kepler-444 b–f | KIC 6278762 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0062/006278762/kplr006278762-2009166043257_llc.fits |
| `kepler42b_kic8561063_q1_llc.csv` | Kepler-42 b/c/d | KIC 8561063 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0085/008561063/kplr008561063-2009166043257_llc.fits |
| `kepler138b_kic7603200_q1_llc.csv` | Kepler-138 b/c/d | KIC 7603200 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0076/007603200/kplr007603200-2009166043257_llc.fits |
| `kepler65b_kic5866724_q1_llc.csv` | Kepler-65 b/c/d | KIC 5866724 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0058/005866724/kplr005866724-2009166043257_llc.fits |
| `kepler32b_kic9787239_q1_llc.csv` | Kepler-32 b–f | KIC 9787239 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0097/009787239/kplr009787239-2009166043257_llc.fits |
| `kepler33b_kic9458613_q1_llc.csv` | Kepler-33 b–f | KIC 9458613 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0094/009458613/kplr009458613-2009166043257_llc.fits |
| `kepler26b_kic9757613_q1_llc.csv` | Kepler-26 b/c/d | KIC 9757613 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0097/009757613/kplr009757613-2009166043257_llc.fits |
| `kepler186b_kic8120608_q1_llc.csv` | Kepler-186 b–e | KIC 8120608 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0081/008120608/kplr008120608-2009166043257_llc.fits |
| `kepler80d_kic4852528_q1_llc.csv` | Kepler-80 c–f | KIC 4852528 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0048/004852528/kplr004852528-2009166043257_llc.fits |
| `kepler29b_kic10358759_q1_llc.csv` | Kepler-29 b/c | KIC 10358759 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0103/010358759/kplr010358759-2009166043257_llc.fits |
| `kepler93b_kic3544595_q1_llc.csv` | Kepler-93 b | KIC 3544595 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0035/003544595/kplr003544595-2009166043257_llc.fits |
| `kepler100b_kic6521045_q1_llc.csv` | Kepler-100 b/c/d | KIC 6521045 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0065/006521045/kplr006521045-2009166043257_llc.fits |
| `kepler88b_kic5446285_q1_llc.csv` | Kepler-88 b | KIC 5446285 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0054/005446285/kplr005446285-2009166043257_llc.fits |
| `kepler23b_kic11512246_q1_llc.csv` | Kepler-23 b/c | KIC 11512246 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0115/011512246/kplr011512246-2009166043257_llc.fits |
| `kepler24b_kic3231341_q1_llc.csv` | Kepler-24 b/c | KIC 3231341 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0032/003231341/kplr003231341-2009166043257_llc.fits |
| `kepler27b_kic5792202_q1_llc.csv` | Kepler-27 b/c | KIC 5792202 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0057/005792202/kplr005792202-2009166043257_llc.fits |
| `kepler28b_kic6949607_q1_llc.csv` | Kepler-28 b/c | KIC 6949607 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0069/006949607/kplr006949607-2009166043257_llc.fits |
| `kepler41b_kic9410930_q1_llc.csv` | Kepler-41 b | KIC 9410930 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0094/009410930/kplr009410930-2009166043257_llc.fits |
| `kepler56b_kic6448890_q1_llc.csv` | Kepler-56 b/c | KIC 6448890 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0064/006448890/kplr006448890-2009166043257_llc.fits |
| `kepler57b_kic8564587_q1_llc.csv` | Kepler-57 b/c | KIC 8564587 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0085/008564587/kplr008564587-2009166043257_llc.fits |
| `kepler69b_kic8692861_q1_llc.csv` | Kepler-69 b | KIC 8692861 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0086/008692861/kplr008692861-2009166043257_llc.fits |
| `kepler76b_kic4570949_q1_llc.csv` | Kepler-76 b | KIC 4570949 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0045/004570949/kplr004570949-2009166043257_llc.fits |
| `kepler58b_kic4077526_q1_llc.csv` | Kepler-58 b/c/d | KIC 4077526 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0040/004077526/kplr004077526-2009166043257_llc.fits |
| `kepler59b_kic9821454_q1_llc.csv` | Kepler-59 b/c | KIC 9821454 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0098/009821454/kplr009821454-2009166043257_llc.fits |
| `kepler60b_kic6768394_q1_llc.csv` | Kepler-60 b/c/d | KIC 6768394 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0067/006768394/kplr006768394-2009166043257_llc.fits |
| `kepler84b_kic5301750_q1_llc.csv` | Kepler-84 b–e | KIC 5301750 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0053/005301750/kplr005301750-2009166043257_llc.fits |
| `kepler85b_kic8950568_q1_llc.csv` | Kepler-85 b–e | KIC 8950568 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0089/008950568/kplr008950568-2009166043257_llc.fits |
| `kepler54b_kic7455287_q1_llc.csv` | Kepler-54 b/c/d | KIC 7455287 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0074/007455287/kplr007455287-2009166043257_llc.fits |
| `kepler55b_kic8150320_q1_llc.csv` | Kepler-55 b–e | KIC 8150320 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0081/008150320/kplr008150320-2009166043257_llc.fits |
| `kepler52b_kic11754553_q1_llc.csv` | Kepler-52 b/c/d | KIC 11754553 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0117/011754553/kplr011754553-2009166043257_llc.fits |
| `kepler53b_kic5358241_q1_llc.csv` | Kepler-53 b/c | KIC 5358241 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0053/005358241/kplr005358241-2009166043257_llc.fits |
| `kepler31b_kic9347899_q1_llc.csv` | Kepler-31 b/c | KIC 9347899 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0093/009347899/kplr009347899-2009166043257_llc.fits |
| `kepler50b_kic11807274_q1_llc.csv` | Kepler-50 b/c | KIC 11807274 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0118/011807274/kplr011807274-2009166043257_llc.fits |
| `kepler81b_kic7287995_q1_llc.csv` | Kepler-81 b/c/d | KIC 7287995 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0072/007287995/kplr007287995-2009166043257_llc.fits |
| `kepler94b_kic10318874_q1_llc.csv` | Kepler-94 b | KIC 10318874 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0103/010318874/kplr010318874-2009166043257_llc.fits |
| `kepler95b_kic8349582_q1_llc.csv` | Kepler-95 b | KIC 8349582 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0083/008349582/kplr008349582-2009166043257_llc.fits |
| `kepler61b_kic6960913_q1_llc.csv` | Kepler-61 b | KIC 6960913 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0069/006960913/kplr006960913-2009166043257_llc.fits |
| `kepler66b_kic9836149_q1_llc.csv` | Kepler-66 b | KIC 9836149 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0098/009836149/kplr009836149-2009166043257_llc.fits |
| `kepler74b_kic6046540_q1_llc.csv` | Kepler-74 b | KIC 6046540 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0060/006046540/kplr006046540-2009166043257_llc.fits |
| `kepler43b_kic9818381_q1_llc.csv` | Kepler-43 b | KIC 9818381 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0098/009818381/kplr009818381-2009166043257_llc.fits |
| `kepler44b_kic9305831_q1_llc.csv` | Kepler-44 b | KIC 9305831 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0093/009305831/kplr009305831-2009166043257_llc.fits |
| `kepler92b_kic6196457_q1_llc.csv` | Kepler-92 b/c/d | KIC 6196457 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0061/006196457/kplr006196457-2009166043257_llc.fits |
| `kepler49b_kic5364071_q1_llc.csv` | Kepler-49 b/c | KIC 5364071 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0053/005364071/kplr005364071-2009166043257_llc.fits |
| `kepler75b_kic757450_q1_llc.csv` | Kepler-75 b | KIC 757450 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0007/000757450/kplr000757450-2009166043257_llc.fits |
| `kepler83b_kic7870390_q1_llc.csv` | Kepler-83 b/c/d | KIC 7870390 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0078/007870390/kplr007870390-2009166043257_llc.fits |
| `kepler39b_kic9478990_q1_llc.csv` | Kepler-39 b | KIC 9478990 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0094/009478990/kplr009478990-2009166043257_llc.fits |
| `kepler40b_kic10418224_q1_llc.csv` | Kepler-40 b | KIC 10418224 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0104/010418224/kplr010418224-2009166043257_llc.fits |
| `kepler45b_kic5794240_q1_llc.csv` | Kepler-45 b | KIC 5794240 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0057/005794240/kplr005794240-2009166043257_llc.fits |
| `kepler46b_kic7109675_q1_llc.csv` | Kepler-46 b | KIC 7109675 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0071/007109675/kplr007109675-2009166043257_llc.fits |
| `kepler63b_kic11554435_q1_llc.csv` | Kepler-63 b | KIC 11554435 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0115/011554435/kplr011554435-2009166043257_llc.fits |
| `kepler82b_kic7366258_q1_llc.csv` | Kepler-82 b/d/e | KIC 7366258 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0073/007366258/kplr007366258-2009166043257_llc.fits |
| `kepler91b_kic8219268_q1_llc.csv` | Kepler-91 b | KIC 8219268 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0082/008219268/kplr008219268-2009166043257_llc.fits |
| `kepler96b_kic5383248_q1_llc.csv` | Kepler-96 b | KIC 5383248 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0053/005383248/kplr005383248-2009166043257_llc.fits |
| `kepler97b_kic11075737_q1_llc.csv` | Kepler-97 b | KIC 11075737 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0110/011075737/kplr011075737-2009166043257_llc.fits |
| `kepler98b_kic2692377_q1_llc.csv` | Kepler-98 b | KIC 2692377 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0026/002692377/kplr002692377-2009166043257_llc.fits |
| `kepler99b_kic6063220_q1_llc.csv` | Kepler-99 b | KIC 6063220 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0060/006063220/kplr006063220-2009166043257_llc.fits |
| `kepler101b_kic10905239_q1_llc.csv` | Kepler-101 b/c | KIC 10905239 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0109/010905239/kplr010905239-2009166043257_llc.fits |
| `kepler103b_kic4914423_q1_llc.csv` | Kepler-103 b | KIC 4914423 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0049/004914423/kplr004914423-2009166043257_llc.fits |
| `kepler104b_kic6678383_q1_llc.csv` | Kepler-104 b/c | KIC 6678383 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0066/006678383/kplr006678383-2009166043257_llc.fits |
| `kepler105b_kic9579641_q1_llc.csv` | Kepler-105 b/c | KIC 9579641 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0095/009579641/kplr009579641-2009166043257_llc.fits |
| `kepler106b_kic8395660_q1_llc.csv` | Kepler-106 b–e | KIC 8395660 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0083/008395660/kplr008395660-2009166043257_llc.fits |
| `kepler107b_kic10875245_q1_llc.csv` | Kepler-107 b–e | KIC 10875245 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0108/010875245/kplr010875245-2009166043257_llc.fits |
| `kepler13b_kic9941662_q1_llc.csv` | Kepler-13 b | KIC 9941662 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0099/009941662/kplr009941662-2009166043257_llc.fits |
| `kepler14b_kic10264660_q1_llc.csv` | Kepler-14 b | KIC 10264660 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0102/010264660/kplr010264660-2009166043257_llc.fits |
| `kepler15b_kic11359879_q1_llc.csv` | Kepler-15 b | KIC 11359879 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0113/011359879/kplr011359879-2009166043257_llc.fits |
| `kepler17b_kic10619192_q1_llc.csv` | Kepler-17 b | KIC 10619192 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0106/010619192/kplr010619192-2009166043257_llc.fits |
| `kepler71b_kic9595827_q1_llc.csv` | Kepler-71 b | KIC 9595827 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0095/009595827/kplr009595827-2009166043257_llc.fits |
| `kepler77b_kic8359498_q1_llc.csv` | Kepler-77 b | KIC 8359498 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0083/008359498/kplr008359498-2009166043257_llc.fits |
| `kepler108b_kic9471974_q1_llc.csv` | Kepler-108 b | KIC 9471974 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0094/009471974/kplr009471974-2009166043257_llc.fits |
| `kepler109b_kic5094751_q1_llc.csv` | Kepler-109 b | KIC 5094751 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0050/005094751/kplr005094751-2009166043257_llc.fits |
| `kepler110b_kic11086270_q1_llc.csv` | Kepler-110 b | KIC 11086270 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0110/011086270/kplr011086270-2009166043257_llc.fits |
| `kepler111b_kic8559644_q1_llc.csv` | Kepler-111 b | KIC 8559644 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0085/008559644/kplr008559644-2009166043257_llc.fits |
| `kepler112b_kic7626506_q1_llc.csv` | Kepler-112 b | KIC 7626506 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0076/007626506/kplr007626506-2009166043257_llc.fits |
| `kepler113b_kic12252424_q1_llc.csv` | Kepler-113 b | KIC 12252424 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0122/012252424/kplr012252424-2009166043257_llc.fits |
| `kepler114b_kic10925104_q1_llc.csv` | Kepler-114 b/c | KIC 10925104 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0109/010925104/kplr010925104-2009166043257_llc.fits |
| `kepler115b_kic8972058_q1_llc.csv` | Kepler-115 b/c | KIC 8972058 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0089/008972058/kplr008972058-2009166043257_llc.fits |
| `kepler116b_kic7831264_q1_llc.csv` | Kepler-116 b/c | KIC 7831264 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0078/007831264/kplr007831264-2009166043257_llc.fits |
| `kepler117b_kic10723750_q1_llc.csv` | Kepler-117 b/c | KIC 10723750 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0107/010723750/kplr010723750-2009166043257_llc.fits |
| `k2_3_epic201367065_c01_llc.csv` | K2-3 b | EPIC 201367065 | K2 C1 LLC | https://archive.stsci.edu/pub/k2/lightcurves/c1/201300000/67000/ktwo201367065-c01_llc.fits |
| `k2_18_epic201912552_c01_llc.csv` | K2-18 b | EPIC 201912552 | K2 C1 LLC | https://archive.stsci.edu/pub/k2/lightcurves/c1/201900000/12000/ktwo201912552-c01_llc.fits |
| `kepler1625b_kic4760478_q8_llc.csv` | Kepler-1625 b | KIC 4760478 | Q8 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0047/004760478/kplr004760478-2011073133259_llc.fits |
| `kepler1708b_kic7906827_q1_llc.csv` | Kepler-1708 b | KIC 7906827 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0079/007906827/kplr007906827-2009166043257_llc.fits |
| `kepler167e_kic3239945_q1_llc.csv` | Kepler-167 e | KIC 3239945 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0032/003239945/kplr003239945-2009166043257_llc.fits |

Columns: `time_bkjd,pdcsap_flux,pdcsap_flux_err,sap_quality` (finite PDCSAP only).
TESS FITS uses `QUALITY` (not `SAP_QUALITY`); the extract header stays the same.

Honesty:

- Kepler-10 b, Kepler-1 b, Kepler-2 b, Kepler-3 b, Kepler-4–8 b, Kepler-9 b/c,
  Kepler-11 b/c/d/e, Kepler-13 b, Kepler-14 b, Kepler-15 b, Kepler-17 b,
  Kepler-18 b/c/d, Kepler-19 b, Kepler-20 b–f, Kepler-21 b,
  Kepler-23 b/c, Kepler-24 b/c, Kepler-26 b/c/d, Kepler-27 b/c, Kepler-28 b/c,
  Kepler-39 b, Kepler-40 b, Kepler-41 b, Kepler-43 b, Kepler-44 b, Kepler-45 b,
  Kepler-46 b, Kepler-49 b/c, Kepler-52 b/c/d, Kepler-53 b/c, Kepler-54 b/c/d, Kepler-55 b–e,
  Kepler-56 b/c, Kepler-57 b/c,
  Kepler-58 b/c/d, Kepler-59 b/c,
  Kepler-60 b/c/d, Kepler-61 b, Kepler-63 b, Kepler-66 b, Kepler-69 b, Kepler-71 b, Kepler-74 b,
  Kepler-75 b, Kepler-76 b, Kepler-77 b, Kepler-82 b/d/e, Kepler-83 b/c/d, Kepler-84 b–e, Kepler-85 b–e,
  Kepler-29 b/c, Kepler-30 b/d, Kepler-31 b/c, Kepler-32 b–f, Kepler-50 b/c,
  Kepler-33 b–f, Kepler-36 b/c, Kepler-37 b/c/d, Kepler-42 b/c/d, Kepler-48 b–d,
  Kepler-51 b, Kepler-62 c/d/e, Kepler-65 b/c/d, Kepler-68 c, Kepler-79 b–e,
  Kepler-80 c–f, Kepler-81 b/c/d, Kepler-88 b, Kepler-89 b–e, Kepler-92 b/c/d, Kepler-93 b,
  Kepler-91 b, Kepler-94 b, Kepler-95 b, Kepler-96 b, Kepler-97 b, Kepler-98 b,
  Kepler-99 b, Kepler-100 b/c/d, Kepler-101 b/c, Kepler-103 b, Kepler-104 b/c,
  Kepler-105 b/c, Kepler-106 b–e, Kepler-107 b–e, Kepler-108 b, Kepler-109 b,
  Kepler-110 b, Kepler-111 b, Kepler-112 b, Kepler-113 b, Kepler-114 b/c,
  Kepler-115 b/c, Kepler-116 b/c, Kepler-117 b/c,
  Kepler-102 b–f, Kepler-138 b/c/d, Kepler-186 b–e, Kepler-444 b–f, Kepler-22 b,
  K2-3 b/c, and K2-18 b are
  **confirmed planets**, used as LC-backed training hosts. They are not
  moon detections. Kepler-12 b, 25 b, 30 c, 51 c/d, 62 b, 62 f, 68 b, 80 b,
  82 c, 103 c, 104 d, and 186 f miss Q1; no extracts were invented. Kepler-67 b has a catalog
  epoch in Q1, but the public Q1 LLC path 404'd; no extract was invented. Kepler-20 c–f reuse the Kepler-20 b
  Q1 file; Kepler-30 d reuses Kepler-30 b; Kepler-36 c reuses Kepler-36 b;
  Kepler-37 c/d reuse Kepler-37 b; Kepler-48 c/d reuse Kepler-48 b;
  Kepler-79 c–e reuse Kepler-79 b; Kepler-89 c–e reuse Kepler-89 b;
  Kepler-102 c–f reuse Kepler-102 b; Kepler-62 d/e reuse Kepler-62 c;
  Kepler-444 c–f reuse Kepler-444 b; Kepler-42 c/d reuse Kepler-42 b;
  Kepler-138 c/d reuse Kepler-138 b; Kepler-65 c/d reuse Kepler-65 b;
  Kepler-32 c–f reuse Kepler-32 b; Kepler-33 c–f reuse Kepler-33 b;
  Kepler-26 c/d reuse Kepler-26 b; Kepler-186 c–e reuse Kepler-186 b;
  Kepler-80 c/e/f reuse Kepler-80 d; Kepler-29 c reuses Kepler-29 b;
  Kepler-100 c/d reuse Kepler-100 b; Kepler-23 c reuses Kepler-23 b;
  Kepler-24 c reuses Kepler-24 b; Kepler-27 c reuses Kepler-27 b;
  Kepler-28 c reuses Kepler-28 b; Kepler-56 c reuses Kepler-56 b;
  Kepler-57 c reuses Kepler-57 b; Kepler-58 c/d reuse Kepler-58 b;
  Kepler-59 c reuses Kepler-59 b; Kepler-60 c/d reuse Kepler-60 b;
  Kepler-84 c–e reuse Kepler-84 b; Kepler-85 c–e reuse Kepler-85 b;
  Kepler-54 c/d reuse Kepler-54 b; Kepler-55 c–e reuse Kepler-55 b;
  Kepler-52 c/d reuse Kepler-52 b; Kepler-53 c reuses Kepler-53 b;
  Kepler-31 c reuses Kepler-31 b; Kepler-50 c reuses Kepler-50 b;
  Kepler-81 c/d reuse Kepler-81 b; Kepler-92 c/d reuse Kepler-92 b;
  Kepler-49 c reuses Kepler-49 b; Kepler-83 c/d reuse Kepler-83 b;
  Kepler-82 d/e reuse Kepler-82 b; Kepler-101 c reuses Kepler-101 b;
  Kepler-104 c reuses Kepler-104 b; Kepler-105 c reuses Kepler-105 b;
  Kepler-106 c–e reuse Kepler-106 b; Kepler-107 c–e reuse Kepler-107 b;
  Kepler-114 c reuses Kepler-114 b; Kepler-115 c reuses Kepler-115 b;
  Kepler-116 c reuses Kepler-116 b; Kepler-117 c reuses Kepler-117 b.
  Kepler-22 b Q1 covers the catalog epoch (t0≈133.70 BKJD); Kepler-11 b/c/d/e
  cover catalog t0≈138.50 / 138.18 / 148.46 / 154.16; Kepler-9 b/c cover
  previous catalog epochs (≈163.27 / ≈136.52); Kepler-37 b/c/d cover
  previous catalog epochs (≈143.97 / ≈149.24 / ≈135.46); Kepler-62 d
  covers the previous catalog epoch (≈162.65); Kepler-138 c/d cover
  previous catalog epochs (≈136.51 / ≈147.92); Kepler-33 d/f cover
  previous catalog epochs (≈146.09 / ≈131.55); Kepler-186 d covers the
  previous catalog epoch (≈136.87); Kepler-80 c covers the previous catalog
  epoch (≈139.40); Kepler-100 c covers the next catalog epoch (≈135.76);
  Kepler-27 b covers previous catalog epochs (≈143.33 / ≈158.67);
  Kepler-54 b covers the previous catalog epoch (≈162.20); Kepler-55 b
  covers the previous catalog epoch (≈150.78); Kepler-52 c covers the
  previous catalog epoch (≈156.34); Kepler-53 b covers the previous
  catalog epoch (≈156.12); Kepler-31 b covers the previous catalog
  epoch (≈159.16); Kepler-49 c covers the previous catalog epoch (≈158.94);
  Kepler-46 b covers the previous catalog epoch (≈153.08); Kepler-96 b
  covers the previous catalog epoch (≈154.78); Kepler-113 b covers the
  next catalog epoch (≈133.30).
  Kepler-42 b, Kepler-32 b/c, Kepler-26 b, Kepler-28 b/c, Kepler-29 b/c,
  Kepler-13 b, Kepler-39 b, Kepler-40 b, Kepler-45 b, Kepler-49 b, Kepler-50 b, Kepler-52 b, Kepler-55 d, Kepler-63 b, Kepler-75 b,
  Kepler-81 b/c, Kepler-82 b, Kepler-83 b/d, Kepler-92 b, Kepler-97 b,
  Kepler-98 b, Kepler-99 b, Kepler-101 b, Kepler-109 b, and Kepler-186 c use folded catalog transits. Those transits were not
  invented. Kepler-11 c/d/e and Kepler-9 c reuse the host-star Q1 extract.
  Kepler-11 f/g catalog epochs do not fall in Q1; no transit was invented.
- Kepler-1625 b, Kepler-1708 b, and Kepler-167 e are **holdouts**.
  Q8 (1625; BKJD ≈ 735–802) and Q1 (1708, 167e; BKJD ≈ 131–165) do **not**
  cover a catalog transit (P ≈ 287 / 737 / 1071 d). No transit was invented.
  Statuses stay **CANDIDATE / SEARCH**.
- Kepler-1708 has a public Kepler LLC (preferred over FFI-only TESS).
  Kepler-167 e likewise. TESS for 1708 is FFI-only (TIC 272716898); no
  TESSCut photometry was invented.
- K2-3 b/c cached PS rows have no transit epoch; extra-dip is unwindowed.
  K2-3 c reuses the K2-3 b C1 extract. Do not invent an epoch.
- No Hubble or Columbia 1625 photometry (Academic Commons `/download` 404).
  JWST GO 6491 is metadata only (`jwst_go6491_*.json/csv` in `data/cache/`),
  not a NIRSpec light curve.
- Extra-dip flags computed from these files are **not** LUNA and **not**
  confirmations. HEK V: photometry-only false-claims moons in ~1/4 of KOIs.
  The crate’s HEK V demo on these LCs is that **caution**, not a detection.

Portals: https://mast.stsci.edu/ · https://archive.stsci.edu/missions-and-data/kepler · https://archive.stsci.edu/missions-and-data/k2 · https://archive.stsci.edu/missions-and-data/tess
