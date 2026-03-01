# Listening1 - Manipulating Sound

*by Dr Archer Endrich*

This first group of sounds comes from Worksheet 1 of *CDP Tutorial Workshop 1*. The source sound comes first, followed by 5 more sounds which illustrate time-varying pitch transposition in the Time Domain with MODIFY SPEED (up/down/both) and then two examples of pitchwarping with DISTORT PITCHWARP: [clip1-all.wav](../sounds/clip1-all.wav)

1. source sound: "The extraticular ..." [capm.wav]

2. time-varying transposition slides up – breakpoint file:
    ```
    0.0 0  ;no transposition at the start
    7.2 12 ;up an octave (12 semitones) by the time the end is reached
    ```

3. time-varying transposition slides down – breakpoint file:
    ```
    0.0 0   ;no transposition at the start
    7.2 -12 ;down an octave (12 semitones) by the time the end is reached
    ```

4. time-varying transposition slides both up and down – breakpoint file:
    ```
    0.0 0  ;no transposition at the start
    1.4 0  ;no change yet
    1.5 7  ;slide up 7 semitones within one-tenth of a second
    2.9 7  ;no change
    3.0 3  ;slide down 4 semitones within one-tenth of a second
    4.5 3  ;no change
    7.2 12 ;slide up 9 semitones over 2.7 seconds
    ```

5. wavecycle pitchwarp distortion *via* time-varying transposition of irregularly spaced wavecycles defined by zero-crossings across specified octaves or parts of octaves – breakpoint file:
    ```
    0.0 0.02  ;no transposition at the start
    1.4 0.02  ;no change yet
    1.5 0.58  ;slide up 7 semitones within one-tenth of a second
    2.9 0.58  ;no change
    3.0 0.33  ;slide down 4 semitones within one-tenth of a second
    4.5 0.33  ;no change
    7.2 1     ;slide up 9 semitones over 2.7 seconds
    ```

6. wavecycle pitchwarp distortion within 1/3 of an octave – the value 0.33 is entered for the *octvary* parameter (not time-varying), so the distortion stays the same throughout.

These sound transformations are very simple as this is the start of the exploration of the CDP soundware for sound design, but nevertheless move more directly into the heart of things by using text breakpoint files to create the time-varying events. The glissandi illustrate that the software 'interpolates' between values (fills in the gaps), implying that something different needs to be done to have immediate changes (stepped pitches rather than glissandi).

The PITCHWARP examples show that much of the software is in fact quite straightforward: a simple parameter value does something. Part of getting deeper into the software is to understand *why* it does what it does, at least to some degree. In this case, the salient issue is what a 'zero crosssing' is, that these are irregularly placed in most soundfiles, and that therefore, when randomly transposed within a defined limit, bits of the soundfile go up and down unpredictably and distort the original. Also note that interpolation between the different pitch levels is also taking place: there is a lot of sliding about, especially when the range increases.

Each sound is going to respond to a process differently. There is no substitute for exploration and curiosity, for 'ringing the changes' by trying different parameter values. A starting point is to use the default values, then to try extreme values, both to see what happens and to find the limits (error messages occur). Then work towards the realisation of what is in your sonic imagination.

Working with sound is straightforward and really very physical. You hear a sound, you do something to it, you hear and feel/experience the result. You like it or you don't think much of it, you consider how it much it contributes to the passage under the microscope, or to the overall plan for your composition. You save the result or discard it and try something else using the same source and process, tweaking the parameters, or the same source and another process, or another source and process altogether, You think about the sound you're trying to attain and trawl the documentation for the process that might achieve it. Composing with sound in a nutshell.

---

Last updated: 20 August 2021
