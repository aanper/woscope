## woscope: oscilloscope emulator

This is a POC oscilloscope emulator with [live demo](http://m1el.github.io/woscope/)

Full explanation available on [the blag](http://m1el.github.io/woscope-how/)

Code is available under MIT license.

### Example
Take example code from demo/index.js. Define `getSource` function
```js
	getSource: function (audio_context) {

        // create sine wave for left channel
        let sineA = audio_context.createOscillator();
        sineA.type = 'sine';
        sineA.frequency.value = 120;

        // scale sine wave
        var gainA = audio_context.createGain();
        gainA.gain.value = 0.4;
        sineA.connect(gainA);

        // create sine fot right channel
        var sineB = audio_context.createOscillator();
        sineB.type = 'sine';
        sineB.frequency.value = 300.1; // pitch freq to make spinning effect

        // and scale it
        var gainB = audio_context.createGain();
        gainB.gain.value = 0.4;
        sineB.connect(gainB);

        // merge two wave to stereo
        let merger = audio_context.createChannelMerger(2);
        gainA.connect(merger, 0, 0);
        gainB.connect(merger, 0, 1);

        sineA.start();
        sineB.start();

        return merger;
    },
```

### Dev commands
```sh
npm install      # install dev dependencies
npm run demo     # run demo locally with livereload
npm run build    # lint and build dist files
```
