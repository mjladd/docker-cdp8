#!/bin/bash

echo tsw6cp.sh - CROSS-PROCESSING BONANZA
echo deletions: tsw6dels.sh
echo	REPITCH COMBINE and other REPITCH Functions

echo SOURCES
echo	vocalsnd.aiff    4.934    sec 44100 mono
echo	drumsnd.aiff     4.95	 sec 44100 mono
echo	hornsnd.aiff     5.386236 sec 44100 mono
echo	bellsnd.aiff     4.899932 sec 44100 mono	
echo 

echo tsw6cp1 = bellsndlp   tsw6cp2 = vocalsnd   tsw6cp3 = drumsnd
read -n1 -sp "press any key..."
echo
echo 

echo PRE-PROCESSING
echo 1. COPY 3 SOURCES TO GENERIC FILENAMES
echo copysfx bellsnd tsw6cp1
copysfx bellsnd tsw6cp1
echo copysfx vocalsnd tsw6cp2
copysfx vocalsnd tsw6cp2
echo copysfx drumsnd tsw6cp3
copysfx drumsnd tsw6cp3
echo 

echo 2. ANALYSE
echo pvoc anal 1 tsw6cp1  tsw6cp1.ana
pvoc anal 1 tsw6cp1  tsw6cp1.ana
echo pvoc anal 1 tsw6cp2 tsw6cp2.ana
pvoc anal 1 tsw6cp2 tsw6cp2.ana
echo pvoc anal 1 tsw6cp3  tsw6cp3.ana
pvoc anal 1 tsw6cp3  tsw6cp3.ana
echo 

echo 3. EXTRACT PITCH TRACE \(.FRQ: binary pitch data file\)
echo repitch getpitch 1 tsw6cp1.ana tsw6cp1hearfrq.ana tsw6cp1.frq
repitch getpitch 1 tsw6cp1.ana tsw6cp1hearfrq.ana tsw6cp1.frq
echo repitch getpitch 1 tsw6cp2.ana tsw6cp2hearfrq.ana tsw6cp2.frq
repitch getpitch 1 tsw6cp2.ana tsw6cp2hearfrq.ana tsw6cp2.frq
echo repitch getpitch 1 tsw6cp3.ana tsw6cp3hearfrq.ana tsw6cp3.frq
repitch getpitch 1 tsw6cp3.ana tsw6cp3hearfrq.ana tsw6cp3.frq
echo 

echo 4. SYNTHESISE INTERMEDIATE .ANA TO AUDITION THE PITCH TRACES
echo pvoc synth tsw6cp1hearfrq.ana tsw6cp1hearfrq
pvoc synth tsw6cp1hearfrq.ana tsw6cp1hearfrq
echo BELLSND PITCH TRACE
read -n1 -sp "press any key..."
echo
paplay tsw6cp1hearfrq.aiff
echo 

echo pvoc synth tsw6cp2hearfrq.ana tsw6cp2hearfrq
pvoc synth tsw6cp2hearfrq.ana tsw6cp2hearfrq
echo VOCALSND PITCH TRACE
read -n1 -sp "press any key..."
echo
paplay tsw6cp2hearfrq.aiff
echo 

echo pvoc synth tsw6cp3hearfrq.ana tsw6cp3hearfrq
pvoc synth tsw6cp3hearfrq.ana tsw6cp3hearfrq
echo DRUMSND PITCH TRACE
read -n1 -sp "press any key..."
echo
paplay tsw6cp3hearfrq.aiff
echo 

echo rm tsw6cp1hearfrq.ana
rm tsw6cp1hearfrq.ana
echo rm tsw6cp2hearfrq.ana
rm tsw6cp2hearfrq.ana
echo rm tsw6cp3hearfrq.ana
rm tsw6cp3hearfrq.ana
read -n1 -sp "press any key..."
echo
echo 

echo WORK OUT VARIOUS COMBINATIONS, COMPLETE THE TRANSPOSITION USING 
echo  TRANSPOSE MODE 4, RESYNTHESISE AND AUDITION
echo 

echo COMBINE 2 DIFFERENT .FRQ TO MAKE A .TRN \(NOTE ORDER OF INPUT FILES\)
echo 1to2 \(BELLSND WITH VOCALSND\)
echo repitch combine 1 tsw6cp1.frq tsw6cp2.frq   tsw6cp1to2.trn
repitch combine 1 tsw6cp1.frq tsw6cp2.frq   tsw6cp1to2.trn
echo TRANSPOSE SOURCE 1 WITH THE 1to2 COMBINATION
echo BELLSND WHIZZES UP AND DOWN WITH VOCAL CONTOUR
echo repitch transpose 4 tsw6cp1.ana tsw6cp1to2.trn tsw6cp1to2.ana
repitch transpose 4 tsw6cp1.ana tsw6cp1to2.trn tsw6cp1to2.ana
echo pvoc synth tsw6cp1to2.ana tsw6cp1to2
pvoc synth tsw6cp1to2.ana tsw6cp1to2
read -n1 -sp "press any key..."
echo
paplay tsw6cp1to2.aiff
echo 

