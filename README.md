# RaveSync

RaveSync is a program that analyzes live audio and turns that data into a light
show that can be interpreted from the window it displays. The window can be
captured using OBS or other streaming software in order to send the codes
through video. As long as the video feed and audio feed are in sync, the lights
will match the audio. This project is intended for use in virtual clubs but
feel free to mess with it however you want.

## How it works

RaveSync has three major components separated by threads.

The `main` thread handles the windowing, communication, and creating of threads.

The `cli` thread handles user input. It sends commands to the main thread.

The `audio` thread handles the audio engine. It holds the audio buffer sending
color data and commands to the main thread.

If the cli thread needs to communicate with the audio thread, it will route the
data through the main thread. The main thread will handle most of the logic that
goes on between the threads meaning all data can be accessed from the same point
making shared data slightly simpler.
