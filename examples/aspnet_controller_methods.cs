using System;

namespace Demo.Api {
    [ApiController]
    [Route("/counters")]
    class CounterController {
        public int Count;

        [HttpGet("/count")]
        int GetCount() {
            return this.Count;
        }
    }

    fn main() {
        CounterController controller = new CounterController { Count = 3 };
        int count = controller.GetCount();
        print(count);
    }
}
