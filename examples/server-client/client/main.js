import init, { handleMessage } from "./pkg/index.js";

const makeAndSendSetupData = document.createElement('button')
makeAndSendSetupData.id = 'button'
makeAndSendSetupData.innerHTML = 'Generate and send setup data'
document.body.appendChild(makeAndSendSetupData)

window.addEventListener("load", async () => {
    await init()
    const ws = new WebSocket(`ws://${location.hostname}:8080/ws`)
    ws.addEventListener("message", (event) => onMessage(ws, event))
    ws.addEventListener("close", console.log("Socket closed"))
    ws.addEventListener("error", (error) => console.log("WebSocket error", error))
    ws.addEventListener("open", () => handleConnection(ws))

})

const button = document.getElementById('button')
button.addEventListener('click', () => handleClick())

function handleConnection(socket) {
    console.log("Connected")
}

// A handler for incoming messages from the server.
async function onMessage(socket, event) {
    let buffer = await event.data.arrayBuffer()
    const data = new Uint8Array(buffer)
    const response = handleMessage(data)
    socket.send(response)
}

const handleClick = () => console.log('click')
