from pynput.keyboard import Key, Controller
from pynput.keyboard._xorg import KeyCode
from pynput._util.xorg import display_manager
import os
import sys
import socket
from Xlib.ext.xtest import fake_input
from Xlib import X
import Xlib

KeyCode._from_symbol("\0")  # test


class MyController(Controller):
    def _handle(self, key, is_press):
        """Resolves a key identifier and sends a keyboard event.
        :param event: The *X* keyboard event.
        :param int keysym: The keysym to handle.
        """
        keysym = self._keysym(key)
        keycode = self._display.keysym_to_keycode(keysym)

        with display_manager(self._display) as dm:
            Xlib.ext.xtest.fake_input(
                dm,
                Xlib.X.KeyPress if is_press else Xlib.X.KeyRelease,
                keycode)



keyboard = MyController()

server_address = sys.argv[1]
if not os.path.exists(os.path.dirname(server_address)):
    os.makedirs(os.path.dirname(server_address))

try:
    os.unlink(server_address)
except OSError:
    if os.path.exists(server_address):
        raise

server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
server.bind(server_address)
server.listen(1)
clientsocket, address = server.accept()
os.system('chmod a+rw %s' % server_address)
print("Got pynput connection")


def loop():
    global keyboard
    buf = []
    while True:
        data = clientsocket.recv(1024)
        if not data:
            print("Connection broken")
            break
        buf.extend(data)
        while buf:
            n = buf[0]
            n = n + 1
            if len(buf) < n:
                break
            msg = bytearray(buf[1:n]).decode("utf-8")
            buf = buf[n:]
            if len(msg) < 2:
                continue
            if msg[1] == "\0":
                keyboard = MyController()
                print("Keyboard reset")
                continue
            if len(msg) == 2:
                name = msg[1]
            else:
                name = KeyCode._from_symbol(msg[1:])
            if str(name) == "<0>":
                continue
            try:
                if msg[0] == "p":
                    keyboard.press(name)
                else:
                    keyboard.release(name)
            except Exception as e:
                print(e)


loop()
clientsocket.close()
server.close()
