#!/bin/bash

echo tsw5.sh - INTERMEDIATE PROCESSING OF .FRQ FILES
echo deletions: tsw5dels.sh
echo  exploring REPITCH processing options

echo INPUTS	
echo	vocalsnd.wav    4.934    sec 44100 mono
echo	drumsnd.wav	4.95	 sec 44100 mono
echo	hornsnd.wav     5.386236 sec 44100 mono
echo	bellsnd.wav     4.899932 sec 44100 mono	
echo BREAKPOINT FILES

echo TEXT FILES
echo	tsw5qset.txt:  54 55 56

echo 

echo PRE-PROCESSING - COPY INPUT TO GENERIC FILENAME, ANALYSE 
echo  AND EXTRACT PITCH TRACE \(.FRQ\)
echo copysfx vocalsnd.wav tsw5
copysfx vocalsnd.wav tsw5
echo pvoc anal 1 tsw5.wav tsw5.ana
pvoc anal 1 tsw5.wav tsw5.ana
echo repitch getpitch 1 tsw5.ana tsw5hearfrq.ana tsw5.frq
repitch getpitch 1 tsw5.ana tsw5hearfrq.ana tsw5.frq
echo pvoc synth tsw5hearfrq.ana tsw5hearfrq.wav
pvoc synth tsw5hearfrq.ana tsw5hearfrq.wav
echo rm tsw5hearfrq.ana
rm tsw5hearfrq.ana
read -n1 -sp "press any key..."
echo
paplay tsw5hearfrq.wav
echo 

echo 1ST PROCESS - EXAGGERATE PITCH CONTOUR
echo repitch exag 5 tsw5.frq tsw5x.frq 55 18 1
repitch exag 5 tsw5.frq tsw5x.frq 55 18 1
echo repitch exag 6 tsw5.frq tsw5x.trn 55 18 1
repitch exag 6 tsw5.frq tsw5x.trn 55 18 1
echo repitch transpose 4 tsw5.ana tsw5x.trn tsw5x.ana
repitch transpose 4 tsw5.ana tsw5x.trn tsw5x.ana
echo pvoc synth tsw5x.ana tsw5x.wav
pvoc synth tsw5x.ana tsw5x.wav
echo rm tsw5x.ana
rm tsw5x.ana
read -n1 -sp "press any key..."
echo
paplay tsw5x.wav
echo 

echo 2nd PROCESS - INVERT PITCH CONTOUR
echo repitch invert 1 tsw5x.frq tsw5xi.frq 0
repitch invert 1 tsw5x.frq tsw5xi.frq 0
echo repitch invert 2 tsw5x.frq tsw5xi.trn 0
repitch invert 2 tsw5x.frq tsw5xi.trn 0
echo repitch transpose 4 tsw5.ana tsw5xi.trn tsw5xi.ana
repitch transpose 4 tsw5.ana tsw5xi.trn tsw5xi.ana
echo pvoc synth tsw5xi.ana tsw5xi.wav
pvoc synth tsw5xi.ana tsw5xi.wav
echo rm tsw5xi.ana
rm tsw5xi.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xi.wav
echo 

echo 3rd PROCESS - QUANTISE PITCH CONTOUR
echo Inexplicable:  the previous INVERT seems to be lost
echo repitch quantise 1 tsw5xi.frq tsw5xiq.frq tsw5qset.txt
repitch quantise 1 tsw5xi.frq tsw5xiq.frq tsw5qset.txt
echo repitch quantise 2 tsw5xi.frq tsw5xiq.trn tsw5qset.txt
repitch quantise 2 tsw5xi.frq tsw5xiq.trn tsw5qset.txt
echo repitch transpose 4 tsw5.ana tsw5xiq.trn tsw5xiq.ana
repitch transpose 4 tsw5.ana tsw5xiq.trn tsw5xiq.ana
echo pvoc synth tsw5xiq.ana tsw5xiq.wav
pvoc synth tsw5xiq.ana tsw5xiq.wav
echo rm tsw5xiq.ana
rm tsw5xiq.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiq.wav
echo 

