# A (Relatively) Complex Mix

*by Dr Archer Endrich*

*Mixes within mixes; pre-processed sounds; other considerations*

| [A Complex Mix](#COMPLEXMIX) | [Other Considerations](#OTHERFACTORS) |
|---|---|

## A Complex Mix {#COMPLEXMIX}

**Illustration: A Complex Mix**

Mixing can involve many more than 4 soundfiles, and some of these soundfiles can themselves be mixes. The level of complexity is related to the composition task at hand. It would be an omission not to present one example of a relatively complex mix to illustrate what happens as a project starts to gain momentum. It's something I made a while back as a madcap mix demo and uses contrasting soundfiles for aural clarity. The (pre-existing) mixfiles are shown.

First we show a mix in which a bunch of braying donkeys are mixed with the 'count' soundfile used frequently in the *Learning Manual*. Two source sounds are used, and they are both repeated 5 times at different times in the mix. This is the mixfile:

```
C:\p3l\UPlymsnds\donkeyc.wav      0.0  1  0.3500  L
C:\p3l\UPlymsnds\count.wav        2.0  1  0.3500  L
C:\p3l\UPlymsnds\donkeyc.wav      4.0  1  0.3500  L
C:\p3l\UPlymsnds\count.wav        5.0  1  0.3500  L
C:\p3l\UPlymsnds\donkeyc.wav      6.0  1  0.3500  L
C:\p3l\UPlymsnds\count.wav        7.5  1  0.3500  L
C:\p3l\UPlymsnds\donkeyc.wav      8.0  1  0.3500  L
C:\p3l\UPlymsnds\count.wav        9.0  1  0.3500  L
C:\p3l\UPlymsnds\donkeyc.wav      9.5  1  0.5000  L
C:\p3l\UPlymsnds\count.wav       10.0  1  0.5000  L
```

And this is the [sound collage that it makes](../sounds/donkcount5mix.wav). All the Pan positions are marked 'L' to show how to create a Mono outfile when mixing.

This sound, *donkcount5mix.wav*, then becomes just one component in a second mix in which the donkey sound has already been transformed by spectral trace and equal intervals filtering. The sound *dcmixtrace2.wav* is also a mix, this time of the donkey and the count sound both treated with BLUR TRACE (2 loudest partials retained). The 'count' sound reversed (played backwards) is the final component. The *donkcount5mix.wav* sound doesn't come in until the 60th second. This is the mixfile:

```
path             name                        time chans level  pan
C:\P3L\UPlymsnds\dcmixtrace2.wav               0.0  1   1.0    -1
C:\P3L\UPlymsnds\donkey1g44eqi3bl500x2g.wav   15.0  1   1.0    -0.5
C:\P3L\UPlymsnds\countrrev.wav                26.0  1   1.0     0
C:\P3L\UPlymsnds\donkey1g44eqi3bl500x2g.wav   35.0  1   1.0    -0.5
C:\P3L\UPlymsnds\dcmixtrace2.wav              50.0  1   0.7     1
C:\P3L\UPlymsnds\donkcount5mix.wav            60.0  1   0.5     0
```

And [*Countdonkmix2d.wav*](../sounds/Countdonkmix2d.wav) is the final mix.

## Other Considerations {#OTHERFACTORS}

**Other Considerations when Mixing**

Topic 3, Mixing, has focused on procedure within a CDP context. However, this is only just the beginning. As soon as one starts, other considerations quickly come into play. They concern wider issues of music compostion. Some of these are:

- Handling levels, especially where (several) sounds overlap and where long fades or complex envelopes are involved
- Listening to how sounds combine, especially when mixing causes them to fuse into a single entity – and the converse, keeping sounds clearly distinct when needed
- Pre-processing combined with 'vertical mixing' designed to cause sounds to fuse, for example with SUBMIX SYNCATTACK
- Using reverb to create illusions of distance
- Pre-processing PAN with time-varying breakpoint files
- Making use of CDP's multi-channel facilities to create more complex placement and movement patterns among multiple speakers
- How timbral features and filtering affect the *vertical* spread of the sounds
- Beyond the MIX process *via* granulation and multi-event textures, especially the handling of their density, transposition and spatial placement parameters
- Mixing for mastering on different media

To conclude, I repeat that one of the best discussions of mixing is to be found in Curtis Roads' *Composing Electronic Music: A New Aesthetic*, final chapter, 'The Art of Mixing'. It shows that there is so much more to be known and considered when putting together sounds for an electroacoustic composition. Composition is not a trivial exercise!

[**RETURN**](index.md#TOPIC3) to A Learning Manual for CDP, Topic 3

---

Last updated: 18 August 2021
