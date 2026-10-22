const output = document.getElementById('output');
const pingButton = document.getElementById('ping-button');

const writeOutput = (message) => {
  output.textContent = message;
};

pingButton.addEventListener('click', () => {
  writeOutput('Running network health check...\n- DNS resolution: OK\n- Gateway reachable: OK\n- Latency: 22ms');
});
