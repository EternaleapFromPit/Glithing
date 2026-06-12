using System;
using System.Collections.Generic;

WebApplication app = new WebApplication();
app.RegisterGet("/health", 200);

HttpRequest request = new HttpRequest("GET", "/health");

int iteration = 0;
int successful = 0;
while (iteration < 2000000) {
    int statusCode = app.Handle(request);
    if (statusCode == 200) {
        successful = successful + 1;
    }
    iteration = iteration + 1;
}

Console.WriteLine(successful);

class HttpRequest {
    public string Method;
    public string Path;

    public HttpRequest(string method, string path) {
        this.Method = method;
        this.Path = path;
    }
}

class HealthController {
    public int Get(HttpRequest request) {
        return 200;
    }
}

class WebApplication {
    public Dictionary<string, int> Routes;
    public HealthController Controller;

    public WebApplication() {
        this.Routes = new Dictionary<string, int>();
        this.Controller = new HealthController();
    }

    public void RegisterGet(string path, int statusCode) {
        this.Routes.Add(path, statusCode);
    }

    public int Handle(HttpRequest request) {
        if (this.Routes.ContainsKey(request.Path)) {
            return this.Controller.Get(request);
        } else {
            return 404;
        }
    }
}
