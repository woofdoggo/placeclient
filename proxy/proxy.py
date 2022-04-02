from http.server import HTTPServer, SimpleHTTPRequestHandler
from ssl import wrap_socket, PROTOCOL_TLS
from urllib.request import urlopen

class Getter(SimpleHTTPRequestHandler):
    def do_GET(self):
        imgurl = "https://hot-potato.reddit.com/media/canvas-images" + self.path
        print(self.path[1:])

        self.send_response(200)
        self.end_headers()

addr = ("localhost", 8000)
httpd = HTTPServer(addr, Getter)
httpd.socket = wrap_socket(
        httpd.socket,
        server_side = True,
        keyfile = "key.pem",
        certfile = "cert.pem",
        ssl_version = PROTOCOL_TLS
)

httpd.serve_forever()
