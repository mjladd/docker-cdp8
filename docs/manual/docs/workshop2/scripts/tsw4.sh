#!/bin/bash

echo tsw4.sh - DO NOT MAINTAIN HARMONICITY \& DO NOT PRESERVE FORMANTS
echo deletions: tsw4dels.sh
echo  exploring STRANGE SHIFT

echo INPUTS	
echo	vocalsnd.aiff    4.934    sec 44100 mono
echo	drumsnd.aiff	4.95	 sec 44100 mono
echo	hornsnd.aiff     5.386236 sec 44100 mono
echo	bellsnd.aiff     4.899932 sec 44100 mono	
echo BREAKPOINT FILES
echo	frqshift.brk
echo	frqdiv.brk
echo	frqlow.brk
echo	frqhigh.brk
echo TEXT FILES
echo	tsw4bits.txt

echo 

echo PRE-PROCESSING FOR PITCH TRANSP - COPY SOURCE TO GENERIC 
echo  AND ANALYSE
echo copysfx vocalsnd tsw4pt
copysfx vocalsnd tsw4pt
echo pvoc anal 1 tsw4pt tsw4pt.ana
pvoc anal 1 tsw4pt tsw4pt.ana
echo 

echo PRE-PROCESSING FOR HILITE BAND - COPY SOURCE TO GENERIC 
echo  AND ANALYSE
echo copysfx vocalsnd tsw4hb
copysfx vocalsnd tsw4hb
echo pvoc anal 1 tsw4hb tsw4hb.ana
pvoc anal 1 tsw4hb tsw4hb.ana
read -n1 -sp "press any key..."
echo
echo 

echo DISPLAY ENERGY FOCUS OF ORIGINAL SOUND
echo specinfo peak tsw4hb.ana tsw4hbpk.txt
specinfo peak tsw4hb.ana tsw4hbpk.txt
echo cat tsw4hbpk.txt
cat tsw4hbpk.txt
read -n1 -sp "press any key..."
echo
echo 

echo PRE-PROCESSING FOR STRANGE SHIFT - COPY SOURCE TO GENERIC 
echo  AND ANALYSE
echo copysfx drumsnd tsw4ss
copysfx drumsnd tsw4ss
echo pvoc anal 1 tsw4ss  tsw4ss.ana
pvoc anal 1 tsw4ss  tsw4ss.ana
read -n1 -sp "press any key..."
echo
echo DISPLAY ENERGY FOCUS OF ORIGINAL SOUND
echo specinfo peak tsw4ss.ana tsw4sspeak.txt
specinfo peak tsw4ss.ana tsw4sspeak.txt
echo cat tsw4sspeak.txt
cat tsw4sspeak.txt
read -n1 -sp "press any key..."
echo
echo PLAY SOURCE SOUNDFILE FOR AURAL REFERENCE
paplay tsw4ss.aiff
echo 

echo 

echo RUN THE 6 MODES OF PITCH TRANSP - shift \(part of\) the spectrum
echo PITCH TRANSP: MODE 1 - 8VE UP, ABOVE FRQ_SPLIT
echo pitch transp 1 tsw4pt.ana tsw4ptm1.ana 1000
pitch transp 1 tsw4pt.ana tsw4ptm1.ana 1000
echo pvoc synth tsw4ptm1.ana tsw4ptm1
pvoc synth tsw4ptm1.ana tsw4ptm1
echo rm tsw4ptm1.ana
rm tsw4ptm1.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ptm1.aiff
echo 

echo PITCH TRANSP:  MODE 2 - 8VE DOWN, BELOW FRQ_SPLIT
echo pitch transp 2 tsw4pt.ana tsw4ptm2.ana 1000
pitch transp 2 tsw4pt.ana tsw4ptm2.ana 1000
echo pvoc synth tsw4ptm2.ana tsw4ptm2
pvoc synth tsw4ptm2.ana tsw4ptm2
echo rm tsw4ptm2.ana
rm tsw4ptm2.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ptm2.aiff
echo 

echo PITCH TRANSP: MODE 3 - 8VE ABOVE AND BELOW FRQ_SPLIT
echo pitch transp 3 tsw4pt.ana tsw4ptm3.ana 1000
pitch transp 3 tsw4pt.ana tsw4ptm3.ana 1000
echo pvoc synth tsw4ptm3.ana tsw4ptm3
pvoc synth tsw4ptm3.ana tsw4ptm3
echo rm tsw4ptm3.ana
rm tsw4ptm3.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ptm3.aiff
echo 

echo PITCH TRANSP: MODE 4 - SHIFT UP, ABOVE FRQ_SHIFT 
echo pitch transp 4 tsw4pt.ana tsw4ptm4.ana 1000  20
pitch transp 4 tsw4pt.ana tsw4ptm4.ana 1000  20
echo pvoc synth tsw4ptm4.ana tsw4ptm4
pvoc synth tsw4ptm4.ana tsw4ptm4
echo rm tsw4ptm4.ana
rm tsw4ptm4.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ptm4.aiff
echo 

