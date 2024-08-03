import http.server
import os

class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):

    def server_index(self) -> None:
        try:
            error_page = "index.html"

            f = open(error_page, 'rb')
            ctype = ctype = self.guess_type(error_page)

            fs = os.fstat(f.fileno())
            self.send_response(http.HTTPStatus.OK)
            self.send_header("Content-type", ctype)
            self.send_header("Content-Length", str(fs[6]))
            self.send_header("Last-Modified",
                self.date_time_string(fs.st_mtime))
            self.end_headers()

            self.copyfile(f, self.wfile)
        except OSError:
            self.send_error(http.HTTPStatus.NOT_FOUND, "File not found")
        finally:
            if f:
                f.close()

    def send_error(self, code: int, message: str | None = None, explain: str | None = None) -> None:
        if code == 404:
            self.server_index()
            return
        return super().send_error(code, message, explain)

HandlerClass = CustomHTTPRequestHandler

# Patch in the correct extensions
HandlerClass.extensions_map['.js'] = 'text/javascript'
HandlerClass.extensions_map['.mjs'] = 'text/javascript'

# Run the server (like `python -m http.server` does)
http.server.test(HandlerClass, port=8000)