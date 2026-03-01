# Set up Commandlines on MAC and PC

*by Dr Archer Endrich*

*direct access, batching, scripting*

| | |
|---|---|
| [**Advantages of a commandline environment**](#CMDLADVANTAGES) | |
| [**MAC Setup**](#CMDLMAC) | [**PC Setup**](#CMDLPC) |

## Some Positives for a Commandline Environment {#CMDLADVANTAGES}

CDP is rooted in a Unix-style command line environment. While it has GUI options, these are in fact 'front ends' to an underlying command line system. This system can also be accessed and used directly. There are times when this is useful, even when the majority of sound transformation work is carried out *via* a GUI.

These are some of the advantages of using a command line environment:

- You are able to check whether an apparent fault in a program is due to the program itself or the GUI.

- When the command line is reasonably short, it can be a fast and direct way of working. Using command history up and down arrows, you can return to previous command lines to audition (with `pvplay soundfilename.wav` or `pvplay analysisfilename.ana`), delete the output sound/analysisfile, alter a variable and run the process again, all in a matter of seconds.

- When the command line is on the long side, it can be placed in a batch file, perhaps with a reminder of the usage, and run in this way. This implies going back and forth between a text editor and a CLI.

- Multi-process batch files can be created. These can contain various soundfiles to be used as inputs, all copied to `infile.wav`, with the ones not being used commented out. Generic soundfile names for the interim processes can be used, deleted within the batch file after processing, and the final output soundfile renamed afterwards if you want to keep it. (Note that *Soundshaper* and *SoundLoom* have batching mechanisms, and *Soundshaper* also has an option to run a command line batch file.).

- The use of batch files enables you to build up a personalised library of the processes or process sequences that have been especially effective. Keeping batch files is a very practical way to keep a record of your work.

- Those with programming skills can also create customised scripts that create command lines, run CDP processes and play soundfiles by using system calls. This makes the whole CDP System available for a variety of inventive scripting applications.

## Running CDP from the Command line (Terminal) on MAC {#CMDLMAC}

1. In normal circumstances, this will be automatically set up by the CDP MAC .mkpg. The user needs to place the MAC's command line interpreter, the 'Terminal' on the task bar and use this to run the CDP programs via command line access. (These are not different versions: there is only one software set. The *Sound Loom* and *Soundshaper* GUIs assemble and run command lines behind the scenes, which is why they are referred to as 'front ends'.) The following summarises information in Richard Dobson's *Manualconfig.pdf*

2. You have to be in your HOME directory to do the following. If necessary **cd ~** will take you there – the tilda character ('~') is the standard shorthand for your home directory. To test whether CDP command line access is in place, first check your environment by entering **env** in the Terminal. This will list all the current environment variables, in particular HOME and PATH. If PATH includes a path to the CDP programs, the following should work. Type the name of a CDP program in the Terminal, e.g., `modify speed`. If the program opens and displays a usage message, command line access to CDP is already in place.

3. If this doesn't work, a path to the CDP programs has to be set up in the (hidden) file *.bash_profile*. This file may not exist yet. See if it does by typing **ls -la** in the Terminal: it should be listed.
   - If it is, it can be opened and edited in TextEdit to add the PATH to the CDP programs. Open it with `open -a TextEdit .bash_profile'`.
   - If it is not already present, open TextEdit with `open -a TextEdit`. and save the file as *.bash_profile*. (The initial dot in the filename is important. This is what marks the file as hidden. It is the **-a** in the Terminal command (show all) that enables it to be listed.) NB: If checked, uncheck the option to add a **.txt** extension if none is provided. TextEdit will warn that you are saving a file which will be hidden; just ignore this warning and save.

4. In both cases, assuming that CDP has been set up in your User directory as the folder 'cdpr7', you add these lines in TextEdit to *.bash_profile* file:

```
PATH=$HOME/cdpr7/_cdp/_cdprogs:$PATH
export PATH
```

5. To make the new path active, close the Terminal session and start a new one, as that is when config files such as *.bash_profile* are read. You should now be good to go.

6. The zsh shell became the default shell with the release of OS X Catalina (10.15). The only relevant difference from the bash shell is that instead of using .bash_profile for user configurations it uses the file .zshrc. Follow the steps above, but (if necessary) create the text file .zshrc in the same way, and enter or append the PATH commands as shown.

7. Please note that supplementary breakpoint and other text files used by the CDP programs need to be in a plain text format. Different types of files have different standard extensions, such as '.brk' for breakpoint files, '.txt' for filter and tuning files (etc.), '.mix' for mixfiles. A full list of these extensions is given at the end of the CDP Desk Reference. A full list of all the different supplementary files and their formats (contents) is given in *CDP Files & Codes* (*filestxt.htm* in /docs/html). This is a crucially important reference document. If possible, enable the display of file extensions to help distinguish between files which have the same name except for the extension, e.g., 'sound.wav' and 'sound.ana' and 'sound.evl' etc.

## Running CDP from the Command line on PC {#CMDLPC}

1. Find the command line interpreter program **cmd.exe** and place a shortcut to it on the Desktop for easy access. (It is on the top level of `C:\Windows\System32\`. When you double-click on it, it will open where it is: `C:\Windows\System32`. You need to 'cd' (change directory) to where your files are/will be, i.e., your current working directory.

2. first move back up to C: (using the opened cmd.exe) with: `cd ..\..` (the '..' means move up a level in the directory listing)

3. then 'cd' to where you want to be. For example, I have this working directory: `C:\p3l\lpae`. Having gone back up to the top level of C:, the Command Line interpreter looks like this: `C:\>` and I would then enter `cd p3l\lpae` and I will now be in the directory 'lpae'. The command `dir` will list all the files in that directory. The command `dirsf *.wav` will list all the soundfiles – after step 4) is successfully completed.

4. The PC needs a system path to the CDP programs, so you need to add this to the Path. Go to the Control Panel and click on 'System', then on 'Advanced system settings', then 'Environment variables'. In the top panel you need to add a NEW environment variable to tell the system about CDP wav soundfiles: Click on NEW and then enter for the variable: `CDP_SOUND_EXT` and for the value: `wav`. Click on OK to confirm these settings.

5. In the lower panel you need to set a path to the CDP executable programs so it knows where to find them from the Command line interpreter. (Scroll down to and) Click on 'Path' and then EDIT. Go to the end of the path (right hand end). If there is no semicolon at the end, add one. This divides the different path settings. Then add the full path to the CDP programs. On my computer it is `C:\CDPR7\_cdp\_cdprogs` – you might want to put a semicolon at the end in preparation for adding another program.

6. Click OK to confirm the new settings. You will then need to reboot (a soft reboot should be OK) for the new settings to 'take'.

Now if you double-click on your desktop **cmd.exe** you can try out changing directory and listing directories. If you `cd` to one of your working directories with soundfiles, `dirsf *.wav` should list them all; `dirsf *.*` will list both sound and analysis files; `dirsf *.ana` will list only analysis files. Text files (in fact, all files in the directory) are listed with the system `dir` command.

Now the crucial test: enter the name of a CDP program – NB: as named in the CDP HTML Reference Documentation – and see if the program Usage comes up (tells you how to use the program). The Reference Documentation is based on command line use so it should give you all the info you need as well, and often has an example command line.

The 'History' function of **cmd.exe** is highly useful: 'uparrow' or 'downarrow' to go to a previous command. This makes it easy to rerun a process with altered parameter values (or input file). You will need either to delete the previous sound first or enter a new output file name, because CDP does not allow a soundfile to be overwritten (though it can be made to do so). Delete with `del soundfilename.wav` (or .ana if an analysis file) – the extension is needed.

Both sound files *and analysis files* can be played on the command line with the CDP play program **pvplay**. The sound or analysis file extension is needed so that **pvplay** knows which type of file it is. The `-i` flag enables immediate playback.

---

Last updated: 25 August 2022
