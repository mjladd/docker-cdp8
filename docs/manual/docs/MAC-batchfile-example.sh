#!/bin/bash

echo MAC-batchfile-example.sh	Example batch file in PC format
echo    Designed to illustrate a chained sequence of processes.
echo    A Endrich - 24 August 2022

echo NB:  change the extension .txt to .sh to run this batch file
echo PLACE SOME SOURCE SOUNDFILES IN THE WORKING DIRECTORY
echo bendy.wav
echo bendymerge3g.wav
echo cracklgdmore.wav
echo chimesc.wav

echo CLEAR THE WAY FOR USING A DIFFERENT SOURCE
rm infile.wav

echo COPY CHOSEN SOURCE TO A GENERIC NAME
echo bendymerge3g.wav was 'developed' from 'bendy.wav' in the 
echo    Topic4-Worksheet (See 7a.) -- try making a batch file 
echo    to do this (but use a different final output filename!)
echo

copysfx bendymerge3g.wav infile.wav
echo

echo TEXTURE USAGE echoINDER (because it's so long)
echo texture simple mode infile outfile ndf outdur packing scatter 
echo    tgrid  snd1st sndlast mingain maxgain  mindur maxdur  
echo    minpch maxpch  -aaten -pposition -sspread -w
echo

echo RUN TEXTURE SIMPLE
texture simple 5 infile.wav outfile.wav ndf76.txt 10 0.8 0.1 0  1 1  70 80  5 8  50 90  -a0.9 -p0.5 -s1 -w
echo

echo INTERIM SOUND CHECK (Comment out if don't want to listen to it)
pvplay outfile.wav
echo

echo FURTHER PROCESSING OF THE TEXTURE OUTFILE	
housekeep chans 4 outfile.wav outfilem.wav
pvoc anal 1 outfilem.wav outfilem.ana
hilite bltr outfilem.ana outfilembltr.ana 100 5
pvoc synth outfilembltr.ana outfilembltr.wav
modify speed 2 outfilembltr.wav outfilembltrd18.wav -18
echo

echo PLAY FINAL RESULT
pvplay outfilembltrd18.wav
echo

echo UNCOMMENT THE NEXT LINE IF WANT TO SAVE FINAL RESULT 
echo    (And probably give it your own name.)
echo ren outfilembltrd18.wav PC-batchfile-example.wav
echo

echo DELETIONS (to prepare for a new run of the batch file)
echo    (May want to make DELETIONS a separate batch file)
echo Manually delete the renamed outfile if don't like it.
echo

rm outfile.wav
rm outfile.ana
rm outfilem.wav
rm outfilem.ana
rm outfilembltr.ana
rm outfilembltr.wav
rm outfilembltrd18.wav
echo
