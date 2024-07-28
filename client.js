const { WebSocket } = require("ws");

let token =
  "query_id=AAEgnJZfAAAAACCcll_j9Uw_&user=%7B%22id%22%3A1603705888%2C%22first_name%22%3A%22Pham%22%2C%22last_name%22%3A%22duc%22%2C%22username%22%3A%22ducpv07%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%7D&auth_date=1720664471&hash=07feea202113f51577c7eb5cb1276143cacddbc8910003912c4d90f0f00b9519";

let endpoint_local = `ws://127.0.0.1:8098/gas-channel?token=${token}`;

const client = new WebSocket(endpoint_local, {
  headers: {
    authorization: `Bearer ${token}`,
  },
});

client.onmessage = function (msg) {
  console.log(JSON.parse(msg.data));
};

process.on("SIGINT", () => {
  client.close();
  process.exit(1);
});

setInterval(() => {
  client.send(
    JSON.stringify({
      type: "Win",
    }),
  );
}, 11_000);

client.onerror = (error) => {
  console.error(error);
  process.exit(1);
};

client.onclose = (e) => {
  console.log("close");
  process.exit(1);
};
