# WaveSurfer.js Interface Analysis

## Overview
WaveSurfer.js provides a comprehensive JavaScript interface for audio waveform visualization and interaction. The library follows an object-oriented architecture with event-driven communication patterns.

## Core Architecture

```mermaid
classDiagram
    class WaveSurfer {
        +options: WaveSurferOptions
        +create(options): WaveSurfer
        +load(url, peaks?, duration?)
        +loadBlob(blob, peaks?, duration?)
        +play(start?, end?)
        +pause()
        +playPause()
        +stop()
        +setTime(time)
        +seekTo(progress)
        +skip(seconds)
        +zoom(minPxPerSec)
        +setVolume(volume)
        +setMuted(muted)
        +setPlaybackRate(rate)
        +getDuration(): number
        +getCurrentTime(): number
        +getDecodedData(): AudioBuffer
        +exportPeaks(): Array
        +exportImage(format, quality, type)
        +registerPlugin(plugin): Plugin
        +unregisterPlugin(plugin)
        +destroy()
    }

    class Player {
        <<abstract>>
        #media: HTMLMediaElement
        +play(): Promise
        +pause()
        +isPlaying(): boolean
        +setTime(time)
        +getDuration(): number
        +getCurrentTime(): number
        +getVolume(): number
        +setVolume(volume)
        +getMuted(): boolean
        +setMuted(muted)
        +getPlaybackRate(): number
        +setPlaybackRate(rate)
        +setSinkId(sinkId): Promise
    }

    class EventEmitter {
        <<abstract>>
        +on(event, listener): Function
        +once(event, listener): Function
        +un(event, listener?)
        +unAll()
        +emit(event, ...args)
    }

    class Renderer {
        -options: RendererOptions
        -audioElement: HTMLMediaElement
        +render(audioBuffer)
        +renderProgress(progress, isPlaying)
        +zoom(minPxPerSec)
        +setOptions(options)
        +getWrapper(): HTMLElement
        +getScroll(): number
        +setScroll(pixels)
        +exportImage(): Promise
        +destroy()
    }

    class BasePlugin {
        <<abstract>>
        #wavesurfer: WaveSurfer
        #subscriptions: Array
        #options: Options
        +_init(wavesurfer)
        +onInit()
        +destroy()
    }

    class Decoder {
        <<static>>
        +decode(arrayBuffer, sampleRate): Promise
        +createBuffer(channelData, duration): AudioBuffer
    }

    class WebAudioPlayer {
        +duration: number
        +playbackRate: number
        +currentTime: number
        +volume: number
        +stopAt(time)
        +play(): Promise
        +pause()
    }

    class Timer {
        +start()
        +stop()
        +destroy()
        +on(event, callback)
    }

    class Fetcher {
        <<static>>
        +fetchBlob(url, onProgress, fetchParams): Promise
    }

    WaveSurfer --|> Player : extends
    Player --|> EventEmitter : extends
    BasePlugin --|> EventEmitter : extends
    WaveSurfer --> Renderer : uses
    WaveSurfer --> Timer : uses
    WaveSurfer --> Decoder : uses
    WaveSurfer --> Fetcher : uses
    WaveSurfer --> BasePlugin : manages
    WaveSurfer --> WebAudioPlayer : optional backend
```

## Event System

```mermaid
graph TB
    subgraph "Lifecycle Events"
        init[init]
        load[load]
        loading[loading %]
        decode[decode]
        ready[ready]
        destroy[destroy]
        error[error]
    end

    subgraph "Playback Events"
        play[play]
        pause[pause]
        finish[finish]
        timeupdate[timeupdate]
        audioprocess[audioprocess]
        seeking[seeking]
    end

    subgraph "Interaction Events"
        click[click]
        dblclick[dblclick]
        interaction[interaction]
        drag[drag]
        dragstart[dragstart]
        dragend[dragend]
    end

    subgraph "View Events"
        redraw[redraw]
        redrawcomplete[redrawcomplete]
        scroll[scroll]
        zoom[zoom]
    end
```

## Plugin Architecture

