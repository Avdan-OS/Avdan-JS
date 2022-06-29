import Net from "@avdan/net";
import Debug from "@avdan/debug"

let b = await Net.fetch(
    "https://postman-echo.com/post",
    {
        method: "post",
        headers: {
            "HeloThere": "Cain and Abel"
        },
        body: "Hello World"
    }
);

Debug.log(b.toString());