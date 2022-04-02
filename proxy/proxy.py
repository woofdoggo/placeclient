from http.server import HTTPServer, SimpleHTTPRequestHandler
from ssl import wrap_socket, PROTOCOL_TLS
from urllib.request import urlopen

import socket

s = socket.socket()
s.connect(("127.0.0.1", 8001))

class Getter(SimpleHTTPRequestHandler):
    def do_GET(self):
        imgurl = "https://hot-potato.reddit.com/media/canvas-images" + self.path

        self.send_response(200)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()

        s.send(imgurl.encode())
        self.copyfile(urlopen(imgurl), self.wfile)

addr = ("127.0.0.1", 8000)
httpd = HTTPServer(addr, Getter)
httpd.socket = wrap_socket(
        httpd.socket,
        server_side = True,
        keyfile = "key.pem",
        certfile = "cert.pem",
        ssl_version = PROTOCOL_TLS
)

httpd.serve_forever()
