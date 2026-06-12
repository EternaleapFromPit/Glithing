namespace Demo.Api {
    class Counter {
        public int Count { get; set; }

        public Counter(int count) {
            this.Count = count;
        }

        int Twice => this.Count + this.Count;
    }

    fn main() {
        Counter counter = new Counter(5);
        print(counter.Twice);
    }
}
