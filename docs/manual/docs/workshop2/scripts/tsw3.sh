#!/bin/bash

echo tsw3.sh - MAINTAIN HARMONICITY \& PRESERVE FORMANTS
echo deletions: tsw3dels.sh
echo	PITCH OCTMOVE and REPITCH TRANSPOSEF
echo	PITCH TRANSPOSE also called upon to enable direct aural 
echo	 comparison with the same transpositions in which the 
echo	 formants move \(not preserved\).

echo SOURCES
echo	vocalsnd.aiff    4.934    sec 44100 mono
echo	drumsnd.aiff	4.95	 sec 44100 mono
echo	hornsnd.aiff     5.386236 sec 44100 mono
echo	bellsnd.aiff     4.899932 sec 44100 mono	

echo 

echo PRE-PROCESSING:  COPY SOURCE TO GENERIC FILENAME AND ANALYSE
echo copysfx vocalsnd tsw3
copysfx vocalsnd tsw3
echo pvoc anal 1 tsw3 tsw3.ana
pvoc anal 1 tsw3 tsw3.ana
echo 

echo EXTRACT PITCH TRACE \(.frq\) AND AUDITION
echo repitch getpitch 1 tsw3.ana tsw3dummy.ana tsw3.frq
repitch getpitch 1 tsw3.ana tsw3dummy.ana tsw3.frq
echo pvoc synth tsw3dummy.ana tsw3dummy
pvoc synth tsw3dummy.ana tsw3dummy
echo rm tsw3dummy.ana
rm tsw3dummy.ana
read -n1 -sp "press any key..."
echo
paplay tsw3dummy.aiff
echo rm tsw3dummy.aiff
rm tsw3dummy.aiff
echo 

echo EXPLORING PITCH OCTMOVE \(FORMANTS PRESERVED\)
echo 

echo UP 2 OCTAVES
echo pitch octmove 1 tsw3.ana tsw3.frq tsw3om1.ana 2
pitch octmove 1 tsw3.ana tsw3.frq tsw3om1.ana 2
echo pvoc synth tsw3om1.ana tsw3om1
pvoc synth tsw3om1.ana tsw3om1
echo rm tsw3om1.ana
rm tsw3om1.ana
read -n1 -sp "press any key..."
echo
paplay tsw3om1.aiff
echo 

echo UP 3 OCTAVES
echo pitch octmove 1 tsw3.ana tsw3.frq tsw3om2.ana 3
pitch octmove 1 tsw3.ana tsw3.frq tsw3om2.ana 3
echo pvoc synth tsw3om2.ana tsw3om2
pvoc synth tsw3om2.ana tsw3om2
echo rm tsw3om2.ana
rm tsw3om2.ana
read -n1 -sp "press any key..."
echo
paplay tsw3om2.aiff
echo 

echo UP 4 OCTAVES
echo pitch octmove 1 tsw3.ana tsw3.frq tsw3om3.ana 4
pitch octmove 1 tsw3.ana tsw3.frq tsw3om3.ana 4
echo pvoc synth tsw3om3.ana tsw3om3
pvoc synth tsw3om3.ana tsw3om3
echo rm tsw3om3.ana
rm tsw3om3.ana
read -n1 -sp "press any key..."
echo
paplay tsw3om3.aiff
echo 

echo UP 3 OCTAVES PLUS BASS-BOOST
echo pitch octmove 3 tsw3.ana tsw3.frq tsw3om4.ana 3 4
pitch octmove 3 tsw3.ana tsw3.frq tsw3om4.ana 3 4
echo pvoc synth tsw3om4.ana tsw3om4
pvoc synth tsw3om4.ana tsw3om4
echo rm tsw3om4.ana
rm tsw3om4.ana
read -n1 -sp "press any key..."
echo
paplay tsw3om4.aiff
echo 

echo 

echo EXPLORING TRANSPOSEF \(FORMANTS PRESERVED, -f and -p options\)
echo  BEST WITH VOCAL SOUNDS
echo 

echo UP 12
echo repitch transposef 3 tsw3.ana tsw3rtfu12.ana -f12 12
repitch transposef 3 tsw3.ana tsw3rtfu12.ana -f12 12
echo pvoc synth tsw3rtfu12.ana tsw3rtfu12
pvoc synth tsw3rtfu12.ana tsw3rtfu12
echo rm tsw3rtfu12.ana
rm tsw3rtfu12.ana
read -n1 -sp "press any key..."
echo
paplay tsw3rtfu12.aiff
echo 

echo UP 19
echo repitch transposef 3 tsw3.ana tsw3rtfu19.ana -f12 19
repitch transposef 3 tsw3.ana tsw3rtfu19.ana -f12 19
echo pvoc synth tsw3rtfu19.ana tsw3rtfu19
pvoc synth tsw3rtfu19.ana tsw3rtfu19
echo rm tsw3rtfu19.ana
rm tsw3rtfu19.ana
read -n1 -sp "press any key..."
echo
paplay tsw3rtfu19.aiff
echo 

echo UP 24
echo repitch transposef 3 tsw3.ana tsw3rtfu24.ana -f12 24
repitch transposef 3 tsw3.ana tsw3rtfu24.ana -f12 24
echo pvoc synth tsw3rtfu24.ana tsw3rtfu24
pvoc synth tsw3rtfu24.ana tsw3rtfu24
echo rm tsw3rtfu24.ana
rm tsw3rtfu24.ana
read -n1 -sp "press any key..."
echo
paplay tsw3rtfu24.aiff
echo 

echo 

echo TRANSPOSE \(FORMANTS NOT PRESERVED - BETTER WITH NON-VOCAL SOUNDS\)
echo 

echo UP 12
echo repitch transpose 3 tsw3.ana tsw3rtu12.ana 12
repitch transpose 3 tsw3.ana tsw3rtu12.ana 12
echo pvoc synth tsw3rtu12.ana tsw3rtu12
pvoc synth tsw3rtu12.ana tsw3rtu12
echo rm tsw3rtu12.ana
rm tsw3rtu12.ana
read -n1 -sp "press any key..."
echo
paplay tsw3rtu12.aiff
echo 

echo UP 19
echo repitch transpose 3 tsw3.ana tsw3rtu19.ana 19
repitch transpose 3 tsw3.ana tsw3rtu19.ana 19
echo pvoc synth tsw3rtu19.ana tsw3rtu19
pvoc synth tsw3rtu19.ana tsw3rtu19
echo rm tsw3rtu19.ana
rm tsw3rtu19.ana
read -n1 -sp "press any key..."
echo
paplay tsw3rtu19.aiff
echo 

echo UP 24
echo repitch transpose 3 tsw3.ana tsw3rtu24.ana 24
repitch transpose 3 tsw3.ana tsw3rtu24.ana 24
echo pvoc synth tsw3rtu24.ana tsw3rtu24
pvoc synth tsw3rtu24.ana tsw3rtu24
echo rm tsw3rtu24.ana
rm tsw3rtu24.ana
read -n1 -sp "press any key..."
echo
paplay tsw3rtu24.aiff
echo 

echo to delete these files and use a different input, run tsw3dels.bat
echo then edit the COPYSFX line\(s\) in tsw3.bat to create new generic infile\(s\)
read -n1 -sp "press any key..."
echo
echo 



