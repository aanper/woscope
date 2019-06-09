'use strict';
/* global WebAssembly */

let woscope = require('../');

function buf2hex(buffer) { // buffer is an ArrayBuffer
  return Array.prototype.map.call(new Uint8Array(buffer), x => ('00' + x.toString(16)).slice(-2)).join('');
}

function handle_audio(audio_context, rust_exports) {

    console.log("use wasm:", rust_exports.add_one(41));

    // create constant wave for left channel
    let left_node = audio_context.createConstantSource();
    let right_node = audio_context.createConstantSource();


    // merge two wave to stereo
    let merger = audio_context.createChannelMerger(2);
    left_node.connect(merger, 0, 0);
    right_node.connect(merger, 0, 1);

    let script_node = audio_context.createScriptProcessor(4096, 2, 2);

    let fs = audio_context.sampleRate;

    let t = 0;

    script_node.onaudioprocess = function(e) {
        // The output buffer contains the samples that will be modified and played
        let inputBuffer = e.inputBuffer;
        let outputBuffer = e.outputBuffer;

        let inputData = [];
        let outputData = [];

        // Loop through the output channels
        for (let channel = 0; channel < outputBuffer.numberOfChannels; channel++) {
            inputData.push(inputBuffer.getChannelData(channel));
            outputData.push(outputBuffer.getChannelData(channel));
        }

        // Loop through the 4096 samples
        for (let sample = 0; sample < inputBuffer.length; sample++) {
            // make output equal to the same as the input
            outputData[0][sample] = 0; // inputData[0][sample] * 0.4;
            outputData[1][sample] = 0; // inputData[1][sample] * 0.2;

            let dp = (t < 2 ? t * 5 : 4 * 5); // 0.5 * Math.sin(2 * Math.PI * 0.1 * t);

            let da = 0;
            let a = 1;

            if(t > 2) {
                da = (t - 2) * 0.01;
            }

            if(t > 5) {
                da = (t - 5) * 1;
            }

            if(t > 7) {
                a = (t - 6.9) * 10;
            }


            let p_0 = 0.05* Math.PI * Math.sin(2 * Math.PI * (360 + dp*5) * t) / (1 + da*100);
            let p_1 = 0.05 * Math.PI * Math.sin(2 * Math.PI * (350 + dp*5.5) * t) / (1 + da*100);

            outputData[0][sample] += p_0 * 0.3 * Math.sin(2 * Math.PI * 120 * t) * dp ;
            outputData[1][sample] += p_1 * 0.3 * Math.sin(2 * Math.PI * 300.2 * t) + p_0;

            outputData[0][sample] += (da/(da+1)) * Math.sin(2 * Math.PI * (240 * (da/(da+2))) * t);
            outputData[1][sample] += (da/(da+1)) * -Math.sin(2 * Math.PI * (120 * (da/(da+2))) * t);
            outputData[0][sample] += (da/(da+1)) * 0.5 * Math.sin(2 * Math.PI * 1 * t);
            outputData[1][sample] += (da/(da+1)) * 0.5 * Math.cos(2 * Math.PI * 1 * t);

            // outputData[0][sample] += 0.01 * Math.sin(2 * Math.PI * 50.1 * t + Math.PI/4);
            // outputData[1][sample] += 0.01 * Math.sin(2 * Math.PI * 80.5 * t);

            // add noise to each output sample
            outputData[0][sample] += ((Math.random() * 2) - 1) * 0.001;
            outputData[1][sample] += ((Math.random() * 2) - 1) * 0.001;

            outputData[0][sample] /= a;
            outputData[1][sample] /= a;

            t += 1/fs;   
        }
    };

    merger.connect(script_node);

    left_node.start();
    right_node.start();

    return script_node;
}

window.onload = function() {
    window.onresize();

    let canvas = document.getElementById('c');

    fetch('/dist/demo.strip.wasm')
    .then(response => response.arrayBuffer())
    .then(bytes => {
        // console.log(buf2hex(bytes));
        return WebAssembly.instantiate(bytes, {});
    })
    .then(results => {
        woscope({
            canvas: canvas,
            audio: null,
            getSource: audio_context => handle_audio(audio_context, results.instance.exports),
            live: true,
        });
    });
};

let mySourceNode;

window.onresize = function () {
    let canvas = document.getElementById('c'),
        length = Math.min(window.innerHeight, canvas.parentNode.offsetWidth);
    canvas.width = length;
    canvas.height = length;
};