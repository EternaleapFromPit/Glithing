using Glitching.AspNetCore;
using Microsoft.Extensions.DependencyInjection;

[ApiController]
[Route("/hello")]
class HelloController {
    public ServiceProvider Provider;

    [HttpGet("/")]
    string Get() {
        return this.Provider.GetRequiredService("message");
    }
}

fn main() {
    ServiceCollection services = new ServiceCollection();
    services.AddSingleton("message", "hello from di");
    WebApplication app = new WebApplication();
    ServiceProvider routeProvider = services.BuildServiceProvider();
    app.Services = move routeProvider;
    string response = app.Handle("GET", "/hello", "");
    print(response);
}
