# Peaks.js Interface Analysis

## Overview
Peaks.js provides a client-side JavaScript SDK for displaying and interacting with audio waveforms in web browsers. The library uses a canvas-based rendering approach with Konva.js and follows a modular, event-driven architecture.

## Core Architecture

```mermaid
classDiagram
    class Peaks {
        +options: PeaksOptions
        +init(options, callback): Peaks
        +setSource(options, callback)
        +destroy()
        +on(event, handler)
        +off(event, handler)
        +emit(event, data)
        +getWaveformData(): WaveformData
    }

    class Player {
        <<interface>>
        +init(eventEmitter)
        +destroy()
        +play(): Promise
        +pause()
        +seek(time)
        +isPlaying(): boolean
        +isSeeking(): boolean
        +getCurrentTime(): number
        +getDuration(): number
        +getVolume(): number
        +setVolume(volume)
        +playSegment(segment, loop)
    }

    class ViewController {
        -_peaks: Peaks
        -_views: Map
        +createZoomview(container): WaveformZoomView
        +createOverview(container): WaveformOverview
        +createScrollbar(container): Scrollbar
        +getView(name): WaveformView
        +destroyZoomview()
        +destroyOverview()
    }

    class WaveformView {
        <<abstract>>
        #_peaks: Peaks
        #_stage: Konva.Stage
        #_waveformLayer: WaveformLayer
        #_segmentsLayer: SegmentsLayer
        #_pointsLayer: PointsLayer
        +setAmplitudeScale(scale)
        +setWaveformColor(color)
        +showPlayheadTime(show)
        +enableAutoScroll(enable, options)
        +fitToContainer()
        +setTimeLabelPrecision(precision)
        +setStartTime(time)
        +setEndTime(time)
    }

    class WaveformZoomView {
        +setWheelMode(mode)
        +setPlayedWaveformColor(color)
        +enableSegmentDragging(enable)
        +setSegmentDragMode(mode)
        +enableMarkerEditing(enable)
    }

    class WaveformOverview {
        +setHighlightOffset(start, end)
        +setHighlightColor(color)
    }

    class WaveformSegments {
        -_peaks: Peaks
        -_segments: Array
        +add(segment): Segment
        +getSegments(): Array
        +getSegment(id): Segment
        +removeByTime(startTime, endTime)
        +removeById(id)
        +removeAll()
    }

    class Segment {
        +id: string
        +startTime: number
        +endTime: number
        +labelText: string
        +color: string
        +borderColor: string
        +editable: boolean
        +update(options)
        +destroy()
    }

    class WaveformPoints {
        -_peaks: Peaks
        -_points: Array
        +add(point): Point
        +getPoints(): Array
        +getPoint(id): Point
        +removeByTime(time)
        +removeById(id)
        +removeAll()
    }

    class Point {
        +id: string
        +time: number
        +labelText: string
        +color: string
        +editable: boolean
        +update(options)
        +destroy()
    }

    class ZoomController {
        -_peaks: Peaks
        -_zoomLevels: Array
        +setZoom(options)
        +zoomIn()
        +zoomOut()
        +getZoom(): number
    }

    class WaveformBuilder {
        <<static>>
        +init(peaks, callback)
        -_getAudioDecoder(): AudioDecoder
        -_buildWaveformDataFromAudio(options)
    }

    class MediaElementPlayer {
        -_mediaElement: HTMLMediaElement
        +init(eventEmitter)
        +play(): Promise
        +pause()
        +seek(time)
        +isPlaying(): boolean
        +getCurrentTime(): number
        +getDuration(): number
    }

    class EventEmitter {
        <<interface>>
        +on(event, listener): Function
        +off(event, listener?)
        +once(event, listener): Function
        +emit(event, ...args)
    }

    Peaks --|> EventEmitter : implements
    Peaks --> Player : uses
    Peaks --> ViewController : owns
    Peaks --> WaveformSegments : owns
    Peaks --> WaveformPoints : owns
    Peaks --> ZoomController : owns
    Peaks --> WaveformBuilder : uses
    ViewController --> WaveformView : creates
    WaveformView <|-- WaveformZoomView
    WaveformView <|-- WaveformOverview
    WaveformSegments --> Segment : manages
    WaveformPoints --> Point : manages
    MediaElementPlayer --|> Player : implements
```

## Event System

