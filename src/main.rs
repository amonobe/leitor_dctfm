use libDCTFM::ClassesDCTFM::{processar_linhas, identificar_registro, processar_registro, obter_campo};


fn main() {



    let exemplo_dctf = 
"DCTFM       202119301445190720001152370SUPERMERCADO IWAMOTO LTDA                                   SP0007903500103202106561154748680106202130062021300620211000997870193070000000314975219000300075107000000000000000000000000000000000000000000000000000000000000000000000440000                                                                                           2356976706
R0144519072000115202106320210630010630061099787019371007000004000000000001401992293068
R0244519072000115202106320210630SUPERMERCADO IWAMOTO LTDA                                                                                          0000AV ANTONIO LOUZADA ANTUNES              282                        SAO MIGUEL PAULISTA SAO PAULO                                         SP0806100011  33854040 11  20459001                 YONE.FERREIRA@GRUPOCHAMA.COM.BR         1987009442
R0344519072000115202106320210630FUMIO IWAMOTO                                               56115474868                                                                       YONE FERREIRA SINZATO TRAJAI                                302148938981SP283593/O-9  SP11  33854040      11  20459001 YONE.FERREIRA@GRUPOCHAMA.COM.BR         3603185680
R104451907200011520210632021063001337301T202102000000000000000000000000000000175019800000942800481
R114451907200011520210632021063001337301T202102000000000000000000000003006202144519072000115337330072021                 000000017501980000000000000000000000000000000000017501983865259221
R104451907200011520210632021063002056107M202106000000000000000000000000000000001291100001400565049
R114451907200011520210632021063002056107M202106000000000000000000000003006202144519072000115056120072021                 000000000012210000000000000000000000000000000000000012211269054534
R114451907200011520210632021063002056107M202106000000000000000000000003006202144519072000115056120072021                 000000000116900000000000000000000000000000000000000116903705114605
R104451907200011520210632021063002170806M202106000000000000000000000000000000003865400003097427198
R114451907200011520210632021063002170806M202106000000000000000000000003006202144519072000115170820072021                 000000000014390000000000000000000000000000000000000014391012310870
R114451907200011520210632021063002170806M202106000000000000000000000003006202144519072000115170820072021                 000000000016180000000000000000000000000000000000000016184127061450
R114451907200011520210632021063002170806M202106000000000000000000000003006202144519072000115170820072021                 000000000027660000000000000000000000000000000000000027662809277252
R114451907200011520210632021063002170806M202106000000000000000000000003006202144519072000115170820072021                 000000000063500000000000000000000000000000000000000063500497703226
R114451907200011520210632021063002170806M202106000000000000000000000003006202144519072000115170820072021                 000000000132110000000000000000000000000000000000000132112071976153
R114451907200011520210632021063002170806M202106000000000000000000000003006202144519072000115170820072021                 000000000132700000000000000000000000000000000000000132703787532414
R104451907200011520210632021063002320806M202106000000000000000000000000000000008906400001319436609
R114451907200011520210632021063002320806M202106000000000000000000000003006202144519072000115320820072021                 000000000890640000000000000000000000000000000000000890641006940898
R104451907200011520210632021063005601201T202102000000000000000000000000000000086687600002386901949
R114451907200011520210632021063005601201T202102000000000000000000000003006202144519072000115601230072021                 000000008668760000000000000000000000000000000000008668761644612705
R104451907200011520210632021063006691201M202106000000000000000000000000000000001603000000166744302
R114451907200011520210632021063006691201M202106000000000000000000000003006202144519072000115691223072021                 000000000826640000000000000000000000000000000000000160303400677769
R104451907200011520210632021063007585601M202106000000000000000000000000000000008871400001901592818
R114451907200011520210632021063007585601M202106000000000000000000000003006202144519072000115585623072021                 000000003956010000000000000000000000000000000000000887143356851161
R104451907200011520210632021063011595207M202106000000000000000000000000000000028730500000177289653
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220072021                 000000000012060000000000000000000000000000000000000012060438111735
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220072021                 000000000019530000000000000000000000000000000000000019530987578194
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220072021                 000000000050140000000000000000000000000000000000000050142112615907
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220072021                 000000000067330000000000000000000000000000000000000067333225646110
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220072021                 000000000097930000000000000000000000000000000000000097931342975477
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220072021                 000000000295280000000000000000000000000000000000000295282075246922
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220072021                 000000000614310000000000000000000000000000000000000614312266631350
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220072021                 000000000639420000000000000000000000000000000000000639421330104078
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220082021                 000000000012060000000000000000000000000000000000000012060121359843
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220082021                 000000000066910000000000000000000000000000000000000066910999761844
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220082021                 000000000085760000000000000000000000000000000000000085760226637290
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220082021                 000000000295280000000000000000000000000000000000000295281721172318
R114451907200011520210632021063011595207M202106000000000000000000000003006202144519072000115595220082021                 000000000617040000000000000000000000000000000000000617040044288210
T94451907200011520210632021063000039                                                        2391966929
DCTFM   44519072000115SUPERMERCADO IWAMOTO LTDA                                                                                          202106109978701937133006202100000003149752190003000751070013006          3426063101
REC0010000000175019800000000000000000000000000000000000014062900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000086687600000000000000000000000000000000000001603000000000000000000000000000000000000008871400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000028730500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001478175418
REC002000000000000000000000000000000000000000000000000000000002692473351
REC003FUMIO IWAMOTO                                               56115474868                                                                       3354675121
R9SUPERMERCADO IWAMOTO LTDA                                                                                          0000408012567963494850875";

let mut linhas = exemplo_dctf.lines();

processar_linhas(linhas);

    
}