echo PITCH TRANSP, MODE 5 - SHIFT DOWN, BELOW FRQ_SPLIT
echo pitch transp 5 tsw4pt.ana tsw4ptm5.ana 1000  20
pitch transp 5 tsw4pt.ana tsw4ptm5.ana 1000  20
echo pvoc synth tsw4ptm5.ana tsw4ptm5
pvoc synth tsw4ptm5.ana tsw4ptm5
echo rm tsw4ptm5.ana
rm tsw4ptm5.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ptm5.aiff
echo 

echo PITCH TRANSP, MODE 6 - SHIFT UP AND DOWN FROM FRQ_SPLIT
echo pitch transp 6 tsw4pt.ana tsw4ptm6.ana 1000  20 10
pitch transp 6 tsw4pt.ana tsw4ptm6.ana 1000  20 10
echo pvoc synth tsw4ptm6.ana tsw4ptm6
pvoc synth tsw4ptm6.ana tsw4ptm6
echo rm tsw4ptm6.ana
rm tsw4ptm6.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ptm6.aiff
echo 

echo 

echo EXAMPLE FOR HILITE BAND WITH DATAFILE
echo cat tsw4bits.txt
cat tsw4bits.txt
read -n1 -sp "press any key..."
echo
echo hilite band tsw4hb.ana tsw4hb1.ana tsw4bits.txt
hilite band tsw4hb.ana tsw4hb1.ana tsw4bits.txt
echo pvoc synth tsw4hb1.ana tsw4hb1
pvoc synth tsw4hb1.ana tsw4hb1
echo rm tsw4hb1.ana
rm tsw4hb1.ana
read -n1 -sp "press any key..."
echo
paplay tsw4hb1.aiff
echo 

echo 

echo RUN THE 5 MODES OF STRANGE SHIFT
echo Mode 1: SHIFT ALL
echo strange shift 1 tsw4ss.ana tsw4ssm1.ana frqshift.brk
strange shift 1 tsw4ss.ana tsw4ssm1.ana frqshift.brk
echo pvoc synth tsw4ssm1.ana tsw4ssm1 
pvoc synth tsw4ssm1.ana tsw4ssm1 
echo rm tsw4ssm1.ana
rm tsw4ssm1.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ssm1.aiff
echo 

echo Mode 2: SHIFT ABOVE FRQ
echo strange shift 2 tsw4ss.ana tsw4ssm2.ana frqshift.brk frqdiv.brk
strange shift 2 tsw4ss.ana tsw4ssm2.ana frqshift.brk frqdiv.brk
echo pvoc synth tsw4ssm2.ana tsw4ssm2 
pvoc synth tsw4ssm2.ana tsw4ssm2 
echo rm tsw4ssm2.ana
rm tsw4ssm2.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ssm2.aiff
echo 

echo Mode 3: SHIFT BELOW FRQ
echo strange shift 3 tsw4ss.ana tsw4ssm3.ana frqshift.brk frqdiv.brk
strange shift 3 tsw4ss.ana tsw4ssm3.ana frqshift.brk frqdiv.brk
echo pvoc synth tsw4ssm3.ana tsw4ssm3 
pvoc synth tsw4ssm3.ana tsw4ssm3 
echo rm tsw4ssm3.ana
rm tsw4ssm3.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ssm3.aiff
echo 

echo Mode 4: SHIFT BETWEEN FRQS
echo strange shift 4 tsw4ss.ana tsw4ssm4.ana 101 frqlow.brk frqhigh.brk
strange shift 4 tsw4ss.ana tsw4ssm4.ana 101 frqlow.brk frqhigh.brk
echo pvoc synth tsw4ssm4.ana tsw4ssm4 
pvoc synth tsw4ssm4.ana tsw4ssm4 
echo rm tsw4ssm4.ana
rm tsw4ssm4.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ssm4.aiff
echo 

echo Mode 5: SHIFT OUTSIDE FRQS
echo strange shift 5 tsw4ss.ana tsw4ssm5.ana frqshift.brk frqlow.brk frqhigh.brk
strange shift 5 tsw4ss.ana tsw4ssm5.ana frqshift.brk frqlow.brk frqhigh.brk
echo pvoc synth tsw4ssm5.ana tsw4ssm5 
pvoc synth tsw4ssm5.ana tsw4ssm5 
echo rm tsw4ssm5.ana
rm tsw4ssm5.ana
read -n1 -sp "press any key..."
echo
paplay tsw4ssm5.aiff
echo 

echo to delete these files and use a different input, run tsw4dels.bat
echo then edit the COPYSFX line\(s\) in tsw4.bat to create new generic infile\(s\)
read -n1 -sp "press any key..."
echo
echo 


