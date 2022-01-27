import Connection from "./connection";
import _sodium from "libsodium-wrappers";
import * as zstd from 'zstddec';
import { CursorData } from "./message";
import { loadOpus, loadVp9 } from "./codec";

var decompressor;
var wasmDsp;

var currentFrame = undefined;
var events = [];

window.curConn = undefined;
window.getRgba = () => {
  const tmp = currentFrame;
  currentFrame = undefined;
  return tmp || null;
}
window.getLanguage = () => navigator.language;

export function msgbox(type, title, text) {
  if (!events) return;
  if (!type) return;
  const text2 = text.toLowerCase();
  var hasRetry = type == "error"
    && title == "Connection Error"
    && text2.indexOf("offline") < 0
    && text2.indexOf("exist") < 0
    && text2.indexOf("handshake") < 0
    && text2.indexOf("failed") < 0
    && text2.indexOf("resolve") < 0
    && text2.indexOf("mismatch") < 0
    && text2.indexOf("manually") < 0;
  events.push({ name: 'msgbox', type, title, text, hasRetry });
}

export function pushEvent(name, payload) {
  if (!events) return;
  payload.name = name;
  events.push(payload);
}

export function draw(frame) {
  currentFrame = I420ToABGR(frame);
}

export function setConn(conn) {
  window.curConn = conn;
}

export function getConn() {
  return window.curConn;
}

export async function startConn(id) {
  try {
    await curConn.start(id);
  } catch (e) {
    console.log(e);
    msgbox('error', 'Error', String(e));
  }
}

export function close() {
  getConn()?.close();
  setConn(undefined);
  currentFrame = undefined;
  events = undefined;
}

export function newConn() {
  window.curConn?.close();
  events = [];
  const conn = new Connection();
  setConn(conn);
  return conn;
}

let sodium;
export async function verify(signed, pk) {
  if (!sodium) {
    await _sodium.ready;
    sodium = _sodium;
  }
  if (typeof pk == 'string') {
    pk = decodeBase64(pk);
  }
  return sodium.crypto_sign_open(signed, pk);
}

export function decodeBase64(pk) {
  return sodium.from_base64(pk, sodium.base64_variants.ORIGINAL);
}

export function genBoxKeyPair() {
  const pair = sodium.crypto_box_keypair();
  const sk = pair.privateKey;
  const pk = pair.publicKey;
  return [sk, pk];
}

export function genSecretKey() {
  return sodium.crypto_secretbox_keygen();
}

export function seal(unsigned, theirPk, ourSk) {
  const nonce = Uint8Array.from(Array(24).fill(0));
  return sodium.crypto_box_easy(unsigned, nonce, theirPk, ourSk);
}

function makeOnce(value) {
  var byteArray = Array(24).fill(0);

  for (var index = 0; index < byteArray.length && value > 0; index++) {
    var byte = value & 0xff;
    byteArray[index] = byte;
    value = (value - byte) / 256;
  }

  return Uint8Array.from(byteArray);
};

export function encrypt(unsigned, nonce, key) {
  return sodium.crypto_secretbox_easy(unsigned, makeOnce(nonce), key);
}

export function decrypt(signed, nonce, key) {
  return sodium.crypto_secretbox_open_easy(signed, makeOnce(nonce), key);
}

export async function decompress(compressedArray) {
  const MAX = 1024 * 1024 * 64;
  const MIN = 1024 * 1024;
  let n = 30 * compressedArray.length;
  if (n > MAX) {
    n = MAX;
  }
  if (n < MIN) {
    n = MIN;
  }
  try {
    if (!decompressor) {
      await initZstd();
    }
    return decompressor.decode(compressedArray, n);
  } catch (e) {
    console.error('decompress failed: ' + e);
  }
}