```mermaid
graph TB
    subgraph "Point Events"
        points.add[points.add]
        points.remove[points.remove]
        points.remove_all[points.remove_all]
        points.click[points.click]
        points.dblclick[points.dblclick]
        points.dragstart[points.dragstart]
        points.dragmove[points.dragmove]
        points.dragend[points.dragend]
        points.mouseenter[points.mouseenter]
        points.mouseleave[points.mouseleave]
        points.update[points.update]
    end

    subgraph "Segment Events"
        segments.add[segments.add]
        segments.remove[segments.remove]
        segments.remove_all[segments.remove_all]
        segments.click[segments.click]
        segments.dblclick[segments.dblclick]
        segments.contextmenu[segments.contextmenu]
        segments.dragstart[segments.dragstart]
        segments.dragged[segments.dragged]
        segments.dragend[segments.dragend]
        segments.mouseenter[segments.mouseenter]
        segments.mouseleave[segments.mouseleave]
        segments.mousedown[segments.mousedown]
        segments.mouseup[segments.mouseup]
        segments.update[segments.update]
        segments.insert[segments.insert]
        segments.enter[segments.enter]
        segments.exit[segments.exit]
    end

    subgraph "View Events"
        zoomview.click[zoomview.click]
        zoomview.dblclick[zoomview.dblclick]
        zoomview.contextmenu[zoomview.contextmenu]
        overview.click[overview.click]
        overview.dblclick[overview.dblclick]
        overview.contextmenu[overview.contextmenu]
        zoom.update[zoom.update]
    end

    subgraph "Player Events"
        player.canplay[player.canplay]
        player.playing[player.playing]
        player.pause[player.pause]
        player.seeked[player.seeked]
        player.ended[player.ended]
        player.timeupdate[player.timeupdate]
    end

    subgraph "Lifecycle Events"
        peaks.ready[peaks.ready]
        cue.enter[cue.enter]
        cue.exit[cue.exit]
    end
```

## Rendering Architecture

```mermaid
classDiagram
    class WaveformLayer {
        -_view: WaveformView
        -_data: WaveformData
        +draw()
        +setWaveformColor(color)
        +setPlayedWaveformColor(color)
    }

    class SegmentsLayer {
        -_view: WaveformView
        -_segments: Array
        +draw()
        +addSegmentShape(segment)
        +removeSegmentShape(segment)
        +updateSegmentShape(segment)
        +enableSegmentDragging(enable)
        +setSegmentDragMode(mode)
    }

    class PointsLayer {
        -_view: WaveformView
        -_points: Array
        +draw()
        +addPointShape(point)
        +removePointShape(point)
        +updatePointShape(point)
        +enableMarkerEditing(enable)
    }

    class PlayheadLayer {
        -_view: WaveformView
        -_playheadColor: string
        -_playheadTextColor: string
        +draw()
        +updatePlayheadPosition(time)
        +showPlayheadTime(show)
    }

    class AxisLayer {
        -_view: WaveformView
        -_axisLabelColor: string
        -_axisGridlineColor: string
        +draw()
        +formatTime(time): string
        +setTimeLabelPrecision(precision)
    }

    class KonvaStage {
        <<external>>
        +add(layer)
        +draw()
        +setSize(width, height)
        +on(event, handler)
        +off(event, handler)
    }

    WaveformView --> WaveformLayer : owns
    WaveformView --> SegmentsLayer : owns
    WaveformView --> PointsLayer : owns
    WaveformView --> PlayheadLayer : owns
    WaveformView --> AxisLayer : owns
    WaveformView --> KonvaStage : uses
```

## Configuration Options

```mermaid
graph LR
    subgraph "Required Options"
        mediaElement[mediaElement: HTMLAudioElement/HTMLVideoElement]
        containers[containers: {zoomview?, overview?}]
    end

    subgraph "Data Source Options"
        dataUri[dataUri: string/object]
        waveformData[waveformData: object/ArrayBuffer]
        webAudio[webAudio: {audioContext, audioBuffer}]
    end

    subgraph "View Options"
        zoomLevels[zoomLevels: Array]
        waveformColor[waveformColor: string/object]
        playedWaveformColor[playedWaveformColor: string/object]
        playheadColor[playheadColor: string]
        showPlayheadTime[showPlayheadTime: boolean]
        axisLabelColor[axisLabelColor: string]
        axisGridlineColor[axisGridlineColor: string]
        fontFamily[fontFamily: string]
        fontSize[fontSize: number]
        fontStyle[fontStyle: string]
        formatPlayheadTime[formatPlayheadTime: Function]
        formatAxisTime[formatAxisTime: Function]
    end

    subgraph "Interaction Options"
        keyboard[keyboard: boolean]
        nudgeIncrement[nudgeIncrement: number]
        emitCueEvents[emitCueEvents: boolean]
        wheelMode[wheelMode: none/scroll/zoom]
        autoScroll[autoScroll: boolean]
        autoScrollOffset[autoScrollOffset: number]
    end

    subgraph "Marker Options"
        createSegmentMarker[createSegmentMarker: Function]
        createSegmentLabel[createSegmentLabel: Function]
        createPointMarker[createPointMarker: Function]
        segmentOptions[segmentOptions: object]
        pointOptions[pointOptions: object]
    end

    subgraph "Player Options"
        player[player: object]
        timeLabelPrecision[timeLabelPrecision: number]
    end
```

## Public API Methods

### Initialization
- `Peaks.init(options, callback)` - Create Peaks instance
- `setSource(options, callback)` - Change audio source
- `destroy()` - Clean up and destroy instance

### Player Control
- `peaks.player.play()` - Start playback
- `peaks.player.pause()` - Pause playback
- `peaks.player.seek(time)` - Jump to time
- `peaks.player.getCurrentTime()` - Get current time
- `peaks.player.getDuration()` - Get duration
- `peaks.player.playSegment(segment, loop)` - Play segment

