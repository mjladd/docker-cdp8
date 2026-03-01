#!/bin/bash

echo tsw1.bat - realising the simple to more complex sequence of outputs
echo the deletions file is tsw1dels.sh

echo INPUTS \(vocal, drum, pitched instr., bell/gong -- each ca 5 sec.\)
echo	vocalsnd.aiff  4.934    sec 44100 mono
echo	drumsnd.aiff	  4.95	 sec 44100 mono
echo	hornsnd.aiff   5.386236 sec 44100 mono
echo	bellsnd.aiff   4.899932 sec 44100 mono
echo BREAKPOINT FILES
echo	glideup.brk:  0 0, 0.49 0, 0.5 -12, 4.0 24, 4.8 -12
echo TEXT FILES	
echo	harmon.txt: 2 0.6, 4  0.8

echo 

echo PRE-PROCESSING
echo 

echo  COPY 1ST SOUND TO GENERIC NAME 
echo copysfx vocalsnd tsw1
copysfx vocalsnd tsw1
echo pvoc anal 1 tsw1 tsw1.ana
pvoc anal 1 tsw1 tsw1.ana
echo repitch getpitch 1 tsw1.ana tsw1ptrace.ana tsw1.frq
repitch getpitch 1 tsw1.ana tsw1ptrace.ana tsw1.frq
echo pvoc synth tsw1ptrace.ana tsw1ptrace
pvoc synth tsw1ptrace.ana tsw1ptrace
echo rm tsw1ptrace.ana
rm tsw1ptrace.ana
read -n1 -sp "press any key..."
echo
echo 

echo	COPY 2ND SOUND TO GENERIC NAME
echo copysfx drumsnd tsw1-2nd
copysfx drumsnd tsw1-2nd
echo pvoc anal 1 tsw1-2nd tsw1-2nd.ana
pvoc anal 1 tsw1-2nd tsw1-2nd.ana
echo repitch getpitch 1 tsw1-2nd.ana tsw1-2ndptrace.ana tsw1-2nd.frq
repitch getpitch 1 tsw1-2nd.ana tsw1-2ndptrace.ana tsw1-2nd.frq
echo pvoc synth tsw1-2ndptrace.ana tsw1-2ndptrace
pvoc synth tsw1-2ndptrace.ana tsw1-2ndptrace
echo rm tsw1-2ndptrace.ana
rm tsw1-2ndptrace.ana
read -n1 -sp "press any key..."
echo
echo 

echo	RAISE THE PITCH TRACE OF THE 1ST SOUND AND THEN 
echo  COMBINE WITH ORIGINAL PITCH TRACE TO MAKE A .TRN FILE
echo repitch pchshift tsw1.frq tsw1u7.frq 7
repitch pchshift tsw1.frq tsw1u7.frq 7
echo repitch combine 1 tsw1.frq tsw1u7.frq tsw1u7.trn
repitch combine 1 tsw1.frq tsw1u7.frq tsw1u7.trn
read -n1 -sp "press any key..."
echo
echo 

echo HARMONICITY AND FORMANTS PRESERVED \(TRANSPOSEF\)
echo 

echo 1a. TRANSPOSEF BY A CONSTANT
echo repitch transposef 3 tsw1.ana tsw1-1a.ana -f24 7
repitch transposef 3 tsw1.ana tsw1-1a.ana -f24 7
echo pvoc synth tsw1-1a.ana tsw1-1a
pvoc synth tsw1-1a.ana tsw1-1a
echo rm tsw1-1a.ana
rm tsw1-1a.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-1a.aiff
echo 

echo 1b. TRANSPOSEF BY A TIME-VARYING BREAKPOINT FILE \(SEMITONES\)
echo repitch transposef 3 tsw1.ana tsw1-1b.ana -f24 glideup.brk
repitch transposef 3 tsw1.ana tsw1-1b.ana -f24 glideup.brk
echo pvoc synth tsw1-1b.ana tsw1-1b
pvoc synth tsw1-1b.ana tsw1-1b
echo rm tsw1-1b.ana
rm tsw1-1b.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-1b.aiff
echo 

