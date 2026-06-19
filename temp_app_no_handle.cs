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
    app.Use("logging");
    app.MapGet("/hello", "\"hello from di\"");
    app.MapGet("/health", HealthEndpoint);
    app.MapPost("/hello", "{\"ok\":true}");
    print("mapped");
}
