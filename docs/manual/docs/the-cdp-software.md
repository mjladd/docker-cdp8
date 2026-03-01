# The CDP Software

*by Dr Archer Endrich*

*Introduction to CDP and its Working Environment*

| [**About CDP and its Software**](#ABOUTCDP) | [**CDP Working Environment**](#CDPENVIRON) |
|---|---|
| [**Getting Started**](#GETSTARTED) | [**Hybrid Environments**](#HYBRID) |

## About CDP and its Software for Sound Design {#ABOUTCDP}

'CDP' – the Composers Desktop Project – was begun as an independent organisation in 1986. In the several years previous, a group of graduates living in York and members of staff at the University of York had begun to explore the possibilities of musical composition with computers. This grew out of previous experience in the classical tape studio and knowledge of existing mainframe computer music systems. The founders of CDP decided to focus on sound transformations of sampled sound, i.e., *musique concrete* using computers, following the lead of Pierre Schaeffer and the GRM in Paris (Groupe de Recherches Musicales). They also wanted to create a composing system that was practical and affordable after leaving behind the facilities at the University.

Trevor Wishart's time at IRCAM (l'Institute de Recherche et Coordination Acoustique/Musique) was crucial for his getting started on the spectral domain programs. Here he received help with morphing and vocoding, which he welcomed after spending 2 whole weeks in the classical tape studio at York University creating just the 10-second transition from voices to bees in *Red Bird*. The first complete CDP system was on Atari ST in 1987, eventually moving to PC and MAC. The Reference Documentation for the expanding program set was written by Archer Endrich over many years, with technical help from the programmers. While there have been many contributors to CDP, most of the software has been written by Trevor Wishart, which is why the software is referred to as the CDP-Wishart Libraries. More detailed information about the history of CDP is currently available on the CDP Website: [http://www.composersdesktop.com/history.html](http://www.composersdesktop.com/history.html).

The CDP software does one thing at a time with an input digitised sound. This makes the software relatively straightforward to use: **sound + process = result**. Like the result? keep it; don't like the result, alter parameters and try again, or try a different process. Different sounds can react in unpredictable ways to the same process, so there is a certain degree of unavoidable experimentation. 'Sound sculpting' is achieved *when one result becomes an input to another process*: i.e., chaining the results until the sound held in the composer's imagination begins to emerge. Experienced users can create automated chains of processes to run all at once; there are various ways to do this.

## CDP Working Environment {#CDPENVIRON}

A working environment can be set up in various ways. The following describes a number of key components that can be put together to create a working environment.

- **HTML Reference Documentation** {#REFERENCE} – The Reference Documentation gives the most complete explanation of each process, along with its command line format for all the parameters. While the GUIs provide concise info as you go along, it can sometimes be useful to dig deeper into the meaning and workings of a process by looking at the Reference Documentation. Therefore a Desktop shortcut to the index of this documentation is highly recommended: **...\docs\index.htm** or **...\docs\html\ccdpndex.htm**.

  Note that the Reference Documents are also available in PDF format, courtesy of Robert Fraser. If a particular program or program group is of special interest, it can be worth printing out the PDF version for extended study.

  The *CDP Desk Reference* complements the HTML documentation by listing all processes and their modes in one handy booklet. Printing the Desk Reference pdf files supplied with the software is recommended so that a physical printout will be readily available. I flip through mine all the time to check on the names of processes and remind myself what is available.

  [CDP Primary Reference Materials](cdp-primary-reference-materials.md) lists the primary reference materials for the CDP software. There are many other documents that give a general overview or provide tutorials on specific program areas. The easiest access is to these files is *via* links in the CDP Reference Documentation index.

- **Graphic User Interfaces (GUIs)** {#GUIS} – The CDP software currently offers two GUI options on PC and one on Mac. *Soundshaper* (Version 5.0 used for these Notes) runs only on PC, and *Sound Loom* runs on either PC or Mac. *Soundshaper* ('SSh' by Robert Fraser) is menu-driven in a typically PC manner, while *Sound Loom* ('SL' by Trevor Wishart) is a more bespoke design based on his own working methods. To help get started with the latter, a practical exercise has been written to take the new user through its basic operation in a methodical manner – see *SLguide-Basic.pdf*. This document provides a hands-on exercise for the first topic.

- **(Plain) Text Editor** {#TEXTED} – A key component for a working environment is a plain text editor, such as *Wordpad* or *TextPad* on PC, or *TextEdit* on Mac. There are more sophisticated editors which programmers tend to use. This is needed to write breakpoint and other text file inputs for CDP processes when using the command line. However, text editing facilities are incorporated in the GUIs, which can create and edit text files or open pre-existing files, so using a separate text editor is optional. My own practice is first to use a separate text editor to create the files and then load them into the GUI, where it can be tweaked by editing if necessary. I suppose I like to plan and prepare things before implementing them *via* a GUI.

- **Command line** {#CMDTERM} – There is another optional way to use the CDP software, and that is *via* a command line interpreter ('CLI'). ON PC this is *cmd.exe* which is found in \Windows\System32. On the Mac, this is called the *Terminal*. The Path to the CDP programs needs to be set properly. The command line format of a CDP process is given to the command line interpreter, which then runs the process. The result is auditioned with the CDP program *pvplay.exe*, which can play both sound and analysis files. The command line is often used to test a program to find out whether a problem is in the program itself or in the GUI. By using batch files and command history, the command line approach can be a very rapid way of working. It also opens the way to using the CDP software within an algorithmic music programming environment or other scripting context.

## Getting Started {#GETSTARTED}

A session begins by gathering together some digital sound files, either from a library of sounds or some other source, or from your own field recordings. These need to be in **wav** format, usually 16-bit or 24-bit, at a sample rate of 44.1 or higher. (See the Reference Manual for COPYSFX for more detail on these matters.)

Then it is a question of

1. **accessing a soundfile on your hard disk,**
2. **loading it ('opening' it),**
3. **performing a process, and then**
4. **auditioning the result, and**
5. **saving the soundfile if you want to keep it, or repeating steps 3 & 4 with altered parameter values**

This five-part sequence is repeated over and over. The aural imagination needs to be activated: to dream up sounds to make and to assess results. It is also a surprisingly physical activity, because listening is involved all the time. The results of a process can be sometimes unpredictable and sometimes amazing, so sound design with computer can be in turn frustrating, surprising, delightful and great fun.

## Hybrid environments {#HYBRID}

Happily, a great deal of software for music is now available, hardly a generation after computer technology began to be used for music. (Some composers can remember cutting up and splicing magnetic tape in the 60's and 70's.) Certain digital audio workstations (DAWs) have become standard, with sequencing, mixing and many audio effects being realised in software. The Pacarana from Symbolic Sound Kyma is described as a supercomputer for sound. There are also 'synthesis engines' such as *Csound* and Max/MSP which create sounds from numbers, providing enormous flexibility for those who master the learning curve. Real-time functionality is more and more important, enabling live improvisation. The CDP software which works outside real-time manipulates sound samples in hundreds of ways, often with semi-algorithmic functions, offering powerful tools to adjust and build sounds. Most creative musicians now work in a hybrid environment, drawing upon different tools as needed. Perhaps, given the return of vinyl, new forms of analog technology are in the offing!

---

Last updated: 01 August 2021
