import * as message from "./message.js";
import * as rendezvous from "./rendezvous.js";
import * as globals from "./globals";

type Keys = "message" | "open" | "close" | "error";

export default class Websock {
  _websocket: WebSocket;
  _eventHandlers: { [key in Keys]: Function };
  _buf: Uint8Array[];
  _status: any;
  _latency: number;
  _secretKey: [Uint8Array, number, number] | undefined;

  constructor(uri: string) {
    this._eventHandlers = {
      message: (_: any) => {},
      open: () => {},
      close: () => {},
      error: () => {},
    };
    this._status = "";
    this._buf = [];
    this._websocket = new WebSocket(uri);
    this._websocket.onmessage = this._recv_message.bind(this);
    this._websocket.binaryType = "arraybuffer";
    this._latency = new Date().getTime();
  }

  latency(): number {
    return this._latency;
  }

  setSecretKey(key: Uint8Array) {
    this._secretKey = [key, 0, 0];
  }

  sendMessage(json: any) {
    let data = message.Message.encode(message.Message.fromPartial(json)).finish();
    let k = this._secretKey;
    if (k) {
      k[1] += 1;
      data = globals.encrypt(data, k[1], k[0]);
    }
    this._websocket.send(data);
  }

  sendRendezvous(data: any) {
    this._websocket.send(
      rendezvous.RendezvousMessage.encode(
        rendezvous.RendezvousMessage.fromPartial(data)
      ).finish()
    );
  }

  parseMessage(data: Uint8Array) {
    return message.Message.decode(data);
  }

  parseRendezvous(data: Uint8Array) {
    return rendezvous.RendezvousMessage.decode(data);
  }

  // Event Handlers
  off(evt: Keys) {
    this._eventHandlers[evt] = () => {};
  }

  on(evt: Keys, handler: Function) {
    this._eventHandlers[evt] = handler;
  }

  async open(timeout: number = 12000): Promise<Websock> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        if (this._status != "open") {
          reject(this._status || "timeout");
        }
      }, timeout);
      this._websocket.onopen = () => {
        this._latency = new Date().getTime() - this._latency;
        this._status = "open";
        console.debug(">> WebSock.onopen");
        if (this._websocket?.protocol) {
          console.info(
            "Server choose sub-protocol: " + this._websocket.protocol
          );
        }

        this._eventHandlers.open();
        console.info("WebSock.onopen");
        resolve(this);
      };
      this._websocket.onclose = (e) => {
        this._status = e;
        console.error("WebSock.onclose: " + e);
        this._eventHandlers.close(e);
        reject(e);
      };
      this._websocket.onerror = (e) => {
        this._status = e;
        console.error("WebSock.onerror: " + e);
        this._eventHandlers.error(e);
        reject(e);
      };
    });
  }

  async next(timeout = 12000): Promise<Uint8Array> {
    const func = (
      resolve: (value: Uint8Array) => void,
      reject: (reason: any) => void,
      tm0: number
    ) => {
      if (this._buf.length) {
        resolve(this._buf[0]);
        this._buf.splice(0, 1);
      } else {
        if (this._status != "open") {
          reject(this._status);
          return;
        }
        if (new Date().getTime() > tm0 + timeout) {
          reject("timeout");
        } else {
          setTimeout(() => func(resolve, reject, tm0), 1);
        }
      }
    };
    return new Promise((resolve, reject) => {
      func(resolve, reject, new Date().getTime());
    });
  }

  close() {
    if (this._websocket) {
      if (
        this._websocket.readyState === WebSocket.OPEN ||
        this._websocket.readyState === WebSocket.CONNECTING
      ) {
        console.info("Closing WebSocket connection");
        this._websocket.close();
      }

      this._websocket.onmessage = () => {};
    }
  }

  _recv_message(e: any) {
    if (e.data instanceof window.ArrayBuffer) {
      let bytes = new Uint8Array(e.data);
      const k = this._secretKey;
      if (k) {
        k[2] += 1;
        bytes = globals.decrypt(bytes, k[2], k[0]);
      }
      this._buf.push(bytes);
    }
    this._eventHandlers.message(e.data);
  }
}