echo 2to1 \(VOCALSND WITH BELLSND\)
echo repitch combine 1 tsw6cp2.frq tsw6cp1.frq   tsw6cp2to1.trn
repitch combine 1 tsw6cp2.frq tsw6cp1.frq   tsw6cp2to1.trn
echo TRANSPOSE SOURCE 2 WITH THE 2to1 COMBINATION
echo VOCALSND SEEMS SQUEEZED THROUGH A NARROW \(TIME-VARYING\) HOLE
echo repitch transpose 4 tsw6cp2.ana tsw6cp2to1.trn tsw6cp2to1.ana
repitch transpose 4 tsw6cp2.ana tsw6cp2to1.trn tsw6cp2to1.ana
echo pvoc synth tsw6cp2to1.ana tsw6cp2to1
pvoc synth tsw6cp2to1.ana tsw6cp2to1
read -n1 -sp "press any key..."
echo
paplay tsw6cp2to1.aiff
echo 

echo 3to2 \(DRUMSND WITH VOCALSND\)
echo repitch combine 1 tsw6cp3.frq tsw6cp2.frq   tsw6cp3to2.trn
repitch combine 1 tsw6cp3.frq tsw6cp2.frq   tsw6cp3to2.trn
echo TRANSPOSE SOURCE 3 WITH THE 3to2 COMBINATION
echo DRUMSND SWINGS UP AND DOWN, WITH GLISSANDOS
echo repitch transpose 4 tsw6cp3.ana tsw6cp3to2.trn tsw6cp3to2.ana
repitch transpose 4 tsw6cp3.ana tsw6cp3to2.trn tsw6cp3to2.ana
echo pvoc synth tsw6cp3to2.ana tsw6cp3to2
pvoc synth tsw6cp3to2.ana tsw6cp3to2
read -n1 -sp "press any key..."
echo
paplay tsw6cp3to2.aiff
echo 

echo COMBINE SOURCE 1 .FRQ WITH 1to2 .TRN, MAKING A .FRQ
echo \(BELLSND WITH BELLSNDtoVOCALSND\)
echo repitch combine 2 tsw6cp1.frq tsw6cp1to2.trn tsw6cp1and1to2.frq
repitch combine 2 tsw6cp1.frq tsw6cp1to2.trn tsw6cp1and1to2.frq
echo APPLY VIBRATO TO PREVIOUS OUTPUT, MAKING A .TRN
echo repitch vibrato 2 tsw6cp1and1to2.frq tsw6cp1and1to2.trn 5 3
repitch vibrato 2 tsw6cp1and1to2.frq tsw6cp1and1to2.trn 5 3
echo TRANSPOSE SOURCE 1 WITH THE 1 and 1to2 VIBRATO COMBINATION
echo BELLSND HAS SIGNIFICANT VIBRATO, THEN SETTLES
echo repitch transpose 4 tsw6cp1.ana tsw6cp1and1to2.trn tsw6cp1and1to2.ana
repitch transpose 4 tsw6cp1.ana tsw6cp1and1to2.trn tsw6cp1and1to2.ana
echo pvoc synth tsw6cp1and1to2.ana tsw6cp1and1to2
pvoc synth tsw6cp1and1to2.ana tsw6cp1and1to2
read -n1 -sp "press any key..."
echo
paplay tsw6cp1and1to2.aiff
echo 

echo COMBINE SOURCE 1 .FRQ WITH 2to1 .TRN, MAKING A .FRQ
echo \(BELLSND WITH VOCALSNDtoBELLSND\)
echo repitch combine 2 tsw6cp1.frq tsw6cp2to1.trn tsw6cp1and2to1.frq
repitch combine 2 tsw6cp1.frq tsw6cp2to1.trn tsw6cp1and2to1.frq
echo APPLY EXAG TO PREVIOUS OUTPUT, MAKING A .TRN
echo repitch exag    2 tsw6cp1and2to1.frq tsw6cp1and2to1.trn 67 1.2
repitch exag    2 tsw6cp1and2to1.frq tsw6cp1and2to1.trn 67 1.2
echo TRANSPOSE SOURCE 1 WITH THE 1 and 2to1 EXAG COMBINATION
echo repitch transpose 4 tsw6cp1.ana tsw6cp1and2to1.trn tsw6cp1and2to1.ana
repitch transpose 4 tsw6cp1.ana tsw6cp1and2to1.trn tsw6cp1and2to1.ana
echo pvoc synth tsw6cp1and2to1.ana tsw6cp1and2to1
pvoc synth tsw6cp1and2to1.ana tsw6cp1and2to1
read -n1 -sp "press any key..."
echo
paplay tsw6cp1and2to1.aiff
echo 

