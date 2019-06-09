'use strict';

let woscope = require('../');

window.onload = function() {
    window.onresize();

    let canvas = document.getElementById('c');

    let myWoscope = woscope({
        canvas: canvas,
        audio: null,
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

        live: true,
    });
};

let mySourceNode;

window.onresize = function () {
    let canvas = document.getElementById('c'),
        length = Math.min(window.innerHeight, canvas.parentNode.offsetWidth);
    canvas.width = length;
    canvas.height = length;
};