echo 4th PROCESS - RANDOMISE PITCH CONTOUR
echo repitch randomise 1 tsw5xiq.frq tsw5xiqr.frq 19 100
repitch randomise 1 tsw5xiq.frq tsw5xiqr.frq 19 100
echo repitch randomise 2 tsw5xiq.frq tsw5xiqr.trn 19 100
repitch randomise 2 tsw5xiq.frq tsw5xiqr.trn 19 100
echo repitch transpose 4 tsw5.ana tsw5xiqr.trn tsw5xiqr.ana
repitch transpose 4 tsw5.ana tsw5xiqr.trn tsw5xiqr.ana
echo pvoc synth tsw5xiqr.ana tsw5xiqr.wav
pvoc synth tsw5xiqr.ana tsw5xiqr.wav
echo rm tsw5xiqr.ana
rm tsw5xiqr.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiqr.wav
echo 

echo 5th PROCESS - SMOOTH PITCH CONTOUR
echo repitch smooth 1 tsw5xiqr.frq tsw5xiqrs.frq 500
repitch smooth 1 tsw5xiqr.frq tsw5xiqrs.frq 500
echo repitch smooth 2 tsw5xiqr.frq tsw5xiqrs.trn 500
repitch smooth 2 tsw5xiqr.frq tsw5xiqrs.trn 500
echo repitch transpose 4 tsw5.ana tsw5xiqrs.trn tsw5xiqrs.ana
repitch transpose 4 tsw5.ana tsw5xiqrs.trn tsw5xiqrs.ana
echo pvoc synth tsw5xiqrs.ana tsw5xiqrs.wav
pvoc synth tsw5xiqrs.ana tsw5xiqrs.wav
echo rm tsw5xiqrs.ana
rm tsw5xiqrs.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiqrs.wav
echo 

echo 6th PROCESS - PCHSHIFT PITCH CONTOUR
echo repitch pchshift tsw5xiqrs.frq tsw5xiqrsp.frq  7
repitch pchshift tsw5xiqrs.frq tsw5xiqrsp.frq  7
echo repitch combine 1 tsw5.frq tsw5xiqrsp.frq tsw5xiqrsp.trn
repitch combine 1 tsw5.frq tsw5xiqrsp.frq tsw5xiqrsp.trn
echo repitch transpose 4 tsw5.ana tsw5xiqrsp.trn tsw5xiqrsp.ana
repitch transpose 4 tsw5.ana tsw5xiqrsp.trn tsw5xiqrsp.ana
echo pvoc synth tsw5xiqrsp.ana tsw5xiqrsp.wav
pvoc synth tsw5xiqrsp.ana tsw5xiqrsp.wav
echo rm tsw5xiqrsp.ana
rm tsw5xiqrsp.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiqrsp.wav
echo 

echo 7th \& FINAL PROCESS - VIBRATO PITCH CONTOUR
echo repitch vibrato 2 tsw5xiqrsp.frq tsw5xiqrspv.trn 20 9
repitch vibrato 2 tsw5xiqrsp.frq tsw5xiqrspv.trn 20 9
echo 

echo CONVERT LAST .TRN TO .ANA and RESYNTHESISE
echo repitch transpose 4 tsw5.ana tsw5xiqrspv.trn tsw5xiqrspv.ana
repitch transpose 4 tsw5.ana tsw5xiqrspv.trn tsw5xiqrspv.ana
echo pvoc synth tsw5xiqrspv.ana tsw5xiqrspv.wav
pvoc synth tsw5xiqrspv.ana tsw5xiqrspv.wav
echo rm tsw5xiqrspv.ana
rm tsw5xiqrspv.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiqrspv.wav
echo 

echo to delete these files and use a different input, run tsw5dels.sh
echo then edit the COPYSFX line\(s\) in tsw5.sh to create new generic infile\(s\)
read -n1 -sp "press any key..."
echo
echo 


