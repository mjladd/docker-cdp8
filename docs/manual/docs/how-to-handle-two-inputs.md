# How to Handle Two Inputs

*by Dr Archer Endrich*

*When a program can open and process more than one soundfile*

| [3 types of program](#THREETYPES) | [2+ on the Command line](#TWOONCMDLINE) |
|---|---|
| [2+ with *Sound Loom*](#TWOPLUSSL) | [2+ with *Soundshaper*](#TWOPLUSSSH) |

## Three Types of Program {#THREETYPES}

**Three types of program**

There are three types of program that can use or require more than one input file:

- SUBMIX MIX, which takes a list of soundfiles. MIX is covered in Topic 3, so here we illustrate procedures for the other two.
- programs whose process does something to two different files, which could be text, sound or analysis files. Examples are envelope crossovers, morphing, and interleaved analysis windows.
- the TEXTURE Set programs, which can take more than one input soundfile – let us say potentially numerous inputs, as there is in fact no upper limit

## 2+ on the Command Line {#TWOONCMDLINE}

**2+ on the Command line**

The simplest place to start is to show how 2 or more inputs works on the command line because it is very straightforward and clarifies what the GUIs are doing. The input files are listed on the command line one by one (a file of names cannot be used except for SUBMIX MIX) followed by the *outfile*, as seen here for the vocode process:

`formants vocode imposedonto.ana imposedfrom.ana vocodedresult.ana ...`

If using TEXTURE, the *sndfirst sndlast* parameters are adjusted according to the numer of inputs, usually 1 for *sndfirst* and the numer of soundfiles for *sndlast*. For example, if there are 2 input soundfiles, these parameters will be 1 and 2 respectively, if 6, they will be 1 and 6. Note that a reference pitch for each sound needs to be provided on the first line of the note data file.

`texture simple 5 sound1.wav sound2.wav outfile.wav ndf.txt outdur packing scatter tgrid 1` (for sndfirst) `2` (for sndlast) etc.

Input text files are given in their proper location on the command line. A note data file for TEXTURE, for example, will be placed after the *outfile* and before the *outdur* parameters.

A program like TEXTURE, which can take multiple input soundfiles, can also make use of the fact that the *sndfirst* and *sndlast* parameters can be time-varying, meaning that their entry into the resulting texture can be timed. This can be illustrated with a hypothetical example. Suppose you wanted to create a long transition between a soft, washy sound, and a sharply percussive rhythmic sound. You can create *count* number of intermediate soundfiles that move from the wash to the rhythm by running SUBMIX INBETWEEN. Then with TEXTURE and time-varying soundfile entries, you can stretch this out to *outdur* length, possibly adding time-varying density (*packing*), upwards/downwards transposition patterns, tempo controls etc. To illustrate, if you wanted a ca 75 second result using 8 of these intermediate files, you might time their entries like this:

```
sndfirst.txt  sndlast.txt
time sound    time sound
 0.0  1        0.0  3
20.0  2       20.0  4
35.0  3       35.0  5
45.0  4       45.0  6
52.0  5       52.0  7
58.0  6       58.0  8
66.0  7       66.0  8
70.0  8       70.0  8
```

Straightforward and immensely powerful. When we turn to the GUIs, using more than one file is simply a matter of learning the procedure for adding these extra input soundfiles.

## Sound Loom – Two Inputs {#TWOPLUSSL}

**_Sound Loom_ – two inputs:**

- **Rule of thumb**: in *Sound Loom* first move the files to be used as inputs from the WORKSPACE to CHOSEN FILES (you may have to GRAB them to the WORKSPACE from your current directory before doing this). They are then in place when you select a procedure that uses those (types of) files.

- **Hint**: *Sound Loom* is designed as an 'intelligent' interface in the sense that it keeps track of what you are doing and tries to point you in the right direction so that mistakes are avoided. Sometimes this results in a Catch-22 such as when you are providing source soundfiles as inputs to a process. *Only the processes that can be used with the inputs you provide will be activated (highlighted).* Sometimes a process you expect to be active will not be highlighted because the inputs or type of inputs it needs are not present.

  - PVOC ANAL(ysis) requires Mono input. If you present a stereo soundfile for analysis, the PVOC button will remain inactive (not highlighted). In general, if a process that you expect to be active is inactive, double-check the Reference Documentation regarding the type(s) of input(s) it requires.
  - Similarly, all the Spectral Domain programs require MONO inputs, and PAN because its whole purpose is to create a defined spatial movement. The DISTORT programs require MONO inputs. You may find TEXTURE not to be active when you supply two soundfiles although it is able to use several soundfiles at once. The reason is likely to be that you are supplying both MONO and STEREO soundfiles – it *can* handle MONO ***or*** STEREO input soundfiles, *but not both at the same time*.
  - Note that *Soundshaper* may be more forgiving in this matter because it has some background mechanisms to handle stereo files.
  - I personally like to work in mono until I really want something specific to happen in stereo. Other composers make an effort to do as much as possible in stereo.
  - You can convert between MONO and STEREO:
    SL: `CHANNELS->extract/convert channels`
    SSh: `Edit/Mix->Channels`.
  - Cmdline: `housekeep chans`, modes 4 and 5

- **Envelope Example**: the ENVELOPE IMPOSE, ENVELOPE EXTRACT combination is used to transfer the amplitude envelope from one soundfile to another soundfile. EXTRACT is used to decipher the amplitude envelope of a sound and save it to a binary (**.evl**) or text (**.txt**) file. The output envelope file is placed on the WORKSPACE, so you need to clear the CHOSEN FILES panel and select from the WORKSPACE the soundfile you want to reshape with this amplitude envelope AND the envelope file (you don't need to leave CHOSEN FILES to do this). **NB**: You need to transfer these files *in the correct order for the program*. In this case, the soundfile comes first and then the envelope file, so this is the order in which they need to be placed on CHOSEN FILES – see the Reference Documentation if in doubt.

  ![CorrectOrder.jpg](images/CorrectOrder.JPG)

  Then both the ENVELOPE process and within it `impose` will be active. The same is true of other processes that require a supplementary text file. The Process window then opens, where you can Run, Audition and Save As. When the envelope of the [clash sound](../sounds/clashmx.wav) is imposed on the [marimba sound](../sounds/marimba.wav), the result is: [marclashmxenv.wav](../sounds/marclashmxenv.wav).

- **Texture Example**: The sound [clash-thrumgtx.wav](../sounds/clash-thrumgtx.wav) illustrates TEXTURE SIMPLE Mode 5 randomly selecting between two source sounds ([clashmx.wav](../sounds/clashmx.wav) and [thrumg.wav](../sounds/thrumg.wav)). Thus these two source sounds are moved to CHOSEN FILES before selecting TEXTURE. The note data file and any other breakpoint files are created or loaded from the WORKSPACE while in the Process window. Then make sure that the *sndlast* parameter is 2 and the note data file has two (nominal) reference pitches. This example also has randomised transposition between Low-C (48) and High-C (72). The output duration is 30 seconds, *packing* is 1.3 and *scatter* 0.8. `Use whole input` is ticked. (TEXTURE is covered in detail in Topic 5.)

## Soundshaper – Two Inputs {#TWOPLUSSSH}

**_Soundshaper_ – two inputs:**

The menu items `EDIT/MIX->Mix>Mix 2 (no params)` and `Merge Two` both require two input soundfiles. (Both of these call SUBMIX MERGE.) More than one input soundfile may be required or may be optional for several other programs as well, notably TEXTURE. If you select such a process, you need to have the first soundfile on the Grid. (You see it named in the SOURCE box, *but not on the Grid below*. However, it *is* on the Grid as a temporary soundfile in cell 'A0' ready for processing.) Having opened your first soundfile, call up, for example, SUBMIX MERGE. This is what happens next:

- A red panel called `Add Input` appears.

- To select the *second* file (and any additional files, which could be text files as well as soundfiles) either:
  - click its cell on the Grid if it is already there (e.g., 'B0'),
  - or select a file *via* the file selector (using an icon or the `File` drop-down menu),
  - or, if your sound is already in a Grid cell, simply click on the cell to select it,
  - or drag and drop a file from the Windows directory onto one of the available < 0 > grid cells on the far left.
  - NB: In *Soundshaper* many CDP types are converted automatically, especially between sound and spectral analysis files.
  - *When all the files you need are selected*, click on 'OK' in the red panel. You are now taken to the parameter page for that process, where you can set the parameters (if there are any) and RUN the process. The name of the process run appears on the Grid, which links to the temporary soundfile created.

To hear the result, press the Space bar or the Right-pointing green button on the play transport. Any grid cell can be selected and its temporary sound played in this way. SAVE a file you want to keep with the

![disc icon](images/DiscIcon.JPG)

which is a Save As operation.

[**RETURN**](index.md#TOPIC3) to A Learning Manual for CDP, Topic 3

---

Last updated: 18 August 2022
