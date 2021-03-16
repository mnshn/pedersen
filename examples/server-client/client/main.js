import init, { handleMessage } from "./pkg/index.js";

window.addEventListener("load", async () => {
    await init();
    const ws = new WebSocket(`ws://${location.hostname}:8080/ws`);
    ws.addEventListener("message", (event) => onMessage(ws, event));
    ws.addEventListener("close", console.log("Socket closed"));
    ws.addEventListener("error", (error) => console.log("WebSocket error", error));
    ws.addEventListener("open", () => handleConnection(ws));
});

function handleConnection(socket) {
    // Do something when the socket is opened
}

// A handler for incoming messages from the server.
async function onMessage(socket, event) {
    let buffer = await event.data.arrayBuffer();
    const data = new Uint8Array(buffer);
    const response = handleMessage(data);
    socket.send(response);
}
