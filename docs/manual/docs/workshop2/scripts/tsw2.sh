#!/bin/bash

echo tsw2.sh - Transposition \& Shifting Workshop Batchfile
echo	SIMPLE TRANSPOSITIONS - all examples: up one octave
echo the deletions file is tsw2dels.sh

echo INPUTS
echo	vocalsnd.aiff    4.934    sec 44100 mono
echo	drumsnd.aiff	4.95	 sec 44100 mono
echo	hornsnd.aiff     5.386236 sec 44100 mono
echo	bellsnd.aiff     4.899932 sec 44100 mono	
echo BREAKPOINT FILES
echo	tsw2harm.txt:  4 0.5, 8 0.5, 16 0.5, 32 0.5
echo	tsw2brk.brk:  0 0, 1 12, 3 -24, 4.5 12

echo 

echo PRE-PROCESSING: COPY SOUND TO GENERIC NAME AND ANALYSE
echo copysfx vocalsnd tsw2
copysfx vocalsnd tsw2
echo pvoc anal 1 tsw2 tsw2.ana
pvoc anal 1 tsw2 tsw2.ana
echo 

echo TRANSPOSE WHOLE SOUND, MAINTAINING HARMONICITY AND 
echo   NOT PRESERVING FORMANTS
echo 

echo PART 1 - IN THE TIME-DOMAIN \(VARIOUS EXAMPLES\)
echo 

echo TRANSPOSE BY A CONSTANT
echo modify speed 2 tsw2 tsw2u12  12
modify speed 2 tsw2 tsw2u12  12
read -n1 -sp "press any key..."
echo
paplay tsw2u12.aiff
echo 

echo TIME-VARYING TRANSPOSITION
echo modify speed 2 tsw2 tsw2tvtrn tsw2brk.brk
modify speed 2 tsw2 tsw2tvtrn tsw2brk.brk
read -n1 -sp "press any key..."
echo
paplay tsw2tvtrn.aiff
echo 

echo ACCELERATING TRANSPOSITION
echo modify speed 5 tsw2 tsw2accel 12 2
modify speed 5 tsw2 tsw2accel 12 2
read -n1 -sp "press any key..."
echo
paplay tsw2accel.aiff
echo 

echo PITCH DISPLACEMENT VIBRATO
echo modify speed 6 tsw2 tsw2vib 12 7
modify speed 6 tsw2 tsw2vib 12 7
read -n1 -sp "press any key..."
echo
paplay tsw2vib.aiff
echo 

echo DISTORTION BY MULTIPLYING PSEUDO.aiffECYCLE FREQUENCIES
echo distort multiply tsw2 tsw2mult 2
distort multiply tsw2 tsw2mult 2
read -n1 -sp "press any key..."
echo
paplay tsw2mult.aiff
echo 

echo DISTORTION BY SUPERIMPOSING PSEUDO-HARMONICS
echo distort harmonic tsw2 tsw2harm tsw2harm.txt -p0.5
distort harmonic tsw2 tsw2harm tsw2harm.txt -p0.5
read -n1 -sp "press any key..."
echo
paplay tsw2harm.aiff
echo 

echo PART 2 - IN THE SPECTRAL-DOMAIN
echo 

echo TRANSP: FRQSPLIT AT BOTTOM \(10Hz\); TRANSPOSE FACTOR IS A CONSTANT
echo pitch transp 4 tsw2.ana tsw2transp.ana 10 12
pitch transp 4 tsw2.ana tsw2transp.ana 10 12
echo pvoc synth tsw2transp.ana tsw2transp
pvoc synth tsw2transp.ana tsw2transp
echo rm tsw2transp.ana
rm tsw2transp.ana
read -n1 -sp "press any key..."
echo
paplay tsw2transp.aiff
echo 

echo TRANSPOSE WITHOUT PRESERVING FORMANTS
echo repitch transpose 3 tsw2.ana tsw2tpose.ana tsw2brk.brk
repitch transpose 3 tsw2.ana tsw2tpose.ana tsw2brk.brk
echo pvoc synth tsw2tpose.ana tsw2tpose
pvoc synth tsw2tpose.ana tsw2tpose
echo rm tsw2tpose.ana
rm tsw2tpose.ana
read -n1 -sp "press any key..."
echo
paplay tsw2tpose.aiff
echo 

echo TRANSPOSEF - FORMANTS PRESERVED
echo repitch transposef 3 tsw2.ana tsw2tposef.ana -f12 tsw2brk.brk
repitch transposef 3 tsw2.ana tsw2tposef.ana -f12 tsw2brk.brk
echo pvoc synth tsw2tposef.ana tsw2tposef
pvoc synth tsw2tposef.ana tsw2tposef
echo rm tsw2tposef.ana
rm tsw2tposef.ana
read -n1 -sp "press any key..."
echo
paplay tsw2tposef.aiff
echo 

echo to delete these files and use a different input, run tsw2dels.bat
echo then edit the COPYSFX line\(s\) in tsw1.bat to create new generic infile\(s\)
read -n1 -sp "press any key..."
echo
echo 


