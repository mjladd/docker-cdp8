#!/bin/bash

echo tsw5.sh - INTERMEDIATE PROCESSING OF .FRQ FILES
echo deletions: tsw5dels.sh
echo  exploring REPITCH processing options

echo INPUTS	
echo	vocalsnd.aiff    4.934    sec 44100 mono
echo	drumsnd.aiff	4.95	 sec 44100 mono
echo	hornsnd.aiff     5.386236 sec 44100 mono
echo	bellsnd.aiff     4.899932 sec 44100 mono	
echo BREAKPOINT FILES

echo TEXT FILES
echo	tsw5qset.txt:  54 55 56

echo 

echo PRE-PROCESSING - COPY INPUT TO GENERIC FILENAME, ANALYSE 
echo  AND EXTRACT PITCH TRACE \(.FRQ\)
echo copysfx vocalsnd tsw5
copysfx vocalsnd tsw5
echo pvoc anal 1 tsw5 tsw5.ana
pvoc anal 1 tsw5 tsw5.ana
echo repitch getpitch 1 tsw5.ana tsw5hearfrq.ana tsw5.frq
repitch getpitch 1 tsw5.ana tsw5hearfrq.ana tsw5.frq
echo pvoc synth tsw5hearfrq.ana tsw5hearfrq
pvoc synth tsw5hearfrq.ana tsw5hearfrq
echo rm tsw5hearfrq.ana
rm tsw5hearfrq.ana
read -n1 -sp "press any key..."
echo
paplay tsw5hearfrq.aiff
echo 

echo 1ST PROCESS - EXAGGERATE PITCH CONTOUR
echo repitch exag 5 tsw5.frq tsw5x.frq 55 18 1
repitch exag 5 tsw5.frq tsw5x.frq 55 18 1
echo repitch exag 6 tsw5.frq tsw5x.trn 55 18 1
repitch exag 6 tsw5.frq tsw5x.trn 55 18 1
echo repitch transpose 4 tsw5.ana tsw5x.trn tsw5x.ana
repitch transpose 4 tsw5.ana tsw5x.trn tsw5x.ana
echo pvoc synth tsw5x.ana tsw5x
pvoc synth tsw5x.ana tsw5x
echo rm tsw5x.ana
rm tsw5x.ana
read -n1 -sp "press any key..."
echo
paplay tsw5x.aiff
echo 

echo 2nd PROCESS - INVERT PITCH CONTOUR
echo repitch invert 1 tsw5x.frq tsw5xi.frq 0
repitch invert 1 tsw5x.frq tsw5xi.frq 0
echo repitch invert 2 tsw5x.frq tsw5xi.trn 0
repitch invert 2 tsw5x.frq tsw5xi.trn 0
echo repitch transpose 4 tsw5.ana tsw5xi.trn tsw5xi.ana
repitch transpose 4 tsw5.ana tsw5xi.trn tsw5xi.ana
echo pvoc synth tsw5xi.ana tsw5xi
pvoc synth tsw5xi.ana tsw5xi
echo rm tsw5xi.ana
rm tsw5xi.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xi.aiff
echo 

echo 3rd PROCESS - QUANTISE PITCH CONTOUR
echo Inexplicable:  the previous INVERT seems to be lost
echo repitch quantise 1 tsw5xi.frq tsw5xiq.frq tsw5qset.txt
repitch quantise 1 tsw5xi.frq tsw5xiq.frq tsw5qset.txt
echo repitch quantise 2 tsw5xi.frq tsw5xiq.trn tsw5qset.txt
repitch quantise 2 tsw5xi.frq tsw5xiq.trn tsw5qset.txt
echo repitch transpose 4 tsw5.ana tsw5xiq.trn tsw5xiq.ana
repitch transpose 4 tsw5.ana tsw5xiq.trn tsw5xiq.ana
echo pvoc synth tsw5xiq.ana tsw5xiq
pvoc synth tsw5xiq.ana tsw5xiq
echo rm tsw5xiq.ana
rm tsw5xiq.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiq.aiff
echo 