### Segments API
- `peaks.segments.add(options)` - Add segment
- `peaks.segments.getSegments()` - Get all segments
- `peaks.segments.getSegment(id)` - Get segment by ID
- `peaks.segments.removeByTime(start, end)` - Remove in range
- `peaks.segments.removeById(id)` - Remove by ID
- `peaks.segments.removeAll()` - Clear all segments

### Points API
- `peaks.points.add(options)` - Add point
- `peaks.points.getPoints()` - Get all points
- `peaks.points.getPoint(id)` - Get point by ID
- `peaks.points.removeByTime(time)` - Remove at time
- `peaks.points.removeById(id)` - Remove by ID
- `peaks.points.removeAll()` - Clear all points

### View Management
- `peaks.views.createZoomview(container)` - Create zoom view
- `peaks.views.createOverview(container)` - Create overview
- `peaks.views.destroyZoomview()` - Remove zoom view
- `peaks.views.destroyOverview()` - Remove overview
- `peaks.views.getView(name)` - Get view by name

### Zoom Control
- `peaks.zoom.zoomIn()` - Increase zoom
- `peaks.zoom.zoomOut()` - Decrease zoom
- `peaks.zoom.setZoom(options)` - Set zoom level
- `peaks.zoom.getZoom()` - Get current zoom

### Event Handling
- `peaks.on(event, handler)` - Add event listener
- `peaks.off(event, handler)` - Remove listener
- `peaks.once(event, handler)` - One-time listener
- `peaks.emit(event, ...args)` - Emit event

## Marker Factories

```javascript
// Custom segment marker factory
function createSegmentMarker(options) {
    // options: { layer, view, segment, startMarker, endMarker }
    return new Konva.Shape({
        // Custom shape configuration
    });
}

// Custom segment label factory
function createSegmentLabel(options) {
    // options: { layer, view, segment, text, fontSize, fontFamily }
    return new Konva.Text({
        // Custom text configuration
    });
}

// Custom point marker factory
function createPointMarker(options) {
    // options: { layer, view, point, draggable }
    return new Konva.Shape({
        // Custom shape configuration
    });
}
```

## Player Adapter Interface

```javascript
// Custom player adapter
const customPlayer = {
    init: function(eventEmitter) {
        // Initialize player
    },
    destroy: function() {
        // Cleanup
    },
    play: function() {
        // Start playback
        return Promise.resolve();
    },
    pause: function() {
        // Pause playback
    },
    seek: function(time) {
        // Seek to time
    },
    isPlaying: function() {
        // Return play state
        return false;
    },
    isSeeking: function() {
        // Return seeking state
        return false;
    },
    getCurrentTime: function() {
        // Return current time
        return 0;
    },
    getDuration: function() {
        // Return duration
        return 0;
    }
};
```

## Usage Example

```javascript
// Initialize Peaks.js
const options = {
    containers: {
        zoomview: document.getElementById('zoomview-container'),
        overview: document.getElementById('overview-container')
    },
    mediaElement: document.getElementById('audio'),
    dataUri: {
        arraybuffer: 'audio.dat',
        json: 'audio.json'
    },
    zoomLevels: [256, 512, 1024, 2048, 4096],
    keyboard: true,
    pointMarkerColor: '#006EB0',
    showPlayheadTime: true
};

Peaks.init(options, function(err, peaks) {
    if (err) {
        console.error('Failed to initialize Peaks.js', err);
        return;
    }

    // Add a segment
    peaks.segments.add({
        startTime: 10,
        endTime: 20,
        labelText: 'Test Segment',
        color: '#ff0000',
        editable: true
    });

    // Add a point
    peaks.points.add({
        time: 15,
        labelText: 'Test Point',
        color: '#00ff00'
    });

    // Listen for segment click
    peaks.on('segments.click', function(event) {
        console.log('Segment clicked:', event.segment);
    });

    // Control playback
    peaks.player.play();

    // Zoom operations
    peaks.zoom.zoomIn();
});
```

## Key Design Patterns

1. **Factory Pattern**: Static `Peaks.init()` method for instance creation
2. **Adapter Pattern**: Player abstraction allows different media players
3. **Observer Pattern**: Comprehensive event system for all interactions
4. **Strategy Pattern**: Multiple waveform data sources (precomputed, Web Audio)
5. **Composite Pattern**: Layered canvas rendering with Konva.js
6. **Command Pattern**: User interactions mapped to API methods
7. **Module Pattern**: Clear separation between core modules

## Technical Highlights

- **Canvas-based Rendering**: High-performance visualization using Konva.js
- **Modular Architecture**: Clear separation of concerns between data, views, and interaction
- **Flexible Data Sources**: Support for precomputed data or runtime generation
- **Extensible Markers**: Factory functions for custom segment and point rendering
- **Keyboard Support**: Optional keyboard navigation and shortcuts
- **Responsive Design**: Views can fit to container and handle resize events
- **Time-based Cues**: Support for triggering events at specific times during playback
