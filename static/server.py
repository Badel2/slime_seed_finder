#!/usr/bin/env python3

import http.server
import socketserver

PORT = 8000

class Handler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        if False:
            # Allows using SharedArrayBuffer
            self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
            self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
            http.server.SimpleHTTPRequestHandler.end_headers(self)

Handler.extensions_map['.shtml'] = 'text/html'
Handler.extensions_map['.wasm'] = 'application/wasm'

httpd = socketserver.TCPServer(("", PORT), Handler)

print("serving at port", PORT)
httpd.serve_forever()
