#!/bin/bash

echo txwsmotf.bat - batch file for Texture Workshop Ex 3-7-11-15

echo 

echo texture prgname Mode inf.. outf ndf.txt outdur pk scatter 
echo tgrid snd-snd gain-gain pch-pch  phgrid gpspace gpsprange 
echo amprise contour  mult-mult [-a] [-p] [-s] [-w]
echo 

echo 

echo TEXTURE MOTIFS EXAMPLES 3-7-11-15
echo Example 3
echo texture motifs 5 marimba txws3test txws3nd.txt 12 2.0 0  0 1 1 40 80  60 60  0 0 1 0 0  0.5 0.5  -p0.5 -s1
texture motifs 5 marimba txws3test txws3nd.txt 12 2.0 0  0 1 1 40 80  60 60  0 0 1 0 0  0.5 0.5  -p0.5 -s1
echo 

echo Example 7
echo texture motifs 5 marimba txws7test txws7nd.txt 12 0.25 0  0 1 1 56 84  57 64  0 0 1 0 0  0.5 0.5  -a0.8 -p0.5 -s1
texture motifs 5 marimba txws7test txws7nd.txt 12 0.25 0  0 1 1 56 84  57 64  0 0 1 0 0  0.5 0.5  -a0.8 -p0.5 -s1
echo 

echo Example 11
echo texture motifsin 3 marimba txws11test txws11nd.txt 12 0.25 0  0 1 1 60 84  58 75  0 0 1 0 0  0.5 0.5  -a0.8 -p0.5 -s1 -w
texture motifsin 3 marimba txws11test txws11nd.txt 12 0.25 0  0 1 1 60 84  58 75  0 0 1 0 0  0.5 0.5  -a0.8 -p0.5 -s1 -w
echo 

echo Example 15
echo texture motifsin 4 marimba txws15test txws15nd.txt 12 txws15pk.brk 0  0 1 1  72 84  60 65  0 0 1 0 0  0.8 1.2  -a0.8 -p0.5 -s1
texture motifsin 4 marimba txws15test txws15nd.txt 12 txws15pk.brk 0  0 1 1  72 84  60 65  0 0 1 0 0  0.8 1.2  -a0.8 -p0.5 -s1
echo 