echo 2a. APPLY .TRN \(RAISED UP 7 SEMITONES\) MADE FROM .FRQ OF 1ST SOUND
echo repitch transposef 4 tsw1.ana tsw1u7.trn tsw1-2a.ana -f24
repitch transposef 4 tsw1.ana tsw1u7.trn tsw1-2a.ana -f24
echo pvoc synth tsw1-2a.ana tsw1-2a
pvoc synth tsw1-2a.ana tsw1-2a
echo rm tsw1-2a.ana
rm tsw1-2a.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-2a.aiff
echo 

echo 2b. .FRQ MADE FROM 1ST SOUND USED AS OCTMOVE TRANSPOSITION PATTERN
echo pitch octmove 1 tsw1.ana tsw1.frq tsw1-2b.ana 3
pitch octmove 1 tsw1.ana tsw1.frq tsw1-2b.ana 3
echo pvoc synth tsw1-2b.ana tsw1-2b
pvoc synth tsw1-2b.ana tsw1-2b
echo rm tsw1-2b.ana 
rm tsw1-2b.ana 
read -n1 -sp "press any key..."
echo
paplay tsw1-2b.aiff
echo 

echo 3. TIME-VARYING TIME-DOMAIN TRANSPOSITION
echo modify speed 2 tsw1 tsw1-3 glideup.brk
modify speed 2 tsw1 tsw1-3 glideup.brk
read -n1 -sp "press any key..."
echo
paplay tsw1-3.aiff
echo 

echo 4. TIME-DOMAIN HARMONIC DISTORTION, WITH harmon.txt
echo distort harmonic tsw1 tsw1-4 harmon.txt -p0.75
distort harmonic tsw1 tsw1-4 harmon.txt -p0.75
read -n1 -sp "press any key..."
echo
paplay tsw1-4.aiff
echo 

echo 5. REPITCH TRANSPOSE \(WITHOUT PRESERVING FORMANTS\)
echo repitch transpose 3 tsw1.ana tsw1-5.ana glideup.brk
repitch transpose 3 tsw1.ana tsw1-5.ana glideup.brk
echo pvoc synth tsw1-5.ana tsw1-5
pvoc synth tsw1-5.ana tsw1-5
echo rm tsw1-5.ana
rm tsw1-5.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-5.aiff
echo 

echo 6. TRANSP Mode 3:  OCTAVE TRANSPOSE UP AND DOWN FROM SPLIT POINT
echo pitch transp 3 tsw1.ana tsw1-6.ana 1000
pitch transp 3 tsw1.ana tsw1-6.ana 1000
echo pvoc synth tsw1-6.ana tsw1-6
pvoc synth tsw1-6.ana tsw1-6
echo rm tsw1-6.ana
rm tsw1-6.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-6.aiff
echo 

echo 7. TRANSP Mode 6:  TRANSPOSE SPECIFIED AMOUNTS UP AND DOWN FROM SPLIT POINT
echo pitch transp 6 tsw1.ana tsw1-7.ana 1000 7 -5
pitch transp 6 tsw1.ana tsw1-7.ana 1000 7 -5
echo pvoc synth tsw1-7.ana tsw1-7
pvoc synth tsw1-7.ana tsw1-7
echo rm tsw1-7.ana
rm tsw1-7.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-7.aiff
echo 

echo 8. STRANGE SHIFT Mode 5:  INHARMONIC SHIFT UP AND DOWN OUTSIDE SPECIFIED RANGE
echo strange shift 5 tsw1.ana tsw1-8.ana 2000 900 1200
strange shift 5 tsw1.ana tsw1-8.ana 2000 900 1200
echo pvoc synth tsw1-8.ana tsw1-8
pvoc synth tsw1-8.ana tsw1-8
echo rm tsw1-8.ana
rm tsw1-8.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-8.aiff
echo 