echo 4th PROCESS - RANDOMISE PITCH CONTOUR
echo repitch randomise 1 tsw5xiq.frq tsw5xiqr.frq 19 100
repitch randomise 1 tsw5xiq.frq tsw5xiqr.frq 19 100
echo repitch randomise 2 tsw5xiq.frq tsw5xiqr.trn 19 100
repitch randomise 2 tsw5xiq.frq tsw5xiqr.trn 19 100
echo repitch transpose 4 tsw5.ana tsw5xiqr.trn tsw5xiqr.ana
repitch transpose 4 tsw5.ana tsw5xiqr.trn tsw5xiqr.ana
echo pvoc synth tsw5xiqr.ana tsw5xiqr
pvoc synth tsw5xiqr.ana tsw5xiqr
echo rm tsw5xiqr.ana
rm tsw5xiqr.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiqr.aiff
echo 

echo 5th PROCESS - SMOOTH PITCH CONTOUR
echo repitch smooth 1 tsw5xiqr.frq tsw5xiqrs.frq 500
repitch smooth 1 tsw5xiqr.frq tsw5xiqrs.frq 500
echo repitch smooth 2 tsw5xiqr.frq tsw5xiqrs.trn 500
repitch smooth 2 tsw5xiqr.frq tsw5xiqrs.trn 500
echo repitch transpose 4 tsw5.ana tsw5xiqrs.trn tsw5xiqrs.ana
repitch transpose 4 tsw5.ana tsw5xiqrs.trn tsw5xiqrs.ana
echo pvoc synth tsw5xiqrs.ana tsw5xiqrs
pvoc synth tsw5xiqrs.ana tsw5xiqrs
echo rm tsw5xiqrs.ana
rm tsw5xiqrs.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiqrs.aiff
echo 

echo 6th PROCESS - PCHSHIFT PITCH CONTOUR
echo repitch pchshift tsw5xiqrs.frq tsw5xiqrsp.frq  7
repitch pchshift tsw5xiqrs.frq tsw5xiqrsp.frq  7
echo repitch combine 1 tsw5.frq tsw5xiqrsp.frq tsw5xiqrsp.trn
repitch combine 1 tsw5.frq tsw5xiqrsp.frq tsw5xiqrsp.trn
echo repitch transpose 4 tsw5.ana tsw5xiqrsp.trn tsw5xiqrsp.ana
repitch transpose 4 tsw5.ana tsw5xiqrsp.trn tsw5xiqrsp.ana
echo pvoc synth tsw5xiqrsp.ana tsw5xiqrsp
pvoc synth tsw5xiqrsp.ana tsw5xiqrsp
echo rm tsw5xiqrsp.ana
rm tsw5xiqrsp.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiqrsp.aiff
echo 

echo 7th \& FINAL PROCESS - VIBRATO PITCH CONTOUR
echo repitch vibrato 2 tsw5xiqrsp.frq tsw5xiqrspv.trn 20 9
repitch vibrato 2 tsw5xiqrsp.frq tsw5xiqrspv.trn 20 9
echo 

echo CONVERT LAST .TRN TO .ANA and RESYNTHESISE
echo repitch transpose 4 tsw5.ana tsw5xiqrspv.trn tsw5xiqrspv.ana
repitch transpose 4 tsw5.ana tsw5xiqrspv.trn tsw5xiqrspv.ana
echo pvoc synth tsw5xiqrspv.ana tsw5xiqrspv
pvoc synth tsw5xiqrspv.ana tsw5xiqrspv
echo rm tsw5xiqrspv.ana
rm tsw5xiqrspv.ana
read -n1 -sp "press any key..."
echo
paplay tsw5xiqrspv.aiff
echo 

echo to delete these files and use a different input, run tsw5dels.bat
echo then edit the COPYSFX line\(s\) in tsw5.bat to create new generic infile\(s\)
read -n1 -sp "press any key..."
echo
echo 


