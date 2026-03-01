# A Learning Manual for CDP
## ~ Contents ~

*by Dr Archer Endrich*

[Quick Overview](data/A%20Learning%20Manual%20for%20CDP-CONTENTS.rtf)

SOUND DESIGN: From this: [source sound](../sounds/count.wav) to this: [vocoding: spectral envelope mixed with that of a tractor](../sounds/trcdtvoccnt.wav) or to this: [sound tuned to an A-minor chord](../sounds/counttuneAminbl70.wav)

I have written this *Learning Manual for CDP* as a way to summarise what I have found to be common and important issues when running the CDP software. There are many sound examples to help illustrate the text. I hope it will also help others to handle the software more quickly and easily, and to become more musically productive. Initial drafts of some of this material were written for my students at Plymouth University and benefited from their feedback. These notes have been extensively revised and expanded, and are set out here as separate documents.

This document outlines the Contents of the *Learning Manual* and links to all the supplementary files, presenting information in relatively bite-size chunks. I have also provided a more concise [rtf](data/A%20Learning%20Manual%20for%20CDP-CONTENTS.rtf) version for easy reference.

---

| A Learning Manual for CDP Sound Design |
|---|
| **MAIN TOPICS:** |
| [**TOPIC 1: How to Start Designing Sounds**](#topic-1-how-to-start-designing-sounds-with-cdp) |
| [**TOPIC 2: How to Do Basic Soundfile Editing**](#topic-2-how-to-make-basic-modifications-to-sounds) |
| [**TOPIC 3: How to Mix Sounds**](#topic-3-how-to-mix-sounds-with-cdp) |
| [**TOPIC 4: How to Mingle Sounds**](#topic-4-how-to-mingle-sounds-with-cdp) |
| [**TOPIC 5: How to Assemble Sounds with TEXTURE**](#topic-5-assembling-sounds-with-the-texture-set) |
| **EXTRA INFORMATION** |
| [**On Reverb and Amplitude**](#extra-information) |
| [**Running CDP from the Terminal (Command Line)**](#extra-information) |
| [**Example Batch Files for MAC and PC**](#extra-information) |
| [**Creative Commons Licence**](#creative-commons-licence) |

**NOTES about the Learning Manual**

A left navigation panel is generally avoided, partly because the files are too short to need one, and partly because it can interfere with printing the document.

It is assumed that you will be using one of the GUIs and will call up the process being discussed. Images of screens are therefore kept to a minimum so that the *Learning Manual* files will be concise and easy to print out.

HTML Reference Manual pages for each program give the command line usage, if that is your preferred way of working or are preparing batch files in a text editor.

Throughout this Manual, 'SL' is used for the *Sound Loom* GUI and 'SSh' for the *Soundshaper* GUI. The command line interface is referred to as 'CLI'.

---

## TOPIC 1: How to Start Designing Sounds with CDP

**PRACTICAL WORK** – Worksheet: [Topic1-Worksheet.txt](data/Topic1-Worksheet.txt)

- [Doc1-1: Listening1-Manipulating Sounds](listening1-manipulating-sounds.md) – listening to a few basic changes made to a sound
- [Doc1-2: SLguide-Basic.pdf](data/SLguide-Basic.pdf) – handling the 3 main panels of *Sound Loom* with a practical exercise in the Time and Spectral domains
- [Doc1-3: Basic Soundfile Editing](basic-soundfile-editing.md) – practical exercises to carry out the basic soundfile editing operations; with [the accompanying worksheet](data/Topic1-Worksheet.txt); uses MAXSAMP (get maximum amplitude), GAIN (increase or decrease amplitude), CUT (excise and save), DOVETAIL (smoothing envelope edges), and SFLEN (get length)

**FURTHER STUDY**

- [Doc1-4: About Sound Design](about-sound-design.md) – context, texture and randomisation, time-varying
- [Doc1-5: About Composing with Sounds](about-composing-with-sounds.md) – recognising sources, designing supplementary files, music and 'passage', purpose, time-patterns
- [Doc1-6: The CDP Software](the-cdp-software.md) – about CDP, working environment, main sequence of actions
- [Doc1-7: Suggestions for Composition Projects](suggestions-for-composition-projects.md) – some processes and ideas that may evoke a musical response
- [Doc1-8: CDP Primary Reference Materials](cdp-primary-reference-materials.md) – the primary set of reference materials for using the CDP sound design software (Tutorial materials are listed in a separate file)

---

## Topic 2: How to Make Basic Modifications to Sounds

**PRACTICAL WORK** Worksheet: [Topic2-Worksheet.txt](data/Topic2-Worksheet.txt)

- [Doc2-1: Listening2-Surfaces](listening2-surfaces.md) – processes that churn up and roughen a sound
- [Doc2-2: Basic Modifications and Transformations](basic-modifications-and-transformations.md) – frequent but modest changes to a sound: transpose, glissando, filter, reverb/echo, pan, loop, reverse, trace, ring modulate, time stretch, vibrato/tremolo
- [Doc2-3: Surface Texturing](surface-texturing.md) – texturing and churning up a sound: texturing, random chunks, segmentations, wave-cycle distortion
- [Doc2-4: Suppleness via Time-varying Parameters](suppleness-via-time-varying-parameters.md) – some detailed examples that show how to create and use breakpoint files

**FURTHER STUDY**

- [Doc2-5: Types of Sound](types-of-sound.md) – broad categories and creative use, recording, sound libraries
- [Doc2-6: Sonic Objectives](sonic-objectives.md) – ways to approach the formulation of composition goals
- [Doc2-7: CDP Tutorial Materials](cdp-tutorial-materials.md) – a list of most of what is currently available

---

## Topic 3: How to Mix Sounds with CDP

**PRACTICAL WORK** (and listening) are built into Docs 2, 3 and 4

- [Doc3-1: Mix Concepts and Planning](mix-concepts-and-planning.md) – 'vertical' vs. 'horizontal' mixes, preparing a mix
- [Doc3-2: Create a Mix in Sound Loom](create-a-mix-in-sound-loom.md) – Concise Procedure with Exercise
- [Doc3-3: Create a Mix in Soundshaper](create-a-mix-in-soundshaper.md) – Concise Procedure with Exercise
- [Doc3-4: A (Relatively) Complex Mix](a-complex-mix.md) – a mix with several soundfiles, one of which was created by a prior mix

**FURTHER STUDY**

- [Doc3-5: How to Handle Two Inputs](how-to-handle-two-inputs.md) – One of the tricky issues as handled in either GUI or the command line
- [Doc3-6: How to Save and Backup Soundfiles](how-to-save-and-backup-soundfiles.md) – on the Command Line and navigating the GUIs to retain your work; keeping logs

---

## Topic 4: How to Mingle Sounds with CDP

**PRACTICAL WORK** Worksheet: [Topic4-Worksheet.txt](data/Topic4-Worksheet.txt)

- [Doc4-1: Listening4-Mods and Minglings](listening4-mods-and-minglings.md) – four sets of transformations of vocal material
- [Doc4-2: Mingling Involving Envelopes](mingling-involving-envelopes.md) – one of the longer articles, it's all about envelopes, with an emphasis on using the envelopes of two different sounds
- [Doc4-3: Mingling Involving Analysis Windows](mingling-involving-analysis-windows.md) – interleaving sounds, selecting max amp windows, convolution, shuffling and weaving windows
- [Doc4-4: Mingling via Sonic Morphs](mingling-via-sonic-morphs.md) – discussion and example of sonic morphing

**FURTHER STUDY**

- To Do: Select and carry out some operations on sound, and mix the results into a short passage of music (ca 30 sec.)
- [Doc4-5: Texture Set - Key Components](texture-set-key-components.md) – In preparation for Topic 5, this seeks to outline the key features of the Texture Set and to get a feel for the different composition contexts for which it might be used

---

## Topic 5: Assembling Sounds with the TEXTURE Set

**PRACTICAL WORK** Worksheet: [Topic5-Worksheet.txt](data/Topic5-Worksheet.txt)

- [Doc5-1: Listening5-Textures](listening5-textures.md) – a selection of musical textures created in various ways
- [Doc5-2: How to Create Strong Tones](how-to-create-strong-tones.md) – using the Texture Set to create rich, full tones; a texture of tones; how to go from single tones to a melody melodic sequence
- [Doc5-3: How to Tune Sounds](how-to-tune-sounds.md) – both tuned textures and the internal tuning of sounds, whether with filters or with partials
- [Doc5-4: How to Create Rhythms and Handle Durations](how-to-create-rhythms-and-handle-durations.md) – expressing rhythms in text form, turning rhythms into breakpoint format (starting at time 0 – also see ALM Durations as Numbers.pdf), rhythmic effects, rhythm templates in the Texture Set
- [Doc5-5: How to Create Multi-event Textures](how-to-create-multi-event-textures.md) – some observations about texture in 20th c. composition are followed by several examples of textures made with the Texture Set of programs; these are of increasing complexity and include a multi-sound example. A reminder of the Texture parameters is included in this file.

**FURTHER STUDY**

- [Doc5-6: Getting a Grip on Packing and Skiptime](getting-a-grip-on-packing-and-skiptime.md) – a detailed study of how these parameters work, with sound examples complete with musical notation so that you can easily follow what happens
- [Doc5-7: ALM Durations as Numbers.pdf](data/ALM%20Durations%20as%20Numbers.pdf) – How to create durations and rhythmic sequences via numerical data, with an example fully written out in numbers and musical notation
- To Do: Complete a short original composition with the CDP software, ca 1 min
- To Do: Document your work on the above composition:
  - source of inspiration
  - the general idea for the piece
  - observations about its form
  - sound sources used
  - sound transformation processes used
  - how assembled

---

## Extra Information

- [Doc6-1: Reverb & Amplitude](reverb-and-amplitude.md) – opportunities for reverb and delay; handling amplitude
- [Doc6-2: Set up Commandlines on MAC and PC](set-up-commandlines-on-mac-and-pc.md) – general discussion of command lines and how to set up using them on MAC and PC
- **Example Batch Files** (in the sounds directory) [Doc6-3(MAC): MAC-batchfile-example.txt](../sounds/MAC-batchfile-example.txt) (change the .txt extension to .sh to run) and [Doc6-3(PC): PC-batchfile-example.txt](../sounds/PC-batchfile-example.txt) (change the .txt extension to .bat to run). These illustrate:
  - placing source sounds in a working directory
  - providing a command line usage reminder if using a function with lots of parameters
  - time domain processing
  - conversion of soundfile to analysis file
  - spectral processing
  - synthesis of analysis file to soundfile
  - playing sound output
  - handling file deletions prior to re-running the batch file
  - saving the final output by renaming it

---

## Creative Commons Licence

*A Learning Manual for CDP*, all documents and sounds, is distributed under the *Creative Commons License: Attribution-NonCommercial-ShareAlike 4.0 International (CC BY-NC-SA 4.0)*. You are free to **Share** (copy and redistribute the material in any medium or format) and **Adapt** (remix, transform, and build upon the material), giving appropriate credit and providing a link to the licence etc. The moral right of the author has been asserted.

Last updated: 25 August 2022
