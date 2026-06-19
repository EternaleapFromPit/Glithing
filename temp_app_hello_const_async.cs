using System.Threading.Tasks;
using Glitching.AspNetCore;

[ApiController]
[Route("/hello")]
class HelloController {
    [HttpGet("/")]
    async Task<string> Get() {
        return "hi";
    }
}

fn main() {
    WebApplication app = new WebApplication();
    string response = app.Handle("GET", "/hello", "");
    print(response);
}
