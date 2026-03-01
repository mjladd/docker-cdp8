#!/bin/bash

echo txwssimp.bat - batch file for Texture Workshop Ex 1-5-9-13

echo 

echo texture simple mode inf.. outf ndf.txt outdur pk scatter 
echo tgrid snd-snd gain-gain dur-dur pch-pch [-a] [-p] [-s] [-w]
echo 

echo 

echo TEXTURE SIMPLE EXAMPLES 1-5-9-13
echo Example 1
echo texture simple 5 marimba txws1test txws1nd.txt 10 0.5 0.15 0  1 1  40 80  0.5 0.7  60 60  -p0.5 -s1
texture simple 5 marimba txws1test txws1nd.txt 10 0.5 0.15 0  1 1  40 80  0.5 0.7  60 60  -p0.5 -s1
echo 

echo Example 5
echo texture simple 5 marimba txws5test txws5nd.txt 13 txws5pk.brk 0.06 0  1 1  64 84  0.5 1.0  60 72  -p0.5 -s1
texture simple 5 marimba txws5test txws5nd.txt 13 txws5pk.brk 0.06 0  1 1  64 84  0.5 1.0  60 72  -p0.5 -s1
echo 

echo Example 9
echo texture simple 3 marimba txws9test txws9nd.txt 13 txws5pk.brk 0.06 0  1 1  64 84  0.5 1.0  60 72  -a0.8 -p0.5 -s1
texture simple 3 marimba txws9test txws9nd.txt 13 txws5pk.brk 0.06 0  1 1  64 84  0.5 1.0  60 72  -a0.8 -p0.5 -s1
echo 

echo Example 13
echo texture simple 4 marimba txws13test txws13nd.txt 13 txws13pk.brk 0.06 0 1 1  40 80  0.5 1.0  txws13pl.brk txws13ph.brk  -a0.8 -p0.5 -s1
texture simple 4 marimba txws13test txws13nd.txt 13 txws13pk.brk 0.06 0 1 1  40 80  0.5 1.0  txws13pl.brk txws13ph.brk  -a0.8 -p0.5 -s1
echo 


