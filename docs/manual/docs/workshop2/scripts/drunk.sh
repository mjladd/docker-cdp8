#!/bin/bash

echo 

echo drunk.bat - batch file to create 7 different results with EXTEND DRUNK
echo 

echo INFILE:  COUNT.aiff  44100, MONO, 8.066 sec
echo change input soundfile to a generic name
echo copysfx count infile
copysfx count infile
read -n1 -sp "press any key..."
echo
paplay infile.aiff
echo 

echo 

echo EXAMPLE 1 - MOVE NARROW AMBITUS FORWARD THROUGH INFILE
echo            INFILE OUTFILE  LENGTH LOCUS       AMBITUS STEP CLOCK
echo extend drunk 1 infile getdrnk1 25     locus1.brk  .5      .1   .2
extend drunk 1 infile getdrnk1 25     locus1.brk  .5      .1   .2
read -n1 -sp "press any key..."
echo
paplay getdrnk1.aiff
echo 

echo 

echo EXAMPLE 2 - WIDEN AMBITUS AND STEP, AND SPEED UP EVENTS
echo extend drunk 1 infile getdrnk2 25 locus1.brk ambitus1.brk step1.brk clock1.brk
extend drunk 1 infile getdrnk2 25 locus1.brk ambitus1.brk step1.brk clock1.brk
read -n1 -sp "press any key..."
echo
paplay getdrnk2.aiff
echo 

echo 

echo EXAMPLE 3 - INCREASED SCATTER: STEP GETS LARGER INSIDE A CONSTANT 2 x AMBITUS
echo extend drunk 1 infile getdrnk3 25 locus1.brk 1.32 step2.brk clock2.brk
extend drunk 1 infile getdrnk3 25 locus1.brk 1.32 step2.brk clock2.brk
read -n1 -sp "press any key..."
echo
paplay getdrnk3.aiff
echo 

echo 

echo EXAMPLE 4 - HOVER:  TINY STEPS THAT STAY CLOSE TO LOCUS POINT
echo extend drunk 1 infile getdrnk4 25 locus2.brk ambitus3.brk step3.brk clock3.brk
extend drunk 1 infile getdrnk4 25 locus2.brk ambitus3.brk step3.brk clock3.brk
read -n1 -sp "press any key..."
echo
paplay getdrnk4.aiff
echo 

echo 

echo EXAMPLE 5 - EXPAND OUTWARDS FROM CENTRAL POSITION IN SOUNDFILE
echo extend drunk 1 infile getdrnk5 25 4 ambitus4.brk step4.brk clock4.brk
extend drunk 1 infile getdrnk5 25 4 ambitus4.brk step4.brk clock4.brk
read -n1 -sp "press any key..."
echo
paplay getdrnk5.aiff
echo 

echo 

echo EXAMPLE 6 - CONTRACT FROM OUTER EDGES TO CENTRE OF SOUNDFILE
echo extend drunk 1 infile getdrnk6 25 4 ambitus5.brk step5.brk clock5.brk
extend drunk 1 infile getdrnk6 25 4 ambitus5.brk step5.brk clock5.brk
read -n1 -sp "press any key..."
echo
paplay getdrnk6.aiff
echo 

echo 

echo EXAMPLE 7 - SWING TIGHT BAND BACK AND FORTH IN THE SOUNDFILE
echo extend drunk 1 infile getdrnk7 25 locus3.brk 0.25 0.1 0.08
extend drunk 1 infile getdrnk7 25 locus3.brk 0.25 0.1 0.08
read -n1 -sp "press any key..."
echo
paplay getdrnk7.aiff
echo 

echo 

echo - NOW AUGMENT THIS EFFECT BY USING PAN
echo modify space 1 getdrnk7 getdrnk7pan panex7.brk
modify space 1 getdrnk7 getdrnk7pan panex7.brk
read -n1 -sp "press any key..."
echo
paplay getdrnk7pan.aiff
echo 

echo 

echo to delete these files and use a different input, run drnkdels.bat
echo then edit the COPYSFX line in drunk.bat to create new generic infile
read -n1 -sp "press any key..."
echo
echo 


