'use strict';

let woscope = require('../');


window.onload = function() {

    window.onresize();

    initWoscope();
};

function initWoscope(config) {
    let canvas = $('c');

    config = Object.assign({
      canvas: canvas,
      audio: null,
      getSource: function (audio_context) {
        let sineA = audio_context.createOscillator();
        sineA.type = 'sine';
        sineA.frequency.value = 120;

        var gainA = audio_context.createGain();
        gainA.gain.value = 0.4;
        sineA.connect(gainA);

        var sineB = audio_context.createOscillator();
        sineB.type = 'sine';
        sineB.frequency.value = 300.1;

        var gainB = audio_context.createGain();
        gainB.gain.value = 0.4;
        sineB.connect(gainB);

        let merger = audio_context.createChannelMerger(2);
        gainA.connect(merger, 0, 0);
        gainB.connect(merger, 0, 1);

        sineA.start();
        sineB.start();

        return merger;
      },
      callback: function () { 
          
      },
      error: function (msg) {
          htmlError.innerHTML = '';
          htmlError.appendChild(renderDom(msg.toString()));
      },
      color: [1/32, 1, 1/32, 1],
      color2: [1, 0, 1, 1],
      background: [0, 0, 0, 1],
      swap: false,
      invert: false,
      sweep: false,
      bloom: false,
      live: true,
    }, config);

    let myWoscope = woscope(config);
}

let mySourceNode;

function resetWoscope(woscopeInstance) {
    // Chrome has limit of one sourceNode per audio element, so keep a reference
    mySourceNode = woscopeInstance.sourceNode || mySourceNode;

    woscopeInstance.destroy();

    // replace canvas. more compatible than restoring gl context on old canvas
    let canvas = $('c');
    let copy = canvas.cloneNode(true);
    canvas.parentNode.replaceChild(copy, canvas);

    // prevent doubled audio
    if (query.live && mySourceNode) {
        mySourceNode.disconnect();
    }

    initWoscope({sourceNode: mySourceNode});
}

window.onresize = function () {
    let canvas = $('c'),
        length = Math.min(window.innerHeight, canvas.parentNode.offsetWidth);
    canvas.width = length;
    canvas.height = length;
};

function $(id) { return document.getElementById(id); }

function renderDom(obj) {
  let dom, idx, attrs;
  if (typeof obj === 'string') {
      return document.createTextNode(obj);
  } else if (Array.isArray(obj)) {
      if (obj[0] === '!comment') {
          return document.createComment(obj[1]);
      }
      dom = document.createElement(obj[0]);
      idx = 1;
      attrs = obj[1];
      if (attrs && Object.getPrototypeOf(attrs) === Object.prototype) {
          idx += 1;
          Object.keys(attrs).forEach(function (key) {
              if (key === 'style') {
                  Object.assign(dom.style, attrs[key]);
              } else if (/^on/.test(key)) {
                  dom[key] = attrs[key];
              } else {
                  dom.setAttribute(key, attrs[key]);
              }
          });
      }
      obj.slice(idx).forEach(function (child) {
          dom.appendChild(renderDom(child));
      });
      return dom;
  } else {
      throw 'Cannot make dom of: ' + obj;
  }
}

function parseq(search) {
    search = search.replace(/^\?/, '');
    let obj = {};
    search.split('&').forEach(function (pair) {
        pair = pair.split('=');
        obj[decodeURIComponent(pair[0])] =
            pair.length > 1 ? decodeURIComponent(pair[1]) : true;
    });
    return obj;
}

function dumpq(obj) {
    return Object.keys(obj).map(function(key) {
        if (obj[key] === true) {
            return encodeURIComponent(key);
        } else {
            return encodeURIComponent(key) + '=' + encodeURIComponent(obj[key]);
        }
    }).join('&');
}
