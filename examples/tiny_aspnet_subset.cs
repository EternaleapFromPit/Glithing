using System.Collections.Generic;
using System.Threading.Tasks;
using System.Text.Json;
using Microsoft.Extensions.DependencyInjection;
using Glitching.AspNetCore;

[ApiController]
[Route("/hello")]
class HelloController {
    public ServiceProvider Provider;

    [HttpGet("/")]
    async Task<string> Get() {
        return this.Provider.GetRequiredService("message");
    }
}

string HealthEndpoint() {
    return "{\"health\":\"ok\"}";
}

fn main() {
    ServiceCollection services = new ServiceCollection();
    services.AddSingleton("message", "hello from di");
    ServiceProvider provider = services.BuildServiceProvider();

    HelloController controller = new HelloController { Provider = move provider };
    Task<string> controllerTask = controller.Get();
    string controllerText = await controllerTask;
    print(controllerText);
    string json = JsonSerializer_SerializeString(controllerText);

    WebApplication app = new WebApplication();
    ServiceProvider routeProvider = services.BuildServiceProvider();
    app.Services = move routeProvider;
    app.Use("logging");
    app.MapGet("/hello", json);
    app.MapGet("/health", HealthEndpoint);
    app.MapPost("/hello", "{\"ok\":true}");
    string response = app.Handle("GET", "/hello", "");
    print(response);
    string health = app.Handle("GET", "/health", "");
    print(health);
}
