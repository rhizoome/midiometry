# MIDIOMETRY

<p align="center">
    <img src=https://github.com/dvub/midiometry/blob/master/media/image.png>
</p>

This is MIDIOMETRY, a free & open-source plugin that visualizes your playing. (This plugin does NOT visualize audio, just MIDI inputs!)

### Installation

(This guide assumes you already know how to add VST3/CLAP plugins to your DAW.)

To install MIDIOMETRY, go to the ["Actions" tab](https://github.com/dvub/midiometry/actions) of this repository, click on a build (you probably want the most recent build, assuming it passed), and scroll down to the "Artifacts" section. From here, download the zip file for your platform\* and extract the contents.

Then, follow your normal process for adding plugins to your DAW, whether it be adding the plugin files to your OS's default plugin locations, or to a custom location.

\*If you're not running Windows, Mac, or Ubuntu, you can try downloading the source code and building the project yourself. However, this might not work.

### DAW-Specific Notes & Usage

#### Ableton

Ableton doesn't have very good support for _pure MIDI plugins_ - **you'll probably have to use a workaround,** e.g. where MIDIOMETRY has its own track, and you send the MIDI out to another track with an instrument on it.

#### FL Studio

I haven't tested FL studio at all.

#### Bitwig

Bitwig has the best support for MIDI-related plugins and **should work seamlessly**.

### OS Notes

#### Linux

This plugin **may or may not work if you're on Linux.** In my testing with Bitwig (individually sandboxed plugins), everything has worked fine. However, if you're not using the same sandboxing, or if you're using a different DAW (Reaper, etc.) this might work sometimes or not at all. Good luck.

### Issues

Please open an issue on this repository if something isn't working, and I'll do my best to respond and work on it.