echo 9. MAKE A .TRN BY INVERTING A .FRQ, THEN TRANSPOSE A SOUND WITH IT
echo    here we invert the pitch trace of the original sound
echo repitch invert 2 tsw1.frq tsw1-9inv.trn 0
repitch invert 2 tsw1.frq tsw1-9inv.trn 0
echo repitch transpose 4 tsw1.ana tsw1-9inv.trn tsw1-9.ana
repitch transpose 4 tsw1.ana tsw1-9inv.trn tsw1-9.ana
echo pvoc synth tsw1-9.ana tsw1-9
pvoc synth tsw1-9.ana tsw1-9
echo rm tsw1-9.ana
rm tsw1-9.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-9.aiff
echo 

echo 10. COMBINATIONS 1: 2 .FRQS FROM DIFFERENT SOUNDS TO MAKE A .TRN, THEN  
echo  TRANSPOSE THE 1ST SOUND WITH IT \(without preserving formants\)
echo repitch combine 1 tsw1.frq tsw1-2nd.frq tsw1-10.trn
repitch combine 1 tsw1.frq tsw1-2nd.frq tsw1-10.trn
echo repitch transpose 4 tsw1.ana tsw1-10.trn tsw1-10.ana
repitch transpose 4 tsw1.ana tsw1-10.trn tsw1-10.ana
echo pvoc synth tsw1-10.ana tsw1-10
pvoc synth tsw1-10.ana tsw1-10
echo rm tsw1-10.ana
rm tsw1-10.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-10.aiff
echo 

echo 11. COMBINATIONS 2: RE-DO INVERTED .FRQ, BUT SAVE AS .FRQ \(INSTEAD 
echo  OF .TRN\) SO CAN APPLY VIBRATO TO IT
echo repitch invert 1 tsw1.frq tsw1-11inv.frq 0
repitch invert 1 tsw1.frq tsw1-11inv.frq 0
echo repitch vibrato 1 tsw1-11inv.frq tsw1-11iv.frq 5 2
repitch vibrato 1 tsw1-11inv.frq tsw1-11iv.frq 5 2
echo combine the 2nd sound .frq with the vibrato\'d inversion of the 1st:
echo repitch combine 1  tsw1-2nd.frq tsw1-11iv.frq tsw1-11iv.trn
repitch combine 1  tsw1-2nd.frq tsw1-11iv.frq tsw1-11iv.trn
echo 

echo     APPLY TO THE 1ST SOUND
echo repitch transpose 4 tsw1.ana tsw1-11iv.trn tsw1-11iva.ana
repitch transpose 4 tsw1.ana tsw1-11iv.trn tsw1-11iva.ana
echo pvoc synth tsw1-11iva.ana tsw1-11iva
pvoc synth tsw1-11iva.ana tsw1-11iva
echo rm tsw1-11iva.ana
rm tsw1-11iva.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-11iva.aiff
echo 

echo     APPLY SAME TO THE 2ND SOUND
echo repitch transpose 4 tsw1-2nd.ana tsw1-11iv.trn tsw1-11ivb.ana
repitch transpose 4 tsw1-2nd.ana tsw1-11iv.trn tsw1-11ivb.ana
echo pvoc synth tsw1-11ivb.ana tsw1-11ivb
pvoc synth tsw1-11ivb.ana tsw1-11ivb
echo rm tsw1-11ivb.ana
rm tsw1-11ivb.ana
read -n1 -sp "press any key..."
echo
paplay tsw1-11ivb.aiff
echo 

echo to delete these files and use a different input, run tsw1dels.bat
echo then edit the COPYSFX line\(s\) in tsw1.bat to create new generic infile\(s\)
read -n1 -sp "press any key..."
echo
echo 

