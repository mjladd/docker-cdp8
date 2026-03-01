#!/bin/bash

echo txwstmot.bat - batch file for Texture Workshop Ex 4-8-12-16

echo 

echo TEXTURE TMOTIFS EXAMPLES 4-8-12-16
echo 

echo texture tmotifs Mode inf.. outf ndf.txt outdur skiptime  
echo snd-snd gain-gain pch-pch  phgrid gpspace gpsprange 
echo amprise contour  mult-mult [-a] [-p] [-s] [-w]
echo 

echo Example 4
echo texture tmotifs 5 marimba txws4test txws4nd.txt 13  1.0  1 1 64 84  60 60  0 0 1 0 0  1 1  -p0.5 -s1
texture tmotifs 5 marimba txws4test txws4nd.txt 13  1.0  1 1 64 84  60 60  0 0 1 0 0  1 1  -p0.5 -s1
echo 

echo Example 8
echo texture tmotifs 5 gtrcdt txws8test txws8nd.txt 12  1.0  1 1 60 84  55 72  0 0 1 0 0  0.5 0.5  -a0.8 -p0.5 -s1
texture tmotifs 5 gtrcdt txws8test txws8nd.txt 12  1.0  1 1 60 84  55 72  0 0 1 0 0  0.5 0.5  -a0.8 -p0.5 -s1
echo 

echo Example 12
echo texture tmotifsin 3 gtrcdt marimba txws12test txws12nd.txt 12  0.5 1 1  64 84  55 79  0 0 1 0 0  0.7 0.7  -a0.7 -p0.5 -s1
texture tmotifsin 3 gtrcdt marimba txws12test txws12nd.txt 12  0.5 1 1  64 84  55 79  0 0 1 0 0  0.7 0.7  -a0.7 -p0.5 -s1
echo 

echo Example 16
echo texture tmotifsin 4 gtrcdt marimba txws16test txws16nd.txt 20  0.33 1 1  64 84  55 79  0 0 1 0 0  0.7 0.7 -a0.7  -p0.5 -s1
texture tmotifsin 4 gtrcdt marimba txws16test txws16nd.txt 20  0.33 1 1  64 84  55 79  0 0 1 0 0  0.7 0.7 -a0.7  -p0.5 -s1
echo 


