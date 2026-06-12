using System;

namespace Demo.Api {
    [ApiController]
    [Route("/users")]
    class UsersController {
        public string Name;
    }

    [HttpGet("/health")]
    string Health() {
        return "ok";
    }
}
