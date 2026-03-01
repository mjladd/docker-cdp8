#!/bin/bash

echo tswpfopt.sh - exploring -f and -p formant extraction methods
echo	see tswpfdel.sh for deleting the remaining outputs
echo INPUTS	
echo	vocalsnd.wav    4.934    sec 44100 mono
echo	drumsnd.wav     4.95     sec 44100 mono
echo	hornsnd.wav     5.386236 sec 44100 mono
echo	bellsnd.wav     4.899932 sec 44100 mono
echo 

echo copysfx hornsnd.wav tswpf.wav
copysfx hornsnd.wav tswpf.wav
echo pvoc anal 1 tswpf.wav tswpf.ana
pvoc anal 1 tswpf.wav tswpf.ana
echo repitch getpitch 1 tswpf.ana tswpfdummy.ana tswpf.frq
repitch getpitch 1 tswpf.ana tswpfdummy.ana tswpf.frq
echo pvoc synth tswpfdummy.ana tswpfdummy.wav
pvoc synth tswpfdummy.ana tswpfdummy.wav
echo rm tswpfdummy.ana
rm tswpfdummy.ana
read -n1 -sp "press any key..."
echo
paplay tswpfdummy.wav
echo rm tswpfdummy.wav
rm tswpfdummy.wav
echo 

echo Making just a slight pitch change so sounds like original
echo   and need to produce a .trn file for input to TRANSPOSEF
echo repitch pchshift tswpf.frq tswpfu1.frq 1
repitch pchshift tswpf.frq tswpfu1.frq 1
echo repitch combine 1 tswpf.frq tswpfu1.frq tswpfu1.trn
repitch combine 1 tswpf.frq tswpfu1.frq tswpfu1.trn
echo 

echo \'fex\' stands for \'formant extraction\'
echo 

echo LET -p VALUE = 4
echo repitch transposef 4 tswpf.ana tswpfu1.trn fexp4.ana -p4
repitch transposef 4 tswpf.ana tswpfu1.trn fexp4.ana -p4
echo pvoc synth fexp4.ana fexp4.wav
pvoc synth fexp4.ana fexp4.wav
echo rm fexp4.ana
rm fexp4.ana
read -n1 -sp "press any key..."
echo
paplay fexp4.wav
echo 

echo LET -f VALUE = 4
echo repitch transposef 4 tswpf.ana tswpfu1.trn fexf4.ana -f4
repitch transposef 4 tswpf.ana tswpfu1.trn fexf4.ana -f4
echo pvoc synth fexf4.ana fexf4.wav
pvoc synth fexf4.ana fexf4.wav
echo rm fexf4.ana
rm fexf4.ana
read -n1 -sp "press any key..."
echo
paplay fexf4.wav
echo 

echo LET -p VALUE = 6
echo repitch transposef 4 tswpf.ana tswpfu1.trn fexp6.ana -p6
repitch transposef 4 tswpf.ana tswpfu1.trn fexp6.ana -p6
echo pvoc synth fexp6.ana fexp6.wav
pvoc synth fexp6.ana fexp6.wav
echo rm fexp6.ana
rm fexp6.ana
read -n1 -sp "press any key..."
echo
paplay fexp6.wav
echo 

echo LET -f VALUE = 6
echo repitch transposef 4 tswpf.ana tswpfu1.trn fexf6.ana -f6
repitch transposef 4 tswpf.ana tswpfu1.trn fexf6.ana -f6
echo pvoc synth fexf6.ana fexf6.wav
pvoc synth fexf6.ana fexf6.wav
echo rm fexf6.ana
rm fexf6.ana
read -n1 -sp "press any key..."
echo
paplay fexf6.wav
echo 

echo LET -p VALUE = 9
echo repitch transposef 4 tswpf.ana tswpfu1.trn fexp9.ana -p9
repitch transposef 4 tswpf.ana tswpfu1.trn fexp9.ana -p9
echo pvoc synth fexp9.ana fexp9.wav
pvoc synth fexp9.ana fexp9.wav
echo rm fexp9.ana
rm fexp9.ana
read -n1 -sp "press any key..."
echo
paplay fexp9.wav
echo 

echo LET -f VALUE = 9
echo repitch transposef 4 tswpf.ana tswpfu1.trn fexf9.ana -f9
repitch transposef 4 tswpf.ana tswpfu1.trn fexf9.ana -f9
echo pvoc synth fexf9.ana fexf9.wav
pvoc synth fexf9.ana fexf9.wav
echo rm fexf9.ana
rm fexf9.ana
read -n1 -sp "press any key..."
echo
paplay fexf9.wav
echo 

echo LET -p VALUE = 12
echo repitch transposef 4 tswpf.ana tswpfu1.trn fexp12.ana -p12
repitch transposef 4 tswpf.ana tswpfu1.trn fexp12.ana -p12
echo pvoc synth fexp12.ana fexp12.wav
pvoc synth fexp12.ana fexp12.wav
echo rm fexp12.ana
rm fexp12.ana
read -n1 -sp "press any key..."
echo
paplay fexp12.wav
echo 

echo LET -f VALUE = 12
echo repitch transposef 4 tswpf.ana tswpfu1.trn fexf12.ana -f12
repitch transposef 4 tswpf.ana tswpfu1.trn fexf12.ana -f12
echo pvoc synth fexf12.ana fexf12.wav
pvoc synth fexf12.ana fexf12.wav
echo rm fexf12.ana
rm fexf12.ana
read -n1 -sp "press any key..."
echo
paplay fexf12.wav
echo 

echo to delete these files and use a different input, run tswpfdel.sh
echo then edit the COPYSFX line\(s\) in tswpfopt.sh to create new generic infile\(s\)
read -n1 -sp "press any key..."
echo
echo 

