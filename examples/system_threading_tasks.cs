using System.Threading.Tasks;

fn worker() {
    print(42);
}

fn main() {
    Task task = Task.Run(worker);
    task.Wait();
}
