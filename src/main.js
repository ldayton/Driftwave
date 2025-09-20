// Import Tauri API
const { invoke } = window.__TAURI__ ? window.__TAURI__.tauri : { invoke: () => console.log('Running without Tauri') };

// Simple logging function that logs to both console and file
async function log(...args) {
    console.log(...args);
    try {
        const message = `[FRONTEND] ${args.join(' ')}`;
        await invoke('log_to_file', { message });
    } catch (e) {
        console.error('Failed to log to file:', e);
    }
}

async function logError(...args) {
    console.error(...args);
    try {
        const message = `[FRONTEND ERROR] ${args.join(' ')}`;
        await invoke('log_to_file', { message });
    } catch (e) {
        console.error('Failed to log error to file:', e);
    }
}

log('Loading main.js...');
log('window.__TAURI__ exists?', !!window.__TAURI__);

// Canvas setup
const canvas = document.getElementById('main-canvas');
const gl = canvas.getContext('webgl2');

if (!gl) {
    alert('WebGL2 not supported');
}

// Resize canvas to window
function resizeCanvas() {
    canvas.width = window.innerWidth * 0.9;
    canvas.height = window.innerHeight * 0.9;
    gl.viewport(0, 0, canvas.width, canvas.height);
}

window.addEventListener('resize', resizeCanvas);
resizeCanvas();

// Basic render loop
function render() {
    gl.clearColor(0.1, 0.1, 0.1, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    requestAnimationFrame(render);
}

render();

// Audio button handling
const playButton = document.getElementById('play-button');
log('Play button element:', playButton);

if (!playButton) {
    logError('Could not find play-button element!');
} else {
    log('Play button found, setting up event listener...');
}

let isPlaying = false;

// Add multiple event listeners to debug
if (playButton) {
    // Test with a simple click first
    playButton.onclick = function() {
        log('onclick fired!');
    };

    playButton.addEventListener('mousedown', () => {
        log('mousedown event fired');
    });

    playButton.addEventListener('mouseup', () => {
        log('mouseup event fired');
    });
}

playButton?.addEventListener('click', async () => {
    log('addEventListener click fired!');
    log('Button clicked, isPlaying:', isPlaying);
    log('invoke function:', invoke);
    log('typeof invoke:', typeof invoke);

    if (!isPlaying) {
        try {
            log('Attempting to play audio...');
            const result = await invoke('play_audio');
            log('Play result:', result);
            // Don't change button text for now - may be causing reload
            // playButton.textContent = 'Stop Audio';
            isPlaying = true;
            log('Audio playing successfully');
        } catch (err) {
            logError('Error playing audio:', err);
            alert('Error playing audio: ' + JSON.stringify(err));
        }
    } else {
        try {
            log('Attempting to stop audio...');
            const result = await invoke('stop_audio');
            log('Stop result:', result);
            // Don't change button text for now - may be causing reload
            // playButton.textContent = 'Play Audio';
            isPlaying = false;
            log('Audio stopped successfully');
        } catch (err) {
            logError('Error stopping audio:', err);
            alert('Error stopping audio: ' + JSON.stringify(err));
        }
    }
});

// Log all elements to debug
log('All body children:', document.body.children.length);
for (let i = 0; i < document.body.children.length; i++) {
    const elem = document.body.children[i];
    log(`Child ${i}:`, elem.tagName, elem.id);
}

// Also check if elements are visible
if (playButton) {
    const rect = playButton.getBoundingClientRect();
    log('Play button position:', JSON.stringify({x: rect.x, y: rect.y, width: rect.width, height: rect.height}));
    log('Play button display:', window.getComputedStyle(playButton).display);
    log('Play button visibility:', window.getComputedStyle(playButton).visibility);
    log('Play button z-index:', window.getComputedStyle(playButton).zIndex);
}