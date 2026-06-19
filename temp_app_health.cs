using Glitching.AspNetCore;
using Microsoft.Extensions.DependencyInjection;

string HealthEndpoint() {
    return "{\"health\":\"ok\"}";
}

fn main() {
    ServiceCollection services = new ServiceCollection();
    services.AddSingleton("message", "hello from di");
    WebApplication app = new WebApplication();
    ServiceProvider routeProvider = services.BuildServiceProvider();
    app.Services = move routeProvider;
    app.MapGet("/hello", "\"hello from di\"");
    app.MapGet("/health", HealthEndpoint);
    string health = app.Handle("GET", "/health", "");
    print(health);
}
