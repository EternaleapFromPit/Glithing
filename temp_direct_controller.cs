using System.Threading.Tasks;
using Microsoft.Extensions.DependencyInjection;

class HelloController {
    public ServiceProvider Provider;

    async Task<string> Get() {
        return this.Provider.GetRequiredService("message");
    }
}

fn main() {
    ServiceCollection services = new ServiceCollection();
    services.AddSingleton("message", "hello from di");
    ServiceProvider provider = services.BuildServiceProvider();
    HelloController controller = new HelloController { Provider = move provider };
    Task<string> controllerTask = controller.Get();
    string controllerText = await controllerTask;
    print(controllerText);
}