```mermaid
classDiagram
    class RegionsPlugin {
        +addRegion(params): Region
        +getRegions(): Region[]
        +clearRegions()
        +enableDragSelection()
    }

    class Region {
        +id: string
        +start: number
        +end: number
        +drag: boolean
        +resize: boolean
        +color: string
        +content?: HTMLElement
        +play()
        +remove()
        +update(params)
        +setContent(content)
    }

    class TimelinePlugin {
        +options: TimelineOptions
        +formatTime(seconds): string
    }

    class MinimapPlugin {
        +options: MinimapOptions
    }

    class SpectrogramPlugin {
        +options: SpectrogramOptions
    }

    class RecordPlugin {
        +startRecording(): Promise
        +stopRecording(): Promise
        +pauseRecording(): Promise
        +resumeRecording(): Promise
    }

    class ZoomPlugin {
        +options: ZoomPluginOptions
    }

    BasePlugin <|-- RegionsPlugin
    BasePlugin <|-- TimelinePlugin
    BasePlugin <|-- MinimapPlugin
    BasePlugin <|-- SpectrogramPlugin
    BasePlugin <|-- RecordPlugin
    BasePlugin <|-- ZoomPlugin
    BasePlugin <|-- HoverPlugin
    BasePlugin <|-- EnvelopePlugin

    RegionsPlugin --> Region : manages
```

## Configuration Options

```mermaid
graph LR
    subgraph "Required Options"
        container[container: HTMLElement/string]
    end

    subgraph "Audio Options"
        url[url: string]
        peaks[peaks: Array]
        duration[duration: number]
        media[media: HTMLMediaElement]
        backend[backend: WebAudio/MediaElement]
        audioRate[audioRate: number]
        autoplay[autoplay: boolean]
    end

    subgraph "Visual Options"
        height[height: number/auto]
        width[width: number/string]
        waveColor[waveColor: string/gradient]
        progressColor[progressColor: string/gradient]
        cursorColor[cursorColor: string]
        cursorWidth[cursorWidth: number]
        barWidth[barWidth: number]
        barGap[barGap: number]
        barRadius[barRadius: number]
        barHeight[barHeight: number]
        barAlign[barAlign: top/bottom]
        normalize[normalize: boolean]
        minPxPerSec[minPxPerSec: number]
        fillParent[fillParent: boolean]
    end

    subgraph "Interaction Options"
        interact[interact: boolean]
        dragToSeek[dragToSeek: boolean/object]
        hideScrollbar[hideScrollbar: boolean]
        autoScroll[autoScroll: boolean]
        autoCenter[autoCenter: boolean]
    end

    subgraph "Advanced Options"
        splitChannels[splitChannels: Array]
        plugins[plugins: Array]
        renderFunction[renderFunction: Function]
        sampleRate[sampleRate: number]
        fetchParams[fetchParams: RequestInit]
    end
```

## Public API Methods

### Audio Control
- `load(url, peaks?, duration?)` - Load audio from URL
- `loadBlob(blob, peaks?, duration?)` - Load audio from Blob
- `play(start?, end?)` - Start playback
- `pause()` - Pause playback
- `playPause()` - Toggle play/pause
- `stop()` - Stop and reset to beginning
- `setTime(seconds)` - Jump to specific time
- `seekTo(progress)` - Seek to position (0-1)
- `skip(seconds)` - Skip forward/backward

### Audio Properties
- `getDuration()` - Get total duration
- `getCurrentTime()` - Get current position
- `getVolume()` - Get volume level
- `setVolume(volume)` - Set volume (0-1)
- `getMuted()` - Get mute state
- `setMuted(muted)` - Set mute state
- `getPlaybackRate()` - Get playback speed
- `setPlaybackRate(rate, preservePitch?)` - Set playback speed

### Visualization
- `zoom(minPxPerSec)` - Change zoom level
- `getDecodedData()` - Get decoded AudioBuffer
- `exportPeaks(options)` - Export waveform peaks
- `exportImage(format, quality, type)` - Export as image

### Plugin Management
- `registerPlugin(plugin)` - Add plugin
- `unregisterPlugin(plugin)` - Remove plugin

### Lifecycle
- `destroy()` - Clean up and destroy instance
- `empty()` - Clear waveform

## Usage Example

```javascript
// Create instance
const wavesurfer = WaveSurfer.create({
    container: '#waveform',
    waveColor: '#A8DBA8',
    progressColor: '#3B8686',
    backend: 'WebAudio',
    plugins: [
        RegionsPlugin.create(),
        TimelinePlugin.create({
            container: '#timeline'
        })
    ]
});

// Load and play audio
wavesurfer.load('audio.mp3');

// Event handling
wavesurfer.on('ready', () => {
    wavesurfer.play();
});

wavesurfer.on('region-created', (region) => {
    console.log('Region created', region);
});

// Control playback
wavesurfer.on('finish', () => {
    wavesurfer.seekTo(0);
});
```

## Key Design Patterns

1. **Event-Driven Architecture**: All components communicate through events
2. **Plugin System**: Extensible through standardized plugin interface
3. **Inheritance Hierarchy**: Core functionality inherited from Player and EventEmitter
4. **Separation of Concerns**: Rendering, audio control, and interaction handled by separate modules
5. **Promise-based APIs**: Asynchronous operations return promises
6. **Configuration over Code**: Extensive options for customization without code changes