window.setByName = (name, value) => {
  try {
    value = JSON.parse(value);
  } catch (e) { }
  switch (name) {
    case 'connect':
      newConn();
      startConn(String(value));
      break;
    case 'login':
      curConn.login(value.password, value.remember || false);
      break;
    case 'close':
      close();
      break;
    case 'refresh':
      curConn.refresh();
      break;
    case 'reconnect':
      curConn.reconnect();
      break;
    case 'toggle_option':
      curConn.toggleOption(value);
      break;
    case 'image_quality':
      curConn.setImageQuality(value);
      break;
    case 'lock_screen':
      curConn.lockScreen();
      break;
    case 'ctrl_alt_del':
      curConn.ctrlAltDe();
      break;
    case 'switch_display':
      curConn.switchDisplay(value);
      break;
    case 'remove':
      const peers = JSON.parse(localStorage.getItem('peers') || '{}');
      delete peers[value];
      localStorage.setItem('peers', JSON.stringify(peers));
      break;
    case 'input_key':
      curConn.inputKey(value.name, value.alt || false, value.ctrl || false, value.shift || false, value.command || false);
      break;
    case 'input_string':
      curConn.inputString(value);
      break;
    case 'send_mouse':
      let mask = 0;
      switch (value.type) {
        case 'down':
          mask = 1;
          break;
        case 'up':
          mask = 2;
          break;
        case 'wheel':
          mask = 3;
          break;
      }
      switch (value.buttons) {
        case 'left':
          mask |= 1 << 3;
          break;
        case 'right':
          mask |= 2 << 3;
          break;
        case 'wheel':
          mask |= 4 << 3;
      }
      curConn.inputMouse(mask, value.x || 0, value.y || 0, value.alt || false, value.ctrl || false, value.shift || false, value.command || false);
      break;
    case 'option':
      localStorage.setItem(value.name, value.value);
      break;
    case 'peer_option':
      curConn.setPeerOption(value.name, value.value);
      break;
    case 'input_os_password':
      curConn.inputOsPassword(value, true);
      break;
    default:
      break;
  }
}

window.getByName = (name, arg) => {
  try {
    arg = JSON.parse(arg);
  } catch (e) { }
  switch (name) {
    case 'peers':
      return localStorage.getItem('peers') || '[]';
      break;
    case 'remote_id':
      return localStorage.getItem('remote-id') || '';
      break;
    case 'remember':
      return curConn.getRemember();
      break;
    case 'event':
      if (events && events.length) {
        const e = events[0];
        events.splice(0, 1);
        return JSON.stringify(e);
      }
      break;
    case 'toggle_option':
      return curConn.getOption(arg);
      break;
    case 'option':
      return localStorage.getItem(arg);
      break;
    case 'image_quality':
      return curConn.getImageQuality();
      break;
    case 'translate':
      return arg.text;
      break;
    case 'peer_option':
      return curConn.getOption(arg);
      break;
    case 'test_if_valid_server':
      break;
  }
  return '';
}

window.init = async () => {
  await initZstd();
}

function I420ToABGR(yb) {
  if (!wasmDsp) return null;
  const yPtr = wasmDsp._malloc(yb.y.bytes.length);
  wasmDsp.HEAPU8.set(yb.y.bytes, yPtr);
  const uPtr = wasmDsp._malloc(yb.u.bytes.length);
  wasmDsp.HEAPU8.set(yb.u.bytes, uPtr);
  const vPtr = wasmDsp._malloc(yb.v.bytes.length);
  wasmDsp.HEAPU8.set(yb.v.bytes, vPtr);
  const oSize = yb.format.width * yb.format.height * 4;
  const outPtr = wasmDsp._malloc(oSize);
  const res = wasmDsp._I420ToABGR(yPtr, yb.y.stride, uPtr, yb.u.stride, vPtr, yb.v.stride, outPtr, yb.format.width * 4,
    yb.format.width, yb.format.height);
  const out = wasmDsp.HEAPU8.slice(outPtr, outPtr + oSize);
  wasmDsp._free(yPtr);
  wasmDsp._free(uPtr);
  wasmDsp._free(vPtr);
  wasmDsp._free(outPtr);
  return out;
}

async function initZstd() {
  loadOpus(() => { });
  loadVp9(() => { });
  fetch('./LibYUV.wasm').then(res => res.arrayBuffer()).then((buffer) => {
    LibYUV['wasmBinary'] = buffer;
    window.wasmDsp = wasmDsp = LibYUV({ wasmBinary: LibYUV.wasmBinary });
    console.log('libyuv ready');
  });
  const tmp = new zstd.ZSTDDecoder();
  await tmp.init();
  console.log('zstd ready');
  decompressor = tmp;
}