fn worker() {
    print(42);
}

fn main() {
    Thread t = new Thread(worker);
    t.Start();
    t.Join();
}