echo COMBINE SOURCE 2 WITH 1to2 .TRN, MAKING A .FRQ
echo \(VOCALSND WITH BELLSNDtoVOCALSND\)
echo repitch combine 2  tsw6cp2.frq tsw6cp1to2.trn tsw6cp2and1to2.frq
repitch combine 2  tsw6cp2.frq tsw6cp1to2.trn tsw6cp2and1to2.frq
echo APPLY INVERT TO PREVIOUS OUTPUT, MAKING A .TRN
echo repitch invert  2 tsw6cp2and1to2.frq tsw6cp2and1to2.trn 0
repitch invert  2 tsw6cp2and1to2.frq tsw6cp2and1to2.trn 0
echo TRANSPOSE SOURCE 2 WITH THE 2 and 1to2 INVERT COMBINATION
echo INVERTed \(DESCENDING\) VOCALSND NOW SWINGS UPWARDS
echo repitch transpose 4 tsw6cp2.ana tsw6cp2and1to2.trn tsw6cp2and1to2.ana
repitch transpose 4 tsw6cp2.ana tsw6cp2and1to2.trn tsw6cp2and1to2.ana
echo pvoc synth tsw6cp2and1to2.ana tsw6cp2and1to2
pvoc synth tsw6cp2and1to2.ana tsw6cp2and1to2
read -n1 -sp "press any key..."
echo
paplay tsw6cp2and1to2.aiff
echo 

echo COMBINE SOURCE 2 .FRQ WITH 2to1 .TRN, MAKING A .FRQ
echo \(VOCALSND WITH VOCALSNDtoBELLSND\)
echo repitch combine 2 tsw6cp2.frq tsw6cp2to1.trn tsw6cp2and2to1.frq
repitch combine 2 tsw6cp2.frq tsw6cp2to1.trn tsw6cp2and2to1.frq
echo APPLY QUANTISE TO PREVIOUS OUTPUT, MAKING A .TRN
echo repitch quantise 2 tsw6cp2and2to1.frq tsw6cp2and2to1.trn q_set.txt -o
repitch quantise 2 tsw6cp2and2to1.frq tsw6cp2and2to1.trn q_set.txt -o
echo TRANSPOSE SOURCE 1 WITH THE 2 and 2to1 QUANTISE COMBINATION
echo QUANTISEd TO C-min-7th BELLSND -- LITTLE CHANGE
echo repitch transpose 4 tsw6cp1.ana tsw6cp2and2to1.trn tsw6cp2and2to1.ana
repitch transpose 4 tsw6cp1.ana tsw6cp2and2to1.trn tsw6cp2and2to1.ana
echo pvoc synth tsw6cp2and2to1.ana tsw6cp2and2to1
pvoc synth tsw6cp2and2to1.ana tsw6cp2and2to1
read -n1 -sp "press any key..."
echo
paplay tsw6cp2and2to1.aiff
echo 

echo COMBINE 1to2 .TRN WITH 3to2 .TRN
echo \(BELLSNDtoVOCALSND WITH DRUMSNDtoVOCALSND\)
echo repitch combine 3 tsw6cp1to2.trn tsw6cp3to2.trn tsw6cp1to2and3to2.trn
repitch combine 3 tsw6cp1to2.trn tsw6cp3to2.trn tsw6cp1to2and3to2.trn
echo TRANSPOSE SOURCE 1 WITH THE 1to2 and 3to2 COMBINATIONS
echo WILD SWINGS, ENDING ON LOW TONE
echo repitch transpose 4 tsw6cp1.ana tsw6cp1to2and3to2.trn tsw6cp1to2and3to2.ana
repitch transpose 4 tsw6cp1.ana tsw6cp1to2and3to2.trn tsw6cp1to2and3to2.ana
echo THIS COMBINATION OVERFLOWED -- REDUCING GAIN BEFORE RESYNTHESIS
echo spec gain tsw6cp1to2and3to2.ana tsw6cp1to2and3to2gd.ana 0.85
spec gain tsw6cp1to2and3to2.ana tsw6cp1to2and3to2gd.ana 0.85
echo pvoc synth tsw6cp1to2and3to2gd.ana tsw6cp1to2and3to2gd
pvoc synth tsw6cp1to2and3to2gd.ana tsw6cp1to2and3to2gd
read -n1 -sp "press any key..."
echo
paplay tsw6cp1to2and3to2gd.aiff
echo 


