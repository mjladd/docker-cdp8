#!/bin/bash

echo txws1720.bat - batch file for Texture Workshop Ex 17-18-19-20

echo 

echo TEXTURE DECOR \& ORNATE EXAMPLES 17-18-19-20:  txws1720.bat
echo 

echo POSTDECOR
echo texture prgname Mode inf.. outf ndf.txt outdur skiptime 
echo snd-snd gain-gain dur-dur phgrid gpspace gpsprange amprise contour 
echo gppack-gppack gprange-gprange centring [-a] [-p] [-s] [-w]
echo 

echo Example 17
echo texture postdecor 4 marimba txws17test txws17nd.txt 25  1.5  1 1 54 84  0.5 1.0  0 4 1 0 0  3 8 30 80 1 4  1  -a0.9 -p0.5 -s1
texture postdecor 4 marimba txws17test txws17nd.txt 25  1.5  1 1 54 84  0.5 1.0  0 4 1 0 0  3 8 30 80 1 4  1  -a0.9 -p0.5 -s1
echo 

echo 

echo POSTORNATE
echo texture prgname Mode inf.. outf ndf.txt outdur skiptime 
echo tgrid snd-snd gain-gain dur-dur  phgrid gpspace gpsprange 
echo amprise contour  mult-mult  [-a] [-p] [-s] [-w]
echo 

echo 

echo Example 18
echo texture postornate 5 gtrcdt marimba txws18test txws18nd.txt 30  4.98 1 2  66 80  0.3 2.1  0 1 1 25 5  0.83 0.83  -a0.9 -p0.5 -s1
texture postornate 5 gtrcdt marimba txws18test txws18nd.txt 30  4.98 1 2  66 80  0.3 2.1  0 1 1 25 5  0.83 0.83  -a0.9 -p0.5 -s1
echo 

echo Example 19
echo texture postornate 3 gtrcdt marimba txws19test txws19nd.txt 30  4.98 1 2  66 80  0.3 2.1  0 1 1 25 5  0.83 0.83  -a0.9 -p0.5 -s1
texture postornate 3 gtrcdt marimba txws19test txws19nd.txt 30  4.98 1 2  66 80  0.3 2.1  0 1 1 25 5  0.83 0.83  -a0.9 -p0.5 -s1
echo 

echo Example 20
echo texture postornate 3 marimba txws20test txws20nd.txt 24  2.0  1 1 76 80  0.3 1.0  0 1 1 25 5  txws20ml.brk txws20mh.brk  -a0.9 -p0.5 -s1
texture postornate 3 marimba txws20test txws20nd.txt 24  2.0  1 1 76 80  0.3 1.0  0 1 1 25 5  txws20ml.brk txws20mh.brk  -a0.9 -p0.5 -s1
echo 




