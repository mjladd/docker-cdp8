#!/bin/bash

echo txwstimd.bat - batch file for Texture Workshop Ex 2-6-10-14

echo 

echo texture timed mode inf.. outf ndf.txt outdur skiptime 
echo snd-snd gain-gain dur-dur pch-pich [-a] [-p] [-s] [-w]
echo 

echo 

echo TEXTURE TIMED EXAMPLES 2-6-10-14
echo 

echo Example 2
echo texture timed 5 marimba txws2test txws2nd.txt 12  1.25  1 1  40 80  0.5 1.0  60 60  -ptxws2ps.brk -s1
texture timed 5 marimba txws2test txws2nd.txt 12  1.25  1 1  40 80  0.5 1.0  60 60  -ptxws2ps.brk -s1
echo 

echo Example 6
echo texture timed 5 marimba txws6test txws2nd.txt 12  1.25  1 1  40 80  0.5 1.0  60 72  -ptxws2ps.brk -s1
texture timed 5 marimba txws6test txws2nd.txt 12  1.25  1 1  40 80  0.5 1.0  60 72  -ptxws2ps.brk -s1
echo 

echo Example 10
echo texture timed 3 marimba txws10test txws10nd.txt 12  1.25  1 1  40 80  0.5 1.0  60 70  -ptxws2ps.brk -s1
texture timed 3 marimba txws10test txws10nd.txt 12  1.25  1 1  40 80  0.5 1.0  60 70  -ptxws2ps.brk -s1
echo 

echo Example 14
echo texture timed 4 marimba txws14test txws14nd.txt 14  0.1  1 1  40 80  0.5 1.0  txws14pl.brk txws14ph.brk  -p0.5 -s1
texture timed 4 marimba txws14test txws14nd.txt 14  0.1  1 1  40 80  0.5 1.0  txws14pl.brk txws14ph.brk  -p0.5 -s1
echo 


