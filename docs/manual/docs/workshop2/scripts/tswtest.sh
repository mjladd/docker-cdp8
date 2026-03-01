#!/bin/bash

echo - tswtest.sh - single operation with which to test setup
echo - deletions: tswtdel.sh

echo SOURCES
echo	vocalsnd.aiff    4.934    sec 44100 mono \(balsam \&/or sheila\)
echo	drumsnd.aiff	4.95	 sec 44100 mono \(drunm riff\)
echo	hornsnd.aiff     5.386236 sec 44100 mono \(horn tone\)
echo	bellsnd.aiff     4.899932 sec 44100 mono	\(Tibetan bowl\)
REM BREAKPOINT FILE
echo	tswtest.brk: 0.0 -12, 1.0 -9, 2.5 -14, 3.5 -12, 3.51 12, 4.9 -12 

echo 

echo PRE-PROCESSING
echo copysfx vocalsnd tswtest
copysfx vocalsnd tswtest
echo 

echo MODIFY SPEED
echo modify speed 2 tswtest tswd12 -12
modify speed 2 tswtest tswd12 -12
read -n1 -sp "press any key..."
echo
paplay tswd12.aiff
echo 

echo run tswtdels.bat to delete soundfiles produced
read -n1 -sp "press any key..."
echo
echo 


