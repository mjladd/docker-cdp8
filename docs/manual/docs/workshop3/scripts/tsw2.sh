#!/bin/bash

echo tsw2.sh - Transposition \& Shifting Workshop Batchfile
echo	SIMPLE TRANSPOSITIONS - all examples: up one octave
echo the deletions file is tsw2dels.sh

echo INPUTS
echo	vocalsnd.wav    4.934    sec 44100 mono
echo	drumsnd.wav	4.95	 sec 44100 mono
echo	hornsnd.wav     5.386236 sec 44100 mono
echo	bellsnd.wav     4.899932 sec 44100 mono	
echo BREAKPOINT FILES
echo	tsw2harm.txt:  4 0.5, 8 0.5, 16 0.5, 32 0.5
echo	tsw2brk.brk:  0 0, 1 12, 3 -24, 4.5 12

echo 

echo PRE-PROCESSING: COPY SOUND TO GENERIC NAME AND ANALYSE
echo copysfx vocalsnd.wav tsw2.wav
copysfx vocalsnd.wav tsw2.wav
echo pvoc anal 1 tsw2.wav tsw2.ana
pvoc anal 1 tsw2.wav tsw2.ana
echo 

echo TRANSPOSE WHOLE SOUND, MAINTAINING HARMONICITY AND 
echo   NOT PRESERVING FORMANTS
echo 

echo PART 1 - IN THE TIME-DOMAIN \(VARIOUS EXAMPLES\)
echo 

echo TRANSPOSE BY A CONSTANT
echo modify speed 2 tsw2.wav tsw2u12.wav  12
modify speed 2 tsw2.wav tsw2u12.wav  12
read -n1 -sp "press any key..."
echo
paplay tsw2u12.wav
echo 

echo TIME-VARYING TRANSPOSITION
echo modify speed 2 tsw2.wav tsw2tvtrn.wav tsw2brk.brk
modify speed 2 tsw2.wav tsw2tvtrn.wav tsw2brk.brk
read -n1 -sp "press any key..."
echo
paplay tsw2tvtrn.wav
echo 

echo ACCELERATING TRANSPOSITION
echo modify speed 5 tsw2.wav tsw2accel.wav 12 2
modify speed 5 tsw2.wav tsw2accel.wav 12 2
read -n1 -sp "press any key..."
echo
paplay tsw2accel.wav
echo 

echo PITCH DISPLACEMENT VIBRATO
echo modify speed 6 tsw2.wav tsw2vib.wav 12 7
modify speed 6 tsw2.wav tsw2vib.wav 12 7
read -n1 -sp "press any key..."
echo
paplay tsw2vib.wav
echo 

echo DISTORTION BY MULTIPLYING PSEUDO.wavECYCLE FREQUENCIES
echo distort multiply tsw2.wav tsw2mult.wav 2
distort multiply tsw2.wav tsw2mult.wav 2
read -n1 -sp "press any key..."
echo
paplay tsw2mult.wav
echo 

echo DISTORTION BY SUPERIMPOSING PSEUDO-HARMONICS
echo distort harmonic tsw2.wav tsw2harm.wav tsw2harm.txt -p0.5
distort harmonic tsw2.wav tsw2harm.wav tsw2harm.txt -p0.5
read -n1 -sp "press any key..."
echo
paplay tsw2harm.wav
echo 

echo PART 2 - IN THE SPECTRAL-DOMAIN
echo 

echo TRANSP: FRQSPLIT AT BOTTOM \(10Hz\); TRANSPOSE FACTOR IS A CONSTANT
echo pitch transp 4 tsw2.ana tsw2transp.ana 10 12
pitch transp 4 tsw2.ana tsw2transp.ana 10 12
echo pvoc synth tsw2transp.ana tsw2transp.wav
pvoc synth tsw2transp.ana tsw2transp.wav
echo rm tsw2transp.ana
rm tsw2transp.ana
read -n1 -sp "press any key..."
echo
paplay tsw2transp.wav
echo 

echo TRANSPOSE WITHOUT PRESERVING FORMANTS
echo repitch transpose 3 tsw2.ana tsw2tpose.ana tsw2brk.brk
repitch transpose 3 tsw2.ana tsw2tpose.ana tsw2brk.brk
echo pvoc synth tsw2tpose.ana tsw2tpose.wav
pvoc synth tsw2tpose.ana tsw2tpose.wav
echo rm tsw2tpose.ana
rm tsw2tpose.ana
read -n1 -sp "press any key..."
echo
paplay tsw2tpose.wav
echo 

echo TRANSPOSEF - FORMANTS PRESERVED
echo repitch transposef 3 tsw2.ana tsw2tposef.ana -f12 tsw2brk.brk
repitch transposef 3 tsw2.ana tsw2tposef.ana -f12 tsw2brk.brk
echo pvoc synth tsw2tposef.ana tsw2tposef.wav
pvoc synth tsw2tposef.ana tsw2tposef.wav
echo rm tsw2tposef.ana
rm tsw2tposef.ana
read -n1 -sp "press any key..."
echo
paplay tsw2tposef.wav
echo 

echo to delete these files and use a different input, run tsw2dels.sh
echo then edit the COPYSFX line\(s\) in tsw2.bat to create new generic infile\(s\)
read -n1 -sp "press any key..."
echo
echo 


